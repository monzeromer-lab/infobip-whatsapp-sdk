//! Send WhatsApp events such as typing indicators to end users.

use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::common::EventResponse;
use crate::models::events::WhatsAppEventRequest;

/// Client for the WhatsApp events endpoint.
///
/// Obtained via [`WhatsAppClient::events`](crate::WhatsAppClient::events).
/// Use this API to send real-time events (e.g. typing indicators) to users.
pub struct EventsApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> EventsApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    /// Send an event such as a typing indicator.
    ///
    /// Posts an event to the recipient via the
    /// `POST /whatsapp/1/events` endpoint. Commonly used to show a
    /// "typing..." status before sending a reply.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_whatsapp_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_whatsapp_sdk::models::events::WhatsAppEventRequest;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let event = WhatsAppEventRequest::typing("441134960000", "441134960001");
    /// let result = client.events().send_event(&event).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_event(&self, request: &WhatsAppEventRequest) -> Result<EventResponse> {
        self.client
            .request(Method::POST, "/whatsapp/1/events", Some(request))
            .await
    }
}
