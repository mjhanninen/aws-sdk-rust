// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    EmptyUploadException(crate::error::EmptyUploadException),
    ImageAlreadyExistsException(crate::error::ImageAlreadyExistsException),
    ImageDigestDoesNotMatchException(crate::error::ImageDigestDoesNotMatchException),
    ImageNotFoundException(crate::error::ImageNotFoundException),
    ImageTagAlreadyExistsException(crate::error::ImageTagAlreadyExistsException),
    InvalidLayerException(crate::error::InvalidLayerException),
    InvalidLayerPartException(crate::error::InvalidLayerPartException),
    InvalidParameterException(crate::error::InvalidParameterException),
    InvalidTagParameterException(crate::error::InvalidTagParameterException),
    LayerAlreadyExistsException(crate::error::LayerAlreadyExistsException),
    LayerPartTooSmallException(crate::error::LayerPartTooSmallException),
    LayersNotFoundException(crate::error::LayersNotFoundException),
    LimitExceededException(crate::error::LimitExceededException),
    ReferencedImagesNotFoundException(crate::error::ReferencedImagesNotFoundException),
    RegistryNotFoundException(crate::error::RegistryNotFoundException),
    RepositoryAlreadyExistsException(crate::error::RepositoryAlreadyExistsException),
    RepositoryNotEmptyException(crate::error::RepositoryNotEmptyException),
    RepositoryNotFoundException(crate::error::RepositoryNotFoundException),
    RepositoryPolicyNotFoundException(crate::error::RepositoryPolicyNotFoundException),
    ServerException(crate::error::ServerException),
    TooManyTagsException(crate::error::TooManyTagsException),
    UnsupportedCommandException(crate::error::UnsupportedCommandException),
    UploadNotFoundException(crate::error::UploadNotFoundException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyUploadException(inner) => inner.fmt(f),
            Error::ImageAlreadyExistsException(inner) => inner.fmt(f),
            Error::ImageDigestDoesNotMatchException(inner) => inner.fmt(f),
            Error::ImageNotFoundException(inner) => inner.fmt(f),
            Error::ImageTagAlreadyExistsException(inner) => inner.fmt(f),
            Error::InvalidLayerException(inner) => inner.fmt(f),
            Error::InvalidLayerPartException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::InvalidTagParameterException(inner) => inner.fmt(f),
            Error::LayerAlreadyExistsException(inner) => inner.fmt(f),
            Error::LayerPartTooSmallException(inner) => inner.fmt(f),
            Error::LayersNotFoundException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ReferencedImagesNotFoundException(inner) => inner.fmt(f),
            Error::RegistryNotFoundException(inner) => inner.fmt(f),
            Error::RepositoryAlreadyExistsException(inner) => inner.fmt(f),
            Error::RepositoryNotEmptyException(inner) => inner.fmt(f),
            Error::RepositoryNotFoundException(inner) => inner.fmt(f),
            Error::RepositoryPolicyNotFoundException(inner) => inner.fmt(f),
            Error::ServerException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::UnsupportedCommandException(inner) => inner.fmt(f),
            Error::UploadNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::BatchCheckLayerAvailabilityError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::BatchCheckLayerAvailabilityError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BatchCheckLayerAvailabilityErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::BatchCheckLayerAvailabilityErrorKind::RegistryNotFoundException(
                    inner,
                ) => Error::RegistryNotFoundException(inner),
                crate::error::BatchCheckLayerAvailabilityErrorKind::RepositoryNotFoundException(
                    inner,
                ) => Error::RepositoryNotFoundException(inner),
                crate::error::BatchCheckLayerAvailabilityErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::BatchCheckLayerAvailabilityErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::BatchDeleteImageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::BatchDeleteImageError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BatchDeleteImageErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::BatchDeleteImageErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::BatchDeleteImageErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::BatchDeleteImageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CompleteLayerUploadError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CompleteLayerUploadError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CompleteLayerUploadErrorKind::EmptyUploadException(inner) => {
                    Error::EmptyUploadException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::InvalidLayerException(inner) => {
                    Error::InvalidLayerException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::LayerAlreadyExistsException(inner) => {
                    Error::LayerAlreadyExistsException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::LayerPartTooSmallException(inner) => {
                    Error::LayerPartTooSmallException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::RegistryNotFoundException(inner) => {
                    Error::RegistryNotFoundException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::UnsupportedCommandException(inner) => {
                    Error::UnsupportedCommandException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::UploadNotFoundException(inner) => {
                    Error::UploadNotFoundException(inner)
                }
                crate::error::CompleteLayerUploadErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateRepositoryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateRepositoryError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateRepositoryErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::CreateRepositoryErrorKind::InvalidTagParameterException(inner) => {
                    Error::InvalidTagParameterException(inner)
                }
                crate::error::CreateRepositoryErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateRepositoryErrorKind::RepositoryAlreadyExistsException(
                    inner,
                ) => Error::RepositoryAlreadyExistsException(inner),
                crate::error::CreateRepositoryErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::CreateRepositoryErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
                }
                crate::error::CreateRepositoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteRepositoryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteRepositoryError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteRepositoryErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DeleteRepositoryErrorKind::RepositoryNotEmptyException(inner) => {
                    Error::RepositoryNotEmptyException(inner)
                }
                crate::error::DeleteRepositoryErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::DeleteRepositoryErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DeleteRepositoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteRepositoryPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeleteRepositoryPolicyError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteRepositoryPolicyErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DeleteRepositoryPolicyErrorKind::RepositoryNotFoundException(inner) => Error::RepositoryNotFoundException(inner),
                crate::error::DeleteRepositoryPolicyErrorKind::RepositoryPolicyNotFoundException(inner) => Error::RepositoryPolicyNotFoundException(inner),
                crate::error::DeleteRepositoryPolicyErrorKind::ServerException(inner) => Error::ServerException(inner),
                crate::error::DeleteRepositoryPolicyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeImagesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeImagesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeImagesErrorKind::ImageNotFoundException(inner) => {
                    Error::ImageNotFoundException(inner)
                }
                crate::error::DescribeImagesErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeImagesErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::DescribeImagesErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeImagesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeImageTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeImageTagsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeImageTagsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeImageTagsErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::DescribeImageTagsErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeImageTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeRegistriesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeRegistriesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeRegistriesErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeRegistriesErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeRegistriesErrorKind::UnsupportedCommandException(inner) => {
                    Error::UnsupportedCommandException(inner)
                }
                crate::error::DescribeRegistriesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeRepositoriesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeRepositoriesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeRepositoriesErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeRepositoriesErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::DescribeRepositoriesErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeRepositoriesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetAuthorizationTokenError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetAuthorizationTokenError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAuthorizationTokenErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetAuthorizationTokenErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::GetAuthorizationTokenErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetRegistryCatalogDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetRegistryCatalogDataError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetRegistryCatalogDataErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::GetRegistryCatalogDataErrorKind::UnsupportedCommandException(
                    inner,
                ) => Error::UnsupportedCommandException(inner),
                crate::error::GetRegistryCatalogDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetRepositoryCatalogDataError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetRepositoryCatalogDataError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetRepositoryCatalogDataErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::GetRepositoryCatalogDataErrorKind::RepositoryNotFoundException(
                    inner,
                ) => Error::RepositoryNotFoundException(inner),
                crate::error::GetRepositoryCatalogDataErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::GetRepositoryCatalogDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetRepositoryPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetRepositoryPolicyError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetRepositoryPolicyErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetRepositoryPolicyErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::GetRepositoryPolicyErrorKind::RepositoryPolicyNotFoundException(
                    inner,
                ) => Error::RepositoryPolicyNotFoundException(inner),
                crate::error::GetRepositoryPolicyErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::GetRepositoryPolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::InitiateLayerUploadError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::InitiateLayerUploadError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::InitiateLayerUploadErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::InitiateLayerUploadErrorKind::RegistryNotFoundException(inner) => {
                    Error::RegistryNotFoundException(inner)
                }
                crate::error::InitiateLayerUploadErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::InitiateLayerUploadErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::InitiateLayerUploadErrorKind::UnsupportedCommandException(inner) => {
                    Error::UnsupportedCommandException(inner)
                }
                crate::error::InitiateLayerUploadErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PutImageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::PutImageError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutImageErrorKind::ImageAlreadyExistsException(inner) => {
                    Error::ImageAlreadyExistsException(inner)
                }
                crate::error::PutImageErrorKind::ImageDigestDoesNotMatchException(inner) => {
                    Error::ImageDigestDoesNotMatchException(inner)
                }
                crate::error::PutImageErrorKind::ImageTagAlreadyExistsException(inner) => {
                    Error::ImageTagAlreadyExistsException(inner)
                }
                crate::error::PutImageErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::PutImageErrorKind::LayersNotFoundException(inner) => {
                    Error::LayersNotFoundException(inner)
                }
                crate::error::PutImageErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::PutImageErrorKind::ReferencedImagesNotFoundException(inner) => {
                    Error::ReferencedImagesNotFoundException(inner)
                }
                crate::error::PutImageErrorKind::RegistryNotFoundException(inner) => {
                    Error::RegistryNotFoundException(inner)
                }
                crate::error::PutImageErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::PutImageErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::PutImageErrorKind::UnsupportedCommandException(inner) => {
                    Error::UnsupportedCommandException(inner)
                }
                crate::error::PutImageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PutRegistryCatalogDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::PutRegistryCatalogDataError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRegistryCatalogDataErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::PutRegistryCatalogDataErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::PutRegistryCatalogDataErrorKind::UnsupportedCommandException(
                    inner,
                ) => Error::UnsupportedCommandException(inner),
                crate::error::PutRegistryCatalogDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PutRepositoryCatalogDataError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::PutRepositoryCatalogDataError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRepositoryCatalogDataErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::PutRepositoryCatalogDataErrorKind::RepositoryNotFoundException(
                    inner,
                ) => Error::RepositoryNotFoundException(inner),
                crate::error::PutRepositoryCatalogDataErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::PutRepositoryCatalogDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::SetRepositoryPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::SetRepositoryPolicyError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SetRepositoryPolicyErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::SetRepositoryPolicyErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::SetRepositoryPolicyErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::SetRepositoryPolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::TagResourceErrorKind::InvalidTagParameterException(inner) => {
                    Error::InvalidTagParameterException(inner)
                }
                crate::error::TagResourceErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::TagResourceErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UntagResourceErrorKind::InvalidTagParameterException(inner) => {
                    Error::InvalidTagParameterException(inner)
                }
                crate::error::UntagResourceErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::UntagResourceErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UploadLayerPartError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UploadLayerPartError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UploadLayerPartErrorKind::InvalidLayerPartException(inner) => {
                    Error::InvalidLayerPartException(inner)
                }
                crate::error::UploadLayerPartErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UploadLayerPartErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UploadLayerPartErrorKind::RegistryNotFoundException(inner) => {
                    Error::RegistryNotFoundException(inner)
                }
                crate::error::UploadLayerPartErrorKind::RepositoryNotFoundException(inner) => {
                    Error::RepositoryNotFoundException(inner)
                }
                crate::error::UploadLayerPartErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::UploadLayerPartErrorKind::UnsupportedCommandException(inner) => {
                    Error::UnsupportedCommandException(inner)
                }
                crate::error::UploadLayerPartErrorKind::UploadNotFoundException(inner) => {
                    Error::UploadNotFoundException(inner)
                }
                crate::error::UploadLayerPartErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
