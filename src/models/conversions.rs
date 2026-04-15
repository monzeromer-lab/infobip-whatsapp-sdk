use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConversionType {
    Purchase,
    Lead,
    CompleteRegistration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetaConversionType {
    Purchase,
    Lead,
    CompleteRegistration,
    AddPaymentInfo,
    AddToCart,
    AddToWishlist,
    InitiatedCheckout,
    Search,
    ContentView,
    Custom,
    Contact,
    CustomizeProduct,
    Donate,
    FindLocation,
    Schedule,
    StartTrial,
    SubmitApplication,
    Subscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionEventRequest {
    pub source_id: String,
    pub conversions: Vec<ConversionEventModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionEventModel {
    pub ctwa_click_id: String,
    #[serde(rename = "type")]
    pub conversion_type: ConversionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub conversion_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WabaConversionEventRequest {
    pub source_id: String,
    pub conversions: Vec<WabaConversionModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WabaConversionModel {
    pub ctwa_click_id: String,
    #[serde(rename = "type")]
    pub conversion_type: MetaConversionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ConversionDetails>,
    pub conversion_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionDetails {
    pub currency: String,
    pub value: f64,
}
