//! Handle inbound WhatsApp media and message read receipts.
//!
//! This module provides methods to download media files sent by end users,
//! retrieve media metadata, and mark incoming messages as read.

use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::inbound::MediaMetadata;
use reqwest::Method;

/// Client for the WhatsApp inbound media and read-receipt endpoints.
///
/// Obtained via [`WhatsAppClient::inbound`](crate::WhatsAppClient::inbound).
/// Use this API to download media that users send to you and to mark their
/// messages as read (showing the blue double-checkmarks).
pub struct InboundApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> InboundApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    /// Download media sent by an end user.
    ///
    /// Fetches the raw bytes of a media file via the
    /// `GET /whatsapp/1/senders/{sender}/media/{media_id}` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let bytes = client.inbound().get_media("441134960000", "media-id-123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_media(&self, sender: &str, media_id: &str) -> Result<Vec<u8>> {
        let path = format!("/whatsapp/1/senders/{sender}/media/{media_id}");
        self.client.request_bytes(Method::GET, &path).await
    }

    /// Get media metadata without downloading the file.
    ///
    /// Sends a HEAD request to retrieve the file size and MIME type via the
    /// `HEAD /whatsapp/1/senders/{sender}/media/{media_id}` endpoint.
    ///
    /// The returned [`MediaMetadata`] contains the `Content-Length` (file size
    /// in bytes) and `Content-Type` (MIME type) headers from the response,
    /// when the server provides them.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// let meta = client.inbound().head_media("441134960000", "media-id-123").await?;
    /// if let Some(size) = meta.content_length {
    ///     println!("File size: {size} bytes");
    /// }
    /// if let Some(mime) = &meta.content_type {
    ///     println!("MIME type: {mime}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn head_media(&self, sender: &str, media_id: &str) -> Result<MediaMetadata> {
        let path = format!("/whatsapp/1/senders/{sender}/media/{media_id}");
        let url = self.client.base_url().join(&path)?;
        let mut req = self.client.http_client().request(Method::HEAD, url);
        req = self.client.auth().apply(req);
        let resp = req.send().await?;
        if resp.status().is_success() {
            let headers = resp.headers();

            let content_length = headers
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            let content_type = headers
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(MediaMetadata {
                content_length,
                content_type,
            })
        } else {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            Err(crate::InfobipError::Api(
                crate::error::ApiError::from_status_and_body(code, &text),
            ))
        }
    }

    /// Mark a message as read.
    ///
    /// Marks an inbound message as read (blue double-checkmarks) via the
    /// `POST /whatsapp/1/senders/{sender}/message/{message_id}/read` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("https://x.api.infobip.com", Auth::ApiKey("k".into()))?;
    /// # let client = WhatsAppClient::new(config);
    /// client.inbound().mark_as_read("441134960000", "msg-id-456").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_as_read(&self, sender: &str, message_id: &str) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/message/{message_id}/read");
        self.client
            .request_no_content(Method::POST, &path, None::<&()>)
            .await
    }
}
