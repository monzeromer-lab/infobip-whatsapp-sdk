//! Send standard WhatsApp messages including text, documents, images, audio,
//! video, stickers, locations, contacts, and bulk template messages.

use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::common::{BulkMessageInfo, SingleMessageInfo};
use crate::models::messages::{
    audio::AudioMessage, contact::ContactsMessage, document::DocumentMessage, image::ImageMessage,
    location::LocationMessage, sticker::StickerMessage, template::BulkMessage, text::TextMessage,
    video::VideoMessage,
};
use reqwest::Method;

/// Client for the WhatsApp outbound messaging endpoints.
///
/// Obtained via [`WhatsAppClient::messages`](crate::WhatsAppClient::messages).
/// Provides methods to send every standard (non-interactive) message type
/// supported by the Infobip WhatsApp API.
pub struct MessagesApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> MessagesApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    /// Send a WhatsApp text message.
    ///
    /// Delivers a plain-text message to the specified recipient via the
    /// `POST /whatsapp/1/message/text` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::text::TextMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = TextMessage::new("441134960000", "441134960001", "Hello!");
    /// let result = client.messages().send_text(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_text(&self, message: &TextMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/text", Some(message))
            .await
    }

    /// Send a document message (PDF, DOCX, etc.).
    ///
    /// Delivers a document file to the recipient via the
    /// `POST /whatsapp/1/message/document` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::document::DocumentMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = DocumentMessage::new("441134960000", "441134960001", "https://example.com/doc.pdf");
    /// let result = client.messages().send_document(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_document(&self, message: &DocumentMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/document", Some(message))
            .await
    }

    /// Send an image message.
    ///
    /// Delivers an image to the recipient via the
    /// `POST /whatsapp/1/message/image` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::image::ImageMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = ImageMessage::new("441134960000", "441134960001", "https://example.com/photo.jpg");
    /// let result = client.messages().send_image(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_image(&self, message: &ImageMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/image", Some(message))
            .await
    }

    /// Send an audio file message.
    ///
    /// Delivers an audio file to the recipient via the
    /// `POST /whatsapp/1/message/audio` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::audio::AudioMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = AudioMessage::new("441134960000", "441134960001", "https://example.com/audio.mp3");
    /// let result = client.messages().send_audio(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_audio(&self, message: &AudioMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/audio", Some(message))
            .await
    }

    /// Send a video message.
    ///
    /// Delivers a video to the recipient via the
    /// `POST /whatsapp/1/message/video` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::video::VideoMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = VideoMessage::new("441134960000", "441134960001", "https://example.com/video.mp4");
    /// let result = client.messages().send_video(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_video(&self, message: &VideoMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/video", Some(message))
            .await
    }

    /// Send a sticker message.
    ///
    /// Delivers a WebP sticker to the recipient via the
    /// `POST /whatsapp/1/message/sticker` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::sticker::StickerMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = StickerMessage::new("441134960000", "441134960001", "https://example.com/sticker.webp");
    /// let result = client.messages().send_sticker(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_sticker(&self, message: &StickerMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/sticker", Some(message))
            .await
    }

    /// Send a location pin message.
    ///
    /// Delivers a geographic location to the recipient via the
    /// `POST /whatsapp/1/message/location` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::location::LocationMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = LocationMessage::new("441134960000", "441134960001", 44.7866, 20.4489);
    /// let result = client.messages().send_location(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_location(&self, message: &LocationMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/location", Some(message))
            .await
    }

    /// Send one or more contact cards.
    ///
    /// Delivers contact information to the recipient via the
    /// `POST /whatsapp/1/message/contact` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::contact::ContactsMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = ContactsMessage::new("441134960000", "441134960001");
    /// let result = client.messages().send_contact(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_contact(&self, message: &ContactsMessage) -> Result<SingleMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/contact", Some(message))
            .await
    }

    /// Send template messages in bulk.
    ///
    /// Delivers one or more pre-approved template messages via the
    /// `POST /whatsapp/1/message/template` endpoint. This is the primary way
    /// to initiate conversations outside the 24-hour messaging window.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # use infobip_sdk::models::messages::template::BulkMessage;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let msg = BulkMessage::new();
    /// let result = client.messages().send_template(&msg).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_template(&self, message: &BulkMessage) -> Result<BulkMessageInfo> {
        self.client
            .request(Method::POST, "/whatsapp/1/message/template", Some(message))
            .await
    }
}
