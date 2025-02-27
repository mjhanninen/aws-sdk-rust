// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a storage container to hold objects. A container is similar to a bucket in
/// the Amazon S3 service.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateContainer {
    _private: (),
}
impl CreateContainer {
    /// Creates a new builder-style object to manufacture [`CreateContainerInput`](crate::input::CreateContainerInput)
    pub fn builder() -> crate::input::create_container_input::Builder {
        crate::input::create_container_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateContainer {
    type Output = std::result::Result<
        crate::output::CreateContainerOutput,
        crate::error::CreateContainerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_container_error(response)
        } else {
            crate::operation_deser::parse_create_container_response(response)
        }
    }
}

/// <p>Deletes the specified container. Before you make a <code>DeleteContainer</code>
/// request, delete any objects in the container or in any folders in the container. You can
/// delete only empty containers. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteContainer {
    _private: (),
}
impl DeleteContainer {
    /// Creates a new builder-style object to manufacture [`DeleteContainerInput`](crate::input::DeleteContainerInput)
    pub fn builder() -> crate::input::delete_container_input::Builder {
        crate::input::delete_container_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteContainer {
    type Output = std::result::Result<
        crate::output::DeleteContainerOutput,
        crate::error::DeleteContainerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_container_error(response)
        } else {
            crate::operation_deser::parse_delete_container_response(response)
        }
    }
}

/// <p>Deletes the access policy that is associated with the specified container.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteContainerPolicy {
    _private: (),
}
impl DeleteContainerPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteContainerPolicyInput`](crate::input::DeleteContainerPolicyInput)
    pub fn builder() -> crate::input::delete_container_policy_input::Builder {
        crate::input::delete_container_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteContainerPolicy {
    type Output = std::result::Result<
        crate::output::DeleteContainerPolicyOutput,
        crate::error::DeleteContainerPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_container_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_container_policy_response(response)
        }
    }
}

/// <p>Deletes the cross-origin resource sharing (CORS) configuration information that is
/// set for the container.</p>
/// <p>To use this operation, you must have permission to perform the
/// <code>MediaStore:DeleteCorsPolicy</code> action. The container owner has this permission
/// by default and can grant this permission to others.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteCorsPolicy {
    _private: (),
}
impl DeleteCorsPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteCorsPolicyInput`](crate::input::DeleteCorsPolicyInput)
    pub fn builder() -> crate::input::delete_cors_policy_input::Builder {
        crate::input::delete_cors_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteCorsPolicy {
    type Output = std::result::Result<
        crate::output::DeleteCorsPolicyOutput,
        crate::error::DeleteCorsPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_cors_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_cors_policy_response(response)
        }
    }
}

/// <p>Removes an object lifecycle policy from a container. It takes up to 20 minutes for the change to take effect.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLifecyclePolicy {
    _private: (),
}
impl DeleteLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`DeleteLifecyclePolicyInput`](crate::input::DeleteLifecyclePolicyInput)
    pub fn builder() -> crate::input::delete_lifecycle_policy_input::Builder {
        crate::input::delete_lifecycle_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteLifecyclePolicy {
    type Output = std::result::Result<
        crate::output::DeleteLifecyclePolicyOutput,
        crate::error::DeleteLifecyclePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_lifecycle_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_lifecycle_policy_response(response)
        }
    }
}

