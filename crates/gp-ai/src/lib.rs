pub mod anki;
pub mod asr;
pub mod custom_dict;
pub mod download;
pub mod http;
pub mod torrserver;
pub mod torznab;
pub mod translate;
pub mod tts;

#[cfg(test)]
mod testserver;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiError {
    MissingKey {
        provider: String,
    },
    Http {
        status: u16,
        body: String,
    },
    Network(String),
    BadResponse(String),
    /// A setting is wrong; the text already says what to change.
    Config(String),
    Unsupported,
    Duplicate,
    /// The server turned the API key down.
    BadKey,
    /// The server understood the request and refused it; the text is its own explanation.
    Rejected(String),
    /// Waited as long as it made sense and got nothing.
    TimedOut,
}

impl AiError {
    /// Worth retrying after a delay: the request itself was fine.
    pub fn is_transient(&self) -> bool {
        match self {
            AiError::Network(_) => true,
            AiError::Http { status, .. } => *status == 429 || *status >= 500,
            _ => false,
        }
    }
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiError::MissingKey { provider } => write!(f, "{provider}: API key is not set"),
            AiError::Http { status, body } => write!(f, "HTTP {status}: {body}"),
            AiError::Network(e) => write!(f, "network error: {e}"),
            AiError::BadResponse(e) => write!(f, "unexpected response: {e}"),
            AiError::Config(e) => write!(f, "{e}"),
            AiError::Unsupported => write!(f, "this provider does not support the operation"),
            AiError::Duplicate => write!(f, "this note already exists"),
            AiError::BadKey => write!(f, "the API key was not accepted"),
            AiError::Rejected(e) => write!(f, "the server refused: {e}"),
            AiError::TimedOut => write!(f, "no answer in time"),
        }
    }
}

impl std::error::Error for AiError {}

/// Downloads a small file, such as a subtitle track of an online video.
pub fn fetch(url: &str) -> Result<Vec<u8>, AiError> {
    http::get_bytes(url)
}

/// Reads where a URL redirects to, without following it.
pub fn redirect_target(url: &str) -> Result<Option<String>, AiError> {
    http::redirect_target(url)
}
