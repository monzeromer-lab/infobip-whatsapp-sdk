//! Error types for the Infobip SDK.
//!
//! All API methods return [`Result<T, InfobipError>`]. The error type is designed
//! so SDK users can:
//!
//! - **Classify errors quickly** — `is_retryable()`, `is_rate_limited()`, `is_unauthorized()`
//! - **Extract details for logging** — `error_code()`, `message()`, `status()`
//! - **Forward to monitoring** — implements [`std::error::Error`] so it works with
//!   Sentry, `anyhow`, `eyre`, and any error-reporting crate
//! - **Access the raw API response** — drill into the two Infobip error formats
//!
//! # Error flow
//!
//! ```text
//! InfobipError
//! ├── Http(reqwest::Error)       — network/transport failures
//! ├── Json(serde_json::Error)    — serialization/deserialization bugs
//! ├── Url(url::ParseError)       — invalid base URL
//! └── Api(ApiError)              — Infobip returned a non-2xx response
//!      ├── status: u16           — HTTP status code
//!      └── kind: ApiErrorKind
//!           ├── Exception(...)   — { requestError: { serviceException: ... } }
//!           ├── Structured(...)  — { errorCode, description, violations, ... }
//!           └── Unknown(String)  — unparseable response body
//! ```
//!
//! # Examples
//!
//! ```rust,no_run
//! # use infobip_whatsapp_sdk::{Auth, ClientConfig, WhatsAppClient};
//! # use infobip_whatsapp_sdk::models::messages::text::TextMessage;
//! # async fn example() {
//! # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into())).unwrap();
//! # let client = WhatsAppClient::new(config);
//! # let msg = TextMessage::new("s", "r", "t");
//! match client.messages().send_text(&msg).await {
//!     Ok(info) => { /* success */ }
//!     Err(e) if e.is_retryable() => {
//!         // 429, 5xx, or network timeout — queue for retry
//!     }
//!     Err(e) => {
//!         if let Some(api) = e.api_error() {
//!             eprintln!("HTTP {} [{}]: {}",
//!                 api.status(),
//!                 api.error_code().unwrap_or("?"),
//!                 api.message().unwrap_or("unknown"),
//!             );
//!         }
//!     }
//! }
//! # }
//! ```

use crate::models::errors::{ApiErrorResponse, ApiExceptionResponse};
use std::fmt;

/// Alias for `Result<T, InfobipError>`.
pub type Result<T> = std::result::Result<T, InfobipError>;

/// Top-level error type returned by all SDK methods.
///
/// Implements [`std::error::Error`] with proper `source()` chaining, so it
/// integrates with `anyhow`, `eyre`, Sentry, and other error-handling crates.
#[derive(Debug)]
pub enum InfobipError {
    /// HTTP transport error (connection refused, timeout, DNS failure, etc.).
    Http(reqwest::Error),
    /// JSON serialization or deserialization error.
    Json(serde_json::Error),
    /// Invalid URL (returned from [`ClientConfig::new`](crate::ClientConfig::new)).
    Url(url::ParseError),
    /// The Infobip API returned a non-2xx HTTP response.
    Api(ApiError),
}

/// A parsed API error response from Infobip.
///
/// Contains the HTTP status code and the parsed error body. Use the
/// convenience methods to classify the error and extract details without
/// matching on the internal [`ApiErrorKind`].
///
/// # Examples
///
/// ```rust,no_run
/// # use infobip_whatsapp_sdk::ApiError;
/// # fn example(api_err: &ApiError) {
/// // Classification
/// assert!(api_err.is_client_error());  // 4xx
/// assert!(api_err.is_rate_limited());  // 429
/// assert!(api_err.is_retryable());     // 429 or 5xx
///
/// // Unified accessors (work across both Infobip error formats)
/// let code: Option<&str> = api_err.error_code();
/// let msg: Option<&str> = api_err.message();
/// let action: Option<&str> = api_err.action();
///
/// // Drill into the specific error format if needed
/// if let Some(exc) = api_err.as_exception() {
///     // requestError.serviceException format (messages, templates, etc.)
///     let validation_errors = &exc.request_error.as_ref()
///         .and_then(|r| r.service_exception.as_ref())
///         .and_then(|s| s.validation_errors.as_ref());
/// }
/// if let Some(err) = api_err.as_structured() {
///     // errorCode/violations format (flows, sender management, etc.)
///     for v in err.violations.iter().flatten() {
///         println!("{}: {}", v.property.as_deref().unwrap_or("?"),
///                            v.violation.as_deref().unwrap_or("?"));
///     }
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ApiError {
    status: u16,
    kind: ApiErrorKind,
}

