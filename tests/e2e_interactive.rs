use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
use infobip_sdk::models::interactive::common::{InteractiveBody, InteractiveFooter};
use infobip_sdk::models::interactive::buttons::*;
use infobip_sdk::models::interactive::list::*;
use infobip_sdk::models::interactive::product::*;
use infobip_sdk::models::interactive::location_request::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn success_response() -> serde_json::Value {
    serde_json::json!({
        "to": "441134960001",
        "messageCount": 1,
        "messageId": "msg-interactive-1",
        "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 7,
            "name": "PENDING_ENROUTE",
            "description": "Message sent to next instance"
        }
    })
}

async fn setup() -> (MockServer, WhatsAppClient) {
    let server = MockServer::start().await;
    let config = ClientConfig::new(&server.uri(), Auth::ApiKey("key".into())).unwrap();
    let client = WhatsAppClient::new(config);
    (server, client)
}

#[tokio::test]
async fn send_interactive_buttons() {
    let (server, client) = setup().await;

    let msg = InteractiveButtonsMessage {
        from: "sender".into(),
        to: "recipient".into(),
        message_id: None,
        content: InteractiveButtonsContent {
            body: InteractiveBody { text: "Pick one".into() },
            action: InteractiveButtonsAction {
                buttons: vec![
                    ReplyButton { button_type: "REPLY".into(), id: "1".into(), title: "Option A".into() },
                    ReplyButton { button_type: "REPLY".into(), id: "2".into(), title: "Option B".into() },
                ],
            },
            header: Some(InteractiveButtonsHeader::Text { text: "Survey".into() }),
            footer: Some(InteractiveFooter { text: "Tap to respond".into() }),
        },
        callback_data: Some("survey-cb".into()),
        notify_url: None,
        url_options: None,
        entity_id: None,
        application_id: None,
        context: None,
    };

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/interactive/buttons"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.interactive().send_buttons(&msg).await.unwrap();
    assert_eq!(result.message_id.as_deref(), Some("msg-interactive-1"));
}

#[tokio::test]
async fn send_interactive_list() {
    let (server, client) = setup().await;

    let msg = InteractiveListMessage {
        from: "sender".into(),
        to: "recipient".into(),
        message_id: None,
        content: InteractiveListContent {
            body: InteractiveBody { text: "Choose a product".into() },
            action: InteractiveListAction {
                title: "Products".into(),
                sections: vec![InteractiveListSection {
                    title: Some("Category A".into()),
                    rows: vec![
                        InteractiveListRow {
                            id: "prod-1".into(),
                            title: "Product 1".into(),
                            description: Some("First product".into()),
                        },
                        InteractiveListRow {
                            id: "prod-2".into(),
                            title: "Product 2".into(),
                            description: None,
                        },
                    ],
                }],
            },
            header: Some(InteractiveListHeader::Text { text: "Our catalog".into() }),
            footer: Some(InteractiveFooter { text: "Scroll to see more".into() }),
        },
        callback_data: None,
        notify_url: None,
        url_options: None,
        entity_id: None,
        application_id: None,
        context: None,
    };

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/interactive/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.interactive().send_list(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_interactive_product() {
    let (server, client) = setup().await;

    let msg = InteractiveProductMessage {
        from: "sender".into(),
        to: "recipient".into(),
        message_id: None,
        content: InteractiveProductContent {
            action: InteractiveProductAction {
                catalog_id: "catalog-1".into(),
                product_retailer_id: "sku-100".into(),
            },
            body: Some(InteractiveBody { text: "Check this out".into() }),
            footer: None,
        },
        callback_data: None,
        notify_url: None,
        url_options: None,
        entity_id: None,
        application_id: None,
        context: None,
    };

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/interactive/product"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.interactive().send_product(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn send_location_request() {
    let (server, client) = setup().await;

    let msg = InteractiveLocationRequestMessage {
        from: "sender".into(),
        to: "recipient".into(),
        message_id: None,
        content: InteractiveLocationRequestContent {
            body: InteractiveBody { text: "Share your location".into() },
        },
        callback_data: None,
        notify_url: None,
        url_options: None,
        entity_id: None,
        application_id: None,
        context: None,
    };

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/interactive/location-request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.interactive().send_location_request(&msg).await.unwrap();
    assert!(result.message_id.is_some());
}

#[tokio::test]
async fn interactive_buttons_error_handling() {
    let (server, client) = setup().await;

    let error_body = serde_json::json!({
        "requestError": {
            "serviceException": {
                "messageId": "BAD_REQUEST",
                "text": "Bad request",
                "validationErrors": {
                    "content.action.buttons": ["size must be between 1 and 3"]
                }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/whatsapp/1/message/interactive/buttons"))
        .respond_with(ResponseTemplate::new(400).set_body_json(error_body))
        .mount(&server)
        .await;

    let msg = InteractiveButtonsMessage {
        from: "s".into(),
        to: "r".into(),
        message_id: None,
        content: InteractiveButtonsContent {
            body: InteractiveBody { text: "x".into() },
            action: InteractiveButtonsAction { buttons: vec![] },
            header: None,
            footer: None,
        },
        callback_data: None,
        notify_url: None,
        url_options: None,
        entity_id: None,
        application_id: None,
        context: None,
    };

    let err = client.interactive().send_buttons(&msg).await.unwrap_err();
    let api_err = err.api_error().expect("expected Api error");
    assert_eq!(api_err.status(), 400);
    assert!(api_err.is_client_error());
    let exc = api_err.as_exception().expect("expected exception body");
    let svc = exc.request_error.as_ref().unwrap().service_exception.as_ref().unwrap();
    let errs = svc.validation_errors.as_ref().unwrap();
    assert!(errs.contains_key("content.action.buttons"));
}
