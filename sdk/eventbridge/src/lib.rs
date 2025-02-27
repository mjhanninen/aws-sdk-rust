#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <p>Amazon EventBridge helps you to respond to state changes in your Amazon Web Services resources. When your
//! resources change state, they automatically send events to an event stream. You can create
//! rules that match selected events in the stream and route them to targets to take action. You
//! can also use rules to take action on a predetermined schedule. For example, you can configure
//! rules to:</p>
//! <ul>
//! <li>
//! <p>Automatically invoke an Lambda function to update DNS entries when an event
//! notifies you that Amazon EC2 instance enters the running state.</p>
//! </li>
//! <li>
//! <p>Direct specific API records from CloudTrail to an Amazon Kinesis data stream for
//! detailed analysis of potential security or availability risks.</p>
//! </li>
//! <li>
//! <p>Periodically invoke a built-in target to create a snapshot of an Amazon EBS
//! volume.</p>
//! </li>
//! </ul>
//! <p>For more information about the features of Amazon EventBridge, see the <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide">Amazon EventBridge User
//! Guide</a>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
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
    aws_http::user_agent::ApiMetadata::new("eventbridge", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
