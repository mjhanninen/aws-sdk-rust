// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Create a new FinSpace environment.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEnvironment {
    _private: (),
}
impl CreateEnvironment {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentInput`](crate::input::CreateEnvironmentInput)
    pub fn builder() -> crate::input::create_environment_input::Builder {
        crate::input::create_environment_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateEnvironment {
    type Output = std::result::Result<
        crate::output::CreateEnvironmentOutput,
        crate::error::CreateEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_environment_error(response)
        } else {
            crate::operation_deser::parse_create_environment_response(response)
        }
    }
}

/// <p>Delete an FinSpace environment.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEnvironment {
    _private: (),
}
impl DeleteEnvironment {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentInput`](crate::input::DeleteEnvironmentInput)
    pub fn builder() -> crate::input::delete_environment_input::Builder {
        crate::input::delete_environment_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteEnvironment {
    type Output = std::result::Result<
        crate::output::DeleteEnvironmentOutput,
        crate::error::DeleteEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_environment_error(response)
        } else {
            crate::operation_deser::parse_delete_environment_response(response)
        }
    }
}

/// <p>Returns the FinSpace environment object.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEnvironment {
    _private: (),
}
impl GetEnvironment {
    /// Creates a new builder-style object to manufacture [`GetEnvironmentInput`](crate::input::GetEnvironmentInput)
    pub fn builder() -> crate::input::get_environment_input::Builder {
        crate::input::get_environment_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetEnvironment {
    type Output =
        std::result::Result<crate::output::GetEnvironmentOutput, crate::error::GetEnvironmentError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_environment_error(response)
        } else {
            crate::operation_deser::parse_get_environment_response(response)
        }
    }
}

/// <p>A list of all of your FinSpace environments.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEnvironments {
    _private: (),
}
impl ListEnvironments {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentsInput`](crate::input::ListEnvironmentsInput)
    pub fn builder() -> crate::input::list_environments_input::Builder {
        crate::input::list_environments_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListEnvironments {
    type Output = std::result::Result<
        crate::output::ListEnvironmentsOutput,
        crate::error::ListEnvironmentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_environments_error(response)
        } else {
            crate::operation_deser::parse_list_environments_response(response)
        }
    }
}

/// <p>A list of all tags for a resource.</p>
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

/// <p>Adds metadata tags to a FinSpace resource.</p>
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

/// <p>Removes metadata tags from a FinSpace resource.</p>
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

/// <p>Update your FinSpace environment.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateEnvironment {
    _private: (),
}
impl UpdateEnvironment {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentInput`](crate::input::UpdateEnvironmentInput)
    pub fn builder() -> crate::input::update_environment_input::Builder {
        crate::input::update_environment_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateEnvironment {
    type Output = std::result::Result<
        crate::output::UpdateEnvironmentOutput,
        crate::error::UpdateEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_environment_error(response)
        } else {
            crate::operation_deser::parse_update_environment_response(response)
        }
    }
}
