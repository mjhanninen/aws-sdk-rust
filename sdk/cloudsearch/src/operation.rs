// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Indexes the search suggestions. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html#configuring-suggesters">Configuring Suggesters</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BuildSuggesters {
    _private: (),
}
impl BuildSuggesters {
    /// Creates a new builder-style object to manufacture [`BuildSuggestersInput`](crate::input::BuildSuggestersInput)
    pub fn builder() -> crate::input::build_suggesters_input::Builder {
        crate::input::build_suggesters_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for BuildSuggesters {
    type Output = std::result::Result<
        crate::output::BuildSuggestersOutput,
        crate::error::BuildSuggestersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_build_suggesters_error(response)
        } else {
            crate::operation_deser::parse_build_suggesters_response(response)
        }
    }
}

/// <p>Creates a new search domain. For more information,
/// see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/creating-domains.html" target="_blank">Creating a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDomain {
    _private: (),
}
impl CreateDomain {
    /// Creates a new builder-style object to manufacture [`CreateDomainInput`](crate::input::CreateDomainInput)
    pub fn builder() -> crate::input::create_domain_input::Builder {
        crate::input::create_domain_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateDomain {
    type Output =
        std::result::Result<crate::output::CreateDomainOutput, crate::error::CreateDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_domain_error(response)
        } else {
            crate::operation_deser::parse_create_domain_response(response)
        }
    }
}

/// <p>Configures an analysis scheme that can be applied to a <code>text</code> or <code>text-array</code> field to define language-specific text processing options. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DefineAnalysisScheme {
    _private: (),
}
impl DefineAnalysisScheme {
    /// Creates a new builder-style object to manufacture [`DefineAnalysisSchemeInput`](crate::input::DefineAnalysisSchemeInput)
    pub fn builder() -> crate::input::define_analysis_scheme_input::Builder {
        crate::input::define_analysis_scheme_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DefineAnalysisScheme {
    type Output = std::result::Result<
        crate::output::DefineAnalysisSchemeOutput,
        crate::error::DefineAnalysisSchemeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_define_analysis_scheme_error(response)
        } else {
            crate::operation_deser::parse_define_analysis_scheme_response(response)
        }
    }
}

/// <p>Configures an <code><a>Expression</a></code> for the search domain. Used to create new expressions and modify existing ones.  If the expression exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DefineExpression {
    _private: (),
}
impl DefineExpression {
    /// Creates a new builder-style object to manufacture [`DefineExpressionInput`](crate::input::DefineExpressionInput)
    pub fn builder() -> crate::input::define_expression_input::Builder {
        crate::input::define_expression_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DefineExpression {
    type Output = std::result::Result<
        crate::output::DefineExpressionOutput,
        crate::error::DefineExpressionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_define_expression_error(response)
        } else {
            crate::operation_deser::parse_define_expression_response(response)
        }
    }
}

/// <p>Configures an <code><a>IndexField</a></code> for the search domain. Used to create new fields and modify existing ones. You must specify the name of the domain you are configuring and an index field configuration. The index field configuration specifies a unique name, the index field type, and the options you want to configure for the field. The options you can specify depend on the <code><a>IndexFieldType</a></code>. If the field exists, the new configuration replaces the old one. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DefineIndexField {
    _private: (),
}
impl DefineIndexField {
    /// Creates a new builder-style object to manufacture [`DefineIndexFieldInput`](crate::input::DefineIndexFieldInput)
    pub fn builder() -> crate::input::define_index_field_input::Builder {
        crate::input::define_index_field_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DefineIndexField {
    type Output = std::result::Result<
        crate::output::DefineIndexFieldOutput,
        crate::error::DefineIndexFieldError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_define_index_field_error(response)
        } else {
            crate::operation_deser::parse_define_index_field_response(response)
        }
    }
}

/// <p>Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. When you configure a suggester, you must specify the name of the text field you want to search for possible matches and a unique name for the suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DefineSuggester {
    _private: (),
}
impl DefineSuggester {
    /// Creates a new builder-style object to manufacture [`DefineSuggesterInput`](crate::input::DefineSuggesterInput)
    pub fn builder() -> crate::input::define_suggester_input::Builder {
        crate::input::define_suggester_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DefineSuggester {
    type Output = std::result::Result<
        crate::output::DefineSuggesterOutput,
        crate::error::DefineSuggesterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_define_suggester_error(response)
        } else {
            crate::operation_deser::parse_define_suggester_response(response)
        }
    }
}

