use serde::{Deserialize, Serialize};
use crate::models::common::{MessageContext, UrlOptions};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: TextContent,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<bool>,
}

impl TextMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: TextContent {
                text: text.into(),
                preview_url: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_required_fields() {
        let msg = TextMessage::new("441134960000", "441134960001", "Hello");
        assert_eq!(msg.from, "441134960000");
        assert_eq!(msg.to, "441134960001");
        assert_eq!(msg.content.text, "Hello");
        assert!(msg.message_id.is_none());
        assert!(msg.callback_data.is_none());
    }

    #[test]
    fn serialize_minimal() {
        let msg = TextMessage::new("sender", "recipient", "Hi");
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["from"], "sender");
        assert_eq!(json["to"], "recipient");
        assert_eq!(json["content"]["text"], "Hi");
        assert!(json.get("messageId").is_none());
        assert!(json.get("callbackData").is_none());
        assert!(json.get("urlOptions").is_none());
    }

    #[test]
    fn serialize_with_optional_fields() {
        let mut msg = TextMessage::new("s", "r", "text");
        msg.message_id = Some("msg-123".into());
        msg.callback_data = Some("cb-data".into());
        msg.content.preview_url = Some(true);
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["messageId"], "msg-123");
        assert_eq!(json["callbackData"], "cb-data");
        assert_eq!(json["content"]["previewUrl"], true);
    }

    #[test]
    fn deserialize_from_json() {
        let json = r#"{
            "from": "441134960000",
            "to": "441134960001",
            "content": { "text": "Hello world", "previewUrl": true }
        }"#;
        let msg: TextMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.from, "441134960000");
        assert_eq!(msg.content.text, "Hello world");
        assert_eq!(msg.content.preview_url, Some(true));
    }

    #[test]
    fn roundtrip_serialization() {
        let mut msg = TextMessage::new("from", "to", "body");
        msg.callback_data = Some("data".into());
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: TextMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.from, msg.from);
        assert_eq!(deserialized.content.text, msg.content.text);
        assert_eq!(deserialized.callback_data, msg.callback_data);
    }

    #[test]
    fn camel_case_keys() {
        let mut msg = TextMessage::new("s", "r", "t");
        msg.callback_data = Some("x".into());
        msg.url_options = Some(UrlOptions {
            shorten_url: Some(true),
            track_clicks: None,
            tracking_url: None,
            remove_protocol: None,
            custom_domain: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("callbackData"));
        assert!(json.contains("urlOptions"));
        assert!(json.contains("shortenUrl"));
        assert!(!json.contains("callback_data"));
        assert!(!json.contains("url_options"));
    }
}
