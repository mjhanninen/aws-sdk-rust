// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Gets the properties associated with a medical entities detection job. Use this operation
/// to get the status of a detection job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEntitiesDetectionV2Job {
    _private: (),
}
impl DescribeEntitiesDetectionV2Job {
    /// Creates a new builder-style object to manufacture [`DescribeEntitiesDetectionV2JobInput`](crate::input::DescribeEntitiesDetectionV2JobInput)
    pub fn builder() -> crate::input::describe_entities_detection_v2_job_input::Builder {
        crate::input::describe_entities_detection_v2_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEntitiesDetectionV2Job {
    type Output = std::result::Result<
        crate::output::DescribeEntitiesDetectionV2JobOutput,
        crate::error::DescribeEntitiesDetectionV2JobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_entities_detection_v2_job_error(response)
        } else {
            crate::operation_deser::parse_describe_entities_detection_v2_job_response(response)
        }
    }
}

/// <p>Gets the properties associated with an InferICD10CM job. Use this operation to get the
/// status of an inference job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeICD10CMInferenceJob {
    _private: (),
}
impl DescribeICD10CMInferenceJob {
    /// Creates a new builder-style object to manufacture [`DescribeIcd10CmInferenceJobInput`](crate::input::DescribeIcd10CmInferenceJobInput)
    pub fn builder() -> crate::input::describe_icd10_cm_inference_job_input::Builder {
        crate::input::describe_icd10_cm_inference_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeICD10CMInferenceJob {
    type Output = std::result::Result<
        crate::output::DescribeIcd10CmInferenceJobOutput,
        crate::error::DescribeICD10CMInferenceJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_icd10_cm_inference_job_error(response)
        } else {
            crate::operation_deser::parse_describe_icd10_cm_inference_job_response(response)
        }
    }
}

/// <p>Gets the properties associated with a protected health information (PHI) detection job.
/// Use this operation to get the status of a detection job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribePHIDetectionJob {
    _private: (),
}
impl DescribePHIDetectionJob {
    /// Creates a new builder-style object to manufacture [`DescribePhiDetectionJobInput`](crate::input::DescribePhiDetectionJobInput)
    pub fn builder() -> crate::input::describe_phi_detection_job_input::Builder {
        crate::input::describe_phi_detection_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribePHIDetectionJob {
    type Output = std::result::Result<
        crate::output::DescribePhiDetectionJobOutput,
        crate::error::DescribePHIDetectionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_phi_detection_job_error(response)
        } else {
            crate::operation_deser::parse_describe_phi_detection_job_response(response)
        }
    }
}

/// <p>Gets the properties associated with an InferRxNorm job. Use this operation to get the
/// status of an inference job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRxNormInferenceJob {
    _private: (),
}
impl DescribeRxNormInferenceJob {
    /// Creates a new builder-style object to manufacture [`DescribeRxNormInferenceJobInput`](crate::input::DescribeRxNormInferenceJobInput)
    pub fn builder() -> crate::input::describe_rx_norm_inference_job_input::Builder {
        crate::input::describe_rx_norm_inference_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeRxNormInferenceJob {
    type Output = std::result::Result<
        crate::output::DescribeRxNormInferenceJobOutput,
        crate::error::DescribeRxNormInferenceJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_rx_norm_inference_job_error(response)
        } else {
            crate::operation_deser::parse_describe_rx_norm_inference_job_response(response)
        }
    }
}

/// <p>The <code>DetectEntities</code> operation is deprecated. You should use the <a>DetectEntitiesV2</a> operation instead.</p>
/// <p> Inspects the clinical text for a variety of medical entities and returns specific
/// information about them such as entity category, location, and confidence score on that
/// information .</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DetectEntities {
    _private: (),
}
impl DetectEntities {
    /// Creates a new builder-style object to manufacture [`DetectEntitiesInput`](crate::input::DetectEntitiesInput)
    pub fn builder() -> crate::input::detect_entities_input::Builder {
        crate::input::detect_entities_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DetectEntities {
    type Output =
        std::result::Result<crate::output::DetectEntitiesOutput, crate::error::DetectEntitiesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_detect_entities_error(response)
        } else {
            crate::operation_deser::parse_detect_entities_response(response)
        }
    }
}

