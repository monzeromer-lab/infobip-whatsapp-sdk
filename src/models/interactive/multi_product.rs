use super::common::{InteractiveBody, InteractiveFooter};
use crate::models::common::{MessageContext, UrlOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveMultiProductMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: InteractiveMultiProductContent,
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

impl InteractiveMultiProductMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: InteractiveMultiProductContent {
                header: InteractiveMultiProductHeader::Text {
                    text: String::new(),
                },
                body: InteractiveBody {
                    text: String::new(),
                },
                action: InteractiveMultiProductAction {
                    catalog_id: String::new(),
                    sections: vec![],
                },
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
pub struct InteractiveMultiProductContent {
    pub header: InteractiveMultiProductHeader,
    pub body: InteractiveBody,
    pub action: InteractiveMultiProductAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveMultiProductHeader {
    #[serde(rename = "TEXT")]
    Text { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveMultiProductAction {
    pub catalog_id: String,
    pub sections: Vec<MultiProductSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiProductSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub product_retailer_ids: Vec<String>,
}
