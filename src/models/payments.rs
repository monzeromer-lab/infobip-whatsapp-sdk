use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    pub reference_id: Option<String>,
    pub payment_id: Option<String>,
    pub payment_status: Option<String>,
    pub currency: Option<String>,
    pub total_amount_value: Option<i64>,
    pub total_amount_offset: Option<i64>,
    pub transactions: Option<Vec<PaymentTransaction>>,
    pub callback_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransaction {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    pub status: Option<String>,
    pub created_timestamp: Option<String>,
    pub updated_timestamp: Option<String>,
}
