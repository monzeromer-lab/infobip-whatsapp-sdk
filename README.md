# infobip-whatsapp-sdk

A type-safe Rust SDK for the [Infobip WhatsApp API](https://www.infobip.com/docs/api/channels/whatsapp).

Full coverage of the Infobip WhatsApp Business API: send messages, manage templates, handle flows, process webhooks, and more.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
infobip-whatsapp-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};
use infobip_sdk::models::messages::text::TextMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new(
        "https://your-base-url.api.infobip.com",
        Auth::ApiKey("your-api-key".into()),
    )?;
    let client = WhatsAppClient::new(config);

    let msg = TextMessage::new("441134960000", "441134960001", "Hello from Rust!");
    let response = client.messages().send_text(&msg).await?;

    println!("Message ID: {:?}", response.message_id);
    Ok(())
}
```

## Authentication

The SDK supports all four Infobip authentication methods:

```rust
use infobip_sdk::Auth;

// API Key (most common - from your Infobip dashboard)
let auth = Auth::ApiKey("your-api-key".into());

// Basic auth
let auth = Auth::Basic {
    username: "user".into(),
    password: "pass".into(),
};

// OAuth2 Bearer token
let auth = Auth::Bearer("oauth-token".into());

// IBSSO token
let auth = Auth::IbSso("sso-token".into());
```

## Sending Messages

### Text

```rust
use infobip_sdk::models::messages::text::TextMessage;

let mut msg = TextMessage::new("441134960000", "441134960001", "Check this out: https://example.com");
msg.content.preview_url = Some(true); // Enable link preview
let result = client.messages().send_text(&msg).await?;
```

### Image

```rust
use infobip_sdk::models::messages::image::ImageMessage;

let mut msg = ImageMessage::new("441134960000", "441134960001", "https://example.com/photo.jpg");
msg.content.caption = Some("Look at this!".into());
let result = client.messages().send_image(&msg).await?;
```

### Document

```rust
use infobip_sdk::models::messages::document::DocumentMessage;

let mut msg = DocumentMessage::new("441134960000", "441134960001", "https://example.com/invoice.pdf");
msg.content.filename = Some("invoice.pdf".into());
msg.content.caption = Some("Your invoice".into());
let result = client.messages().send_document(&msg).await?;
```

### Location

```rust
use infobip_sdk::models::messages::location::LocationMessage;

let mut msg = LocationMessage::new("441134960000", "441134960001", 44.7866, 20.4489);
msg.content.name = Some("Belgrade".into());
msg.content.address = Some("Knez Mihailova".into());
let result = client.messages().send_location(&msg).await?;
```

### Template

```rust
use infobip_sdk::models::messages::template::*;

