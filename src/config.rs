//! Client configuration.

use url::Url;
use crate::Auth;

/// Configuration for creating a [`WhatsAppClient`](crate::WhatsAppClient).
///
/// Holds the base URL of your Infobip API endpoint and authentication credentials.
///
/// # Examples
///
/// ```rust
/// use infobip_whatsapp_sdk::{Auth, ClientConfig, WhatsAppClient};
///
/// let config = ClientConfig::new(
///     "https://your-base-url.api.infobip.com",
///     Auth::ApiKey("your-api-key".into()),
/// ).expect("invalid base URL");
///
/// let client = WhatsAppClient::new(config);
/// ```
///
/// Your base URL is shown in the Infobip dashboard. It typically looks like
/// `https://xxxxx.api.infobip.com`.
pub struct ClientConfig {
    /// The Infobip API base URL (e.g. `https://xxxxx.api.infobip.com`).
    pub base_url: Url,
    /// Authentication credentials.
    pub auth: Auth,
}

impl ClientConfig {
    /// Create a new client configuration.
    ///
    /// # Errors
    ///
    /// Returns [`InfobipError::Url`](crate::InfobipError::Url) if `base_url` is not a valid URL.
    pub fn new(base_url: &str, auth: Auth) -> crate::error::Result<Self> {
        let base_url = Url::parse(base_url)?;
        Ok(Self { base_url, auth })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_url_creates_config() {
        let config = ClientConfig::new("https://api.infobip.com", Auth::ApiKey("key".into()));
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.base_url.as_str(), "https://api.infobip.com/");
    }

    #[test]
    fn invalid_url_returns_error() {
        let config = ClientConfig::new("not a url", Auth::ApiKey("key".into()));
        assert!(config.is_err());
    }

    #[test]
    fn trailing_slash_preserved() {
        let config = ClientConfig::new("https://api.infobip.com/", Auth::ApiKey("k".into())).unwrap();
        assert_eq!(config.base_url.as_str(), "https://api.infobip.com/");
    }
}