/// The parsed body of an API error response.
///
/// Infobip uses two different error response formats depending on the endpoint:
///
/// - **Exception** — `{ "requestError": { "serviceException": { ... } } }` —
///   used by message sending, template management, and most v1/v2 endpoints.
/// - **Structured** — `{ "errorCode": "...", "description": "...", "violations": [...] }` —
///   used by flow management, sender management, and newer endpoints.
/// - **Unknown** — the raw response body when neither format could be parsed.
#[derive(Debug, Clone)]
pub enum ApiErrorKind {
    /// The `requestError.serviceException` format.
    Exception(ApiExceptionResponse),
    /// The `errorCode`/`description`/`violations` format.
    Structured(ApiErrorResponse),
    /// Unparseable response body.
    Unknown(String),
}

impl ApiError {
    pub(crate) fn from_status_and_body(status: u16, body: &str) -> Self {
        let kind = if let Ok(exc) = serde_json::from_str::<ApiExceptionResponse>(body) {
            if exc.request_error.is_some() {
                ApiErrorKind::Exception(exc)
            } else if let Ok(structured) = serde_json::from_str::<ApiErrorResponse>(body) {
                if structured.error_code.is_some() {
                    ApiErrorKind::Structured(structured)
                } else {
                    ApiErrorKind::Exception(exc)
                }
            } else {
                ApiErrorKind::Exception(exc)
            }
        } else if let Ok(structured) = serde_json::from_str::<ApiErrorResponse>(body) {
            ApiErrorKind::Structured(structured)
        } else {
            ApiErrorKind::Unknown(body.to_string())
        };
        Self { status, kind }
    }

    /// The HTTP status code (e.g. 400, 401, 429, 500).
    pub fn status(&self) -> u16 {
        self.status
    }

    /// The parsed error body variant.
    pub fn kind(&self) -> &ApiErrorKind {
        &self.kind
    }

    /// `true` if the status code is in the 4xx range.
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    /// `true` if the status code is 500 or above.
    pub fn is_server_error(&self) -> bool {
        self.status >= 500
    }

    /// `true` if the status code is 429 (Too Many Requests).
    pub fn is_rate_limited(&self) -> bool {
        self.status == 429
    }

    /// `true` if the status code is 401 (Unauthorized).
    pub fn is_unauthorized(&self) -> bool {
        self.status == 401
    }

    /// `true` if the status code is 403 (Forbidden).
    pub fn is_forbidden(&self) -> bool {
        self.status == 403
    }

    /// `true` if the status code is 404 (Not Found).
    pub fn is_not_found(&self) -> bool {
        self.status == 404
    }

    /// `true` if this error is safe to retry (429 or 5xx).
    pub fn is_retryable(&self) -> bool {
        self.status == 429 || self.status >= 500
    }

    /// The error code from the API response, regardless of format.
    ///
    /// - For exception format: returns `serviceException.messageId` (e.g. `"BAD_REQUEST"`)
    /// - For structured format: returns `errorCode` (e.g. `"40001"`)
    pub fn error_code(&self) -> Option<&str> {
        match &self.kind {
            ApiErrorKind::Structured(e) => e.error_code.as_deref(),
            ApiErrorKind::Exception(e) => e
                .request_error
                .as_ref()
                .and_then(|r| r.service_exception.as_ref())
                .and_then(|s| s.message_id.as_deref()),
            ApiErrorKind::Unknown(_) => None,
        }
    }

