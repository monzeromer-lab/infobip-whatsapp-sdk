//! WhatsApp Business API modules.
//!
//! This module provides access to all WhatsApp Business API endpoints organized
//! by functional area. Each sub-module exposes an API struct that can be obtained
//! from a [`WhatsAppClient`](crate::WhatsAppClient) instance.
//!
//! # Available APIs
//!
//! | API | Description |
//! |-----|-------------|
//! | [`MessagesApi`] | Send text, media, location, contact, and template messages |
//! | [`InteractiveApi`] | Send interactive messages (buttons, lists, carousels, flows, etc.) |
//! | [`EventsApi`] | Send events such as typing indicators |
//! | [`InboundApi`] | Download inbound media and mark messages as read |
//! | [`TemplatesApi`] | Create, list, update, and delete message templates |
//! | [`FlowsApi`] | Manage WhatsApp Flows (create, publish, deprecate, etc.) |
//! | [`PaymentsApi`] | Query payment statuses (UPI, Brazil) |
//! | [`SenderApi`] | Manage sender quality, business profile, and encryption keys |
//! | [`IdentityApi`] | Manage end-user identity information |
//! | [`ConversionsApi`] | Submit conversion tracking events |
//! | [`MediaApi`] | Delete previously uploaded media |
//! | [`RegistrationApi`] | Register and verify sender numbers |

pub mod messages;
pub mod interactive;
pub mod events;
pub mod inbound;
pub mod templates;
pub mod flows;
pub mod payments;
pub mod sender;
pub mod identity;
pub mod conversions;
pub mod media;
pub mod registration;

pub use messages::MessagesApi;
pub use interactive::InteractiveApi;
pub use events::EventsApi;
pub use inbound::InboundApi;
pub use templates::TemplatesApi;
pub use flows::FlowsApi;
pub use payments::PaymentsApi;
pub use sender::SenderApi;
pub use identity::IdentityApi;
pub use conversions::ConversionsApi;
pub use media::MediaApi;
pub use registration::RegistrationApi;