/// <p>Deletes an analysis scheme. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAnalysisScheme {
    _private: (),
}
impl DeleteAnalysisScheme {
    /// Creates a new builder-style object to manufacture [`DeleteAnalysisSchemeInput`](crate::input::DeleteAnalysisSchemeInput)
    pub fn builder() -> crate::input::delete_analysis_scheme_input::Builder {
        crate::input::delete_analysis_scheme_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteAnalysisScheme {
    type Output = std::result::Result<
        crate::output::DeleteAnalysisSchemeOutput,
        crate::error::DeleteAnalysisSchemeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_analysis_scheme_error(response)
        } else {
            crate::operation_deser::parse_delete_analysis_scheme_response(response)
        }
    }
}

/// <p>Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. For more information,
/// see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/deleting-domains.html" target="_blank">Deleting a Search  Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDomain {
    _private: (),
}
impl DeleteDomain {
    /// Creates a new builder-style object to manufacture [`DeleteDomainInput`](crate::input::DeleteDomainInput)
    pub fn builder() -> crate::input::delete_domain_input::Builder {
        crate::input::delete_domain_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteDomain {
    type Output =
        std::result::Result<crate::output::DeleteDomainOutput, crate::error::DeleteDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_domain_error(response)
        } else {
            crate::operation_deser::parse_delete_domain_response(response)
        }
    }
}

/// <p>Removes an <code><a>Expression</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteExpression {
    _private: (),
}
impl DeleteExpression {
    /// Creates a new builder-style object to manufacture [`DeleteExpressionInput`](crate::input::DeleteExpressionInput)
    pub fn builder() -> crate::input::delete_expression_input::Builder {
        crate::input::delete_expression_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteExpression {
    type Output = std::result::Result<
        crate::output::DeleteExpressionOutput,
        crate::error::DeleteExpressionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_expression_error(response)
        } else {
            crate::operation_deser::parse_delete_expression_response(response)
        }
    }
}

/// <p>Removes an <code><a>IndexField</a></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteIndexField {
    _private: (),
}
impl DeleteIndexField {
    /// Creates a new builder-style object to manufacture [`DeleteIndexFieldInput`](crate::input::DeleteIndexFieldInput)
    pub fn builder() -> crate::input::delete_index_field_input::Builder {
        crate::input::delete_index_field_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteIndexField {
    type Output = std::result::Result<
        crate::output::DeleteIndexFieldOutput,
        crate::error::DeleteIndexFieldError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_index_field_error(response)
        } else {
            crate::operation_deser::parse_delete_index_field_response(response)
        }
    }
}

/// <p>Deletes a suggester. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSuggester {
    _private: (),
}
impl DeleteSuggester {
    /// Creates a new builder-style object to manufacture [`DeleteSuggesterInput`](crate::input::DeleteSuggesterInput)
    pub fn builder() -> crate::input::delete_suggester_input::Builder {
        crate::input::delete_suggester_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSuggester {
    type Output = std::result::Result<
        crate::output::DeleteSuggesterOutput,
        crate::error::DeleteSuggesterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_suggester_error(response)
        } else {
            crate::operation_deser::parse_delete_suggester_response(response)
        }
    }
}

/// <p>Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a <code>text</code> field. Can be limited to specific analysis schemes by name.  By default, shows all analysis schemes and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes.  For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-analysis-schemes.html" target="_blank">Configuring Analysis Schemes</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAnalysisSchemes {
    _private: (),
}
impl DescribeAnalysisSchemes {
    /// Creates a new builder-style object to manufacture [`DescribeAnalysisSchemesInput`](crate::input::DescribeAnalysisSchemesInput)
    pub fn builder() -> crate::input::describe_analysis_schemes_input::Builder {
        crate::input::describe_analysis_schemes_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAnalysisSchemes {
    type Output = std::result::Result<
        crate::output::DescribeAnalysisSchemesOutput,
        crate::error::DescribeAnalysisSchemesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_analysis_schemes_error(response)
        } else {
            crate::operation_deser::parse_describe_analysis_schemes_response(response)
        }
    }
}

/// <p>Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see  <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAvailabilityOptions {
    _private: (),
}
impl DescribeAvailabilityOptions {
    /// Creates a new builder-style object to manufacture [`DescribeAvailabilityOptionsInput`](crate::input::DescribeAvailabilityOptionsInput)
    pub fn builder() -> crate::input::describe_availability_options_input::Builder {
        crate::input::describe_availability_options_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAvailabilityOptions {
    type Output = std::result::Result<
        crate::output::DescribeAvailabilityOptionsOutput,
        crate::error::DescribeAvailabilityOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_availability_options_error(response)
        } else {
            crate::operation_deser::parse_describe_availability_options_response(response)
        }
    }
}