/// <p>Inspects the clinical text for a variety of medical entities and returns specific
/// information about them such as entity category, location, and confidence score on that
/// information. Amazon Comprehend Medical only detects medical entities in English language
/// texts.</p>
/// <p>The <code>DetectEntitiesV2</code> operation replaces the <a>DetectEntities</a>
/// operation. This new action uses a different model for determining the entities in your medical
/// text and changes the way that some entities are returned in the output. You should use the
/// <code>DetectEntitiesV2</code> operation in all new applications.</p>
/// <p>The <code>DetectEntitiesV2</code> operation returns the <code>Acuity</code> and
/// <code>Direction</code> entities as attributes instead of types. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DetectEntitiesV2 {
    _private: (),
}
impl DetectEntitiesV2 {
    /// Creates a new builder-style object to manufacture [`DetectEntitiesV2Input`](crate::input::DetectEntitiesV2Input)
    pub fn builder() -> crate::input::detect_entities_v2_input::Builder {
        crate::input::detect_entities_v2_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DetectEntitiesV2 {
    type Output = std::result::Result<
        crate::output::DetectEntitiesV2Output,
        crate::error::DetectEntitiesV2Error,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_detect_entities_v2_error(response)
        } else {
            crate::operation_deser::parse_detect_entities_v2_response(response)
        }
    }
}

/// <p> Inspects the clinical text for protected health information (PHI) entities and returns
/// the entity category, location, and confidence score for each entity. Amazon Comprehend Medical
/// only detects entities in English language texts.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DetectPHI {
    _private: (),
}
impl DetectPHI {
    /// Creates a new builder-style object to manufacture [`DetectPhiInput`](crate::input::DetectPhiInput)
    pub fn builder() -> crate::input::detect_phi_input::Builder {
        crate::input::detect_phi_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DetectPHI {
    type Output = std::result::Result<crate::output::DetectPhiOutput, crate::error::DetectPHIError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_detect_phi_error(response)
        } else {
            crate::operation_deser::parse_detect_phi_response(response)
        }
    }
}

/// <p>InferICD10CM detects medical conditions as entities listed in a patient record and links
/// those entities to normalized concept identifiers in the ICD-10-CM knowledge base from the
/// Centers for Disease Control. Amazon Comprehend Medical only detects medical entities in
/// English language texts.  </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct InferICD10CM {
    _private: (),
}
impl InferICD10CM {
    /// Creates a new builder-style object to manufacture [`InferIcd10CmInput`](crate::input::InferIcd10CmInput)
    pub fn builder() -> crate::input::infer_icd10_cm_input::Builder {
        crate::input::infer_icd10_cm_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for InferICD10CM {
    type Output =
        std::result::Result<crate::output::InferIcd10CmOutput, crate::error::InferICD10CMError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_infer_icd10_cm_error(response)
        } else {
            crate::operation_deser::parse_infer_icd10_cm_response(response)
        }
    }
}

/// <p>InferRxNorm detects medications as entities listed in a patient record and links to the
/// normalized concept identifiers in the RxNorm database from the National Library of Medicine.
/// Amazon Comprehend Medical only detects medical entities in English language texts.  </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct InferRxNorm {
    _private: (),
}
impl InferRxNorm {
    /// Creates a new builder-style object to manufacture [`InferRxNormInput`](crate::input::InferRxNormInput)
    pub fn builder() -> crate::input::infer_rx_norm_input::Builder {
        crate::input::infer_rx_norm_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for InferRxNorm {
    type Output =
        std::result::Result<crate::output::InferRxNormOutput, crate::error::InferRxNormError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_infer_rx_norm_error(response)
        } else {
            crate::operation_deser::parse_infer_rx_norm_response(response)
        }
    }
}

/// <p>Gets a list of medical entity detection jobs that you have submitted.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEntitiesDetectionV2Jobs {
    _private: (),
}
impl ListEntitiesDetectionV2Jobs {
    /// Creates a new builder-style object to manufacture [`ListEntitiesDetectionV2JobsInput`](crate::input::ListEntitiesDetectionV2JobsInput)
    pub fn builder() -> crate::input::list_entities_detection_v2_jobs_input::Builder {
        crate::input::list_entities_detection_v2_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListEntitiesDetectionV2Jobs {
    type Output = std::result::Result<
        crate::output::ListEntitiesDetectionV2JobsOutput,
        crate::error::ListEntitiesDetectionV2JobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_entities_detection_v2_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_entities_detection_v2_jobs_response(response)
        }
    }
}

