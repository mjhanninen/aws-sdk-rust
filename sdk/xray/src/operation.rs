// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Retrieves a list of traces specified by ID. Each trace is a collection of segment
/// documents that originates from a single request. Use <code>GetTraceSummaries</code> to get a
/// list of trace IDs.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchGetTraces {
    _private: (),
}
impl BatchGetTraces {
    /// Creates a new builder-style object to manufacture [`BatchGetTracesInput`](crate::input::BatchGetTracesInput)
    pub fn builder() -> crate::input::batch_get_traces_input::Builder {
        crate::input::batch_get_traces_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for BatchGetTraces {
    type Output =
        std::result::Result<crate::output::BatchGetTracesOutput, crate::error::BatchGetTracesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_traces_error(response)
        } else {
            crate::operation_deser::parse_batch_get_traces_response(response)
        }
    }
}

/// <p>Creates a group resource with a name and a filter expression. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateGroup {
    _private: (),
}
impl CreateGroup {
    /// Creates a new builder-style object to manufacture [`CreateGroupInput`](crate::input::CreateGroupInput)
    pub fn builder() -> crate::input::create_group_input::Builder {
        crate::input::create_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateGroup {
    type Output =
        std::result::Result<crate::output::CreateGroupOutput, crate::error::CreateGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_group_error(response)
        } else {
            crate::operation_deser::parse_create_group_response(response)
        }
    }
}

/// <p>Creates a rule to control sampling behavior for instrumented applications. Services
/// retrieve rules with <a>GetSamplingRules</a>, and evaluate each rule in ascending
/// order of <i>priority</i> for each request. If a rule matches, the service
/// records a trace, borrowing it from the reservoir size. After 10 seconds, the service
/// reports back to X-Ray with <a>GetSamplingTargets</a> to get updated versions of
/// each in-use rule. The updated rule contains a trace quota that the service can use instead
/// of borrowing from the reservoir.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSamplingRule {
    _private: (),
}
impl CreateSamplingRule {
    /// Creates a new builder-style object to manufacture [`CreateSamplingRuleInput`](crate::input::CreateSamplingRuleInput)
    pub fn builder() -> crate::input::create_sampling_rule_input::Builder {
        crate::input::create_sampling_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateSamplingRule {
    type Output = std::result::Result<
        crate::output::CreateSamplingRuleOutput,
        crate::error::CreateSamplingRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_sampling_rule_error(response)
        } else {
            crate::operation_deser::parse_create_sampling_rule_response(response)
        }
    }
}

/// <p>Deletes a group resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteGroup {
    _private: (),
}
impl DeleteGroup {
    /// Creates a new builder-style object to manufacture [`DeleteGroupInput`](crate::input::DeleteGroupInput)
    pub fn builder() -> crate::input::delete_group_input::Builder {
        crate::input::delete_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteGroup {
    type Output =
        std::result::Result<crate::output::DeleteGroupOutput, crate::error::DeleteGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_group_error(response)
        } else {
            crate::operation_deser::parse_delete_group_response(response)
        }
    }
}

/// <p>Deletes a sampling rule.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSamplingRule {
    _private: (),
}
impl DeleteSamplingRule {
    /// Creates a new builder-style object to manufacture [`DeleteSamplingRuleInput`](crate::input::DeleteSamplingRuleInput)
    pub fn builder() -> crate::input::delete_sampling_rule_input::Builder {
        crate::input::delete_sampling_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSamplingRule {
    type Output = std::result::Result<
        crate::output::DeleteSamplingRuleOutput,
        crate::error::DeleteSamplingRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_sampling_rule_error(response)
        } else {
            crate::operation_deser::parse_delete_sampling_rule_response(response)
        }
    }
}

/// <p>Retrieves the current encryption configuration for X-Ray data.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEncryptionConfig {
    _private: (),
}
impl GetEncryptionConfig {
    /// Creates a new builder-style object to manufacture [`GetEncryptionConfigInput`](crate::input::GetEncryptionConfigInput)
    pub fn builder() -> crate::input::get_encryption_config_input::Builder {
        crate::input::get_encryption_config_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetEncryptionConfig {
    type Output = std::result::Result<
        crate::output::GetEncryptionConfigOutput,
        crate::error::GetEncryptionConfigError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_encryption_config_error(response)
        } else {
            crate::operation_deser::parse_get_encryption_config_response(response)
        }
    }
}

