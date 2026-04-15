use super::common::{InteractiveBody, InteractiveFooter};
use crate::models::common::{MessageContext, UrlOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveButtonsMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: InteractiveButtonsContent,
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

impl InteractiveButtonsMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: InteractiveButtonsContent {
                body: InteractiveBody {
                    text: String::new(),
                },
                action: InteractiveButtonsAction { buttons: vec![] },
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
pub struct InteractiveButtonsContent {
    pub body: InteractiveBody,
    pub action: InteractiveButtonsAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveButtonsHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveButtonsAction {
    pub buttons: Vec<ReplyButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyButton {
    #[serde(rename = "type")]
    pub button_type: String,
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveButtonsHeader {
    #[serde(rename = "TEXT")]
    Text { text: String },
    #[serde(rename = "VIDEO", rename_all = "camelCase")]
    Video { media_url: String },
    #[serde(rename = "IMAGE", rename_all = "camelCase")]
    Image { media_url: String },
    #[serde(rename = "DOCUMENT", rename_all = "camelCase")]
    Document {
        media_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buttons_message() -> InteractiveButtonsMessage {
        InteractiveButtonsMessage {
            from: "441134960000".into(),
            to: "441134960001".into(),
            message_id: None,
            content: InteractiveButtonsContent {
                body: InteractiveBody {
                    text: "Choose an option".into(),
                },
                action: InteractiveButtonsAction {
                    buttons: vec![
                        ReplyButton {
                            button_type: "REPLY".into(),
                            id: "1".into(),
                            title: "Yes".into(),
                        },
                        ReplyButton {
                            button_type: "REPLY".into(),
                            id: "2".into(),
                            title: "No".into(),
                        },
                    ],
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

    #[test]
    fn serialize_buttons_message() {
        let msg = make_buttons_message();
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["content"]["body"]["text"], "Choose an option");
        let buttons = json["content"]["action"]["buttons"].as_array().unwrap();
        assert_eq!(buttons.len(), 2);
        assert_eq!(buttons[0]["type"], "REPLY");
        assert_eq!(buttons[0]["id"], "1");
        assert_eq!(buttons[0]["title"], "Yes");
    }

    #[test]
    fn text_header_tagged() {
        let header = InteractiveButtonsHeader::Text {
            text: "Header".into(),
        };
        let json = serde_json::to_value(&header).unwrap();
        assert_eq!(json["type"], "TEXT");
        assert_eq!(json["text"], "Header");
    }

    #[test]
    fn image_header_tagged() {
        let header = InteractiveButtonsHeader::Image {
            media_url: "https://example.com/img.jpg".into(),
        };
        let json = serde_json::to_value(&header).unwrap();
        assert_eq!(json["type"], "IMAGE");
        assert_eq!(json["mediaUrl"], "https://example.com/img.jpg");
    }

    #[test]
    fn with_header_and_footer() {
        let mut msg = make_buttons_message();
        msg.content.header = Some(InteractiveButtonsHeader::Text {
            text: "Important".into(),
        });
        msg.content.footer = Some(InteractiveFooter {
            text: "Tap a button".into(),
        });
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["content"]["header"]["type"], "TEXT");
        assert_eq!(json["content"]["footer"]["text"], "Tap a button");
    }

    #[test]
    fn roundtrip_buttons() {
        let msg = make_buttons_message();
        let s = serde_json::to_string(&msg).unwrap();
        let d: InteractiveButtonsMessage = serde_json::from_str(&s).unwrap();
        assert_eq!(d.content.action.buttons.len(), 2);
        assert_eq!(d.content.action.buttons[1].title, "No");
    }
}