/// <p>Gets a list of InferICD10CM jobs that you have submitted.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListICD10CMInferenceJobs {
    _private: (),
}
impl ListICD10CMInferenceJobs {
    /// Creates a new builder-style object to manufacture [`ListIcd10CmInferenceJobsInput`](crate::input::ListIcd10CmInferenceJobsInput)
    pub fn builder() -> crate::input::list_icd10_cm_inference_jobs_input::Builder {
        crate::input::list_icd10_cm_inference_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListICD10CMInferenceJobs {
    type Output = std::result::Result<
        crate::output::ListIcd10CmInferenceJobsOutput,
        crate::error::ListICD10CMInferenceJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_icd10_cm_inference_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_icd10_cm_inference_jobs_response(response)
        }
    }
}

/// <p>Gets a list of protected health information (PHI) detection jobs that you have
/// submitted.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPHIDetectionJobs {
    _private: (),
}
impl ListPHIDetectionJobs {
    /// Creates a new builder-style object to manufacture [`ListPhiDetectionJobsInput`](crate::input::ListPhiDetectionJobsInput)
    pub fn builder() -> crate::input::list_phi_detection_jobs_input::Builder {
        crate::input::list_phi_detection_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPHIDetectionJobs {
    type Output = std::result::Result<
        crate::output::ListPhiDetectionJobsOutput,
        crate::error::ListPHIDetectionJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_phi_detection_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_phi_detection_jobs_response(response)
        }
    }
}

/// <p>Gets a list of InferRxNorm jobs that you have submitted.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRxNormInferenceJobs {
    _private: (),
}
impl ListRxNormInferenceJobs {
    /// Creates a new builder-style object to manufacture [`ListRxNormInferenceJobsInput`](crate::input::ListRxNormInferenceJobsInput)
    pub fn builder() -> crate::input::list_rx_norm_inference_jobs_input::Builder {
        crate::input::list_rx_norm_inference_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListRxNormInferenceJobs {
    type Output = std::result::Result<
        crate::output::ListRxNormInferenceJobsOutput,
        crate::error::ListRxNormInferenceJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_rx_norm_inference_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_rx_norm_inference_jobs_response(response)
        }
    }
}

/// <p>Starts an asynchronous medical entity detection job for a collection of documents. Use the
/// <code>DescribeEntitiesDetectionV2Job</code> operation to track the status of a job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartEntitiesDetectionV2Job {
    _private: (),
}
impl StartEntitiesDetectionV2Job {
    /// Creates a new builder-style object to manufacture [`StartEntitiesDetectionV2JobInput`](crate::input::StartEntitiesDetectionV2JobInput)
    pub fn builder() -> crate::input::start_entities_detection_v2_job_input::Builder {
        crate::input::start_entities_detection_v2_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartEntitiesDetectionV2Job {
    type Output = std::result::Result<
        crate::output::StartEntitiesDetectionV2JobOutput,
        crate::error::StartEntitiesDetectionV2JobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_entities_detection_v2_job_error(response)
        } else {
            crate::operation_deser::parse_start_entities_detection_v2_job_response(response)
        }
    }
}

/// <p>Starts an asynchronous job to detect medical conditions and link them to the ICD-10-CM
/// ontology. Use the <code>DescribeICD10CMInferenceJob</code> operation to track the status of a
/// job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartICD10CMInferenceJob {
    _private: (),
}
impl StartICD10CMInferenceJob {
    /// Creates a new builder-style object to manufacture [`StartIcd10CmInferenceJobInput`](crate::input::StartIcd10CmInferenceJobInput)
    pub fn builder() -> crate::input::start_icd10_cm_inference_job_input::Builder {
        crate::input::start_icd10_cm_inference_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartICD10CMInferenceJob {
    type Output = std::result::Result<
        crate::output::StartIcd10CmInferenceJobOutput,
        crate::error::StartICD10CMInferenceJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_icd10_cm_inference_job_error(response)
        } else {
            crate::operation_deser::parse_start_icd10_cm_inference_job_response(response)
        }
    }
}

