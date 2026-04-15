use serde::{Deserialize, Serialize};
use crate::models::common::MessageContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationMessage {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub content: LocationContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<MessageContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationContent {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl LocationMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>, latitude: f64, longitude: f64) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content: LocationContent { latitude, longitude, name: None, address: None },
            callback_data: None,
            notify_url: None,
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
    fn new_sets_coordinates() {
        let msg = LocationMessage::new("s", "r", 44.7866, 20.4489);
        assert_eq!(msg.content.latitude, 44.7866);
        assert_eq!(msg.content.longitude, 20.4489);
        assert!(msg.content.name.is_none());
    }

    #[test]
    fn serialize_with_name_and_address() {
        let mut msg = LocationMessage::new("s", "r", 44.0, 20.0);
        msg.content.name = Some("Belgrade".into());
        msg.content.address = Some("Knez Mihailova".into());
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["content"]["name"], "Belgrade");
        assert_eq!(json["content"]["address"], "Knez Mihailova");
    }

    #[test]
    fn omits_none_optional_fields() {
        let msg = LocationMessage::new("s", "r", 0.0, 0.0);
        let json = serde_json::to_value(&msg).unwrap();
        assert!(json.get("name").is_none() || json["content"].get("name").is_none());
        assert!(json.get("messageId").is_none());
    }
}
