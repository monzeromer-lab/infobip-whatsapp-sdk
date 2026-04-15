//! Shared request and response types used across multiple WhatsApp API endpoints.
//!
//! Contains common structures for message status, scheduling, URL options,
//! platform identification, and event responses.

use serde::{Deserialize, Serialize};

/// Response returned after sending a single WhatsApp message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleMessageInfo {
    /// The recipient's WhatsApp phone number.
    pub to: Option<String>,
    /// Number of messages required to deliver the content.
    pub message_count: Option<i32>,
    /// Unique message identifier assigned by the platform.
    pub message_id: Option<String>,
    pub status: Option<SingleMessageStatus>,
}

/// Status details for a sent message (group, name, description).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleMessageStatus {
    /// Numeric status group identifier (e.g. 1 = PENDING, 3 = DELIVERED).
    pub group_id: Option<i32>,
    /// Human-readable status group name (e.g. "PENDING", "DELIVERED").
    pub group_name: Option<String>,
    /// Specific status code within the group.
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    /// Suggested corrective action, if any.
    pub action: Option<String>,
}

/// Response returned after sending a bulk template message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkMessageInfo {
    pub messages: Option<Vec<SingleMessageInfo>>,
    /// Identifier for the entire bulk send operation.
    pub bulk_id: Option<String>,
}

/// URL shortening and click-tracking options for messages containing links.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlOptions {
    /// Whether to shorten URLs in the message body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shorten_url: Option<bool>,
    /// Whether to track link clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_clicks: Option<bool>,
    /// Custom tracking URL to use instead of the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    /// Whether to strip the protocol prefix (e.g. "https://") from shortened URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_protocol: Option<bool>,
    /// Custom domain for shortened URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
}

/// Context for replying to a specific message in a WhatsApp conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContext {
    /// The message ID of the message being replied to.
    pub reply_to_message_id: String,
}

/// Scheduling settings for bulk message sends, including send time and rate limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestSchedulingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
    /// ISO 8601 datetime for scheduled delivery (e.g. "2024-06-01T12:00:00.000+0000").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_speed_limit: Option<SendingSpeedLimit>,
}

/// Rate limit for bulk message sending (e.g. 100 messages per MINUTE).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendingSpeedLimit {
    /// Maximum number of messages to send per time unit.
    pub amount: i32,
    /// Defaults to MINUTE if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<SpeedLimitTimeUnit>,
}

/// Time unit for [`SendingSpeedLimit`]. Serializes as SCREAMING_SNAKE_CASE.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpeedLimitTimeUnit {
    Minute,
    Hour,
    Day,
}

/// How long the platform should attempt to deliver a message before giving up.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidityPeriod {
    pub amount: i32,
    /// Defaults to MINUTES if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<ValidityPeriodTimeUnit>,
}

/// Time unit for [`ValidityPeriod`]. Serializes as SCREAMING_SNAKE_CASE.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidityPeriodTimeUnit {
    Minutes,
    Hours,
    Days,
}

/// Platform entity and application identifiers for message attribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}

/// Response returned from the WhatsApp event API (e.g. after sending typing indicators).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventResponse {
    pub bulk_id: Option<String>,
    pub messages: Option<Vec<EventMessageResponse>>,
}

/// Per-message result within an [`EventResponse`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageResponse {
    pub message_id: Option<String>,
    pub status: Option<SingleMessageStatus>,
    pub destination: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_single_message_info() {
        let json = r#"{
            "to": "441134960001",
            "messageCount": 1,
            "messageId": "abc-123",
            "status": {
                "groupId": 1,
                "groupName": "PENDING",
                "id": 7,
                "name": "PENDING_ENROUTE",
                "description": "Message sent to next instance"
            }
        }"#;
        let info: SingleMessageInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.to.as_deref(), Some("441134960001"));
        assert_eq!(info.message_count, Some(1));
        assert_eq!(info.message_id.as_deref(), Some("abc-123"));
        let status = info.status.unwrap();
        assert_eq!(status.group_id, Some(1));
        assert_eq!(status.group_name.as_deref(), Some("PENDING"));
    }

    #[test]
    fn deserialize_bulk_message_info() {
        let json = r#"{
            "messages": [{
                "to": "441134960001",
                "messageCount": 1,
                "messageId": "msg-1",
                "status": { "groupId": 1, "groupName": "PENDING", "id": 7, "name": "PENDING_ENROUTE", "description": "Sent" }
            }],
            "bulkId": "bulk-123"
        }"#;
        let info: BulkMessageInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.bulk_id.as_deref(), Some("bulk-123"));
        assert_eq!(info.messages.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn url_options_skips_none_fields() {
        let opts = UrlOptions {
            shorten_url: Some(true),
            track_clicks: None,
            tracking_url: None,
            remove_protocol: None,
            custom_domain: None,
        };
        let json = serde_json::to_value(&opts).unwrap();
        assert_eq!(json["shortenUrl"], true);
        assert!(json.get("trackClicks").is_none());
        assert!(json.get("trackingUrl").is_none());
    }

    #[test]
    fn message_context_serialization() {
        let ctx = MessageContext {
            reply_to_message_id: "original-msg-id".into(),
        };
        let json = serde_json::to_value(&ctx).unwrap();
        assert_eq!(json["replyToMessageId"], "original-msg-id");
    }

    #[test]
    fn speed_limit_time_unit_serialization() {
        let limit = SendingSpeedLimit {
            amount: 100,
            time_unit: Some(SpeedLimitTimeUnit::Minute),
        };
        let json = serde_json::to_value(&limit).unwrap();
        assert_eq!(json["amount"], 100);
        assert_eq!(json["timeUnit"], "MINUTE");
    }

    #[test]
    fn validity_period_time_unit_serialization() {
        let vp = ValidityPeriod {
            amount: 30,
            time_unit: Some(ValidityPeriodTimeUnit::Minutes),
        };
        let json = serde_json::to_value(&vp).unwrap();
        assert_eq!(json["amount"], 30);
        assert_eq!(json["timeUnit"], "MINUTES");
    }
}
