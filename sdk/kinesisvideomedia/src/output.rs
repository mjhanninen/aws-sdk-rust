// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
pub struct GetMediaOutput {
    /// <p>The content type of the requested media.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified
    /// stream. For information about the chunks, see . The
    /// chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the
    /// following additional Matroska (MKV) tags: </p>
    /// <ul>
    /// <li>
    /// <p>AWS_KINESISVIDEO_CONTINUATION_TOKEN (UTF-8 string) - In the event your
    /// <code>GetMedia</code> call terminates, you can use this continuation token in your next
    /// request to get the next chunk where the last request terminated.</p>
    /// </li>
    /// <li>
    /// <p>AWS_KINESISVIDEO_MILLIS_BEHIND_NOW (UTF-8 string) - Client applications can use
    /// this tag value to determine how far behind the chunk returned in the response is from the
    /// latest chunk on the stream. </p>
    /// </li>
    /// <li>
    /// <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p>
    /// </li>
    /// <li>
    /// <p>AWS_KINESISVIDEO_SERVER_TIMESTAMP - Server timestamp of the fragment.</p>
    /// </li>
    /// <li>
    /// <p>AWS_KINESISVIDEO_PRODUCER_TIMESTAMP - Producer timestamp of the fragment.</p>
    /// </li>
    /// </ul>
    /// <p>The following tags will be present if an error occurs:</p>
    /// <ul>
    /// <li>
    /// <p>AWS_KINESISVIDEO_ERROR_CODE - String description of an error that caused GetMedia
    /// to stop.</p>
    /// </li>
    /// <li>
    /// <p>AWS_KINESISVIDEO_ERROR_ID: Integer code of the error.</p>
    /// </li>
    /// </ul>
    /// <p>The error codes are as follows:</p>
    /// <ul>
    /// <li>
    /// <p>3002 - Error writing to the stream</p>
    /// </li>
    /// <li>
    /// <p>4000 - Requested fragment is not found</p>
    /// </li>
    /// <li>
    /// <p>4500 - Access denied for the stream's KMS key</p>
    /// </li>
    /// <li>
    /// <p>4501 - Stream's KMS key is disabled</p>
    /// </li>
    /// <li>
    /// <p>4502 - Validation error on the stream's KMS key</p>
    /// </li>
    /// <li>
    /// <p>4503 - KMS key specified in the stream is unavailable</p>
    /// </li>
    /// <li>
    /// <p>4504 - Invalid usage of the KMS key specified in the stream</p>
    /// </li>
    /// <li>
    /// <p>4505 - Invalid state of the KMS key specified in the stream</p>
    /// </li>
    /// <li>
    /// <p>4506 - Unable to find the KMS key specified in the stream</p>
    /// </li>
    /// <li>
    /// <p>5000 - Internal error</p>
    /// </li>
    /// </ul>
    pub payload: smithy_http::byte_stream::ByteStream,
}
impl std::fmt::Debug for GetMediaOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetMediaOutput");
        formatter.field("content_type", &self.content_type);
        formatter.field("payload", &self.payload);
        formatter.finish()
    }
}
/// See [`GetMediaOutput`](crate::output::GetMediaOutput)
pub mod get_media_output {
    /// A builder for [`GetMediaOutput`](crate::output::GetMediaOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) payload: std::option::Option<smithy_http::byte_stream::ByteStream>,
    }
    impl Builder {
        /// <p>The content type of the requested media.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified
        /// stream. For information about the chunks, see . The
        /// chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the
        /// following additional Matroska (MKV) tags: </p>
        /// <ul>
        /// <li>
        /// <p>AWS_KINESISVIDEO_CONTINUATION_TOKEN (UTF-8 string) - In the event your
        /// <code>GetMedia</code> call terminates, you can use this continuation token in your next
        /// request to get the next chunk where the last request terminated.</p>
        /// </li>
        /// <li>
        /// <p>AWS_KINESISVIDEO_MILLIS_BEHIND_NOW (UTF-8 string) - Client applications can use
        /// this tag value to determine how far behind the chunk returned in the response is from the
        /// latest chunk on the stream. </p>
        /// </li>
        /// <li>
        /// <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p>
        /// </li>
        /// <li>
        /// <p>AWS_KINESISVIDEO_SERVER_TIMESTAMP - Server timestamp of the fragment.</p>
        /// </li>
        /// <li>
        /// <p>AWS_KINESISVIDEO_PRODUCER_TIMESTAMP - Producer timestamp of the fragment.</p>
        /// </li>
        /// </ul>
        /// <p>The following tags will be present if an error occurs:</p>
        /// <ul>
        /// <li>
        /// <p>AWS_KINESISVIDEO_ERROR_CODE - String description of an error that caused GetMedia
        /// to stop.</p>
        /// </li>
        /// <li>
        /// <p>AWS_KINESISVIDEO_ERROR_ID: Integer code of the error.</p>
        /// </li>
        /// </ul>
        /// <p>The error codes are as follows:</p>
        /// <ul>
        /// <li>
        /// <p>3002 - Error writing to the stream</p>
        /// </li>
        /// <li>
        /// <p>4000 - Requested fragment is not found</p>
        /// </li>
        /// <li>
        /// <p>4500 - Access denied for the stream's KMS key</p>
        /// </li>
        /// <li>
        /// <p>4501 - Stream's KMS key is disabled</p>
        /// </li>
        /// <li>
        /// <p>4502 - Validation error on the stream's KMS key</p>
        /// </li>
        /// <li>
        /// <p>4503 - KMS key specified in the stream is unavailable</p>
        /// </li>
        /// <li>
        /// <p>4504 - Invalid usage of the KMS key specified in the stream</p>
        /// </li>
        /// <li>
        /// <p>4505 - Invalid state of the KMS key specified in the stream</p>
        /// </li>
        /// <li>
        /// <p>4506 - Unable to find the KMS key specified in the stream</p>
        /// </li>
        /// <li>
        /// <p>5000 - Internal error</p>
        /// </li>
        /// </ul>
        pub fn payload(mut self, input: smithy_http::byte_stream::ByteStream) -> Self {
            self.payload = Some(input);
            self
        }
        pub fn set_payload(
            mut self,
            input: std::option::Option<smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.payload = input;
            self
        }
        /// Consumes the builder and constructs a [`GetMediaOutput`](crate::output::GetMediaOutput)
        pub fn build(self) -> crate::output::GetMediaOutput {
            crate::output::GetMediaOutput {
                content_type: self.content_type,
                payload: self.payload.unwrap_or_default(),
            }
        }
    }
}
impl GetMediaOutput {
    /// Creates a new builder-style object to manufacture [`GetMediaOutput`](crate::output::GetMediaOutput)
    pub fn builder() -> crate::output::get_media_output::Builder {
        crate::output::get_media_output::Builder::default()
    }
}
