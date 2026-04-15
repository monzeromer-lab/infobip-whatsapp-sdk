use infobip_whatsapp_sdk::models::conversions::*;
use infobip_whatsapp_sdk::models::events::*;
use infobip_whatsapp_sdk::models::flows::*;
use infobip_whatsapp_sdk::models::identity::*;
use infobip_whatsapp_sdk::models::templates::v2::*;
use infobip_whatsapp_sdk::{Auth, ClientConfig, WhatsAppClient};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup() -> (MockServer, WhatsAppClient) {
    let server = MockServer::start().await;
    let config = ClientConfig::new(&server.uri(), Auth::ApiKey("key".into())).unwrap();
    let client = WhatsAppClient::new(config);
    (server, client)
}

// --- Templates ---

#[tokio::test]
async fn get_templates() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "templates": [{
            "id": "tmpl-1",
            "businessAccountId": 12345,
            "name": "welcome",
            "language": "en",
            "status": "APPROVED",
            "category": "MARKETING",
            "structure": {
                "body": { "text": "Hello {{1}}" },
                "type": "TEXT"
            },
            "quality": "HIGH"
        }]
    });

    Mock::given(method("GET"))
        .and(path("/whatsapp/2/senders/441134960000/templates"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .templates()
        .get_templates("441134960000")
        .await
        .unwrap();
    let templates = result.templates.unwrap();
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name.as_deref(), Some("welcome"));
    assert_eq!(templates[0].status.as_deref(), Some("APPROVED"));
}

