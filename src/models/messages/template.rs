use crate::models::common::{MessageContext, UrlOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkMessage {
    pub messages: Vec<FailoverMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
}

impl BulkMessage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailoverMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: TemplateContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_options: Option<UrlOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_failover: Option<SmsFailover>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<MessageContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateContent {
    pub template_name: String,
    pub template_data: TemplateDataContent,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateDataContent {
    pub body: TemplateBodyContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<TemplateHeaderContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<TemplateButtonContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carousel: Option<TemplateCarouselContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_time_offer: Option<TemplateLimitedTimeOfferContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<TemplateOrderStatusContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateBodyContent {
    pub placeholders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateHeaderContent {
    #[serde(rename = "TEXT")]
    Text { placeholder: String },
    #[serde(rename = "DOCUMENT", rename_all = "camelCase")]
    Document {
        media_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
    },
    #[serde(rename = "IMAGE", rename_all = "camelCase")]
    Image { media_url: String },
    #[serde(rename = "VIDEO", rename_all = "camelCase")]
    Video { media_url: String },
    #[serde(rename = "LOCATION")]
    Location { latitude: f64, longitude: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateButtonContent {
    #[serde(rename = "QUICK_REPLY")]
    QuickReply { parameter: String },
    #[serde(rename = "URL")]
    Url {
        #[serde(skip_serializing_if = "Option::is_none")]
        parameter: Option<String>,
    },
    #[serde(rename = "COPY_CODE")]
    CopyCode { parameter: String },
    #[serde(rename = "FLOW", rename_all = "camelCase")]
    Flow {
        #[serde(skip_serializing_if = "Option::is_none")]
        flow_token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<HashMap<String, serde_json::Value>>,
    },
    #[serde(rename = "CATALOG", rename_all = "camelCase")]
    Catalog {
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_product_retailer_id: Option<String>,
    },
    #[serde(rename = "MULTI_PRODUCT", rename_all = "camelCase")]
    MultiProduct {
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_product_retailer_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sections: Option<Vec<MultiProductSection>>,
    },
    #[serde(rename = "ORDER_DETAILS")]
    OrderDetails { action: serde_json::Value },
    #[serde(rename = "VOICE_CALL", rename_all = "camelCase")]
    VoiceCall {
        #[serde(skip_serializing_if = "Option::is_none")]
        call_validity_period: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        call_context_payload: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiProductSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub product_retailer_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCarouselContent {
    pub cards: Vec<TemplateCardContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCardContent {
    pub header: TemplateCardHeaderContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<TemplateBodyContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<TemplateButtonContent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateCardHeaderContent {
    #[serde(rename = "IMAGE", rename_all = "camelCase")]
    Image { media_url: String },
    #[serde(rename = "VIDEO", rename_all = "camelCase")]
    Video { media_url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateLimitedTimeOfferContent {
    pub expiration_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateOrderStatusContent {
    pub order_statuses: Vec<TemplateOrderStatusEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateOrderStatusEntry {
    #[serde(rename = "type")]
    pub status_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsFailover {
    pub from: String,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_template() -> BulkMessage {
        BulkMessage {
            messages: vec![FailoverMessage {
                from: "441134960000".into(),
                to: "441134960001".into(),
                message_id: None,
                content: TemplateContent {
                    template_name: "welcome".into(),
                    template_data: TemplateDataContent {
                        body: TemplateBodyContent {
                            placeholders: vec!["John".into()],
                        },
                        header: None,
                        buttons: None,
                        carousel: None,
                        limited_time_offer: None,
                        order_status: None,
                    },
                    language: "en".into(),
                },
                callback_data: None,
                notify_url: None,
                url_options: None,
                sms_failover: None,
                entity_id: None,
                application_id: None,
                context: None,
            }],
            bulk_id: None,
        }
    }

    #[test]
    fn serialize_simple_template() {
        let msg = make_simple_template();
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["messages"][0]["from"], "441134960000");
        assert_eq!(json["messages"][0]["content"]["templateName"], "welcome");
        assert_eq!(json["messages"][0]["content"]["language"], "en");
        assert_eq!(
            json["messages"][0]["content"]["templateData"]["body"]["placeholders"][0],
            "John"
        );
    }

    #[test]
    fn header_text_variant_tagged() {
        let header = TemplateHeaderContent::Text {
            placeholder: "Welcome".into(),
        };
        let json = serde_json::to_value(&header).unwrap();
        assert_eq!(json["type"], "TEXT");
        assert_eq!(json["placeholder"], "Welcome");
    }

    #[test]
    fn header_image_variant_tagged() {
        let header = TemplateHeaderContent::Image {
            media_url: "https://example.com/img.jpg".into(),
        };
        let json = serde_json::to_value(&header).unwrap();
        assert_eq!(json["type"], "IMAGE");
        assert_eq!(json["mediaUrl"], "https://example.com/img.jpg");
    }

    #[test]
    fn header_location_variant_tagged() {
        let header = TemplateHeaderContent::Location {
            latitude: 44.0,
            longitude: 20.5,
        };
        let json = serde_json::to_value(&header).unwrap();
        assert_eq!(json["type"], "LOCATION");
        assert_eq!(json["latitude"], 44.0);
        assert_eq!(json["longitude"], 20.5);
    }

    #[test]
    fn button_quick_reply_tagged() {
        let btn = TemplateButtonContent::QuickReply {
            parameter: "yes".into(),
        };
        let json = serde_json::to_value(&btn).unwrap();
        assert_eq!(json["type"], "QUICK_REPLY");
        assert_eq!(json["parameter"], "yes");
    }

    #[test]
    fn button_copy_code_tagged() {
        let btn = TemplateButtonContent::CopyCode {
            parameter: "DISCOUNT20".into(),
        };
        let json = serde_json::to_value(&btn).unwrap();
        assert_eq!(json["type"], "COPY_CODE");
        assert_eq!(json["parameter"], "DISCOUNT20");
    }

    #[test]
    fn sms_failover_serialization() {
        let mut msg = make_simple_template();
        msg.messages[0].sms_failover = Some(SmsFailover {
            from: "InfoSMS".into(),
            text: "Fallback SMS text".into(),
        });
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["messages"][0]["smsFailover"]["from"], "InfoSMS");
        assert_eq!(
            json["messages"][0]["smsFailover"]["text"],
            "Fallback SMS text"
        );
    }

    #[test]
    fn roundtrip_template_message() {
        let msg = make_simple_template();
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: BulkMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.messages.len(), 1);
        assert_eq!(deserialized.messages[0].content.template_name, "welcome");
    }

    #[test]
    fn deserialize_header_from_json() {
        let json = r#"{"type":"DOCUMENT","mediaUrl":"https://example.com/doc.pdf","filename":"invoice.pdf"}"#;
        let header: TemplateHeaderContent = serde_json::from_str(json).unwrap();
        match header {
            TemplateHeaderContent::Document {
                media_url,
                filename,
            } => {
                assert_eq!(media_url, "https://example.com/doc.pdf");
                assert_eq!(filename.as_deref(), Some("invoice.pdf"));
            }
            _ => panic!("expected Document variant"),
        }
    }
}
