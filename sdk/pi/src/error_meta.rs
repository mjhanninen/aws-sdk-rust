// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    InternalServiceError(crate::error::InternalServiceError),
    InvalidArgumentException(crate::error::InvalidArgumentException),
    NotAuthorizedException(crate::error::NotAuthorizedException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServiceError(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::NotAuthorizedException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeDimensionKeysError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeDimensionKeysError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDimensionKeysErrorKind::InternalServiceError(inner) => {
                    Error::InternalServiceError(inner)
                }
                crate::error::DescribeDimensionKeysErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::DescribeDimensionKeysErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeDimensionKeysErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetDimensionKeyDetailsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetDimensionKeyDetailsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDimensionKeyDetailsErrorKind::InternalServiceError(inner) => {
                    Error::InternalServiceError(inner)
                }
                crate::error::GetDimensionKeyDetailsErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::GetDimensionKeyDetailsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetDimensionKeyDetailsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetResourceMetricsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetResourceMetricsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetResourceMetricsErrorKind::InternalServiceError(inner) => {
                    Error::InternalServiceError(inner)
                }
                crate::error::GetResourceMetricsErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::GetResourceMetricsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetResourceMetricsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
