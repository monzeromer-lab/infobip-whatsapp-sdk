//! The main HTTP client for the Infobip WhatsApp API.

use reqwest::{Client, Method};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::api::*;
use crate::auth::Auth;
use crate::config::ClientConfig;
use crate::error::{ApiError, InfobipError, Result};

/// The main entry point for interacting with the Infobip WhatsApp API.
///
/// Create a client with [`WhatsAppClient::new`], then access each API domain
/// through the accessor methods (`.messages()`, `.templates()`, etc.).
///
/// # Examples
///
/// ```rust,no_run
/// use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
/// use infobip_sdk::models::messages::text::TextMessage;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ClientConfig::new(
///     "https://xxxxx.api.infobip.com",
///     Auth::ApiKey("your-api-key".into()),
/// )?;
/// let client = WhatsAppClient::new(config);
///
/// // Send a text message
/// let msg = TextMessage::new("441134960000", "441134960001", "Hello!");
/// let result = client.messages().send_text(&msg).await?;
/// println!("Sent with ID: {:?}", result.message_id);
///
/// // Check sender quality
/// let quality = client.sender().get_quality(&["441134960000"]).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Custom HTTP Client
///
/// Use [`WhatsAppClient::with_http_client`] to supply a pre-configured
/// `reqwest::Client` (e.g. with custom timeouts or proxy settings):
///
/// ```rust,no_run
/// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
/// let http = reqwest::Client::builder()
///     .timeout(std::time::Duration::from_secs(30))
///     .build()
///     .unwrap();
///
/// let config = ClientConfig::new(
///     "https://xxxxx.api.infobip.com",
///     Auth::ApiKey("key".into()),
/// ).unwrap();
///
/// let client = WhatsAppClient::with_http_client(config, http);
/// ```
pub struct WhatsAppClient {
    http: Client,
    base_url: Url,
    auth: Auth,
}

impl WhatsAppClient {
    /// Create a new client with default HTTP settings.
    pub fn new(config: ClientConfig) -> Self {
        Self {
            http: Client::new(),
            base_url: config.base_url,
            auth: config.auth,
        }
    }

    /// Create a new client with a custom `reqwest::Client` (for timeouts, proxies, etc.).
    pub fn with_http_client(config: ClientConfig, http: Client) -> Self {
        Self {
            http,
            base_url: config.base_url,
            auth: config.auth,
        }
    }

    pub(crate) fn http_client(&self) -> &Client {
        &self.http
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn auth(&self) -> &Auth {
        &self.auth
    }

    /// Send text, media, template, contact, and location messages.
    pub fn messages(&self) -> MessagesApi<'_> {
        MessagesApi::new(self)
    }

    /// Send interactive messages (buttons, lists, products, flows, carousels).
    pub fn interactive(&self) -> InteractiveApi<'_> {
        InteractiveApi::new(self)
    }

    /// Send events like typing indicators.
    pub fn events(&self) -> EventsApi<'_> {
        EventsApi::new(self)
    }

    /// Download inbound media and mark messages as read.
    pub fn inbound(&self) -> InboundApi<'_> {
        InboundApi::new(self)
    }

    /// Create, read, update, and delete WhatsApp message templates.
    pub fn templates(&self) -> TemplatesApi<'_> {
        TemplatesApi::new(self)
    }

    /// Manage WhatsApp Flows (create, publish, deprecate, preview, etc.).
    pub fn flows(&self) -> FlowsApi<'_> {
        FlowsApi::new(self)
    }

    /// Check payment transaction statuses (UPI, Brazil, PayU, Razorpay).
    pub fn payments(&self) -> PaymentsApi<'_> {
        PaymentsApi::new(self)
    }

    /// Sender quality, business info, logo, calling permissions, public keys.
    pub fn sender(&self) -> SenderApi<'_> {
        SenderApi::new(self)
    }

    /// Manage end-user identity change notifications and confirmations.
    pub fn identity(&self) -> IdentityApi<'_> {
        IdentityApi::new(self)
    }

    /// Submit ad conversion events for WhatsApp click-to-action campaigns.
    pub fn conversions(&self) -> ConversionsApi<'_> {
        ConversionsApi::new(self)
    }

    /// Delete previously uploaded media.
    pub fn media(&self) -> MediaApi<'_> {
        MediaApi::new(self)
    }

    /// Register and verify WhatsApp sender phone numbers.
    pub fn registration(&self) -> RegistrationApi<'_> {
        RegistrationApi::new(self)
    }

    fn make_api_error(status: u16, body: &str) -> InfobipError {
        InfobipError::Api(ApiError::from_status_and_body(status, body))
    }

    pub(crate) async fn request<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = self.base_url.join(path)?;
        let mut req = self.http.request(method, url);
        req = self.auth.apply(req);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        if resp.status().is_success() {
            Ok(resp.json::<T>().await?)
        } else {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            Err(Self::make_api_error(code, &text))
        }
    }

    pub(crate) async fn request_no_content<B: Serialize + Sync>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<()> {
        let url = self.base_url.join(path)?;
        let mut req = self.http.request(method, url);
        req = self.auth.apply(req);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            Err(Self::make_api_error(code, &text))
        }
    }

    pub(crate) async fn request_bytes(
        &self,
        method: Method,
        path: &str,
    ) -> Result<Vec<u8>> {
        let url = self.base_url.join(path)?;
        let mut req = self.http.request(method, url);
        req = self.auth.apply(req);
        let resp = req.send().await?;
        if resp.status().is_success() {
            Ok(resp.bytes().await?.to_vec())
        } else {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            Err(Self::make_api_error(code, &text))
        }
    }
}