/// <p>Returns the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see  <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-domain-endpoint-options.html" target="_blank">Configuring Domain Endpoint Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDomainEndpointOptions {
    _private: (),
}
impl DescribeDomainEndpointOptions {
    /// Creates a new builder-style object to manufacture [`DescribeDomainEndpointOptionsInput`](crate::input::DescribeDomainEndpointOptionsInput)
    pub fn builder() -> crate::input::describe_domain_endpoint_options_input::Builder {
        crate::input::describe_domain_endpoint_options_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeDomainEndpointOptions {
    type Output = std::result::Result<
        crate::output::DescribeDomainEndpointOptionsOutput,
        crate::error::DescribeDomainEndpointOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_domain_endpoint_options_error(response)
        } else {
            crate::operation_deser::parse_describe_domain_endpoint_options_response(response)
        }
    }
}

/// <p>Gets information about the search domains owned by this account. Can be limited to specific domains. Shows
/// all domains by default. To get the number of searchable documents in a domain, use the console or submit a <code>matchall</code> request to your domain's search endpoint: <code>q=matchall&amp;q.parser=structured&amp;size=0</code>. For more information,
/// see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Information about a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDomains {
    _private: (),
}
impl DescribeDomains {
    /// Creates a new builder-style object to manufacture [`DescribeDomainsInput`](crate::input::DescribeDomainsInput)
    pub fn builder() -> crate::input::describe_domains_input::Builder {
        crate::input::describe_domains_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeDomains {
    type Output = std::result::Result<
        crate::output::DescribeDomainsOutput,
        crate::error::DescribeDomainsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_domains_error(response)
        } else {
            crate::operation_deser::parse_describe_domains_response(response)
        }
    }
}

/// <p>Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see  <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeExpressions {
    _private: (),
}
impl DescribeExpressions {
    /// Creates a new builder-style object to manufacture [`DescribeExpressionsInput`](crate::input::DescribeExpressionsInput)
    pub fn builder() -> crate::input::describe_expressions_input::Builder {
        crate::input::describe_expressions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeExpressions {
    type Output = std::result::Result<
        crate::output::DescribeExpressionsOutput,
        crate::error::DescribeExpressionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_expressions_error(response)
        } else {
            crate::operation_deser::parse_describe_expressions_response(response)
        }
    }
}

/// <p>Gets information about the index fields configured for the search domain.
/// Can be limited to specific fields by name.  By default, shows all fields and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information,
/// see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Domain Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeIndexFields {
    _private: (),
}
impl DescribeIndexFields {
    /// Creates a new builder-style object to manufacture [`DescribeIndexFieldsInput`](crate::input::DescribeIndexFieldsInput)
    pub fn builder() -> crate::input::describe_index_fields_input::Builder {
        crate::input::describe_index_fields_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeIndexFields {
    type Output = std::result::Result<
        crate::output::DescribeIndexFieldsOutput,
        crate::error::DescribeIndexFieldsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_index_fields_error(response)
        } else {
            crate::operation_deser::parse_describe_index_fields_response(response)
        }
    }
}

/// <p>Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. For more information, see   <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeScalingParameters {
    _private: (),
}
impl DescribeScalingParameters {
    /// Creates a new builder-style object to manufacture [`DescribeScalingParametersInput`](crate::input::DescribeScalingParametersInput)
    pub fn builder() -> crate::input::describe_scaling_parameters_input::Builder {
        crate::input::describe_scaling_parameters_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeScalingParameters {
    type Output = std::result::Result<
        crate::output::DescribeScalingParametersOutput,
        crate::error::DescribeScalingParametersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_scaling_parameters_error(response)
        } else {
            crate::operation_deser::parse_describe_scaling_parameters_response(response)
        }
    }
}

/// <p>Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information,
/// see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">Configuring Access for a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeServiceAccessPolicies {
    _private: (),
}
impl DescribeServiceAccessPolicies {
    /// Creates a new builder-style object to manufacture [`DescribeServiceAccessPoliciesInput`](crate::input::DescribeServiceAccessPoliciesInput)
    pub fn builder() -> crate::input::describe_service_access_policies_input::Builder {
        crate::input::describe_service_access_policies_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeServiceAccessPolicies {
    type Output = std::result::Result<
        crate::output::DescribeServiceAccessPoliciesOutput,
        crate::error::DescribeServiceAccessPoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_service_access_policies_error(response)
        } else {
            crate::operation_deser::parse_describe_service_access_policies_response(response)
        }
    }
}

/// <p>Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries.  Can be limited to specific suggesters by name.  By default, shows all suggesters and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes.  For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html" target="_blank">Getting Search Suggestions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSuggesters {
    _private: (),
}
impl DescribeSuggesters {
    /// Creates a new builder-style object to manufacture [`DescribeSuggestersInput`](crate::input::DescribeSuggestersInput)
    pub fn builder() -> crate::input::describe_suggesters_input::Builder {
        crate::input::describe_suggesters_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeSuggesters {
    type Output = std::result::Result<
        crate::output::DescribeSuggestersOutput,
        crate::error::DescribeSuggestersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_suggesters_error(response)
        } else {
            crate::operation_deser::parse_describe_suggesters_response(response)
        }
    }
}

