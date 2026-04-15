use super::common::{InteractiveBody, InteractiveFooter};
use crate::models::common::{MessageContext, UrlOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveOrderDetailsMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: InteractiveOrderDetailsContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_options: Option<UrlOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<MessageContext>,
}

impl InteractiveOrderDetailsMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: InteractiveOrderDetailsContent {
                body: InteractiveBody {
                    text: String::new(),
                },
                action: serde_json::Value::Object(Default::default()),
                header: None,
                footer: None,
            },
            callback_data: None,
            notify_url: None,
            url_options: None,
            entity_id: None,
            application_id: None,
            context: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveOrderDetailsContent {
    pub body: InteractiveBody,
    pub action: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
}