/// <p>Retrieves group resource details.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetGroup {
    _private: (),
}
impl GetGroup {
    /// Creates a new builder-style object to manufacture [`GetGroupInput`](crate::input::GetGroupInput)
    pub fn builder() -> crate::input::get_group_input::Builder {
        crate::input::get_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetGroup {
    type Output = std::result::Result<crate::output::GetGroupOutput, crate::error::GetGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_group_error(response)
        } else {
            crate::operation_deser::parse_get_group_response(response)
        }
    }
}

/// <p>Retrieves all active group details.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetGroups {
    _private: (),
}
impl GetGroups {
    /// Creates a new builder-style object to manufacture [`GetGroupsInput`](crate::input::GetGroupsInput)
    pub fn builder() -> crate::input::get_groups_input::Builder {
        crate::input::get_groups_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetGroups {
    type Output = std::result::Result<crate::output::GetGroupsOutput, crate::error::GetGroupsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_groups_error(response)
        } else {
            crate::operation_deser::parse_get_groups_response(response)
        }
    }
}

/// <p>Retrieves the summary information of an insight. This includes impact to clients and
/// root cause services, the top anomalous services, the category, the state of the insight,
/// and the start and end time of the insight.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInsight {
    _private: (),
}
impl GetInsight {
    /// Creates a new builder-style object to manufacture [`GetInsightInput`](crate::input::GetInsightInput)
    pub fn builder() -> crate::input::get_insight_input::Builder {
        crate::input::get_insight_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetInsight {
    type Output =
        std::result::Result<crate::output::GetInsightOutput, crate::error::GetInsightError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_error(response)
        } else {
            crate::operation_deser::parse_get_insight_response(response)
        }
    }
}

/// <p>X-Ray reevaluates insights periodically until they're resolved, and records each intermediate state as an
/// event. You can review an insight's events in the Impact Timeline on the Inspect page in the X-Ray
/// console.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInsightEvents {
    _private: (),
}
impl GetInsightEvents {
    /// Creates a new builder-style object to manufacture [`GetInsightEventsInput`](crate::input::GetInsightEventsInput)
    pub fn builder() -> crate::input::get_insight_events_input::Builder {
        crate::input::get_insight_events_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetInsightEvents {
    type Output = std::result::Result<
        crate::output::GetInsightEventsOutput,
        crate::error::GetInsightEventsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_events_error(response)
        } else {
            crate::operation_deser::parse_get_insight_events_response(response)
        }
    }
}

/// <p>Retrieves a service graph structure filtered by the specified insight. The service graph is limited to only
/// structural information. For a complete service graph, use this API with the GetServiceGraph API.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInsightImpactGraph {
    _private: (),
}
impl GetInsightImpactGraph {
    /// Creates a new builder-style object to manufacture [`GetInsightImpactGraphInput`](crate::input::GetInsightImpactGraphInput)
    pub fn builder() -> crate::input::get_insight_impact_graph_input::Builder {
        crate::input::get_insight_impact_graph_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetInsightImpactGraph {
    type Output = std::result::Result<
        crate::output::GetInsightImpactGraphOutput,
        crate::error::GetInsightImpactGraphError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_impact_graph_error(response)
        } else {
            crate::operation_deser::parse_get_insight_impact_graph_response(response)
        }
    }
}

/// <p>Retrieves the summaries of all insights in the specified group matching the provided filter values.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInsightSummaries {
    _private: (),
}
impl GetInsightSummaries {
    /// Creates a new builder-style object to manufacture [`GetInsightSummariesInput`](crate::input::GetInsightSummariesInput)
    pub fn builder() -> crate::input::get_insight_summaries_input::Builder {
        crate::input::get_insight_summaries_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetInsightSummaries {
    type Output = std::result::Result<
        crate::output::GetInsightSummariesOutput,
        crate::error::GetInsightSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_insight_summaries_response(response)
        }
    }
}

