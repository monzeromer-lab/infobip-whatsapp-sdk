use super::common::InteractiveBody;
use crate::models::common::UrlOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveVoiceButtonMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: InteractiveVoiceButtonContent,
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
}

impl InteractiveVoiceButtonMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: InteractiveVoiceButtonContent {
                body: InteractiveBody {
                    text: String::new(),
                },
                action: None,
            },
            callback_data: None,
            notify_url: None,
            url_options: None,
            entity_id: None,
            application_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveVoiceButtonContent {
    pub body: InteractiveBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<VoiceButtonAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceButtonAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_display_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_validity_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_context_payload: Option<String>,
}