let msg = BulkMessage {
    messages: vec![FailoverMessage {
        from: "441134960000".into(),
        to: "441134960001".into(),
        message_id: None,
        content: TemplateContent {
            template_name: "welcome_msg".into(),
            template_data: TemplateDataContent {
                body: TemplateBodyContent {
                    placeholders: vec!["John".into()],
                },
                header: Some(TemplateHeaderContent::Image {
                    media_url: "https://example.com/header.jpg".into(),
                }),
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
let result = client.messages().send_template(&msg).await?;
```

## Interactive Messages

### Reply Buttons

```rust
use infobip_sdk::models::interactive::common::*;
use infobip_sdk::models::interactive::buttons::*;

let msg = InteractiveButtonsMessage {
    from: "441134960000".into(),
    to: "441134960001".into(),
    message_id: None,
    content: InteractiveButtonsContent {
        body: InteractiveBody { text: "Rate your experience".into() },
        action: InteractiveButtonsAction {
            buttons: vec![
                ReplyButton { button_type: "REPLY".into(), id: "1".into(), title: "Great".into() },
                ReplyButton { button_type: "REPLY".into(), id: "2".into(), title: "OK".into() },
                ReplyButton { button_type: "REPLY".into(), id: "3".into(), title: "Poor".into() },
            ],
        },
        header: Some(InteractiveButtonsHeader::Text { text: "Feedback".into() }),
        footer: Some(InteractiveFooter { text: "Tap to respond".into() }),
    },
    callback_data: None,
    notify_url: None,
    url_options: None,
    entity_id: None,
    application_id: None,
    context: None,
};
let result = client.interactive().send_buttons(&msg).await?;
```

### List

```rust
use infobip_sdk::models::interactive::common::*;
use infobip_sdk::models::interactive::list::*;

let msg = InteractiveListMessage {
    from: "441134960000".into(),
    to: "441134960001".into(),
    message_id: None,
    content: InteractiveListContent {
        body: InteractiveBody { text: "Choose a product".into() },
        action: InteractiveListAction {
            title: "View Products".into(),
            sections: vec![InteractiveListSection {
                title: Some("Electronics".into()),
                rows: vec![
                    InteractiveListRow { id: "1".into(), title: "Laptop".into(), description: Some("$999".into()) },
                    InteractiveListRow { id: "2".into(), title: "Phone".into(), description: Some("$699".into()) },
                ],
            }],
        },
        header: None,
        footer: None,
    },
    callback_data: None, notify_url: None, url_options: None,
    entity_id: None, application_id: None, context: None,
};
let result = client.interactive().send_list(&msg).await?;
```

## Template Management

```rust
use infobip_sdk::models::templates::v2::*;

// List templates
let templates = client.templates().get_templates("441134960000").await?;

// Create a template
let req = TemplatePublicApiRequest {
    name: "order_update".into(),
    language: "en".into(),
    category: "UTILITY".into(),
    structure: TemplateStructure {
        body: Some(TemplateBodyData {
            text: Some("Your order {{1}} has been shipped!".into()),
            examples: Some(vec!["#12345".into()]),
            add_security_recommendation: None,
        }),
        header: None, footer: None, buttons: None, carousel: None,
        limited_time_offer: None, call_permission_request: None,
        shortening_options: None, structure_type: None,
    },
    platform: None, validity_period: None, sub_category: None,
};
let created = client.templates().create_template("441134960000", &req).await?;

// Delete a template
client.templates().delete_template("441134960000", "old_template").await?;
```

## Error Handling

The SDK provides rich error types for classification, logging, and retry logic:

```rust
match client.messages().send_text(&msg).await {
    Ok(info) => {
        println!("Sent: {:?}", info.message_id);
    }
    Err(e) => {
        // Quick classification
        if e.is_retryable() {
            // 429 rate-limited, 5xx server error, or network timeout
            println!("Will retry: {e}");
            return;
        }

        // Detailed inspection
        if let Some(api_err) = e.api_error() {
            println!("HTTP {}", api_err.status());
            println!("Code: {:?}", api_err.error_code());
            println!("Message: {:?}", api_err.message());

            // Status helpers
            if api_err.is_unauthorized() { /* re-authenticate */ }
            if api_err.is_rate_limited() { /* back off */ }
            if api_err.is_not_found() { /* resource doesn't exist */ }

            // Access validation errors
            if let Some(exc) = api_err.as_exception() {
                if let Some(svc) = exc.request_error.as_ref()
                    .and_then(|r| r.service_exception.as_ref())
                {
                    for (field, msgs) in svc.validation_errors.iter().flatten() {
                        println!("  {field}: {msgs:?}");
                    }
                }
            }
        }

        // Works with Sentry, anyhow, eyre out of the box
        // sentry::capture_error(&e);
    }
}
```

## Webhook Models

Deserialize incoming webhooks from Infobip:

```rust
use infobip_sdk::models::webhooks::{DeliveryResults, SeenResults};
use infobip_sdk::models::inbound::WhatsAppInboundMessages;

// Delivery report webhook
fn handle_delivery(body: &str) {
    let report: DeliveryResults = serde_json::from_str(body).unwrap();
    for r in report.results.unwrap_or_default() {
        println!("{}: {:?}", r.message_id.unwrap_or_default(),
            r.status.map(|s| s.group_name));
    }
}

// Seen report webhook
fn handle_seen(body: &str) {
    let report: SeenResults = serde_json::from_str(body).unwrap();
    for r in report.results.unwrap_or_default() {
        println!("{} seen at {:?}", r.message_id.unwrap_or_default(), r.seen_at);
    }
}
```

## Custom HTTP Client

Configure timeouts, proxies, or TLS settings:

```rust
use infobip_sdk::{Auth, ClientConfig, WhatsAppClient};

let http = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .connect_timeout(std::time::Duration::from_secs(5))
    .build()?;

let config = ClientConfig::new(
    "https://your-base-url.api.infobip.com",
    Auth::ApiKey("key".into()),
)?;

let client = WhatsAppClient::with_http_client(config, http);
```

## API Coverage

| API Domain | Methods | Description |
|---|---|---|
| Messages | 9 | Text, document, image, audio, video, sticker, location, contact, template |
| Interactive | 12 | Buttons, list, product, multi-product, flow, URL button, voice button, carousel, order details/status, location request, call permission |
| Templates | 8 | CRUD (v2), list all, deprecated v1 |
| Flows | 10 | CRUD, publish, deprecate, preview, JSON management, AI generate |
| Events | 1 | Typing indicators |
| Inbound | 3 | Download media, head media, mark as read |
| Payments | 3 | UPI PayU, Brazil, UPI status |
| Sender | 7 | Quality, business info, logo, calling permissions, public key |
| Identity | 3 | Get, confirm, delete |
| Conversions | 2 | Standard and WABA conversion events |
| Media | 1 | Delete media |
| Registration | 3 | Register sender, send verification, verify code |

**Total: 62 API methods covering 100% of the Infobip WhatsApp OpenAPI spec.**

## License

MIT