/// <p>Retrieves all sampling rules.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSamplingRules {
    _private: (),
}
impl GetSamplingRules {
    /// Creates a new builder-style object to manufacture [`GetSamplingRulesInput`](crate::input::GetSamplingRulesInput)
    pub fn builder() -> crate::input::get_sampling_rules_input::Builder {
        crate::input::get_sampling_rules_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetSamplingRules {
    type Output = std::result::Result<
        crate::output::GetSamplingRulesOutput,
        crate::error::GetSamplingRulesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_sampling_rules_error(response)
        } else {
            crate::operation_deser::parse_get_sampling_rules_response(response)
        }
    }
}

/// <p>Retrieves information about recent sampling results for all sampling rules.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSamplingStatisticSummaries {
    _private: (),
}
impl GetSamplingStatisticSummaries {
    /// Creates a new builder-style object to manufacture [`GetSamplingStatisticSummariesInput`](crate::input::GetSamplingStatisticSummariesInput)
    pub fn builder() -> crate::input::get_sampling_statistic_summaries_input::Builder {
        crate::input::get_sampling_statistic_summaries_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetSamplingStatisticSummaries {
    type Output = std::result::Result<
        crate::output::GetSamplingStatisticSummariesOutput,
        crate::error::GetSamplingStatisticSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_sampling_statistic_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_sampling_statistic_summaries_response(response)
        }
    }
}

/// <p>Requests a sampling quota for rules that the service is using to sample requests.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSamplingTargets {
    _private: (),
}
impl GetSamplingTargets {
    /// Creates a new builder-style object to manufacture [`GetSamplingTargetsInput`](crate::input::GetSamplingTargetsInput)
    pub fn builder() -> crate::input::get_sampling_targets_input::Builder {
        crate::input::get_sampling_targets_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetSamplingTargets {
    type Output = std::result::Result<
        crate::output::GetSamplingTargetsOutput,
        crate::error::GetSamplingTargetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_sampling_targets_error(response)
        } else {
            crate::operation_deser::parse_get_sampling_targets_response(response)
        }
    }
}

/// <p>Retrieves a document that describes services that process incoming requests, and
/// downstream services that they call as a result. Root services process incoming requests and
/// make calls to downstream services. Root services are applications that use the <a href="https://docs.aws.amazon.com/xray/index.html">AWS X-Ray SDK</a>.
/// Downstream services can be other applications, AWS resources, HTTP web APIs, or SQL
/// databases.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetServiceGraph {
    _private: (),
}
impl GetServiceGraph {
    /// Creates a new builder-style object to manufacture [`GetServiceGraphInput`](crate::input::GetServiceGraphInput)
    pub fn builder() -> crate::input::get_service_graph_input::Builder {
        crate::input::get_service_graph_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetServiceGraph {
    type Output = std::result::Result<
        crate::output::GetServiceGraphOutput,
        crate::error::GetServiceGraphError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_graph_error(response)
        } else {
            crate::operation_deser::parse_get_service_graph_response(response)
        }
    }
}

/// <p>Get an aggregation of service statistics defined by a specific time
/// range.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTimeSeriesServiceStatistics {
    _private: (),
}
impl GetTimeSeriesServiceStatistics {
    /// Creates a new builder-style object to manufacture [`GetTimeSeriesServiceStatisticsInput`](crate::input::GetTimeSeriesServiceStatisticsInput)
    pub fn builder() -> crate::input::get_time_series_service_statistics_input::Builder {
        crate::input::get_time_series_service_statistics_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetTimeSeriesServiceStatistics {
    type Output = std::result::Result<
        crate::output::GetTimeSeriesServiceStatisticsOutput,
        crate::error::GetTimeSeriesServiceStatisticsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_time_series_service_statistics_error(response)
        } else {
            crate::operation_deser::parse_get_time_series_service_statistics_response(response)
        }
    }
}

