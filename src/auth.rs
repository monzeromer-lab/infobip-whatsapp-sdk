//! Authentication methods for the Infobip API.
//!
//! Infobip supports four authentication mechanisms. Choose the one that
//! matches your account setup. See
//! [API Authentication](https://www.infobip.com/docs/essentials/api-authentication)
//! for details.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

/// Authentication credentials for Infobip API requests.
///
/// Every request to the Infobip API must be authenticated. Pass one of these
/// variants to [`ClientConfig::new`](crate::ClientConfig::new) when creating your client.
///
/// # Examples
///
/// ```rust
/// use infobip_sdk::Auth;
///
/// // Most common: API key from your Infobip dashboard
/// let auth = Auth::ApiKey("your-api-key-here".into());
///
/// // Basic HTTP auth
/// let auth = Auth::Basic {
///     username: "your-username".into(),
///     password: "your-password".into(),
/// };
///
/// // OAuth2 access token
/// let auth = Auth::Bearer("eyJhbGciOi...".into());
///
/// // Infobip SSO token
/// let auth = Auth::IbSso("sso-session-token".into());
/// ```
#[derive(Debug, Clone)]
pub enum Auth {
    /// API key authentication. Sends `Authorization: App {key}`.
    ///
    /// This is the most common method. Find your API key in the
    /// Infobip web dashboard under **Developer Tools > API Keys**.
    ApiKey(String),

    /// HTTP Basic authentication. Sends `Authorization: Basic {base64(user:pass)}`.
    Basic {
        /// Your Infobip account username.
        username: String,
        /// Your Infobip account password.
        password: String,
    },

    /// OAuth2 Bearer token. Sends `Authorization: Bearer {token}`.
    ///
    /// Obtain a token from the Infobip OAuth2 token endpoint first.
    Bearer(String),

    /// Infobip SSO token. Sends `Authorization: IBSSO {token}`.
    IbSso(String),
}

impl Auth {
    pub(crate) fn apply(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match self {
            Auth::ApiKey(key) => req.header("Authorization", format!("App {key}")),
            Auth::Basic { username, password } => {
                let encoded = STANDARD.encode(format!("{username}:{password}"));
                req.header("Authorization", format!("Basic {encoded}"))
            }
            Auth::Bearer(token) => req.bearer_auth(token),
            Auth::IbSso(token) => req.header("Authorization", format!("IBSSO {token}")),
        }
    }

    #[cfg(test)]
    pub(crate) fn header_value(&self) -> String {
        match self {
            Auth::ApiKey(key) => format!("App {key}"),
            Auth::Basic { username, password } => {
                let encoded = STANDARD.encode(format!("{username}:{password}"));
                format!("Basic {encoded}")
            }
            Auth::Bearer(token) => format!("Bearer {token}"),
            Auth::IbSso(token) => format!("IBSSO {token}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_header() {
        let auth = Auth::ApiKey("my-api-key".into());
        assert_eq!(auth.header_value(), "App my-api-key");
    }

    #[test]
    fn basic_auth_header() {
        let auth = Auth::Basic {
            username: "user".into(),
            password: "pass".into(),
        };
        let val = auth.header_value();
        assert!(val.starts_with("Basic "));
        let decoded = STANDARD.decode(val.strip_prefix("Basic ").unwrap()).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "user:pass");
    }

    #[test]
    fn bearer_header() {
        let auth = Auth::Bearer("tok123".into());
        assert_eq!(auth.header_value(), "Bearer tok123");
    }

    #[test]
    fn ibsso_header() {
        let auth = Auth::IbSso("sso-token".into());
        assert_eq!(auth.header_value(), "IBSSO sso-token");
    }

    #[test]
    fn clone_preserves_variant() {
        let auth = Auth::ApiKey("key".into());
        let cloned = auth.clone();
        assert_eq!(auth.header_value(), cloned.header_value());
    }
}