/// <p>Deletes the metric policy that is associated with the specified container. If there is no metric policy associated with the container, MediaStore doesn't send metrics to CloudWatch.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMetricPolicy {
    _private: (),
}
impl DeleteMetricPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteMetricPolicyInput`](crate::input::DeleteMetricPolicyInput)
    pub fn builder() -> crate::input::delete_metric_policy_input::Builder {
        crate::input::delete_metric_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteMetricPolicy {
    type Output = std::result::Result<
        crate::output::DeleteMetricPolicyOutput,
        crate::error::DeleteMetricPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_metric_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_metric_policy_response(response)
        }
    }
}

/// <p>Retrieves the properties of the requested container. This request is commonly used to
/// retrieve the endpoint of a container. An endpoint is a value assigned by the service when a
/// new container is created. A container's endpoint does not change after it has been
/// assigned. The <code>DescribeContainer</code> request returns a single
/// <code>Container</code> object based on <code>ContainerName</code>. To return all
/// <code>Container</code> objects that are associated with a specified AWS account, use
/// <a>ListContainers</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeContainer {
    _private: (),
}
impl DescribeContainer {
    /// Creates a new builder-style object to manufacture [`DescribeContainerInput`](crate::input::DescribeContainerInput)
    pub fn builder() -> crate::input::describe_container_input::Builder {
        crate::input::describe_container_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeContainer {
    type Output = std::result::Result<
        crate::output::DescribeContainerOutput,
        crate::error::DescribeContainerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_container_error(response)
        } else {
            crate::operation_deser::parse_describe_container_response(response)
        }
    }
}

/// <p>Retrieves the access policy for the specified container. For information about the
/// data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User
/// Guide</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetContainerPolicy {
    _private: (),
}
impl GetContainerPolicy {
    /// Creates a new builder-style object to manufacture [`GetContainerPolicyInput`](crate::input::GetContainerPolicyInput)
    pub fn builder() -> crate::input::get_container_policy_input::Builder {
        crate::input::get_container_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetContainerPolicy {
    type Output = std::result::Result<
        crate::output::GetContainerPolicyOutput,
        crate::error::GetContainerPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_container_policy_error(response)
        } else {
            crate::operation_deser::parse_get_container_policy_response(response)
        }
    }
}

/// <p>Returns the cross-origin resource sharing (CORS) configuration information that is
/// set for the container.</p>
/// <p>To use this operation, you must have permission to perform the
/// <code>MediaStore:GetCorsPolicy</code> action. By default, the container owner has this
/// permission and can grant it to others.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetCorsPolicy {
    _private: (),
}
impl GetCorsPolicy {
    /// Creates a new builder-style object to manufacture [`GetCorsPolicyInput`](crate::input::GetCorsPolicyInput)
    pub fn builder() -> crate::input::get_cors_policy_input::Builder {
        crate::input::get_cors_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetCorsPolicy {
    type Output =
        std::result::Result<crate::output::GetCorsPolicyOutput, crate::error::GetCorsPolicyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_cors_policy_error(response)
        } else {
            crate::operation_deser::parse_get_cors_policy_response(response)
        }
    }
}

/// <p>Retrieves the object lifecycle policy that is assigned to a container.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetLifecyclePolicy {
    _private: (),
}
impl GetLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`GetLifecyclePolicyInput`](crate::input::GetLifecyclePolicyInput)
    pub fn builder() -> crate::input::get_lifecycle_policy_input::Builder {
        crate::input::get_lifecycle_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetLifecyclePolicy {
    type Output = std::result::Result<
        crate::output::GetLifecyclePolicyOutput,
        crate::error::GetLifecyclePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lifecycle_policy_error(response)
        } else {
            crate::operation_deser::parse_get_lifecycle_policy_response(response)
        }
    }
}

/// <p>Returns the metric policy for the specified container. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMetricPolicy {
    _private: (),
}
impl GetMetricPolicy {
    /// Creates a new builder-style object to manufacture [`GetMetricPolicyInput`](crate::input::GetMetricPolicyInput)
    pub fn builder() -> crate::input::get_metric_policy_input::Builder {
        crate::input::get_metric_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetMetricPolicy {
    type Output = std::result::Result<
        crate::output::GetMetricPolicyOutput,
        crate::error::GetMetricPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_metric_policy_error(response)
        } else {
            crate::operation_deser::parse_get_metric_policy_response(response)
        }
    }
}

/// <p>Lists the properties of all containers in AWS Elemental MediaStore. </p>
/// <p>You can query to receive all the containers in one response. Or you can include the
/// <code>MaxResults</code> parameter to receive a limited number of containers in each
/// response. In this case, the response includes a token. To get the next set of containers,
/// send the command again, this time with the <code>NextToken</code> parameter (with the
/// returned token as its value). The next set of responses appears, with a token if there are
/// still more containers to receive. </p>
/// <p>See also <a>DescribeContainer</a>, which gets the properties of one
/// container. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListContainers {
    _private: (),
}
impl ListContainers {
    /// Creates a new builder-style object to manufacture [`ListContainersInput`](crate::input::ListContainersInput)
    pub fn builder() -> crate::input::list_containers_input::Builder {
        crate::input::list_containers_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListContainers {
    type Output =
        std::result::Result<crate::output::ListContainersOutput, crate::error::ListContainersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_containers_error(response)
        } else {
            crate::operation_deser::parse_list_containers_response(response)
        }
    }
}

/// <p>Returns a list of the tags assigned to the specified container. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Creates an access policy for the specified container to restrict the users and
/// clients that can access it. For information about the data that is included in an access
/// policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and
/// Access Management User Guide</a>.</p>
/// <p>For this release of the REST API, you can create only one policy for a container. If
/// you enter <code>PutContainerPolicy</code> twice, the second command modifies the existing
/// policy. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutContainerPolicy {
    _private: (),
}
impl PutContainerPolicy {
    /// Creates a new builder-style object to manufacture [`PutContainerPolicyInput`](crate::input::PutContainerPolicyInput)
    pub fn builder() -> crate::input::put_container_policy_input::Builder {
        crate::input::put_container_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutContainerPolicy {
    type Output = std::result::Result<
        crate::output::PutContainerPolicyOutput,
        crate::error::PutContainerPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_container_policy_error(response)
        } else {
            crate::operation_deser::parse_put_container_policy_response(response)
        }
    }
}

/// <p>Sets the cross-origin resource sharing (CORS) configuration on a container so that
/// the container can service cross-origin requests. For example, you might want to enable a
/// request whose origin is http://www.example.com to access your AWS Elemental MediaStore
/// container at my.example.container.com by using the browser's XMLHttpRequest
/// capability.</p>
/// <p>To enable CORS on a container, you attach a CORS policy to the container. In the CORS
/// policy, you configure rules that identify origins and the HTTP methods that can be executed
/// on your container. The policy can contain up to 398,000 characters. You can add up to 100
/// rules to a CORS policy. If more than one rule applies, the service uses the first
/// applicable rule listed.</p>
/// <p>To learn more about CORS, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/cors-policy.html">Cross-Origin Resource Sharing (CORS) in AWS Elemental MediaStore</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutCorsPolicy {
    _private: (),
}
impl PutCorsPolicy {
    /// Creates a new builder-style object to manufacture [`PutCorsPolicyInput`](crate::input::PutCorsPolicyInput)
    pub fn builder() -> crate::input::put_cors_policy_input::Builder {
        crate::input::put_cors_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutCorsPolicy {
    type Output =
        std::result::Result<crate::output::PutCorsPolicyOutput, crate::error::PutCorsPolicyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_cors_policy_error(response)
        } else {
            crate::operation_deser::parse_put_cors_policy_response(response)
        }
    }
}

/// <p>Writes an object lifecycle policy to a container. If the container already has an object lifecycle policy, the service replaces the existing policy with the new policy. It takes up to 20 minutes for the change to take effect.</p>
/// <p>For information about how to construct an object lifecycle policy, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/policies-object-lifecycle-components.html">Components of an Object Lifecycle Policy</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutLifecyclePolicy {
    _private: (),
}
impl PutLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`PutLifecyclePolicyInput`](crate::input::PutLifecyclePolicyInput)
    pub fn builder() -> crate::input::put_lifecycle_policy_input::Builder {
        crate::input::put_lifecycle_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutLifecyclePolicy {
    type Output = std::result::Result<
        crate::output::PutLifecyclePolicyOutput,
        crate::error::PutLifecyclePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_lifecycle_policy_error(response)
        } else {
            crate::operation_deser::parse_put_lifecycle_policy_response(response)
        }
    }
}

/// <p>The metric policy that you want to add to the container. A metric policy allows AWS Elemental MediaStore to send metrics to Amazon CloudWatch. It takes up to 20 minutes for the new policy to take effect.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutMetricPolicy {
    _private: (),
}
impl PutMetricPolicy {
    /// Creates a new builder-style object to manufacture [`PutMetricPolicyInput`](crate::input::PutMetricPolicyInput)
    pub fn builder() -> crate::input::put_metric_policy_input::Builder {
        crate::input::put_metric_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutMetricPolicy {
    type Output = std::result::Result<
        crate::output::PutMetricPolicyOutput,
        crate::error::PutMetricPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_metric_policy_error(response)
        } else {
            crate::operation_deser::parse_put_metric_policy_response(response)
        }
    }
}

/// <p>Starts access logging on the specified container. When you enable access logging on a container, MediaStore delivers access logs for objects stored in that container to Amazon CloudWatch Logs.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartAccessLogging {
    _private: (),
}
impl StartAccessLogging {
    /// Creates a new builder-style object to manufacture [`StartAccessLoggingInput`](crate::input::StartAccessLoggingInput)
    pub fn builder() -> crate::input::start_access_logging_input::Builder {
        crate::input::start_access_logging_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartAccessLogging {
    type Output = std::result::Result<
        crate::output::StartAccessLoggingOutput,
        crate::error::StartAccessLoggingError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_access_logging_error(response)
        } else {
            crate::operation_deser::parse_start_access_logging_response(response)
        }
    }
}

/// <p>Stops access logging on the specified container. When you stop access logging on a container, MediaStore stops sending access logs to Amazon CloudWatch Logs. These access logs are not saved and are not retrievable.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopAccessLogging {
    _private: (),
}
impl StopAccessLogging {
    /// Creates a new builder-style object to manufacture [`StopAccessLoggingInput`](crate::input::StopAccessLoggingInput)
    pub fn builder() -> crate::input::stop_access_logging_input::Builder {
        crate::input::stop_access_logging_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopAccessLogging {
    type Output = std::result::Result<
        crate::output::StopAccessLoggingOutput,
        crate::error::StopAccessLoggingError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_access_logging_error(response)
        } else {
            crate::operation_deser::parse_stop_access_logging_response(response)
        }
    }
}

/// <p>Adds tags to the specified AWS Elemental MediaStore container. Tags are key:value pairs that you can associate with AWS resources. For example, the
/// tag key might be "customer" and the tag value might be "companyA." You can specify one or more tags to add to each container. You can add up to 50
/// tags to each container. For more information about tagging, including naming and usage conventions, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/tagging.html">Tagging Resources in MediaStore</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes tags from the specified container. You can specify one or more tags to remove. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}
