use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryReport {
    pub bulk_id: Option<String>,
    pub message_id: Option<String>,
    pub to: Option<String>,
    pub sent_at: Option<String>,
    pub done_at: Option<String>,
    pub message_count: Option<i32>,
    pub price: Option<ReportPrice>,
    pub status: Option<ReportStatus>,
    pub error: Option<ReportError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryResults {
    pub results: Option<Vec<DeliveryReport>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportPrice {
    pub price_per_message: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportStatus {
    pub group_id: Option<i32>,
    pub group_name: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportError {
    pub group_id: Option<i32>,
    pub group_name: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub permanent: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeenReport {
    pub message_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub sent_at: Option<String>,
    pub seen_at: Option<String>,
    pub application_id: Option<String>,
    pub entity_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeenResults {
    pub results: Option<Vec<SeenReport>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_delivery_report() {
        let json = r#"{
            "results": [{
                "bulkId": "bulk-1",
                "messageId": "msg-1",
                "to": "441134960001",
                "sentAt": "2024-01-01T12:00:00.000+0000",
                "doneAt": "2024-01-01T12:00:01.000+0000",
                "messageCount": 1,
                "price": { "pricePerMessage": 0.01, "currency": "EUR" },
                "status": {
                    "groupId": 3,
                    "groupName": "DELIVERED",
                    "id": 5,
                    "name": "DELIVERED_TO_HANDSET",
                    "description": "Message delivered to handset"
                },
                "error": {
                    "groupId": 0,
                    "groupName": "OK",
                    "id": 0,
                    "name": "NO_ERROR",
                    "description": "No Error",
                    "permanent": false
                }
            }]
        }"#;
        let results: DeliveryResults = serde_json::from_str(json).unwrap();
        let reports = results.results.unwrap();
        assert_eq!(reports.len(), 1);
        let report = &reports[0];
        assert_eq!(report.bulk_id.as_deref(), Some("bulk-1"));
        assert_eq!(report.status.as_ref().unwrap().group_name.as_deref(), Some("DELIVERED"));
        assert_eq!(report.price.as_ref().unwrap().price_per_message, Some(0.01));
        assert_eq!(report.error.as_ref().unwrap().permanent, Some(false));
    }

    #[test]
    fn deserialize_seen_report() {
        let json = r#"{
            "results": [{
                "messageId": "msg-1",
                "from": "441134960001",
                "to": "441134960000",
                "sentAt": "2024-01-01T12:00:00.000+0000",
                "seenAt": "2024-01-01T12:05:00.000+0000"
            }]
        }"#;
        let results: SeenResults = serde_json::from_str(json).unwrap();
        let reports = results.results.unwrap();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].message_id.as_deref(), Some("msg-1"));
        assert!(reports[0].seen_at.is_some());
    }
}
