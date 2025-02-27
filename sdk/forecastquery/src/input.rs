// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`QueryForecastInput`](crate::input::QueryForecastInput)
pub mod query_forecast_input {
    /// A builder for [`QueryForecastInput`](crate::input::QueryForecastInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) forecast_arn: std::option::Option<std::string::String>,
        pub(crate) start_date: std::option::Option<std::string::String>,
        pub(crate) end_date: std::option::Option<std::string::String>,
        pub(crate) filters: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
        pub fn forecast_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.forecast_arn = Some(input.into());
            self
        }
        pub fn set_forecast_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.forecast_arn = input;
            self
        }
        /// <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss
        /// (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
        pub fn start_date(mut self, input: impl Into<std::string::String>) -> Self {
            self.start_date = Some(input.into());
            self
        }
        pub fn set_start_date(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.start_date = input;
            self
        }
        /// <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss
        /// (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
        pub fn end_date(mut self, input: impl Into<std::string::String>) -> Self {
            self.end_date = Some(input.into());
            self
        }
        pub fn set_end_date(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.end_date = input;
            self
        }
        pub fn filters(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.filters.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.filters = Some(hash_map);
            self
        }
        pub fn set_filters(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.filters = input;
            self
        }
        /// <p>If the result of the previous request was truncated, the response includes a
        /// <code>NextToken</code>. To retrieve the next set of results, use the token in the next
        /// request. Tokens expire after 24 hours.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`QueryForecastInput`](crate::input::QueryForecastInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::QueryForecastInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::QueryForecastInput {
                forecast_arn: self.forecast_arn,
                start_date: self.start_date,
                end_date: self.end_date,
                filters: self.filters,
                next_token: self.next_token,
            })
        }
    }
}
#[doc(hidden)]
pub type QueryForecastInputOperationOutputAlias = crate::operation::QueryForecast;
#[doc(hidden)]
pub type QueryForecastInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl QueryForecastInput {
    /// Consumes the builder and constructs an Operation<[`QueryForecast`](crate::operation::QueryForecast)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::QueryForecast,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_query_forecast(&self).map_err(|err| {
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
                crate::operation::QueryForecast::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "QueryForecast",
                "forecastquery",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "AmazonForecastRuntime.QueryForecast",
        );
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
    /// Creates a new builder-style object to manufacture [`QueryForecastInput`](crate::input::QueryForecastInput)
    pub fn builder() -> crate::input::query_forecast_input::Builder {
        crate::input::query_forecast_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct QueryForecastInput {
    /// <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
    pub forecast_arn: std::option::Option<std::string::String>,
    /// <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss
    /// (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    pub start_date: std::option::Option<std::string::String>,
    /// <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss
    /// (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    pub end_date: std::option::Option<std::string::String>,
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the
    /// forecast for <code>client_21</code> in the electricity usage dataset, specify the
    /// following:</p>
    /// <p>
    /// <code>{"item_id" : "client_21"}</code>
    /// </p>
    /// <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p>
    pub filters:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>If the result of the previous request was truncated, the response includes a
    /// <code>NextToken</code>. To retrieve the next set of results, use the token in the next
    /// request. Tokens expire after 24 hours.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for QueryForecastInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("QueryForecastInput");
        formatter.field("forecast_arn", &self.forecast_arn);
        formatter.field("start_date", &self.start_date);
        formatter.field("end_date", &self.end_date);
        formatter.field("filters", &self.filters);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