    /// The human-readable error message, regardless of format.
    ///
    /// - For exception format: returns `serviceException.text`
    /// - For structured format: returns `description`
    /// - For unknown format: returns the raw response body
    pub fn message(&self) -> Option<&str> {
        match &self.kind {
            ApiErrorKind::Structured(e) => e.description.as_deref(),
            ApiErrorKind::Exception(e) => e
                .request_error
                .as_ref()
                .and_then(|r| r.service_exception.as_ref())
                .and_then(|s| s.text.as_deref()),
            ApiErrorKind::Unknown(text) => Some(text.as_str()),
        }
    }

    /// The suggested action to fix the error (structured format only).
    ///
    /// Example: `"Check the syntax, violations and adjust the request."`
    pub fn action(&self) -> Option<&str> {
        match &self.kind {
            ApiErrorKind::Structured(e) => e.action.as_deref(),
            _ => None,
        }
    }

    /// Access the full exception response body (`requestError.serviceException` format).
    ///
    /// Returns `None` if the error was in structured or unknown format.
    pub fn as_exception(&self) -> Option<&ApiExceptionResponse> {
        match &self.kind {
            ApiErrorKind::Exception(e) => Some(e),
            _ => None,
        }
    }

    /// Access the full structured error body (`errorCode`/`violations` format).
    ///
    /// Returns `None` if the error was in exception or unknown format.
    pub fn as_structured(&self) -> Option<&ApiErrorResponse> {
        match &self.kind {
            ApiErrorKind::Structured(e) => Some(e),
            _ => None,
        }
    }

    /// The raw response body, if neither API error format could be parsed.
    pub fn raw_body(&self) -> Option<&str> {
        match &self.kind {
            ApiErrorKind::Unknown(text) => Some(text.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}", self.status)?;
        if let Some(code) = self.error_code() {
            write!(f, " [{code}]")?;
        }
        if let Some(msg) = self.message() {
            write!(f, ": {msg}")?;
        }
        Ok(())
    }
}

impl fmt::Display for InfobipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfobipError::Http(e) => write!(f, "HTTP transport error: {e}"),
            InfobipError::Json(e) => write!(f, "JSON serialization error: {e}"),
            InfobipError::Url(e) => write!(f, "URL parse error: {e}"),
            InfobipError::Api(e) => write!(f, "Infobip API error: {e}"),
        }
    }
}

impl std::error::Error for InfobipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InfobipError::Http(e) => Some(e),
            InfobipError::Json(e) => Some(e),
            InfobipError::Url(e) => Some(e),
            InfobipError::Api(_) => None,
        }
    }
}

impl From<reqwest::Error> for InfobipError {
    fn from(e: reqwest::Error) -> Self {
        InfobipError::Http(e)
    }
}

impl From<serde_json::Error> for InfobipError {
    fn from(e: serde_json::Error) -> Self {
        InfobipError::Json(e)
    }
}

impl From<url::ParseError> for InfobipError {
    fn from(e: url::ParseError) -> Self {
        InfobipError::Url(e)
    }
}

impl InfobipError {
    /// `true` if this is an API error (non-2xx response from Infobip).
    pub fn is_api_error(&self) -> bool {
        matches!(self, InfobipError::Api(_))
    }

    /// Returns the [`ApiError`] if this is an API error, `None` otherwise.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            InfobipError::Api(e) => Some(e),
            _ => None,
        }
    }

    /// The HTTP status code, if this is an API error.
    pub fn status(&self) -> Option<u16> {
        self.api_error().map(|e| e.status())
    }

    /// `true` if this error is safe to retry.
    ///
    /// Returns `true` for:
    /// - API errors with status 429 (rate limited) or 5xx (server error)
    /// - HTTP transport errors caused by timeouts or connection failures
    pub fn is_retryable(&self) -> bool {
        match self {
            InfobipError::Api(e) => e.is_retryable(),
            InfobipError::Http(e) => e.is_timeout() || e.is_connect(),
            _ => false,
        }
    }
}
