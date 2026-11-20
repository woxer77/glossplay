use crate::AiError;
use serde_json::Value;
#[cfg(test)]
use serde_json::json;
use std::sync::OnceLock;
use std::time::Duration;

const BODY_SNIPPET: usize = 300;
// Without a connect timeout a dead or filtered host blocks for the OS limit (about 21 s on
// Windows) or, failing that, for the whole 120 s global budget.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

fn agent() -> &'static ureq::Agent {
    static AGENT: OnceLock<ureq::Agent> = OnceLock::new();
    AGENT.get_or_init(|| {
        ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(120)))
            .timeout_connect(Some(CONNECT_TIMEOUT))
            .http_status_as_error(false)
            // API calls carry keys in custom headers (x-api-key, DeepL-Auth-Key) that ureq would
            // forward to another host on a redirect; none of these APIs redirect.
            .max_redirects(0)
            .build()
            .into()
    })
}

const MAX_DOWNLOAD_REDIRECTS: u32 = 5;

fn finish(
    result: Result<ureq::http::Response<ureq::Body>, ureq::Error>,
) -> Result<String, AiError> {
    let mut resp = result.map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    let body = resp
        .body_mut()
        .read_to_string()
        .map_err(|e| AiError::Network(e.to_string()))?;
    if (200..300).contains(&status) {
        Ok(body)
    } else {
        Err(AiError::Http {
            status,
            body: body.chars().take(BODY_SNIPPET).collect(),
        })
    }
}

pub fn post_json(url: &str, headers: &[(&str, &str)], body: &Value) -> Result<Value, AiError> {
    let mut req = agent().post(url);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    parse(&finish(req.send_json(body))?)
}

/// A JSON request whose answer may be empty or not JSON at all.
pub fn post_json_text(url: &str, body: &Value) -> Result<String, AiError> {
    finish(agent().post(url).send_json(body))
}

/// A JSON request with custom headers that accepts any 2xx response without requiring JSON.
pub fn post_json_empty(url: &str, headers: &[(&str, &str)], body: &Value) -> Result<(), AiError> {
    let mut req = agent().post(url);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    let mut resp = req
        .send_json(body)
        .map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    if (200..300).contains(&status) {
        Ok(())
    } else {
        let body = resp
            .body_mut()
            .with_config()
            .limit(64 * 1024)
            .read_to_string()
            .unwrap_or_default();
        Err(AiError::Http {
            status,
            body: body.chars().take(BODY_SNIPPET).collect(),
        })
    }
}

/// Lightweight check of an endpoint; accepts 2xx and 405 (Method Not Allowed for POST-only APIs).
pub fn probe_endpoint(url: &str, headers: &[(&str, &str)]) -> Result<u16, AiError> {
    let mut req = agent().get(url);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    let mut resp = req.call().map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    if (200..300).contains(&status) || status == 405 {
        Ok(status)
    } else {
        let body = resp.body_mut().read_to_string().unwrap_or_default();
        Err(AiError::Http {
            status,
            body: body.chars().take(BODY_SNIPPET).collect(),
        })
    }
}

pub fn post_form(url: &str, form: &[(&str, &str)]) -> Result<Value, AiError> {
    parse(&finish(agent().post(url).send_form(form.iter().copied()))?)
}

pub fn post_bytes(url: &str, content_type: &str, body: &[u8]) -> Result<Value, AiError> {
    parse(&finish(
        agent()
            .post(url)
            .header("Content-Type", content_type)
            .send(body),
    )?)
}

/// A JSON request whose answer is a file, such as synthesized speech.
pub fn post_json_for_bytes(url: &str, body: &Value) -> Result<Vec<u8>, AiError> {
    let mut resp = agent()
        .post(url)
        .send_json(body)
        .map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    let bytes = resp
        .body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| AiError::Network(e.to_string()))?;
    if (200..300).contains(&status) {
        Ok(bytes)
    } else {
        Err(AiError::Http {
            status,
            body: String::from_utf8_lossy(&bytes)
                .chars()
                .take(BODY_SNIPPET)
                .collect(),
        })
    }
}

pub fn get(url: &str) -> Result<String, AiError> {
    finish(agent().get(url).call())
}

fn parse(body: &str) -> Result<Value, AiError> {
    serde_json::from_str(body).map_err(|e| {
        AiError::BadResponse(format!(
            "{e}: {}",
            body.chars().take(BODY_SNIPPET).collect::<String>()
        ))
    })
}

