use crate::models::common::{Platform, RequestSchedulingSettings, ValidityPeriod};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsAppEventRequest {
    pub events: Vec<WhatsAppEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DefaultEventRequestOptions>,
}

impl WhatsAppEventRequest {
    /// Creates a typing indicator event from `from` to `to`.
    pub fn typing(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            events: vec![WhatsAppEvent {
                sender: from.into(),
                destinations: vec![EventDestination {
                    to: to.into(),
                    message_id: None,
                }],
                content: EventContent::TypingIndicator { message_id: None },
                options: None,
            }],
            options: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultEventRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RequestSchedulingSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsAppEvent {
    pub sender: String,
    pub destinations: Vec<EventDestination>,
    pub content: EventContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<WhatsAppEventOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDestination {
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventContent {
    #[serde(rename = "TYPING_INDICATOR")]
    TypingIndicator {
        #[serde(skip_serializing_if = "Option::is_none", rename = "messageId")]
        message_id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsAppEventOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<ValidityPeriod>,
}
