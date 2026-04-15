use serde::{Deserialize, Serialize};
use crate::models::common::Platform;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateApiResponse {
    pub id: Option<String>,
    pub business_account_id: Option<i64>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub structure: Option<TemplateStructure>,
    pub quality: Option<String>,
    pub platform: Option<Platform>,
    pub created_at: Option<String>,
    pub last_updated_at: Option<String>,
    pub sub_category: Option<String>,
    pub validity_period: Option<TemplateValidityPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplatesApiResponse {
    pub templates: Option<Vec<TemplateApiResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplatesApiResponsePaginated {
    pub results: Option<Vec<TemplateApiResponse>>,
    pub paging: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub total_pages: Option<i32>,
    pub total_results: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateStructure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<TemplateBodyData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<TemplateFooterData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carousel: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_time_offer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_permission_request: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortening_options: Option<serde_json::Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub structure_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateBodyData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_security_recommendation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateFooterData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_expiration_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplatePublicApiRequest {
    pub name: String,
    pub language: String,
    pub category: String,
    pub structure: TemplateStructure,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TemplateValidityPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<TemplateStructure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TemplateValidityPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateValidityPeriod {
    pub amount: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}
