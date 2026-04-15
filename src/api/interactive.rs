//! Send interactive WhatsApp messages such as reply buttons, lists, carousels,
//! product catalogs, order details, flows, URL buttons, and voice/call actions.

use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::common::SingleMessageInfo;
use crate::models::interactive::{
    buttons::InteractiveButtonsMessage, call_permission::InteractiveCallPermissionRequestMessage,
    flow::InteractiveFlowMessage, list::InteractiveListMessage,
    location_request::InteractiveLocationRequestMessage,
    media_carousel::InteractiveMediaCarouselMessage,
    multi_product::InteractiveMultiProductMessage,
    order_details::InteractiveOrderDetailsMessage,
    order_status::InteractiveOrderStatusMessage, product::InteractiveProductMessage,
    url_button::InteractiveUrlButtonMessage, voice_button::InteractiveVoiceButtonMessage,
};

/// Client for the WhatsApp interactive messaging endpoints.
///
/// Obtained via [`WhatsAppClient::interactive`](crate::WhatsAppClient::interactive).
/// Interactive messages let users tap buttons, scroll lists, browse products,
/// and complete flows directly inside the WhatsApp chat.
pub struct InteractiveApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> InteractiveApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    /// Send interactive reply buttons (up to 3).
    ///
    /// Presents the user with up to three tappable buttons via the
    /// `POST /whatsapp/1/message/interactive/buttons` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::buttons::InteractiveButtonsMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveButtonsMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_buttons(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_buttons(&self, message: &InteractiveButtonsMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/buttons", Some(message))
            .await
    }

    /// Send an interactive list with sections and rows.
    ///
    /// Presents the user with a menu of selectable options via the
    /// `POST /whatsapp/1/message/interactive/list` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::list::InteractiveListMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveListMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_list(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_list(&self, message: &InteractiveListMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/list", Some(message))
            .await
    }

    /// Send a scrollable media carousel.
    ///
    /// Delivers a horizontally-scrollable set of media cards via the
    /// `POST /whatsapp/1/message/interactive/media-carousel` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::media_carousel::InteractiveMediaCarouselMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveMediaCarouselMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_media_carousel(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_media_carousel(&self, message: &InteractiveMediaCarouselMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/media-carousel", Some(message))
            .await
    }

    /// Request the user's current location.
    ///
    /// Prompts the user to share their location via the
    /// `POST /whatsapp/1/message/interactive/location-request` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::location_request::InteractiveLocationRequestMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveLocationRequestMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_location_request(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_location_request(&self, message: &InteractiveLocationRequestMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/location-request", Some(message))
            .await
    }

    /// Send a single product from a catalog.
    ///
    /// Displays a single product card from a connected catalog via the
    /// `POST /whatsapp/1/message/interactive/product` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::product::InteractiveProductMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveProductMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_product(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_product(&self, message: &InteractiveProductMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/product", Some(message))
            .await
    }

    /// Send multiple products from a catalog.
    ///
    /// Displays several product cards from a connected catalog via the
    /// `POST /whatsapp/1/message/interactive/multi-product` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::multi_product::InteractiveMultiProductMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveMultiProductMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_multi_product(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_multi_product(&self, message: &InteractiveMultiProductMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/multi-product", Some(message))
            .await
    }

    /// Send order details with payment information.
    ///
    /// Delivers an order summary (items, totals, payment info) via the
    /// `POST /whatsapp/1/message/interactive/order-details` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::order_details::InteractiveOrderDetailsMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveOrderDetailsMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_order_details(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_order_details(&self, message: &InteractiveOrderDetailsMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/order-details", Some(message))
            .await
    }

    /// Send an order status update.
    ///
    /// Notifies the user about an order status change via the
    /// `POST /whatsapp/1/message/interactive/order-status` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::order_status::InteractiveOrderStatusMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveOrderStatusMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_order_status(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_order_status(&self, message: &InteractiveOrderStatusMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/order-status", Some(message))
            .await
    }

    /// Send a WhatsApp Flow.
    ///
    /// Launches an interactive flow (multi-screen form) via the
    /// `POST /whatsapp/1/message/interactive/flow` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::flow::InteractiveFlowMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveFlowMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_flow(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_flow(&self, message: &InteractiveFlowMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/flow", Some(message))
            .await
    }

    /// Send a CTA URL button.
    ///
    /// Presents the user with a tappable link button via the
    /// `POST /whatsapp/1/message/interactive/url-button` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::url_button::InteractiveUrlButtonMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveUrlButtonMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_url_button(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_url_button(&self, message: &InteractiveUrlButtonMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/url-button", Some(message))
            .await
    }

    /// Request permission to call the user.
    ///
    /// Sends a call-permission request prompt via the
    /// `POST /whatsapp/1/message/interactive/call-permission-request` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::call_permission::InteractiveCallPermissionRequestMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveCallPermissionRequestMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_call_permission_request(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_call_permission_request(&self, message: &InteractiveCallPermissionRequestMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/call-permission-request", Some(message))
            .await
    }

    /// Send a voice call button.
    ///
    /// Presents the user with a button to initiate a voice call via the
    /// `POST /whatsapp/1/message/interactive/voice-button` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::interactive::voice_button::InteractiveVoiceButtonMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = InteractiveVoiceButtonMessage::new("441134960000", "441134960001");
    /// let result = client.interactive().send_voice_button(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_voice_button(&self, message: &InteractiveVoiceButtonMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/interactive/voice-button", Some(message))
            .await
    }
}
