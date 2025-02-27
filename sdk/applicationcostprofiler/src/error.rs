// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteReportDefinitionError {
    pub kind: DeleteReportDefinitionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteReportDefinitionErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServerException(crate::error::InternalServerException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DeleteReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteReportDefinitionErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            DeleteReportDefinitionErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DeleteReportDefinitionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            DeleteReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            DeleteReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for DeleteReportDefinitionError {
    fn code(&self) -> Option<&str> {
        DeleteReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteReportDefinitionError {
    pub fn new(kind: DeleteReportDefinitionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteReportDefinitionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteReportDefinitionErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteReportDefinitionErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteReportDefinitionErrorKind::InternalServerException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteReportDefinitionErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for DeleteReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteReportDefinitionErrorKind::AccessDeniedException(_inner) => Some(_inner),
            DeleteReportDefinitionErrorKind::InternalServerException(_inner) => Some(_inner),
            DeleteReportDefinitionErrorKind::ThrottlingException(_inner) => Some(_inner),
            DeleteReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            DeleteReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetReportDefinitionError {
    pub kind: GetReportDefinitionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetReportDefinitionErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServerException(crate::error::InternalServerException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetReportDefinitionErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            GetReportDefinitionErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            GetReportDefinitionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            GetReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            GetReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for GetReportDefinitionError {
    fn code(&self) -> Option<&str> {
        GetReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetReportDefinitionError {
    pub fn new(kind: GetReportDefinitionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetReportDefinitionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetReportDefinitionErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetReportDefinitionErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetReportDefinitionErrorKind::InternalServerException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetReportDefinitionErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for GetReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetReportDefinitionErrorKind::AccessDeniedException(_inner) => Some(_inner),
            GetReportDefinitionErrorKind::InternalServerException(_inner) => Some(_inner),
            GetReportDefinitionErrorKind::ThrottlingException(_inner) => Some(_inner),
            GetReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            GetReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ImportApplicationUsageError {
    pub kind: ImportApplicationUsageErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ImportApplicationUsageErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServerException(crate::error::InternalServerException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ImportApplicationUsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ImportApplicationUsageErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            ImportApplicationUsageErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            ImportApplicationUsageErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            ImportApplicationUsageErrorKind::ValidationException(_inner) => _inner.fmt(f),
            ImportApplicationUsageErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for ImportApplicationUsageError {
    fn code(&self) -> Option<&str> {
        ImportApplicationUsageError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl ImportApplicationUsageError {
    pub fn new(kind: ImportApplicationUsageErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ImportApplicationUsageErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ImportApplicationUsageErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            ImportApplicationUsageErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            ImportApplicationUsageErrorKind::InternalServerException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            ImportApplicationUsageErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            ImportApplicationUsageErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for ImportApplicationUsageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ImportApplicationUsageErrorKind::AccessDeniedException(_inner) => Some(_inner),
            ImportApplicationUsageErrorKind::InternalServerException(_inner) => Some(_inner),
            ImportApplicationUsageErrorKind::ThrottlingException(_inner) => Some(_inner),
            ImportApplicationUsageErrorKind::ValidationException(_inner) => Some(_inner),
            ImportApplicationUsageErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListReportDefinitionsError {
    pub kind: ListReportDefinitionsErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListReportDefinitionsErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServerException(crate::error::InternalServerException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListReportDefinitionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListReportDefinitionsErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            ListReportDefinitionsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            ListReportDefinitionsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            ListReportDefinitionsErrorKind::ValidationException(_inner) => _inner.fmt(f),
            ListReportDefinitionsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for ListReportDefinitionsError {
    fn code(&self) -> Option<&str> {
        ListReportDefinitionsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListReportDefinitionsError {
    pub fn new(kind: ListReportDefinitionsErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListReportDefinitionsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListReportDefinitionsErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListReportDefinitionsErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListReportDefinitionsErrorKind::InternalServerException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListReportDefinitionsErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListReportDefinitionsErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for ListReportDefinitionsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListReportDefinitionsErrorKind::AccessDeniedException(_inner) => Some(_inner),
            ListReportDefinitionsErrorKind::InternalServerException(_inner) => Some(_inner),
            ListReportDefinitionsErrorKind::ThrottlingException(_inner) => Some(_inner),
            ListReportDefinitionsErrorKind::ValidationException(_inner) => Some(_inner),
            ListReportDefinitionsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutReportDefinitionError {
    pub kind: PutReportDefinitionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutReportDefinitionErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServerException(crate::error::InternalServerException),
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for PutReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutReportDefinitionErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::ServiceQuotaExceededException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            PutReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for PutReportDefinitionError {
    fn code(&self) -> Option<&str> {
        PutReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutReportDefinitionError {
    pub fn new(kind: PutReportDefinitionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutReportDefinitionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutReportDefinitionErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::InternalServerException(_)
        )
    }
    pub fn is_service_quota_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::ServiceQuotaExceededException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for PutReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutReportDefinitionErrorKind::AccessDeniedException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::InternalServerException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::ServiceQuotaExceededException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::ThrottlingException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            PutReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UpdateReportDefinitionError {
    pub kind: UpdateReportDefinitionErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UpdateReportDefinitionErrorKind {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServerException(crate::error::InternalServerException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for UpdateReportDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UpdateReportDefinitionErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            UpdateReportDefinitionErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            UpdateReportDefinitionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            UpdateReportDefinitionErrorKind::ValidationException(_inner) => _inner.fmt(f),
            UpdateReportDefinitionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for UpdateReportDefinitionError {
    fn code(&self) -> Option<&str> {
        UpdateReportDefinitionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl UpdateReportDefinitionError {
    pub fn new(kind: UpdateReportDefinitionErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UpdateReportDefinitionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UpdateReportDefinitionErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateReportDefinitionErrorKind::AccessDeniedException(_)
        )
    }
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateReportDefinitionErrorKind::InternalServerException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateReportDefinitionErrorKind::ThrottlingException(_)
        )
    }
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateReportDefinitionErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for UpdateReportDefinitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UpdateReportDefinitionErrorKind::AccessDeniedException(_inner) => Some(_inner),
            UpdateReportDefinitionErrorKind::InternalServerException(_inner) => Some(_inner),
            UpdateReportDefinitionErrorKind::ThrottlingException(_inner) => Some(_inner),
            UpdateReportDefinitionErrorKind::ValidationException(_inner) => Some(_inner),
            UpdateReportDefinitionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The input fails to satisfy the constraints for the API.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ValidationException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
/// See [`ValidationException`](crate::error::ValidationException)
pub mod validation_exception {
    /// A builder for [`ValidationException`](crate::error::ValidationException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ValidationException`](crate::error::ValidationException)
        pub fn build(self) -> crate::error::ValidationException {
            crate::error::ValidationException {
                message: self.message,
            }
        }
    }
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::error::ValidationException)
    pub fn builder() -> crate::error::validation_exception::Builder {
        crate::error::validation_exception::Builder::default()
    }
}

/// <p>The calls to AWS Application Cost Profiler API are throttled. The request was denied.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ThrottlingException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ThrottlingException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException)
pub mod throttling_exception {
    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException)
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException)
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>An internal server error occurred. Retry your request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException)
pub mod internal_server_exception {
    /// A builder for [`InternalServerException`](crate::error::InternalServerException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException)
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message,
            }
        }
    }
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException)
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// <p>You do not have permission to perform this action.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AccessDeniedException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessDeniedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl AccessDeniedException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessDeniedException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for AccessDeniedException {}
/// See [`AccessDeniedException`](crate::error::AccessDeniedException)
pub mod access_denied_exception {
    /// A builder for [`AccessDeniedException`](crate::error::AccessDeniedException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`AccessDeniedException`](crate::error::AccessDeniedException)
        pub fn build(self) -> crate::error::AccessDeniedException {
            crate::error::AccessDeniedException {
                message: self.message,
            }
        }
    }
}
impl AccessDeniedException {
    /// Creates a new builder-style object to manufacture [`AccessDeniedException`](crate::error::AccessDeniedException)
    pub fn builder() -> crate::error::access_denied_exception::Builder {
        crate::error::access_denied_exception::Builder::default()
    }
}

/// <p>Your request exceeds one or more of the service quotas.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ServiceQuotaExceededException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ServiceQuotaExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceQuotaExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ServiceQuotaExceededException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceQuotaExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceQuotaExceededException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for ServiceQuotaExceededException {}
/// See [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
pub mod service_quota_exceeded_exception {
    /// A builder for [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
        pub fn build(self) -> crate::error::ServiceQuotaExceededException {
            crate::error::ServiceQuotaExceededException {
                message: self.message,
            }
        }
    }
}
impl ServiceQuotaExceededException {
    /// Creates a new builder-style object to manufacture [`ServiceQuotaExceededException`](crate::error::ServiceQuotaExceededException)
    pub fn builder() -> crate::error::service_quota_exceeded_exception::Builder {
        crate::error::service_quota_exceeded_exception::Builder::default()
    }
}
