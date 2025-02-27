// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`CreateChangesetInput`](crate::input::CreateChangesetInput)
pub mod create_changeset_input {
    /// A builder for [`CreateChangesetInput`](crate::input::CreateChangesetInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) dataset_id: std::option::Option<std::string::String>,
        pub(crate) change_type: std::option::Option<crate::model::ChangeType>,
        pub(crate) source_type: std::option::Option<crate::model::SourceType>,
        pub(crate) source_params: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) format_type: std::option::Option<crate::model::FormatType>,
        pub(crate) format_params: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The unique identifier for the FinSpace dataset in which the changeset will be
        /// created.</p>
        pub fn dataset_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.dataset_id = Some(input.into());
            self
        }
        pub fn set_dataset_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.dataset_id = input;
            self
        }
        /// <p>Option to indicate how a changeset will be applied to a dataset.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>REPLACE</code> - Changeset will be considered as a replacement to all prior
        /// loaded changesets.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>APPEND</code> - Changeset will be considered as an addition to the end of all
        /// prior loaded changesets.</p>
        /// </li>
        /// </ul>
        pub fn change_type(mut self, input: crate::model::ChangeType) -> Self {
            self.change_type = Some(input);
            self
        }
        pub fn set_change_type(
            mut self,
            input: std::option::Option<crate::model::ChangeType>,
        ) -> Self {
            self.change_type = input;
            self
        }
        /// <p>Type of the data source from which the files to create the changeset will be
        /// sourced.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>S3</code> - Amazon S3.</p>
        /// </li>
        /// </ul>
        pub fn source_type(mut self, input: crate::model::SourceType) -> Self {
            self.source_type = Some(input);
            self
        }
        pub fn set_source_type(
            mut self,
            input: std::option::Option<crate::model::SourceType>,
        ) -> Self {
            self.source_type = input;
            self
        }
        pub fn source_params(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.source_params.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.source_params = Some(hash_map);
            self
        }
        pub fn set_source_params(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.source_params = input;
            self
        }
        /// <p>Format type of the input files being loaded into the changeset.</p>
        pub fn format_type(mut self, input: crate::model::FormatType) -> Self {
            self.format_type = Some(input);
            self
        }
        pub fn set_format_type(
            mut self,
            input: std::option::Option<crate::model::FormatType>,
        ) -> Self {
            self.format_type = input;
            self
        }
        pub fn format_params(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.format_params.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.format_params = Some(hash_map);
            self
        }
        pub fn set_format_params(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.format_params = input;
            self
        }
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateChangesetInput`](crate::input::CreateChangesetInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::CreateChangesetInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::CreateChangesetInput {
                dataset_id: self.dataset_id,
                change_type: self.change_type,
                source_type: self.source_type,
                source_params: self.source_params,
                format_type: self.format_type,
                format_params: self.format_params,
                tags: self.tags,
            })
        }
    }
}
#[doc(hidden)]
pub type CreateChangesetInputOperationOutputAlias = crate::operation::CreateChangeset;
#[doc(hidden)]
pub type CreateChangesetInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl CreateChangesetInput {
    /// Consumes the builder and constructs an Operation<[`CreateChangeset`](crate::operation::CreateChangeset)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::CreateChangeset,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_create_changeset(&self).map_err(
                |err| smithy_http::operation::BuildError::SerializationError(err.into()),
            )?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::CreateChangeset::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "CreateChangeset",
                "finspacedata",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let input_1 = &self.dataset_id;
        let input_1 = input_1
            .as_ref()
            .ok_or(smithy_http::operation::BuildError::MissingField {
                field: "dataset_id",
                details: "cannot be empty or unset",
            })?;
        let dataset_id = smithy_http::label::fmt_string(input_1, false);
        if dataset_id.is_empty() {
            return Err(smithy_http::operation::BuildError::MissingField {
                field: "dataset_id",
                details: "cannot be empty or unset",
            });
        }
        write!(
            output,
            "/datasets/{datasetId}/changesets",
            datasetId = dataset_id
        )
        .expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`CreateChangesetInput`](crate::input::CreateChangesetInput)
    pub fn builder() -> crate::input::create_changeset_input::Builder {
        crate::input::create_changeset_input::Builder::default()
    }
}

/// See [`GetProgrammaticAccessCredentialsInput`](crate::input::GetProgrammaticAccessCredentialsInput)
pub mod get_programmatic_access_credentials_input {
    /// A builder for [`GetProgrammaticAccessCredentialsInput`](crate::input::GetProgrammaticAccessCredentialsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) duration_in_minutes: std::option::Option<i64>,
        pub(crate) environment_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The time duration in which the credentials remain valid. </p>
        pub fn duration_in_minutes(mut self, input: i64) -> Self {
            self.duration_in_minutes = Some(input);
            self
        }
        pub fn set_duration_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.duration_in_minutes = input;
            self
        }
        /// <p>The habanero environment identifier.</p>
        pub fn environment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_id = Some(input.into());
            self
        }
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.environment_id = input;
            self
        }
        /// Consumes the builder and constructs a [`GetProgrammaticAccessCredentialsInput`](crate::input::GetProgrammaticAccessCredentialsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetProgrammaticAccessCredentialsInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetProgrammaticAccessCredentialsInput {
                duration_in_minutes: self.duration_in_minutes.unwrap_or_default(),
                environment_id: self.environment_id,
            })
        }
    }
}
#[doc(hidden)]
pub type GetProgrammaticAccessCredentialsInputOperationOutputAlias =
    crate::operation::GetProgrammaticAccessCredentials;
#[doc(hidden)]
pub type GetProgrammaticAccessCredentialsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetProgrammaticAccessCredentialsInput {
    /// Consumes the builder and constructs an Operation<[`GetProgrammaticAccessCredentials`](crate::operation::GetProgrammaticAccessCredentials)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetProgrammaticAccessCredentials,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::GetProgrammaticAccessCredentials::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetProgrammaticAccessCredentials",
                "finspacedata",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/credentials/programmatic").expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if self.duration_in_minutes != 0 {
            query.push_kv(
                "durationInMinutes",
                &smithy_types::primitive::Encoder::from(self.duration_in_minutes).encode(),
            );
        }
        if let Some(inner_2) = &self.environment_id {
            query.push_kv("environmentId", &smithy_http::query::fmt_string(&inner_2));
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("GET").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetProgrammaticAccessCredentialsInput`](crate::input::GetProgrammaticAccessCredentialsInput)
    pub fn builder() -> crate::input::get_programmatic_access_credentials_input::Builder {
        crate::input::get_programmatic_access_credentials_input::Builder::default()
    }
}

