//! API error response models.
//!
//! Infobip uses two error formats depending on the endpoint:
//! - [`ApiExceptionResponse`] - The `requestError.serviceException` format (most endpoints).
//! - [`ApiErrorResponse`] - The `errorCode/violations` format (template and management endpoints).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error response using the `requestError.serviceException` envelope format.
///
/// Most WhatsApp API endpoints return errors in this shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiExceptionResponse {
    pub request_error: Option<ApiRequestError>,
}

/// Wrapper containing the service exception details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiRequestError {
    pub service_exception: Option<ApiRequestErrorDetails>,
}

/// Detailed error information including message ID, description, and per-field validation errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiRequestErrorDetails {
    /// Error code identifier (e.g. "BAD_REQUEST").
    pub message_id: Option<String>,
    /// Human-readable error description.
    pub text: Option<String>,
    /// Per-field validation errors, keyed by JSON path (e.g. "content.text").
    pub validation_errors: Option<HashMap<String, Vec<String>>>,
}

/// Error response using the `errorCode/violations` format.
///
/// Used by template management and some newer API endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    /// Numeric error code as a string (e.g. "40001").
    pub error_code: Option<String>,
    pub description: Option<String>,
    /// Suggested corrective action.
    pub action: Option<String>,
    /// List of field-level constraint violations.
    pub violations: Option<Vec<ApiErrorViolation>>,
    /// Links to relevant documentation or resources.
    pub resources: Option<Vec<ApiErrorResource>>,
}

/// A single field validation violation within an [`ApiErrorResponse`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorViolation {
    pub property: Option<String>,
    pub violation: Option<String>,
}

/// A documentation or help resource linked from an error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResource {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_service_exception_response() {
        let json = r#"{
            "requestError": {
                "serviceException": {
                    "messageId": "BAD_REQUEST",
                    "text": "Bad request",
                    "validationErrors": {
                        "content.text": ["must not be blank"]
                    }
                }
            }
        }"#;
        let err: ApiExceptionResponse = serde_json::from_str(json).unwrap();
        let svc = err.request_error.unwrap().service_exception.unwrap();
        assert_eq!(svc.message_id.as_deref(), Some("BAD_REQUEST"));
        assert_eq!(svc.text.as_deref(), Some("Bad request"));
        let errors = svc.validation_errors.unwrap();
        assert_eq!(errors["content.text"], vec!["must not be blank"]);
    }

    #[test]
    fn deserialize_api_error_response() {
        let json = r#"{
            "errorCode": "40001",
            "description": "Invalid request",
            "action": "Check your request parameters",
            "violations": [
                { "property": "name", "violation": "must not be null" }
            ],
            "resources": [
                { "name": "API Docs", "url": "https://example.com/docs" }
            ]
        }"#;
        let err: ApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err.error_code.as_deref(), Some("40001"));
        assert_eq!(err.description.as_deref(), Some("Invalid request"));
        let violations = err.violations.unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].property.as_deref(), Some("name"));
        let resources = err.resources.unwrap();
        assert_eq!(resources[0].name.as_deref(), Some("API Docs"));
    }

    #[test]
    fn deserialize_empty_error() {
        let json = "{}";
        let err: ApiExceptionResponse = serde_json::from_str(json).unwrap();
        assert!(err.request_error.is_none());
    }
}
