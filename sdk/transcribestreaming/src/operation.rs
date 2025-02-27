// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Starts a bidirectional HTTP/2 stream where audio is streamed to Amazon Transcribe Medical and the
/// transcription results are streamed to your application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartMedicalStreamTranscription {
    _private: (),
}
impl StartMedicalStreamTranscription {
    /// Creates a new builder-style object to manufacture [`StartMedicalStreamTranscriptionInput`](crate::input::StartMedicalStreamTranscriptionInput)
    pub fn builder() -> crate::input::start_medical_stream_transcription_input::Builder {
        crate::input::start_medical_stream_transcription_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse for StartMedicalStreamTranscription {
    type Output = std::result::Result<
        crate::output::StartMedicalStreamTranscriptionOutput,
        crate::error::StartMedicalStreamTranscriptionError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_start_medical_stream_transcription(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_start_medical_stream_transcription_error(response)
    }
}

/// <p>Starts a bidirectional HTTP2 stream where audio is streamed to Amazon Transcribe and the transcription
/// results are streamed to your application.</p>
/// <p>The following are encoded as HTTP2 headers:</p>
/// <ul>
/// <li>
/// <p>x-amzn-transcribe-language-code</p>
/// </li>
/// <li>
/// <p>x-amzn-transcribe-media-encoding</p>
/// </li>
/// <li>
/// <p>x-amzn-transcribe-sample-rate</p>
/// </li>
/// <li>
/// <p>x-amzn-transcribe-session-id</p>
/// </li>
/// </ul>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartStreamTranscription {
    _private: (),
}
impl StartStreamTranscription {
    /// Creates a new builder-style object to manufacture [`StartStreamTranscriptionInput`](crate::input::StartStreamTranscriptionInput)
    pub fn builder() -> crate::input::start_stream_transcription_input::Builder {
        crate::input::start_stream_transcription_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse for StartStreamTranscription {
    type Output = std::result::Result<
        crate::output::StartStreamTranscriptionOutput,
        crate::error::StartStreamTranscriptionError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_start_stream_transcription(
            response,
        ))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_start_stream_transcription_error(response)
    }
}
