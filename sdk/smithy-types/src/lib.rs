/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![feature(int_roundings)]

pub mod base64;
pub mod instant;
pub mod primitive;
pub mod retry;

use std::collections::HashMap;

pub use crate::instant::Instant;

#[derive(Debug, PartialEq, Clone)]
pub struct Blob {
    inner: Vec<u8>,
}

impl Blob {
    pub fn new<T: Into<Vec<u8>>>(input: T) -> Self {
        Blob {
            inner: input.into(),
        }
    }
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

impl AsRef<[u8]> for Blob {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

/* ANCHOR: document */

/// Document Type
///
/// Document types represents protocol-agnostic open content that is accessed like JSON data.
/// Open content is useful for modeling unstructured data that has no schema, data that can't be
/// modeled using rigid types, or data that has a schema that evolves outside of the purview of a model.
/// The serialization format of a document is an implementation detail of a protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum Document {
    Object(HashMap<String, Document>),
    Array(Vec<Document>),
    Number(Number),
    String(String),
    Bool(bool),
    Null,
}

/// A number type that implements Javascript / JSON semantics, modeled on serde_json:
/// https://docs.serde.rs/src/serde_json/number.rs.html#20-22
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

macro_rules! to_num_fn {
    ($name:ident, $typ:ident) => {
        /// Converts to a `$typ`. This conversion may be lossy.
        pub fn $name(&self) -> $typ {
            match self {
                Number::PosInt(val) => *val as $typ,
                Number::NegInt(val) => *val as $typ,
                Number::Float(val) => *val as $typ,
            }
        }
    };
}

impl Number {
    to_num_fn!(to_f32, f32);
    to_num_fn!(to_f64, f64);

    to_num_fn!(to_i8, i8);
    to_num_fn!(to_i16, i16);
    to_num_fn!(to_i32, i32);
    to_num_fn!(to_i64, i64);

    to_num_fn!(to_u8, u8);
    to_num_fn!(to_u16, u16);
    to_num_fn!(to_u32, u32);
    to_num_fn!(to_u64, u64);
}

/* ANCHOR_END: document */

pub use error::Error;

pub mod error {
    use crate::retry::{ErrorKind, ProvideErrorKind};
    use std::collections::HashMap;
    use std::fmt;
    use std::fmt::{Display, Formatter};

    /// Generic Error type
    ///
    /// For many services, Errors are modeled. However, many services only partially model errors or don't
    /// model errors at all. In these cases, the SDK will return this generic error type to expose the
    /// `code`, `message` and `request_id`.
    #[derive(Debug, Eq, PartialEq, Default, Clone)]
    pub struct Error {
        code: Option<String>,
        message: Option<String>,
        request_id: Option<String>,
        extras: HashMap<&'static str, String>,
    }

    #[derive(Default)]
    pub struct Builder {
        inner: Error,
    }

    impl Builder {
        pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
            self.inner.message = Some(message.into());
            self
        }

        pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
            self.inner.code = Some(code.into());
            self
        }

        pub fn request_id(&mut self, request_id: impl Into<String>) -> &mut Self {
            self.inner.request_id = Some(request_id.into());
            self
        }

        /// Set a custom field on the error metadata
        ///
        /// Typically, these will be accessed with an extension trait:
        /// ```rust
        /// use smithy_types::Error;
        /// const HOST_ID: &str = "host_id";
        /// trait S3ErrorExt {
        ///     fn extended_request_id(&self) -> Option<&str>;
        /// }
        ///
        /// impl S3ErrorExt for Error {
        ///     fn extended_request_id(&self) -> Option<&str> {
        ///         self.extra(HOST_ID)
        ///     }
        /// }
        ///
        /// fn main() {
        ///     // Extension trait must be brought into scope
        ///     use S3ErrorExt;
        ///     let sdk_response: Result<(), Error> = Err(Error::builder().custom(HOST_ID, "x-1234").build());
        ///     if let Err(err) = sdk_response {
        ///         println!("request id: {:?}, extended request id: {:?}", err.request_id(), err.extended_request_id());
        ///     }
        /// }
        /// ```
        pub fn custom(&mut self, key: &'static str, value: impl Into<String>) -> &mut Self {
            self.inner.extras.insert(key, value.into());
            self
        }

        pub fn build(&mut self) -> Error {
            std::mem::take(&mut self.inner)
        }
    }

    impl Error {
        pub fn code(&self) -> Option<&str> {
            self.code.as_deref()
        }
        pub fn message(&self) -> Option<&str> {
            self.message.as_deref()
        }
        pub fn request_id(&self) -> Option<&str> {
            self.request_id.as_deref()
        }
        pub fn extra(&self, key: &'static str) -> Option<&str> {
            self.extras.get(key).map(|k| k.as_str())
        }

        pub fn builder() -> Builder {
            Builder::default()
        }

        pub fn into_builder(self) -> Builder {
            Builder { inner: self }
        }
    }

    impl ProvideErrorKind for Error {
        fn retryable_error_kind(&self) -> Option<ErrorKind> {
            None
        }

        fn code(&self) -> Option<&str> {
            Error::code(self)
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let mut fmt = f.debug_struct("Error");
            if let Some(code) = &self.code {
                fmt.field("code", code);
            }
            if let Some(message) = &self.message {
                fmt.field("message", message);
            }
            if let Some(req_id) = &self.request_id {
                fmt.field("request_id", req_id);
            }
            for (k, v) in &self.extras {
                fmt.field(k, &v);
            }
            fmt.finish()
        }
    }

    impl std::error::Error for Error {}
}
