use serde::{Deserialize, Serialize};

/// Metadata about a media file (from HEAD request).
///
/// Returned by [`InboundApi::head_media`](crate::api::inbound::InboundApi::head_media)
/// after sending a HEAD request to the media endpoint. The server responds with
/// `Content-Length` and `Content-Type` headers describing the file.
#[derive(Debug, Clone)]
pub struct MediaMetadata {
    /// File size in bytes, if provided by the server.
    pub content_length: Option<u64>,
    /// MIME type of the file, if provided by the server.
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsAppInboundMessages {
    pub results: Option<Vec<InboundMessage>>,
    pub message_count: Option<i32>,
    pub pending_message_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundMessage {
    pub from: Option<String>,
    pub to: Option<String>,
    pub integration_type: Option<String>,
    pub received_at: Option<String>,
    pub message_id: Option<String>,
    pub paired_message_id: Option<String>,
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<InboundIdentity>,
    pub message: Option<serde_json::Value>,
    pub price: Option<MessagePrice>,
    pub contact: Option<InboundContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePrice {
    pub price_per_message: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundContact {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlDeletionRequest {
    pub url: String,
}