/// <p>Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose <a>OptionStatus</a> is  <code>RequiresIndexDocuments</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct IndexDocuments {
    _private: (),
}
impl IndexDocuments {
    /// Creates a new builder-style object to manufacture [`IndexDocumentsInput`](crate::input::IndexDocumentsInput)
    pub fn builder() -> crate::input::index_documents_input::Builder {
        crate::input::index_documents_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for IndexDocuments {
    type Output =
        std::result::Result<crate::output::IndexDocumentsOutput, crate::error::IndexDocumentsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_index_documents_error(response)
        } else {
            crate::operation_deser::parse_index_documents_response(response)
        }
    }
}

/// <p>Lists all search domains owned by an account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDomainNames {
    _private: (),
}
impl ListDomainNames {
    /// Creates a new builder-style object to manufacture [`ListDomainNamesInput`](crate::input::ListDomainNamesInput)
    pub fn builder() -> crate::input::list_domain_names_input::Builder {
        crate::input::list_domain_names_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListDomainNames {
    type Output = std::result::Result<
        crate::output::ListDomainNamesOutput,
        crate::error::ListDomainNamesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_domain_names_error(response)
        } else {
            crate::operation_deser::parse_list_domain_names_response(response)
        }
    }
}

/// <p>Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a service disruption. Changes to the Multi-AZ option can take about half an hour to become active. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-availability-options.html" target="_blank">Configuring Availability Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAvailabilityOptions {
    _private: (),
}
impl UpdateAvailabilityOptions {
    /// Creates a new builder-style object to manufacture [`UpdateAvailabilityOptionsInput`](crate::input::UpdateAvailabilityOptionsInput)
    pub fn builder() -> crate::input::update_availability_options_input::Builder {
        crate::input::update_availability_options_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateAvailabilityOptions {
    type Output = std::result::Result<
        crate::output::UpdateAvailabilityOptionsOutput,
        crate::error::UpdateAvailabilityOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_availability_options_error(response)
        } else {
            crate::operation_deser::parse_update_availability_options_response(response)
        }
    }
}

/// <p>Updates the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-domain-endpoint-options.html" target="_blank">Configuring Domain Endpoint Options</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDomainEndpointOptions {
    _private: (),
}
impl UpdateDomainEndpointOptions {
    /// Creates a new builder-style object to manufacture [`UpdateDomainEndpointOptionsInput`](crate::input::UpdateDomainEndpointOptionsInput)
    pub fn builder() -> crate::input::update_domain_endpoint_options_input::Builder {
        crate::input::update_domain_endpoint_options_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateDomainEndpointOptions {
    type Output = std::result::Result<
        crate::output::UpdateDomainEndpointOptionsOutput,
        crate::error::UpdateDomainEndpointOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_domain_endpoint_options_error(response)
        } else {
            crate::operation_deser::parse_update_domain_endpoint_options_response(response)
        }
    }
}

/// <p>Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. Amazon CloudSearch will still automatically scale your domain based on the volume of data and traffic, but not below the desired instance type and replication count. If the Multi-AZ option is enabled, these values control the resources used per Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-scaling-options.html" target="_blank">Configuring Scaling Options</a> in the <i>Amazon CloudSearch Developer Guide</i>. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateScalingParameters {
    _private: (),
}
impl UpdateScalingParameters {
    /// Creates a new builder-style object to manufacture [`UpdateScalingParametersInput`](crate::input::UpdateScalingParametersInput)
    pub fn builder() -> crate::input::update_scaling_parameters_input::Builder {
        crate::input::update_scaling_parameters_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateScalingParameters {
    type Output = std::result::Result<
        crate::output::UpdateScalingParametersOutput,
        crate::error::UpdateScalingParametersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_scaling_parameters_error(response)
        } else {
            crate::operation_deser::parse_update_scaling_parameters_response(response)
        }
    }
}

/// <p>Configures the access rules that control access to the domain's document and search endpoints.
/// For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">
/// Configuring Access for an Amazon CloudSearch Domain</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateServiceAccessPolicies {
    _private: (),
}
impl UpdateServiceAccessPolicies {
    /// Creates a new builder-style object to manufacture [`UpdateServiceAccessPoliciesInput`](crate::input::UpdateServiceAccessPoliciesInput)
    pub fn builder() -> crate::input::update_service_access_policies_input::Builder {
        crate::input::update_service_access_policies_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateServiceAccessPolicies {
    type Output = std::result::Result<
        crate::output::UpdateServiceAccessPoliciesOutput,
        crate::error::UpdateServiceAccessPoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_service_access_policies_error(response)
        } else {
            crate::operation_deser::parse_update_service_access_policies_response(response)
        }
    }
}
