// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `MediaStoreObject_20170901`.
///
/// This client allows ergonomic access to a `MediaStoreObject_20170901`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn delete_object(&self) -> fluent_builders::DeleteObject<C, M, R> {
        fluent_builders::DeleteObject::new(self.handle.clone())
    }
    pub fn describe_object(&self) -> fluent_builders::DescribeObject<C, M, R> {
        fluent_builders::DescribeObject::new(self.handle.clone())
    }
    pub fn get_object(&self) -> fluent_builders::GetObject<C, M, R> {
        fluent_builders::GetObject::new(self.handle.clone())
    }
    pub fn list_items(&self) -> fluent_builders::ListItems<C, M, R> {
        fluent_builders::ListItems::new(self.handle.clone())
    }
    pub fn put_object(&self) -> fluent_builders::PutObject<C, M, R> {
        fluent_builders::PutObject::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DeleteObject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_object_input::Builder,
    }
    impl<C, M, R> DeleteObject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteObjectOutput,
            smithy_http::result::SdkError<crate::error::DeleteObjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteObjectInputOperationOutputAlias,
                crate::output::DeleteObjectOutput,
                crate::error::DeleteObjectError,
                crate::input::DeleteObjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The path (including the file name) where the object is stored in the container.
        /// Format: <folder name>/<folder name>/<file name></p>
        pub fn path(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.path(inp);
            self
        }
        pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_path(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeObject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_object_input::Builder,
    }
    impl<C, M, R> DescribeObject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeObjectOutput,
            smithy_http::result::SdkError<crate::error::DescribeObjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeObjectInputOperationOutputAlias,
                crate::output::DescribeObjectOutput,
                crate::error::DescribeObjectError,
                crate::input::DescribeObjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The path (including the file name) where the object is stored in the container.
        /// Format: <folder name>/<folder name>/<file name></p>
        pub fn path(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.path(inp);
            self
        }
        pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_path(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetObject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_object_input::Builder,
    }
    impl<C, M, R> GetObject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetObjectOutput,
            smithy_http::result::SdkError<crate::error::GetObjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetObjectInputOperationOutputAlias,
                crate::output::GetObjectOutput,
                crate::error::GetObjectError,
                crate::input::GetObjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The path (including the file name) where the object is stored in the container.
        /// Format: <folder name>/<folder name>/<file name></p>
        /// <p>For example, to upload the file <code>mlaw.avi</code> to the folder path
        /// <code>premium\canada</code> in the container <code>movies</code>, enter the path
        /// <code>premium/canada/mlaw.avi</code>.</p>
        /// <p>Do not include the container name in this path.</p>
        /// <p>If the path includes any folders that don't exist yet, the service creates them. For
        /// example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify
        /// <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the
        /// <code>premium</code> folder. You then have two subfolders, <code>usa</code> and
        /// <code>canada</code>, in the <code>premium</code> folder. </p>
        /// <p>There is no correlation between the path to the source and the path (folders) in the
        /// container in AWS Elemental MediaStore.</p>
        /// <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User
        /// Guide</a>.</p>
        /// <p>The file name is the name that is assigned to the file that you upload. The file can
        /// have the same name inside and outside of AWS Elemental MediaStore, or it can have the same
        /// name. The file name can include or omit an extension. </p>
        pub fn path(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.path(inp);
            self
        }
        pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_path(input);
            self
        }
        /// <p>The range bytes of an object to retrieve. For more information about the
        /// <code>Range</code> header, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>. AWS Elemental MediaStore ignores this header for partially uploaded objects that have streaming upload availability.</p>
        pub fn range(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.range(inp);
            self
        }
        pub fn set_range(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_range(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListItems<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_items_input::Builder,
    }
    impl<C, M, R> ListItems<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListItemsOutput,
            smithy_http::result::SdkError<crate::error::ListItemsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListItemsInputOperationOutputAlias,
                crate::output::ListItemsOutput,
                crate::error::ListItemsError,
                crate::input::ListItemsInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The path in the container from which to retrieve items. Format: <folder
        /// name>/<folder name>/<file name></p>
        pub fn path(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.path(inp);
            self
        }
        pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_path(input);
            self
        }
        /// <p>The maximum number of results to return per API request. For example, you submit a
        /// <code>ListItems</code> request with <code>MaxResults</code> set at 500. Although 2,000
        /// items match your request, the service returns no more than the first 500 items. (The
        /// service also returns a <code>NextToken</code> value that you can use to fetch the next
        /// batch of results.) The service might return fewer results than the <code>MaxResults</code>
        /// value.</p>
        /// <p>If <code>MaxResults</code> is not included in the request, the service defaults to
        /// pagination with a maximum of 1,000 results per page.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token that identifies which batch of results that you want to see. For example,
        /// you submit a <code>ListItems</code> request with <code>MaxResults</code> set at 500. The
        /// service returns the first batch of results (up to 500) and a <code>NextToken</code> value.
        /// To see the next batch of results, you can submit the <code>ListItems</code> request a
        /// second time and specify the <code>NextToken</code> value.</p>
        /// <p>Tokens expire after 15 minutes.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct PutObject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_object_input::Builder,
    }
    impl<C, M, R> PutObject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PutObjectOutput,
            smithy_http::result::SdkError<crate::error::PutObjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutObjectInputOperationOutputAlias,
                crate::output::PutObjectOutput,
                crate::error::PutObjectError,
                crate::input::PutObjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The bytes to be stored. </p>
        pub fn body(mut self, inp: smithy_http::byte_stream::ByteStream) -> Self {
            self.inner = self.inner.body(inp);
            self
        }
        pub fn set_body(
            mut self,
            input: std::option::Option<smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.inner = self.inner.set_body(input);
            self
        }
        /// <p>The path (including the file name) where the object is stored in the container.
        /// Format: <folder name>/<folder name>/<file name></p>
        /// <p>For example, to upload the file <code>mlaw.avi</code> to the folder path
        /// <code>premium\canada</code> in the container <code>movies</code>, enter the path
        /// <code>premium/canada/mlaw.avi</code>.</p>
        /// <p>Do not include the container name in this path.</p>
        /// <p>If the path includes any folders that don't exist yet, the service creates them. For
        /// example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify
        /// <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the
        /// <code>premium</code> folder. You then have two subfolders, <code>usa</code> and
        /// <code>canada</code>, in the <code>premium</code> folder. </p>
        /// <p>There is no correlation between the path to the source and the path (folders) in the
        /// container in AWS Elemental MediaStore.</p>
        /// <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User
        /// Guide</a>.</p>
        /// <p>The file name is the name that is assigned to the file that you upload. The file can
        /// have the same name inside and outside of AWS Elemental MediaStore, or it can have the same
        /// name. The file name can include or omit an extension. </p>
        pub fn path(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.path(inp);
            self
        }
        pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_path(input);
            self
        }
        /// <p>The content type of the object.</p>
        pub fn content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content_type(inp);
            self
        }
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content_type(input);
            self
        }
        /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
        /// object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
        /// <p>Headers with a custom user-defined value are also accepted.</p>
        pub fn cache_control(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.cache_control(inp);
            self
        }
        pub fn set_cache_control(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_cache_control(input);
            self
        }
        /// <p>Indicates the storage class of a <code>Put</code> request. Defaults to
        /// high-performance temporal storage class, and objects are persisted into durable storage
        /// shortly after being received.</p>
        pub fn storage_class(mut self, inp: crate::model::StorageClass) -> Self {
            self.inner = self.inner.storage_class(inp);
            self
        }
        pub fn set_storage_class(
            mut self,
            input: std::option::Option<crate::model::StorageClass>,
        ) -> Self {
            self.inner = self.inner.set_storage_class(input);
            self
        }
        /// <p>Indicates the availability of an object while it is still uploading. If the value is set to <code>streaming</code>, the object is available for
        /// downloading after some initial buffering but before the object is uploaded completely. If the value is set to <code>standard</code>, the object is
        /// available for downloading only when it is uploaded completely. The default value for this header is <code>standard</code>.</p>
        /// <p>To use this header, you must also set the HTTP <code>Transfer-Encoding</code> header to <code>chunked</code>.</p>
        pub fn upload_availability(mut self, inp: crate::model::UploadAvailability) -> Self {
            self.inner = self.inner.upload_availability(inp);
            self
        }
        pub fn set_upload_availability(
            mut self,
            input: std::option::Option<crate::model::UploadAvailability>,
        ) -> Self {
            self.inner = self.inner.set_upload_availability(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