/// See [`GetWorkingLocationInput`](crate::input::GetWorkingLocationInput)
pub mod get_working_location_input {
    /// A builder for [`GetWorkingLocationInput`](crate::input::GetWorkingLocationInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) location_type: std::option::Option<crate::model::LocationType>,
    }
    impl Builder {
        /// <p>Specify the type of the working location.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>SAGEMAKER</code> - Use the Amazon S3 location as a temporary location to store data content when
        /// working with FinSpace Notebooks that run on SageMaker studio.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>INGESTION</code> - Use the Amazon S3 location as a staging location to copy your
        /// data content and then use the location with the changeset creation operation.</p>
        /// </li>
        /// </ul>
        pub fn location_type(mut self, input: crate::model::LocationType) -> Self {
            self.location_type = Some(input);
            self
        }
        pub fn set_location_type(
            mut self,
            input: std::option::Option<crate::model::LocationType>,
        ) -> Self {
            self.location_type = input;
            self
        }
        /// Consumes the builder and constructs a [`GetWorkingLocationInput`](crate::input::GetWorkingLocationInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetWorkingLocationInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetWorkingLocationInput {
                location_type: self.location_type,
            })
        }
    }
}
#[doc(hidden)]
pub type GetWorkingLocationInputOperationOutputAlias = crate::operation::GetWorkingLocation;
#[doc(hidden)]
pub type GetWorkingLocationInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetWorkingLocationInput {
    /// Consumes the builder and constructs an Operation<[`GetWorkingLocation`](crate::operation::GetWorkingLocation)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetWorkingLocation,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_get_working_location(&self)
                .map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::GetWorkingLocation::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetWorkingLocation",
                "finspacedata",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/workingLocationV1").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetWorkingLocationInput`](crate::input::GetWorkingLocationInput)
    pub fn builder() -> crate::input::get_working_location_input::Builder {
        crate::input::get_working_location_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetWorkingLocationInput {
    /// <p>Specify the type of the working location.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>SAGEMAKER</code> - Use the Amazon S3 location as a temporary location to store data content when
    /// working with FinSpace Notebooks that run on SageMaker studio.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>INGESTION</code> - Use the Amazon S3 location as a staging location to copy your
    /// data content and then use the location with the changeset creation operation.</p>
    /// </li>
    /// </ul>
    pub location_type: std::option::Option<crate::model::LocationType>,
}
impl std::fmt::Debug for GetWorkingLocationInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetWorkingLocationInput");
        formatter.field("location_type", &self.location_type);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetProgrammaticAccessCredentialsInput {
    /// <p>The time duration in which the credentials remain valid. </p>
    pub duration_in_minutes: i64,
    /// <p>The habanero environment identifier.</p>
    pub environment_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for GetProgrammaticAccessCredentialsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetProgrammaticAccessCredentialsInput");
        formatter.field("duration_in_minutes", &self.duration_in_minutes);
        formatter.field("environment_id", &self.environment_id);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateChangesetInput {
    /// <p>The unique identifier for the FinSpace dataset in which the changeset will be
    /// created.</p>
    pub dataset_id: std::option::Option<std::string::String>,
    /// <p>Option to indicate how a changeset will be applied to a dataset.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>REPLACE</code> - Changeset will be considered as a replacement to all prior
    /// loaded changesets.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>APPEND</code> - Changeset will be considered as an addition to the end of all
    /// prior loaded changesets.</p>
    /// </li>
    /// </ul>
    pub change_type: std::option::Option<crate::model::ChangeType>,
    /// <p>Type of the data source from which the files to create the changeset will be
    /// sourced.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>S3</code> - Amazon S3.</p>
    /// </li>
    /// </ul>
    pub source_type: std::option::Option<crate::model::SourceType>,
    /// <p>Source path from which the files to create the changeset will be sourced.</p>
    pub source_params:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>Format type of the input files being loaded into the changeset.</p>
    pub format_type: std::option::Option<crate::model::FormatType>,
    /// <p>Options that define the structure of the source file(s).</p>
    pub format_params:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>Metadata tags to apply to this changeset.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for CreateChangesetInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateChangesetInput");
        formatter.field("dataset_id", &self.dataset_id);
        formatter.field("change_type", &self.change_type);
        formatter.field("source_type", &self.source_type);
        formatter.field("source_params", &self.source_params);
        formatter.field("format_type", &self.format_type);
        formatter.field("format_params", &self.format_params);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
