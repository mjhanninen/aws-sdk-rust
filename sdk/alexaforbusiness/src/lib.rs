#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <p>Alexa for Business helps you use Alexa in your organization. Alexa for Business provides you with the tools
//! to manage Alexa devices, enroll your users, and assign skills, at scale. You can build your
//! own context-aware voice skills using the Alexa Skills Kit and the Alexa for Business API operations.
//! You can also make these available as private skills for your organization. Alexa for Business makes it
//! efficient to voice-enable your products and services, thus providing context-aware voice
//! experiences for your customers. Device makers building with the Alexa Voice Service (AVS)
//! can create fully integrated solutions, register their products with Alexa for Business, and manage them
//! as shared devices in their organization. </p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod idempotency_token;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("alexaforbusiness", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
