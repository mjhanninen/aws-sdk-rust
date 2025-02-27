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

/// An ergonomic service client for `dlm_20180112`.
///
/// This client allows ergonomic access to a `dlm_20180112`-shaped service.
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
    pub fn create_lifecycle_policy(&self) -> fluent_builders::CreateLifecyclePolicy<C, M, R> {
        fluent_builders::CreateLifecyclePolicy::new(self.handle.clone())
    }
    pub fn delete_lifecycle_policy(&self) -> fluent_builders::DeleteLifecyclePolicy<C, M, R> {
        fluent_builders::DeleteLifecyclePolicy::new(self.handle.clone())
    }
    pub fn get_lifecycle_policies(&self) -> fluent_builders::GetLifecyclePolicies<C, M, R> {
        fluent_builders::GetLifecyclePolicies::new(self.handle.clone())
    }
    pub fn get_lifecycle_policy(&self) -> fluent_builders::GetLifecyclePolicy<C, M, R> {
        fluent_builders::GetLifecyclePolicy::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C, M, R> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C, M, R> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C, M, R> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
    pub fn update_lifecycle_policy(&self) -> fluent_builders::UpdateLifecyclePolicy<C, M, R> {
        fluent_builders::UpdateLifecyclePolicy::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateLifecyclePolicy<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_lifecycle_policy_input::Builder,
    }
    impl<C, M, R> CreateLifecyclePolicy<C, M, R>
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
            crate::output::CreateLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::CreateLifecyclePolicyError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateLifecyclePolicyInputOperationOutputAlias,
                crate::output::CreateLifecyclePolicyOutput,
                crate::error::CreateLifecyclePolicyError,
                crate::input::CreateLifecyclePolicyInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by
        /// the lifecycle policy.</p>
        pub fn execution_role_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.execution_role_arn(inp);
            self
        }
        pub fn set_execution_role_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_execution_role_arn(input);
            self
        }
        /// <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are
        /// supported.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The desired activation state of the lifecycle policy after creation.</p>
        pub fn state(mut self, inp: crate::model::SettablePolicyStateValues) -> Self {
            self.inner = self.inner.state(inp);
            self
        }
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::SettablePolicyStateValues>,
        ) -> Self {
            self.inner = self.inner.set_state(input);
            self
        }
        /// <p>The configuration details of the lifecycle policy.</p>
        pub fn policy_details(mut self, inp: crate::model::PolicyDetails) -> Self {
            self.inner = self.inner.policy_details(inp);
            self
        }
        pub fn set_policy_details(
            mut self,
            input: std::option::Option<crate::model::PolicyDetails>,
        ) -> Self {
            self.inner = self.inner.set_policy_details(input);
            self
        }
        /// Adds a key-value pair to `Tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>The tags to apply to the lifecycle policy during creation.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteLifecyclePolicy<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_lifecycle_policy_input::Builder,
    }
    impl<C, M, R> DeleteLifecyclePolicy<C, M, R>
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
            crate::output::DeleteLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::DeleteLifecyclePolicyError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteLifecyclePolicyInputOperationOutputAlias,
                crate::output::DeleteLifecyclePolicyOutput,
                crate::error::DeleteLifecyclePolicyError,
                crate::input::DeleteLifecyclePolicyInputOperationRetryAlias,
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
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_id(inp);
            self
        }
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_policy_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetLifecyclePolicies<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_lifecycle_policies_input::Builder,
    }
    impl<C, M, R> GetLifecyclePolicies<C, M, R>
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
            crate::output::GetLifecyclePoliciesOutput,
            smithy_http::result::SdkError<crate::error::GetLifecyclePoliciesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetLifecyclePoliciesInputOperationOutputAlias,
                crate::output::GetLifecyclePoliciesOutput,
                crate::error::GetLifecyclePoliciesError,
                crate::input::GetLifecyclePoliciesInputOperationRetryAlias,
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
        /// Appends an item to `PolicyIds`.
        ///
        /// To override the contents of this collection use [`set_policy_ids`](Self::set_policy_ids).
        /// <p>The identifiers of the data lifecycle policies.</p>
        pub fn policy_ids(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_ids(inp);
            self
        }
        pub fn set_policy_ids(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_policy_ids(input);
            self
        }
        /// <p>The activation state.</p>
        pub fn state(mut self, inp: crate::model::GettablePolicyStateValues) -> Self {
            self.inner = self.inner.state(inp);
            self
        }
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::GettablePolicyStateValues>,
        ) -> Self {
            self.inner = self.inner.set_state(input);
            self
        }
        /// Appends an item to `ResourceTypes`.
        ///
        /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
        /// <p>The resource type.</p>
        pub fn resource_types(mut self, inp: impl Into<crate::model::ResourceTypeValues>) -> Self {
            self.inner = self.inner.resource_types(inp);
            self
        }
        pub fn set_resource_types(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResourceTypeValues>>,
        ) -> Self {
            self.inner = self.inner.set_resource_types(input);
            self
        }
        /// Appends an item to `TargetTags`.
        ///
        /// To override the contents of this collection use [`set_target_tags`](Self::set_target_tags).
        /// <p>The target tag for a policy.</p>
        /// <p>Tags are strings in the format <code>key=value</code>.</p>
        pub fn target_tags(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_tags(inp);
            self
        }
        pub fn set_target_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_target_tags(input);
            self
        }
        /// Appends an item to `TagsToAdd`.
        ///
        /// To override the contents of this collection use [`set_tags_to_add`](Self::set_tags_to_add).
        /// <p>The tags to add to objects created by the policy.</p>
        /// <p>Tags are strings in the format <code>key=value</code>.</p>
        /// <p>These user-defined tags are added in addition to the Amazon Web Services-added lifecycle tags.</p>
        pub fn tags_to_add(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tags_to_add(inp);
            self
        }
        pub fn set_tags_to_add(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tags_to_add(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetLifecyclePolicy<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_lifecycle_policy_input::Builder,
    }
    impl<C, M, R> GetLifecyclePolicy<C, M, R>
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
            crate::output::GetLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::GetLifecyclePolicyError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetLifecyclePolicyInputOperationOutputAlias,
                crate::output::GetLifecyclePolicyOutput,
                crate::error::GetLifecyclePolicyError,
                crate::input::GetLifecyclePolicyInputOperationRetryAlias,
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
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_id(inp);
            self
        }
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_policy_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListTagsForResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl<C, M, R> ListTagsForResource<C, M, R>
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
            crate::output::ListTagsForResourceOutput,
            smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListTagsForResourceInputOperationOutputAlias,
                crate::output::ListTagsForResourceOutput,
                crate::error::ListTagsForResourceError,
                crate::input::ListTagsForResourceInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct TagResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl<C, M, R> TagResource<C, M, R>
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
            crate::output::TagResourceOutput,
            smithy_http::result::SdkError<crate::error::TagResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::TagResourceInputOperationOutputAlias,
                crate::output::TagResourceOutput,
                crate::error::TagResourceError,
                crate::input::TagResourceInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Adds a key-value pair to `Tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>One or more tags.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UntagResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl<C, M, R> UntagResource<C, M, R>
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
            crate::output::UntagResourceOutput,
            smithy_http::result::SdkError<crate::error::UntagResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UntagResourceInputOperationOutputAlias,
                crate::output::UntagResourceOutput,
                crate::error::UntagResourceError,
                crate::input::UntagResourceInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Appends an item to `TagKeys`.
        ///
        /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
        /// <p>The tag keys.</p>
        pub fn tag_keys(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(inp);
            self
        }
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateLifecyclePolicy<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_lifecycle_policy_input::Builder,
    }
    impl<C, M, R> UpdateLifecyclePolicy<C, M, R>
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
            crate::output::UpdateLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::UpdateLifecyclePolicyError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateLifecyclePolicyInputOperationOutputAlias,
                crate::output::UpdateLifecyclePolicyOutput,
                crate::error::UpdateLifecyclePolicyError,
                crate::input::UpdateLifecyclePolicyInputOperationRetryAlias,
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
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_id(inp);
            self
        }
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_policy_id(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by
        /// the lifecycle policy.</p>
        pub fn execution_role_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.execution_role_arn(inp);
            self
        }
        pub fn set_execution_role_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_execution_role_arn(input);
            self
        }
        /// <p>The desired activation state of the lifecycle policy after creation.</p>
        pub fn state(mut self, inp: crate::model::SettablePolicyStateValues) -> Self {
            self.inner = self.inner.state(inp);
            self
        }
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::SettablePolicyStateValues>,
        ) -> Self {
            self.inner = self.inner.set_state(input);
            self
        }
        /// <p>A description of the lifecycle policy.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The configuration of the lifecycle policy. You cannot update the policy type or the
        /// resource type.</p>
        pub fn policy_details(mut self, inp: crate::model::PolicyDetails) -> Self {
            self.inner = self.inner.policy_details(inp);
            self
        }
        pub fn set_policy_details(
            mut self,
            input: std::option::Option<crate::model::PolicyDetails>,
        ) -> Self {
            self.inner = self.inner.set_policy_details(input);
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
