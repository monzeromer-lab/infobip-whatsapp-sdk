use super::common::{InteractiveBody, InteractiveFooter};
use crate::models::common::{MessageContext, UrlOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveFlowMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: InteractiveFlowContent,
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

impl InteractiveFlowMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: InteractiveFlowContent {
                body: InteractiveBody {
                    text: String::new(),
                },
                action: InteractiveFlowAction {
                    mode: None,
                    flow_message_version: None,
                    flow_token: None,
                    flow_id: String::new(),
                    call_to_action_button: String::new(),
                    flow_action: None,
                    flow_action_payload: None,
                },
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
pub struct InteractiveFlowContent {
    pub body: InteractiveBody,
    pub action: InteractiveFlowAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveFlowHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveFlowAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_message_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_token: Option<String>,
    pub flow_id: String,
    pub call_to_action_button: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_action_payload: Option<FlowActionPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowActionPayload {
    pub screen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveFlowHeader {
    #[serde(rename = "TEXT")]
    Text { text: String },
}
