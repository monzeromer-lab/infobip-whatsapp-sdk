use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
use infobip_sdk::models::messages::text::TextMessage;
use infobip_sdk::models::messages::document::DocumentMessage;
use infobip_sdk::models::messages::image::ImageMessage;
use infobip_sdk::models::messages::audio::AudioMessage;
use infobip_sdk::models::messages::video::VideoMessage;
use infobip_sdk::models::messages::sticker::StickerMessage;
use infobip_sdk::models::messages::location::LocationMessage;
use infobip_sdk::models::messages::template::{
    BulkMessage, FailoverMessage, TemplateContent, TemplateDataContent, TemplateBodyContent,
};
use wiremock::matchers::{method, path, header, body_json};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn success_response() -> serde_json::Value {
    serde_json::json!({
        "to": "441134960001",
        "messageCount": 1,
        "messageId": "msg-id-123",
        "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 7,
            "name": "PENDING_ENROUTE",
            "description": "Message sent to next instance"
        }
    })
}

fn bulk_success_response() -> serde_json::Value {
    serde_json::json!({
        "messages": [{
            "to": "441134960001",
            "messageCount": 1,
            "messageId": "msg-id-456",
            "status": {
                "groupId": 1,
                "groupName": "PENDING",
                "id": 7,
                "name": "PENDING_ENROUTE",
                "description": "Message sent to next instance"
            }
        }],
        "bulkId": "bulk-id-789"
    })
}

async fn setup() -> (MockServer, WhatsAppClient) {
    let server = MockServer::start().await;
    let config = ClientConfig::new(&server.uri(), Auth::ApiKey("test-api-key".into())).unwrap();
    let client = WhatsAppClient::new(config);
    (server, client)
}

#[tokio::test]
async fn send_text_message() {
    let (server, client) = setup().await;

    let msg = TextMessage::new("441134960000", "441134960001", "Hello world");

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .and(header("Authorization", "App test-api-key"))
        .and(body_json(&msg))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.messages().send_text(&msg).await.unwrap();
    assert_eq!(result.to.as_deref(), Some("441134960001"));
    assert_eq!(result.message_id.as_deref(), Some("msg-id-123"));
    assert_eq!(result.message_count, Some(1));
    let status = result.status.unwrap();
    assert_eq!(status.group_name.as_deref(), Some("PENDING"));
}

#[tokio::test]
async fn send_text_with_preview_url() {
    let (server, client) = setup().await;

    let mut msg = TextMessage::new("sender", "recipient", "Check https://example.com");
    msg.content.preview_url = Some(true);

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_text(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_document_message() {
    let (server, client) = setup().await;

    let mut msg = DocumentMessage::new("sender", "recipient", "https://example.com/doc.pdf");
    msg.content.caption = Some("Invoice".into());
    msg.content.filename = Some("invoice.pdf".into());

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/document"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_document(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_image_message() {
    let (server, client) = setup().await;

    let msg = ImageMessage::new("sender", "recipient", "https://example.com/img.jpg");

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/image"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_image(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_audio_message() {
    let (server, client) = setup().await;

    let msg = AudioMessage::new("sender", "recipient", "https://example.com/audio.mp3");

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/audio"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_audio(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_video_message() {
    let (server, client) = setup().await;

    let msg = VideoMessage::new("sender", "recipient", "https://example.com/video.mp4");

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/video"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_video(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_sticker_message() {
    let (server, client) = setup().await;

    let msg = StickerMessage::new("sender", "recipient", "https://example.com/sticker.webp");

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/sticker"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_sticker(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_location_message() {
    let (server, client) = setup().await;

    let mut msg = LocationMessage::new("sender", "recipient", 44.7866, 20.4489);
    msg.content.name = Some("Belgrade".into());
    msg.content.address = Some("Knez Mihailova".into());

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/location"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_location(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_template_message() {
    let (server, client) = setup().await;

    let msg = BulkMessage {
        messages: vec![FailoverMessage {
            from: "441134960000".into(),
            to: "441134960001".into(),
            message_id: None,
            content: TemplateContent {
                template_name: "welcome_msg".into(),
                template_data: TemplateDataContent {
                    body: TemplateBodyContent {
                        placeholders: vec!["John".into(), "Doe".into()],
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
    };

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/template"))
        .respond_with(ResponseTemplate::new(200).set_body_json(bulk_success_response()))
        .mount(&server)
        .await;

    let result = client.messages().send_template(&msg).await.unwrap();
    assert_eq!(result.bulk_id.as_deref(), Some("bulk-id-789"));
    assert_eq!(result.messages.as_ref().unwrap().len(), 1);
}

#[tokio::test]
async fn api_error_400_parsed() {
    let (server, client) = setup().await;

    let error_body = serde_json::json!({
        "requestError": {
            "serviceException": {
                "messageId": "BAD_REQUEST",
                "text": "Bad request",
                "validationErrors": {
                    "content.text": ["must not be blank"]
                }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .respond_with(ResponseTemplate::new(400).set_body_json(error_body))
        .mount(&server)
        .await;

    let msg = TextMessage::new("sender", "recipient", "");
    let err = client.messages().send_text(&msg).await.unwrap_err();
    let api_err = err.api_error().expect("expected Api error");
    assert_eq!(api_err.status(), 400);
    assert!(api_err.is_client_error());
    assert!(!api_err.is_retryable());
    assert_eq!(api_err.error_code(), Some("BAD_REQUEST"));
    let exc = api_err.as_exception().expect("expected exception body");
    let svc = exc.request_error.as_ref().unwrap().service_exception.as_ref().unwrap();
    assert_eq!(svc.message_id.as_deref(), Some("BAD_REQUEST"));
}

#[tokio::test]
async fn api_error_401_unauthorized() {
    let (server, client) = setup().await;

    let error_body = serde_json::json!({
        "requestError": {
            "serviceException": {
                "messageId": "UNAUTHORIZED",
                "text": "Invalid login details"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body))
        .mount(&server)
        .await;

    let msg = TextMessage::new("s", "r", "t");
    let err = client.messages().send_text(&msg).await.unwrap_err();
    let api_err = err.api_error().expect("expected Api error");
    assert_eq!(api_err.status(), 401);
    assert!(api_err.is_unauthorized());
    assert!(!api_err.is_retryable());
}

#[tokio::test]
async fn api_error_unparseable_body() {
    let (server, client) = setup().await;

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&server)
        .await;

    let msg = TextMessage::new("s", "r", "t");
    let err = client.messages().send_text(&msg).await.unwrap_err();
    let api_err = err.api_error().expect("expected Api error");
    assert_eq!(api_err.status(), 500);
    assert!(api_err.is_server_error());
    assert!(api_err.is_retryable());
    assert_eq!(api_err.raw_body(), Some("Internal Server Error"));
}

#[tokio::test]
async fn auth_header_sent_correctly() {
    let server = MockServer::start().await;
    let config = ClientConfig::new(
        &server.uri(),
        Auth::Basic { username: "user".into(), password: "pass".into() },
    ).unwrap();
    let client = WhatsAppClient::new(config);

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/text"))
        .and(header("Authorization", "Basic dXNlcjpwYXNz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .expect(1)
        .mount(&server)
        .await;

    let msg = TextMessage::new("s", "r", "t");
    client.messages().send_text(&msg).await.unwrap();
}
