//! WhatsApp Business API data models.
//!
//! This module contains all request and response types used by the Infobip WhatsApp API:
//!
//! - [`common`] - Shared types: message status, scheduling, URL options, platform IDs.
//! - [`errors`] - API error response formats (`ApiExceptionResponse`, `ApiErrorResponse`).
//! - [`messages`] - Outbound message types: text, image, document, audio, video, sticker, location, contact, template.
//! - [`interactive`] - Interactive message types: buttons, lists, products, flows, carousels, order details.
//! - [`events`] - WhatsApp event models (e.g. typing indicators).
//! - [`payments`] - Payment status response models.
//! - [`templates`] - Template management (create, edit, list) via the v2 API.
//! - [`flows`] - WhatsApp Flows CRUD and generation models.
//! - [`sender`] - Sender quality, business info, and public key models.
//! - [`registration`] - Phone number registration and OTP verification models.
//! - [`identity`] - User identity change detection and confirmation.
//! - [`conversions`] - Click-to-WhatsApp conversion tracking models.
//! - [`inbound`] - Inbound (received) message and media models.
//! - [`webhooks`] - Webhook payloads for delivery reports and seen reports.

pub mod common;
pub mod errors;
pub mod messages;
pub mod interactive;
pub mod events;
pub mod payments;
pub mod templates;
pub mod flows;
pub mod sender;
pub mod registration;
pub mod identity;
pub mod conversions;
pub mod inbound;
pub mod webhooks;