/// <p>Starts an asynchronous job to detect protected health information (PHI). Use the
/// <code>DescribePHIDetectionJob</code> operation to track the status of a job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartPHIDetectionJob {
    _private: (),
}
impl StartPHIDetectionJob {
    /// Creates a new builder-style object to manufacture [`StartPhiDetectionJobInput`](crate::input::StartPhiDetectionJobInput)
    pub fn builder() -> crate::input::start_phi_detection_job_input::Builder {
        crate::input::start_phi_detection_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartPHIDetectionJob {
    type Output = std::result::Result<
        crate::output::StartPhiDetectionJobOutput,
        crate::error::StartPHIDetectionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_phi_detection_job_error(response)
        } else {
            crate::operation_deser::parse_start_phi_detection_job_response(response)
        }
    }
}

/// <p>Starts an asynchronous job to detect medication entities and link them to the RxNorm
/// ontology. Use the <code>DescribeRxNormInferenceJob</code> operation to track the status of a
/// job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartRxNormInferenceJob {
    _private: (),
}
impl StartRxNormInferenceJob {
    /// Creates a new builder-style object to manufacture [`StartRxNormInferenceJobInput`](crate::input::StartRxNormInferenceJobInput)
    pub fn builder() -> crate::input::start_rx_norm_inference_job_input::Builder {
        crate::input::start_rx_norm_inference_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartRxNormInferenceJob {
    type Output = std::result::Result<
        crate::output::StartRxNormInferenceJobOutput,
        crate::error::StartRxNormInferenceJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_rx_norm_inference_job_error(response)
        } else {
            crate::operation_deser::parse_start_rx_norm_inference_job_response(response)
        }
    }
}

/// <p>Stops a medical entities detection job in progress.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopEntitiesDetectionV2Job {
    _private: (),
}
impl StopEntitiesDetectionV2Job {
    /// Creates a new builder-style object to manufacture [`StopEntitiesDetectionV2JobInput`](crate::input::StopEntitiesDetectionV2JobInput)
    pub fn builder() -> crate::input::stop_entities_detection_v2_job_input::Builder {
        crate::input::stop_entities_detection_v2_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopEntitiesDetectionV2Job {
    type Output = std::result::Result<
        crate::output::StopEntitiesDetectionV2JobOutput,
        crate::error::StopEntitiesDetectionV2JobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_entities_detection_v2_job_error(response)
        } else {
            crate::operation_deser::parse_stop_entities_detection_v2_job_response(response)
        }
    }
}

/// <p>Stops an InferICD10CM inference job in progress.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopICD10CMInferenceJob {
    _private: (),
}
impl StopICD10CMInferenceJob {
    /// Creates a new builder-style object to manufacture [`StopIcd10CmInferenceJobInput`](crate::input::StopIcd10CmInferenceJobInput)
    pub fn builder() -> crate::input::stop_icd10_cm_inference_job_input::Builder {
        crate::input::stop_icd10_cm_inference_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopICD10CMInferenceJob {
    type Output = std::result::Result<
        crate::output::StopIcd10CmInferenceJobOutput,
        crate::error::StopICD10CMInferenceJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_icd10_cm_inference_job_error(response)
        } else {
            crate::operation_deser::parse_stop_icd10_cm_inference_job_response(response)
        }
    }
}

/// <p>Stops a protected health information (PHI) detection job in progress.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopPHIDetectionJob {
    _private: (),
}
impl StopPHIDetectionJob {
    /// Creates a new builder-style object to manufacture [`StopPhiDetectionJobInput`](crate::input::StopPhiDetectionJobInput)
    pub fn builder() -> crate::input::stop_phi_detection_job_input::Builder {
        crate::input::stop_phi_detection_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopPHIDetectionJob {
    type Output = std::result::Result<
        crate::output::StopPhiDetectionJobOutput,
        crate::error::StopPHIDetectionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_phi_detection_job_error(response)
        } else {
            crate::operation_deser::parse_stop_phi_detection_job_response(response)
        }
    }
}

/// <p>Stops an InferRxNorm inference job in progress.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopRxNormInferenceJob {
    _private: (),
}
impl StopRxNormInferenceJob {
    /// Creates a new builder-style object to manufacture [`StopRxNormInferenceJobInput`](crate::input::StopRxNormInferenceJobInput)
    pub fn builder() -> crate::input::stop_rx_norm_inference_job_input::Builder {
        crate::input::stop_rx_norm_inference_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopRxNormInferenceJob {
    type Output = std::result::Result<
        crate::output::StopRxNormInferenceJobOutput,
        crate::error::StopRxNormInferenceJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_rx_norm_inference_job_error(response)
        } else {
            crate::operation_deser::parse_stop_rx_norm_inference_job_response(response)
        }
    }
}
