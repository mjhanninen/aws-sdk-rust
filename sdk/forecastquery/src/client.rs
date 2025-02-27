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

/// An ergonomic service client for `AmazonForecastRuntime`.
///
/// This client allows ergonomic access to a `AmazonForecastRuntime`-shaped service.
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
    pub fn query_forecast(&self) -> fluent_builders::QueryForecast<C, M, R> {
        fluent_builders::QueryForecast::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct QueryForecast<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::query_forecast_input::Builder,
    }
    impl<C, M, R> QueryForecast<C, M, R>
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
            crate::output::QueryForecastOutput,
            smithy_http::result::SdkError<crate::error::QueryForecastError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::QueryForecastInputOperationOutputAlias,
                crate::output::QueryForecastOutput,
                crate::error::QueryForecastError,
                crate::input::QueryForecastInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
        pub fn forecast_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.forecast_arn(inp);
            self
        }
        pub fn set_forecast_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_forecast_arn(input);
            self
        }
        /// <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss
        /// (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
        pub fn start_date(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.start_date(inp);
            self
        }
        pub fn set_start_date(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_start_date(input);
            self
        }
        /// <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss
        /// (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
        pub fn end_date(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.end_date(inp);
            self
        }
        pub fn set_end_date(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_end_date(input);
            self
        }
        /// Adds a key-value pair to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the
        /// forecast for <code>client_21</code> in the electricity usage dataset, specify the
        /// following:</p>
        /// <p>
        /// <code>{"item_id" : "client_21"}</code>
        /// </p>
        /// <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p>
        pub fn filters(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.filters(k, v);
            self
        }
        pub fn set_filters(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
        /// <p>If the result of the previous request was truncated, the response includes a
        /// <code>NextToken</code>. To retrieve the next set of results, use the token in the next
        /// request. Tokens expire after 24 hours.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
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
