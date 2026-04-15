use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsAppSenderQualityResponse {
    pub results: Option<Vec<WhatsAppSenderQuality>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsAppSenderQuality {
    pub sender: Option<String>,
    pub quality_rating: Option<String>,
    pub status: Option<String>,
    pub current_limit: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessInfoResponse {
    pub about: Option<String>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub vertical: Option<String>,
    pub websites: Option<Vec<String>>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websites: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallingPermissionResponse {
    pub status: Option<String>,
    pub actions: Option<Vec<CallingPermissionAction>>,
    pub expiration_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallingPermissionAction {
    pub name: Option<String>,
    pub can_perform_action: Option<bool>,
    pub limits: Option<Vec<CallingPermissionActionLimit>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallingPermissionActionLimit {
    pub time_period: Option<i64>,
    pub max_allowed: Option<i64>,
    pub current_usage: Option<i64>,
    pub limit_expiration_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SenderPublicKeyResponse {
    pub public_key: Option<String>,
    pub public_key_signature_status: Option<String>,
}