/// <p>Retrieves a service graph for one or more specific trace IDs.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTraceGraph {
    _private: (),
}
impl GetTraceGraph {
    /// Creates a new builder-style object to manufacture [`GetTraceGraphInput`](crate::input::GetTraceGraphInput)
    pub fn builder() -> crate::input::get_trace_graph_input::Builder {
        crate::input::get_trace_graph_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetTraceGraph {
    type Output =
        std::result::Result<crate::output::GetTraceGraphOutput, crate::error::GetTraceGraphError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_trace_graph_error(response)
        } else {
            crate::operation_deser::parse_get_trace_graph_response(response)
        }
    }
}

/// <p>Retrieves IDs and annotations for traces available for a specified time frame using an
/// optional filter. To get the full traces, pass the trace IDs to
/// <code>BatchGetTraces</code>.</p>
/// <p>A filter expression can target traced requests that hit specific service nodes or
/// edges, have errors, or come from a known user. For example, the following filter expression
/// targets traces that pass through <code>api.example.com</code>:</p>
/// <p>
/// <code>service("api.example.com")</code>
/// </p>
/// <p>This filter expression finds traces that have an annotation named <code>account</code>
/// with the value <code>12345</code>:</p>
/// <p>
/// <code>annotation.account = "12345"</code>
/// </p>
/// <p>For a full list of indexed fields and keywords that you can use in filter expressions,
/// see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html">Using Filter
/// Expressions</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTraceSummaries {
    _private: (),
}
impl GetTraceSummaries {
    /// Creates a new builder-style object to manufacture [`GetTraceSummariesInput`](crate::input::GetTraceSummariesInput)
    pub fn builder() -> crate::input::get_trace_summaries_input::Builder {
        crate::input::get_trace_summaries_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetTraceSummaries {
    type Output = std::result::Result<
        crate::output::GetTraceSummariesOutput,
        crate::error::GetTraceSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_trace_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_trace_summaries_response(response)
        }
    }
}

/// <p>Returns a list of tags that are applied to the specified AWS X-Ray group or sampling rule.</p>
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

/// <p>Updates the encryption configuration for X-Ray data.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutEncryptionConfig {
    _private: (),
}
impl PutEncryptionConfig {
    /// Creates a new builder-style object to manufacture [`PutEncryptionConfigInput`](crate::input::PutEncryptionConfigInput)
    pub fn builder() -> crate::input::put_encryption_config_input::Builder {
        crate::input::put_encryption_config_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutEncryptionConfig {
    type Output = std::result::Result<
        crate::output::PutEncryptionConfigOutput,
        crate::error::PutEncryptionConfigError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_encryption_config_error(response)
        } else {
            crate::operation_deser::parse_put_encryption_config_response(response)
        }
    }
}

/// <p>Used by the AWS X-Ray daemon to upload telemetry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutTelemetryRecords {
    _private: (),
}
impl PutTelemetryRecords {
    /// Creates a new builder-style object to manufacture [`PutTelemetryRecordsInput`](crate::input::PutTelemetryRecordsInput)
    pub fn builder() -> crate::input::put_telemetry_records_input::Builder {
        crate::input::put_telemetry_records_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutTelemetryRecords {
    type Output = std::result::Result<
        crate::output::PutTelemetryRecordsOutput,
        crate::error::PutTelemetryRecordsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_telemetry_records_error(response)
        } else {
            crate::operation_deser::parse_put_telemetry_records_response(response)
        }
    }
}