pub fn get_bytes(url: &str) -> Result<Vec<u8>, AiError> {
    let mut resp = agent()
        .get(url)
        .config()
        .max_redirects(MAX_DOWNLOAD_REDIRECTS)
        // An https URL must not be bounced to plain http; an explicit http URL stays allowed.
        .https_only(url.starts_with("https://"))
        .build()
        .call()
        .map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    if !(200..300).contains(&status) {
        return Err(AiError::Http {
            status,
            body: String::new(),
        });
    }
    // The default cap is 10 MB; yt-dlp alone is about 18.
    resp.body_mut()
        .with_config()
        .limit(256 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| AiError::Network(e.to_string()))
}

/// A GET with query parameters whose answer is read whatever its status: some servers
/// explain an error in the body of a 4xx reply.
pub fn get_with_query(url: &str, query: &[(&str, &str)]) -> Result<(u16, String), AiError> {
    let mut resp = agent()
        .get(url)
        .query_pairs(query.iter().copied())
        .call()
        .map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    let body = resp
        .body_mut()
        .with_config()
        .limit(32 * 1024 * 1024)
        .read_to_string()
        .map_err(|e| AiError::Network(e.to_string()))?;
    Ok((status, body))
}

/// The answer to one GET that was not allowed to redirect.
pub struct Single {
    pub status: u16,
    pub location: Option<String>,
    pub body: Vec<u8>,
}

/// One GET without following a redirect; a 3xx reply comes back with its `Location`.
pub fn get_single(url: &str) -> Result<Single, AiError> {
    static AGENT: OnceLock<ureq::Agent> = OnceLock::new();
    let agent = AGENT.get_or_init(|| {
        ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .timeout_connect(Some(CONNECT_TIMEOUT))
            .http_status_as_error(false)
            .max_redirects(0)
            .build()
            .into()
    });
    let mut resp = agent
        .get(url)
        .call()
        .map_err(|e| AiError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    let location = resp
        .headers()
        .get("location")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let body = if (300..400).contains(&status) {
        Vec::new()
    } else {
        resp.body_mut()
            .with_config()
            .limit(16 * 1024 * 1024)
            .read_to_vec()
            .map_err(|e| AiError::Network(e.to_string()))?
    };
    Ok(Single {
        status,
        location,
        body,
    })
}

/// Returns the `Location` of a 3xx response without following it; `None` if there was no redirect.
pub fn redirect_target(url: &str) -> Result<Option<String>, AiError> {
    let reply = get_single(url)?;
    match reply.status {
        300..400 => Ok(reply.location),
        200..300 => Ok(None),
        status => Err(AiError::Http {
            status,
            body: String::new(),
        }),
    }
}

/// Uploads one file as `multipart/form-data`, with plain text fields next to it.
pub fn post_file(
    url: &str,
    field: &str,
    file_name: &str,
    content: &[u8],
    fields: &[(&str, &str)],
) -> Result<Value, AiError> {
    let boundary = "----glossplay-7d1c5e0a93b44f2e";
    let mut body = Vec::with_capacity(content.len() + 512);
    for (name, value) in fields {
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
            )
            .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{field}\"; filename=\"{}\"\r\nContent-Type: application/x-bittorrent\r\n\r\n",
            file_name.replace(['"', '\r', '\n'], "_")
        )
        .as_bytes(),
    );
    body.extend_from_slice(content);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    post_bytes(
        url,
        &format!("multipart/form-data; boundary={boundary}"),
        &body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testserver::TestServer;

    #[test]
    fn clients_have_a_connect_timeout_shorter_than_the_global_one() {
        let t = agent().config().timeouts();
        assert_eq!(t.connect, Some(CONNECT_TIMEOUT));
        assert!(t.global.unwrap() > CONNECT_TIMEOUT);
    }

    #[test]
    fn api_posts_do_not_follow_redirects() {
        let srv = TestServer::start_with_headers(
            vec![(302, "{}".into())],
            "Location: /elsewhere\r\n".into(),
        );
        let e = post_json(&srv.url, &[("x-api-key", "secret")], &json!({})).unwrap_err();
        assert!(matches!(e, AiError::Http { status: 302, .. }), "{e:?}");
        assert_eq!(srv.count(), 1);
    }

    #[test]
    fn downloads_follow_redirects() {
        let srv = TestServer::start_with_headers(
            vec![(302, "".into()), (200, "data".into())],
            "Location: /next\r\n".into(),
        );
        assert_eq!(get_bytes(&srv.url).unwrap(), b"data");
        assert_eq!(srv.count(), 2);
    }

    #[test]
    fn downloads_stop_after_five_redirects() {
        let srv =
            TestServer::start_with_headers(vec![(302, "".into())], "Location: /again\r\n".into());
        assert!(matches!(get_bytes(&srv.url), Err(AiError::Network(_))));
        assert_eq!(srv.count(), 6);
    }
}