#[tokio::test]
async fn create_template() {
    let (server, client) = setup().await;

    let req = TemplatePublicApiRequest {
        name: "new_template".into(),
        language: "en".into(),
        category: "MARKETING".into(),
        structure: TemplateStructure {
            body: Some(TemplateBodyData {
                text: Some("Hello {{1}}".into()),
                examples: Some(vec!["World".into()]),
                add_security_recommendation: None,
            }),
            header: None,
            footer: None,
            buttons: None,
            carousel: None,
            limited_time_offer: None,
            call_permission_request: None,
            shortening_options: None,
            structure_type: None,
        },
        platform: None,
        validity_period: None,
        sub_category: None,
    };

    let response_body = serde_json::json!({
        "id": "tmpl-new",
        "businessAccountId": 123,
        "name": "new_template",
        "language": "en",
        "status": "PENDING",
        "category": "MARKETING",
        "structure": {
            "body": { "text": "Hello {{1}}" },
            "type": "TEXT"
        }
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/2/senders/441134960000/templates"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .templates()
        .create_template("441134960000", &req)
        .await
        .unwrap();
    assert_eq!(result.name.as_deref(), Some("new_template"));
    assert_eq!(result.status.as_deref(), Some("PENDING"));
}

#[tokio::test]
async fn delete_template() {
    let (server, client) = setup().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/whatsapp/2/senders/441134960000/templates/old_template",
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client
        .templates()
        .delete_template("441134960000", "old_template")
        .await
        .unwrap();
}

// --- Flows ---

#[tokio::test]
async fn get_flows() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "flows": [{
            "id": "flow-1",
            "name": "Survey Flow",
            "businessAccountId": 123,
            "categories": ["SURVEY"],
            "status": "DRAFT"
        }]
    });

    Mock::given(method("GET"))
        .and(path("/whatsapp/1/senders/441134960000/flows"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.flows().get_flows("441134960000").await.unwrap();
    let flows = result.flows.unwrap();
    assert_eq!(flows.len(), 1);
    assert_eq!(flows[0].name.as_deref(), Some("Survey Flow"));
}

#[tokio::test]
async fn create_flow() {
    let (server, client) = setup().await;

    let req = CreateFlowRequest {
        name: "New Flow".into(),
        categories: vec!["SURVEY".into()],
        clone_flow_id: None,
    };

    let response_body = serde_json::json!({
        "id": "flow-new",
        "name": "New Flow",
        "businessAccountId": 123,
        "categories": ["SURVEY"],
        "status": "DRAFT"
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/senders/441134960000/flows"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .flows()
        .create_flow("441134960000", &req)
        .await
        .unwrap();
    assert_eq!(result.name.as_deref(), Some("New Flow"));
    assert_eq!(result.status.as_deref(), Some("DRAFT"));
}

#[tokio::test]
async fn publish_flow() {
    let (server, client) = setup().await;

    Mock::given(method("POST"))
        .and(path(
            "/whatsapp/1/senders/441134960000/flows/flow-1/publish",
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client
        .flows()
        .publish_flow("441134960000", "flow-1")
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_flow() {
    let (server, client) = setup().await;

    Mock::given(method("DELETE"))
        .and(path("/whatsapp/1/senders/441134960000/flows/flow-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client
        .flows()
        .delete_flow("441134960000", "flow-1")
        .await
        .unwrap();
}

// --- Sender ---

#[tokio::test]
async fn get_sender_quality() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "results": [{
            "sender": "441134960000",
            "qualityRating": "GREEN",
            "status": "CONNECTED",
            "currentLimit": "TIER_10K",
            "lastUpdated": "2024-01-01T00:00:00Z"
        }]
    });

    Mock::given(method("GET"))
        .and(path("/whatsapp/1/senders/quality"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .sender()
        .get_quality(&["441134960000"])
        .await
        .unwrap();
    let results = result.results.unwrap();
    assert_eq!(results[0].quality_rating.as_deref(), Some("GREEN"));
}

#[tokio::test]
async fn get_business_info() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "about": "We are a company",
        "address": "123 Main St",
        "description": "Our services",
        "email": "info@example.com",
        "vertical": "TECH",
        "websites": ["https://example.com"],
        "displayName": "Example Corp"
    });

    Mock::given(method("GET"))
        .and(path("/whatsapp/1/senders/441134960000/business-info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .sender()
        .get_business_info("441134960000")
        .await
        .unwrap();
    assert_eq!(result.display_name.as_deref(), Some("Example Corp"));
    assert_eq!(result.websites.as_ref().unwrap().len(), 1);
}

// --- Identity ---

#[tokio::test]
async fn get_identity() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "acknowledged": true,
        "hash": "abc123hash",
        "createdAt": "2024-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path(
            "/whatsapp/1/441134960000/contacts/441134960001/identity",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .identity()
        .get_identity("441134960000", "441134960001")
        .await
        .unwrap();
    assert_eq!(result.acknowledged, Some(true));
    assert_eq!(result.hash.as_deref(), Some("abc123hash"));
}

#[tokio::test]
async fn confirm_identity() {
    let (server, client) = setup().await;

    Mock::given(method("PUT"))
        .and(path(
            "/whatsapp/1/441134960000/contacts/441134960001/identity",
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let confirmation = IdentityConfirmation {
        hash: "abc123hash".into(),
    };
    client
        .identity()
        .confirm_identity("441134960000", "441134960001", &confirmation)
        .await
        .unwrap();
}

// --- Events ---

#[tokio::test]
async fn send_typing_indicator() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "bulkId": "evt-bulk-1",
        "messages": [{
            "messageId": "evt-msg-1",
            "status": { "groupId": 1, "groupName": "PENDING", "id": 7, "name": "PENDING_ENROUTE", "description": "Sent" },
            "destination": "441134960001"
        }]
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let req = WhatsAppEventRequest {
        events: vec![WhatsAppEvent {
            sender: "441134960000".into(),
            destinations: vec![EventDestination {
                to: "441134960001".into(),
                message_id: None,
            }],
            content: EventContent::TypingIndicator { message_id: None },
            options: None,
        }],
        options: None,
    };

    let result = client.events().send_event(&req).await.unwrap();
    assert_eq!(result.bulk_id.as_deref(), Some("evt-bulk-1"));
}

// --- Inbound ---

#[tokio::test]
async fn get_media() {
    let (server, client) = setup().await;

    let media_bytes = b"fake-media-content";

    Mock::given(method("GET"))
        .and(path("/whatsapp/1/senders/441134960000/media/media-123"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(media_bytes.as_slice()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .inbound()
        .get_media("441134960000", "media-123")
        .await
        .unwrap();
    assert_eq!(result, media_bytes);
}

#[tokio::test]
async fn mark_as_read() {
    let (server, client) = setup().await;

    Mock::given(method("POST"))
        .and(path(
            "/whatsapp/1/senders/441134960000/message/msg-123/read",
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client
        .inbound()
        .mark_as_read("441134960000", "msg-123")
        .await
        .unwrap();
}

// --- Media ---

#[tokio::test]
async fn delete_media() {
    let (server, client) = setup().await;

    Mock::given(method("DELETE"))
        .and(path("/whatsapp/1/senders/441134960000/media"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client
        .media()
        .delete_media("441134960000", "https://example.com/media.jpg")
        .await
        .unwrap();
}

// --- Conversions ---

#[tokio::test]
async fn submit_conversion() {
    let (server, client) = setup().await;

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/conversions"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let req = ConversionEventRequest {
        source_id: "source-1".into(),
        conversions: vec![ConversionEventModel {
            ctwa_click_id: "click-123".into(),
            conversion_type: ConversionType::Purchase,
            details: Some(serde_json::json!({"currency": "USD", "value": 99.99})),
            conversion_time: "2024-01-01T00:00:00Z".into(),
        }],
    };

    client.conversions().submit_conversion(&req).await.unwrap();
}

// --- Payments ---

#[tokio::test]
async fn get_payment_status() {
    let (server, client) = setup().await;

    let body = serde_json::json!({
        "referenceId": "ref-1",
        "paymentId": "pay-1",
        "paymentStatus": "CAPTURED",
        "currency": "INR",
        "totalAmountValue": 10000,
        "totalAmountOffset": 100,
        "transactions": [{
            "id": "txn-1",
            "type": "UPI",
            "status": "SUCCESS",
            "createdTimestamp": "2024-01-01T00:00:00Z",
            "updatedTimestamp": "2024-01-01T00:01:00Z"
        }]
    });

    Mock::given(method("GET"))
        .and(path(
            "/whatsapp/1/senders/441134960000/payments/upi/payu/pay-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let result = client
        .payments()
        .get_upi_payu_status("441134960000", "pay-1")
        .await
        .unwrap();
    assert_eq!(result.payment_status.as_deref(), Some("CAPTURED"));
    assert_eq!(result.transactions.as_ref().unwrap().len(), 1);
}

// --- Rate limiting ---

#[tokio::test]
async fn rate_limit_429() {
    let (server, client) = setup().await;

    let error_body = serde_json::json!({
        "requestError": {
            "serviceException": {
                "messageId": "TOO_MANY_REQUESTS",
                "text": "Too many requests"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .respond_with(ResponseTemplate::new(429).set_body_json(error_body))
        .mount(&server)
        .await;

    let msg = infobip_whatsapp_sdk::models::messages::text::TextMessage::new("s", "r", "t");
    let err = client.messages().send_text(&msg).await.unwrap_err();
    let api_err = err.api_error().expect("expected Api error");
    assert_eq!(api_err.status(), 429);
    assert!(api_err.is_rate_limited());
    assert!(api_err.is_retryable());
    assert_eq!(api_err.error_code(), Some("TOO_MANY_REQUESTS"));
    assert_eq!(api_err.message(), Some("Too many requests"));
}