/// <p>Uploads segment documents to AWS X-Ray. The <a href="https://docs.aws.amazon.com/xray/index.html">X-Ray SDK</a> generates segment documents and sends them to the X-Ray daemon, which uploads them in
/// batches. A segment document can be a completed segment, an in-progress segment, or an array of
/// subsegments.</p>
/// <p>Segments must include the following fields. For the full segment document schema, see
/// <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray
/// Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
/// <p class="title">
/// <b>Required segment document fields</b>
/// </p>
/// <ul>
/// <li>
/// <p>
/// <code>name</code> - The name of the service that handled the request.</p>
/// </li>
/// <li>
/// <p>
/// <code>id</code> - A 64-bit identifier for the segment, unique among segments in the same trace, in 16
/// hexadecimal digits.</p>
/// </li>
/// <li>
/// <p>
/// <code>trace_id</code> - A unique identifier that connects all segments and subsegments originating from
/// a single client request.</p>
/// </li>
/// <li>
/// <p>
/// <code>start_time</code> - Time the segment or subsegment was created, in floating point seconds in
/// epoch time, accurate to milliseconds. For example, <code>1480615200.010</code> or
/// <code>1.480615200010E9</code>.</p>
/// </li>
/// <li>
/// <p>
/// <code>end_time</code> - Time the segment or subsegment was closed. For example,
/// <code>1480615200.090</code> or <code>1.480615200090E9</code>. Specify either an <code>end_time</code> or
/// <code>in_progress</code>.</p>
/// </li>
/// <li>
/// <p>
/// <code>in_progress</code> - Set to <code>true</code> instead of specifying an <code>end_time</code> to
/// record that a segment has been started, but is not complete. Send an in-progress segment when your application
/// receives a request that will take a long time to serve, to trace that the request was received. When the
/// response is sent, send the complete segment to overwrite the in-progress segment.</p>
/// </li>
/// </ul>
/// <p>A <code>trace_id</code> consists of three numbers separated by hyphens. For example,
/// 1-58406520-a006649127e371903a2de979. This includes:</p>
/// <p class="title">
/// <b>Trace ID Format</b>
/// </p>
/// <ul>
/// <li>
/// <p>The version number, for instance, <code>1</code>.</p>
/// </li>
/// <li>
/// <p>The time of the original request, in Unix epoch time, in 8 hexadecimal digits. For
/// example, 10:00AM December 2nd, 2016 PST in epoch time is <code>1480615200</code> seconds,
/// or <code>58406520</code> in hexadecimal.</p>
/// </li>
/// <li>
/// <p>A 96-bit identifier for the trace, globally unique, in 24 hexadecimal
/// digits.</p>
/// </li>
/// </ul>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutTraceSegments {
    _private: (),
}
impl PutTraceSegments {
    /// Creates a new builder-style object to manufacture [`PutTraceSegmentsInput`](crate::input::PutTraceSegmentsInput)
    pub fn builder() -> crate::input::put_trace_segments_input::Builder {
        crate::input::put_trace_segments_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutTraceSegments {
    type Output = std::result::Result<
        crate::output::PutTraceSegmentsOutput,
        crate::error::PutTraceSegmentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_trace_segments_error(response)
        } else {
            crate::operation_deser::parse_put_trace_segments_response(response)
        }
    }
}

/// <p>Applies tags to an existing AWS X-Ray group or sampling rule.</p>
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

/// <p>Removes tags from an AWS X-Ray group or sampling rule. You cannot edit or delete system
/// tags (those with an <code>aws:</code> prefix).</p>
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

/// <p>Updates a group resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateGroup {
    _private: (),
}
impl UpdateGroup {
    /// Creates a new builder-style object to manufacture [`UpdateGroupInput`](crate::input::UpdateGroupInput)
    pub fn builder() -> crate::input::update_group_input::Builder {
        crate::input::update_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateGroup {
    type Output =
        std::result::Result<crate::output::UpdateGroupOutput, crate::error::UpdateGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_group_error(response)
        } else {
            crate::operation_deser::parse_update_group_response(response)
        }
    }
}

/// <p>Modifies a sampling rule's configuration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSamplingRule {
    _private: (),
}
impl UpdateSamplingRule {
    /// Creates a new builder-style object to manufacture [`UpdateSamplingRuleInput`](crate::input::UpdateSamplingRuleInput)
    pub fn builder() -> crate::input::update_sampling_rule_input::Builder {
        crate::input::update_sampling_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateSamplingRule {
    type Output = std::result::Result<
        crate::output::UpdateSamplingRuleOutput,
        crate::error::UpdateSamplingRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_sampling_rule_error(response)
        } else {
            crate::operation_deser::parse_update_sampling_rule_response(response)
        }
    }
}
