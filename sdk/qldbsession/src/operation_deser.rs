// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_command_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::SendCommandOutput, crate::error::SendCommandError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::SendCommandError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::SendCommandError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::SendCommandError {
            meta: generic,
            kind: crate::error::SendCommandErrorKind::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_bad_request_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::SendCommandError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "CapacityExceededException" => crate::error::SendCommandError {
            meta: generic,
            kind: crate::error::SendCommandErrorKind::CapacityExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::capacity_exceeded_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_capacity_exceeded_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::SendCommandError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidSessionException" => crate::error::SendCommandError {
            meta: generic,
            kind: crate::error::SendCommandErrorKind::InvalidSessionException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_session_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_invalid_session_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::SendCommandError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "LimitExceededException" => crate::error::SendCommandError {
            meta: generic,
            kind: crate::error::SendCommandErrorKind::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_limit_exceeded_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::SendCommandError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "OccConflictException" => crate::error::SendCommandError {
            meta: generic,
            kind: crate::error::SendCommandErrorKind::OccConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::occ_conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_occ_conflict_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::SendCommandError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "RateExceededException" => crate::error::SendCommandError {
            meta: generic,
            kind: crate::error::SendCommandErrorKind::RateExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::rate_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_rate_exceeded_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::SendCommandError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::SendCommandError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_command_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::SendCommandOutput, crate::error::SendCommandError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::send_command_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_send_command(response.body().as_ref(), output)
            .map_err(crate::error::SendCommandError::unhandled)?;
        output.build()
    })
}
