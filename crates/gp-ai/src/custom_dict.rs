use crate::{AiError, http};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct WordPayload<'a> {
    pub word: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct CustomDictClient {
    url: String,
    token: Option<String>,
}

impl CustomDictClient {
    pub fn new(url: &str, token: Option<&str>) -> Self {
        CustomDictClient {
            url: url.trim().to_owned(),
            token: token.map(|t| t.trim().to_owned()).filter(|t| !t.is_empty()),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn is_configured(&self) -> bool {
        !self.url.is_empty()
    }

    fn auth_header(&self) -> Option<String> {
        self.token.as_ref().map(|t| format!("Bearer {t}"))
    }

    /// Fast reachability check.
    pub fn probe(&self) -> Result<u16, AiError> {
        let auth = self.auth_header();
        let mut headers = Vec::new();
        if let Some(ref bearer) = auth {
            headers.push(("Authorization", bearer.as_str()));
        }
        http::probe_endpoint(&self.url, &headers)
    }

    /// Sends a word, optional context and translation to the custom dictionary endpoint.
    pub fn send_word(
        &self,
        word: &str,
        context: Option<&str>,
        translation: Option<&str>,
    ) -> Result<(), AiError> {
        let payload = WordPayload {
            word,
            context,
            translation,
        };
        let body: Value =
            serde_json::to_value(&payload).map_err(|e| AiError::BadResponse(e.to_string()))?;
        let auth = self.auth_header();
        let mut headers = Vec::new();
        if let Some(ref bearer) = auth {
            headers.push(("Authorization", bearer.as_str()));
        }
        http::post_json_empty(&self.url, &headers, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testserver::TestServer;

    #[test]
    fn send_word_ac2_full_payload_with_token() {
        let srv = TestServer::start(vec![(200, "".into())]);
        let client = CustomDictClient::new(&srv.url, Some("test-token"));
        client
            .send_word("apple", Some("An apple a day"), Some("яблоко"))
            .unwrap();

        let got = srv.received.lock().unwrap();
        assert_eq!(got.len(), 1);
        assert!(
            got[0]
                .headers
                .contains("authorization: Bearer test-token\r\n")
                || got[0]
                    .headers
                    .to_lowercase()
                    .contains("authorization: bearer test-token")
        );

        let body: Value = serde_json::from_str(&got[0].body).unwrap();
        assert_eq!(body["word"], "apple");
        assert_eq!(body["context"], "An apple a day");
        assert_eq!(body["translation"], "яблоко");
    }

    #[test]
    fn send_word_accepts_204_and_empty_responses() {
        let srv = TestServer::start(vec![(204, "".into())]);
        let client = CustomDictClient::new(&srv.url, None);
        assert!(client.send_word("cat", None, None).is_ok());

        let got = srv.received.lock().unwrap();
        let body: Value = serde_json::from_str(&got[0].body).unwrap();
        assert_eq!(body["word"], "cat");
        assert!(body.get("context").is_none());
        assert!(body.get("translation").is_none());
    }

    #[test]
    fn probe_returns_status() {
        let srv = TestServer::start(vec![(200, "ok".into())]);
        let client = CustomDictClient::new(&srv.url, Some("secret"));
        assert_eq!(client.probe().unwrap(), 200);
    }
}
