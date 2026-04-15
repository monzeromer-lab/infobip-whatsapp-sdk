//! # Infobip WhatsApp SDK
//!
//! A type-safe Rust client for the [Infobip WhatsApp API](https://www.infobip.com/docs/api/channels/whatsapp).
//!
//! This SDK provides full coverage of the Infobip WhatsApp Business API, including
//! sending messages (text, media, templates, interactive), managing templates and flows,
//! handling webhooks, and more.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use infobip_whatsapp_sdk::{Auth, ClientConfig, WhatsAppClient};
//! use infobip_whatsapp_sdk::models::messages::text::TextMessage;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with API key authentication
//!     let config = ClientConfig::new(
//!         "https://your-base-url.api.infobip.com",
//!         Auth::ApiKey("your-api-key".into()),
//!     )?;
//!     let client = WhatsAppClient::new(config);
//!
//!     // Send a text message
//!     let msg = TextMessage::new("441134960000", "441134960001", "Hello from Rust!");
//!     let response = client.messages().send_text(&msg).await?;
//!
//!     println!("Message ID: {:?}", response.message_id);
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! The SDK supports all four Infobip authentication methods:
//!
//! ```rust
//! use infobip_whatsapp_sdk::Auth;
//!
//! // API Key (most common)
//! let auth = Auth::ApiKey("your-api-key".into());
//!
//! // Basic auth
//! let auth = Auth::Basic {
//!     username: "user".into(),
//!     password: "pass".into(),
//! };
//!
//! // OAuth2 Bearer token
//! let auth = Auth::Bearer("oauth-token".into());
//!
//! // IBSSO token
//! let auth = Auth::IbSso("sso-token".into());
//! ```
//!
//! ## Error Handling
//!
//! Every API call returns [`Result<T, InfobipError>`]. The error type provides
//! rich inspection for logging, monitoring (Sentry, Datadog), and retry logic:
//!
//! ```rust,no_run
//! # use infobip_whatsapp_sdk::{Auth, ClientConfig, WhatsAppClient};
//! # use infobip_whatsapp_sdk::models::messages::text::TextMessage;
//! # async fn example() {
//! # let config = ClientConfig::new("https://example.com", Auth::ApiKey("k".into())).unwrap();
//! # let client = WhatsAppClient::new(config);
//! # let msg = TextMessage::new("s", "r", "t");
//! match client.messages().send_text(&msg).await {
//!     Ok(info) => println!("Sent: {:?}", info.message_id),
//!     Err(e) => {
//!         // Quick classification
//!         if e.is_retryable() {
//!             // 429 rate-limited, 5xx server error, or network timeout
//!             println!("Retryable error, will try again");
//!         }
//!
//!         // Drill into API-specific error details
//!         if let Some(api_err) = e.api_error() {
//!             println!("HTTP {}", api_err.status());
//!             println!("Code: {:?}", api_err.error_code());
//!             println!("Message: {:?}", api_err.message());
//!
//!             if api_err.is_rate_limited() {
//!                 // Back off and retry
//!             }
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ## API Domains
//!
//! Access each API domain through the client:
//!
//! | Accessor | API Domain | Description |
//! |---|---|---|
//! | [`client.messages()`](WhatsAppClient::messages) | Messages | Send text, media, template, contact, location messages |
//! | [`client.interactive()`](WhatsAppClient::interactive) | Interactive | Buttons, lists, products, flows, carousels |
//! | [`client.templates()`](WhatsAppClient::templates) | Templates | Create, read, update, delete message templates |
//! | [`client.flows()`](WhatsAppClient::flows) | Flows | Manage WhatsApp Flows |
//! | [`client.events()`](WhatsAppClient::events) | Events | Send typing indicators |
//! | [`client.inbound()`](WhatsAppClient::inbound) | Inbound | Download media, mark messages as read |
//! | [`client.payments()`](WhatsAppClient::payments) | Payments | Check payment transaction status |
//! | [`client.sender()`](WhatsAppClient::sender) | Sender | Quality ratings, business info, public keys |
//! | [`client.identity()`](WhatsAppClient::identity) | Identity | Manage end-user identity verification |
//! | [`client.conversions()`](WhatsAppClient::conversions) | Conversions | Track ad conversion events |
//! | [`client.media()`](WhatsAppClient::media) | Media | Delete uploaded media |
//! | [`client.registration()`](WhatsAppClient::registration) | Registration | Register and verify sender numbers |
//!
//! ## Webhook Models
//!
//! The [`models::webhooks`] module provides types for deserializing webhook payloads
//! that Infobip sends to your server (delivery reports, seen reports, etc.):
//!
//! ```rust,no_run
//! use infobip_whatsapp_sdk::models::webhooks::DeliveryResults;
//!
//! // In your webhook handler:
//! fn handle_delivery_report(body: &str) {
//!     let report: DeliveryResults = serde_json::from_str(body).unwrap();
//!     for result in report.results.unwrap_or_default() {
//!         println!("Message {} status: {:?}",
//!             result.message_id.unwrap_or_default(),
//!             result.status.map(|s| s.group_name));
//!     }
//! }
//! ```

pub mod auth;
pub mod client;
pub mod config;
pub mod error;

pub mod models;
pub mod api;

pub use auth::Auth;
pub use client::WhatsAppClient;
pub use config::ClientConfig;
pub use error::{InfobipError, ApiError, ApiErrorKind};
