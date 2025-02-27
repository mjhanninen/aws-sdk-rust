// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_object_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteObjectOutput, crate::error::DeleteObjectError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DeleteObjectError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DeleteObjectError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContainerNotFoundException" => {
            crate::error::DeleteObjectError {
                meta: generic,
                kind: crate::error::DeleteObjectErrorKind::ContainerNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::container_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_container_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DeleteObjectError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerError" => crate::error::DeleteObjectError {
            meta: generic,
            kind: crate::error::DeleteObjectErrorKind::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_errorjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ObjectNotFoundException" => crate::error::DeleteObjectError {
            meta: generic,
            kind: crate::error::DeleteObjectErrorKind::ObjectNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_object_not_found_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DeleteObjectError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_object_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteObjectOutput, crate::error::DeleteObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_object_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_object_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeObjectOutput, crate::error::DescribeObjectError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeObjectError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeObjectError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContainerNotFoundException" => {
            crate::error::DescribeObjectError {
                meta: generic,
                kind: crate::error::DescribeObjectErrorKind::ContainerNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::container_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_container_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeObjectError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerError" => crate::error::DescribeObjectError {
            meta: generic,
            kind: crate::error::DescribeObjectErrorKind::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_errorjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ObjectNotFoundException" => crate::error::DescribeObjectError {
            meta: generic,
            kind: crate::error::DescribeObjectErrorKind::ObjectNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_object_not_found_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeObjectError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_object_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeObjectOutput, crate::error::DescribeObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_object_output::Builder::default();
        let _ = response;
        output = output.set_cache_control(
            crate::http_serde::deser_header_describe_object_describe_object_output_cache_control(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::DescribeObjectError::unhandled(
                    "Failed to parse CacheControl from header `Cache-Control",
                )
            })?,
        );
        output = output.set_content_length(
            crate::http_serde::deser_header_describe_object_describe_object_output_content_length(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::DescribeObjectError::unhandled(
                    "Failed to parse ContentLength from header `Content-Length",
                )
            })?,
        );
        output = output.set_content_type(
            crate::http_serde::deser_header_describe_object_describe_object_output_content_type(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::DescribeObjectError::unhandled(
                    "Failed to parse ContentType from header `Content-Type",
                )
            })?,
        );
        output = output.set_e_tag(
            crate::http_serde::deser_header_describe_object_describe_object_output_e_tag(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::DescribeObjectError::unhandled(
                    "Failed to parse ETag from header `ETag",
                )
            })?,
        );
        output = output.set_last_modified(
            crate::http_serde::deser_header_describe_object_describe_object_output_last_modified(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::DescribeObjectError::unhandled(
                    "Failed to parse LastModified from header `Last-Modified",
                )
            })?,
        );
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_object(
    op_response: &mut smithy_http::operation::Response,
) -> std::result::Result<crate::output::GetObjectOutput, crate::error::GetObjectError> {
    let response = op_response.http_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_object_output::Builder::default();
        let _ = response;
        output = output.set_body(Some(
            crate::http_serde::deser_payload_get_object_get_object_output_body(
                response.body_mut(),
            )?,
        ));
        output = output.set_cache_control(
            crate::http_serde::deser_header_get_object_get_object_output_cache_control(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::GetObjectError::unhandled(
                    "Failed to parse CacheControl from header `Cache-Control",
                )
            })?,
        );
        output = output.set_content_length(
            crate::http_serde::deser_header_get_object_get_object_output_content_length(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::GetObjectError::unhandled(
                    "Failed to parse ContentLength from header `Content-Length",
                )
            })?,
        );
        output = output.set_content_range(
            crate::http_serde::deser_header_get_object_get_object_output_content_range(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::GetObjectError::unhandled(
                    "Failed to parse ContentRange from header `Content-Range",
                )
            })?,
        );
        output = output.set_content_type(
            crate::http_serde::deser_header_get_object_get_object_output_content_type(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::GetObjectError::unhandled(
                    "Failed to parse ContentType from header `Content-Type",
                )
            })?,
        );
        output = output.set_e_tag(
            crate::http_serde::deser_header_get_object_get_object_output_e_tag(response.headers())
                .map_err(|_| {
                    crate::error::GetObjectError::unhandled(
                        "Failed to parse ETag from header `ETag",
                    )
                })?,
        );
        output = output.set_last_modified(
            crate::http_serde::deser_header_get_object_get_object_output_last_modified(
                response.headers(),
            )
            .map_err(|_| {
                crate::error::GetObjectError::unhandled(
                    "Failed to parse LastModified from header `Last-Modified",
                )
            })?,
        );
        output = output.set_status_code(Some(response.status().as_u16() as _));
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_object_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetObjectOutput, crate::error::GetObjectError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetObjectError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetObjectError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContainerNotFoundException" => {
            crate::error::GetObjectError {
                meta: generic,
                kind: crate::error::GetObjectErrorKind::ContainerNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::container_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_container_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetObjectError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerError" => crate::error::GetObjectError {
            meta: generic,
            kind: crate::error::GetObjectErrorKind::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_errorjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ObjectNotFoundException" => crate::error::GetObjectError {
            meta: generic,
            kind: crate::error::GetObjectErrorKind::ObjectNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_object_not_found_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "RequestedRangeNotSatisfiableException" => {
            crate::error::GetObjectError {
                meta: generic,
                kind: crate::error::GetObjectErrorKind::RequestedRangeNotSatisfiableException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]let mut output = crate::error::requested_range_not_satisfiable_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_requested_range_not_satisfiable_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetObjectError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::GetObjectError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_items_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListItemsOutput, crate::error::ListItemsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListItemsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListItemsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContainerNotFoundException" => {
            crate::error::ListItemsError {
                meta: generic,
                kind: crate::error::ListItemsErrorKind::ContainerNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::container_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_container_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListItemsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerError" => crate::error::ListItemsError {
            meta: generic,
            kind: crate::error::ListItemsErrorKind::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_errorjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListItemsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListItemsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_items_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListItemsOutput, crate::error::ListItemsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_items_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_list_items(response.body().as_ref(), output)
            .map_err(crate::error::ListItemsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_put_object_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::PutObjectOutput, crate::error::PutObjectError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::PutObjectError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::PutObjectError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContainerNotFoundException" => {
            crate::error::PutObjectError {
                meta: generic,
                kind: crate::error::PutObjectErrorKind::ContainerNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::container_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_container_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::PutObjectError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerError" => crate::error::PutObjectError {
            meta: generic,
            kind: crate::error::PutObjectErrorKind::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_errorjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::PutObjectError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::PutObjectError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_put_object_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::PutObjectOutput, crate::error::PutObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_object_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_put_object(response.body().as_ref(), output)
            .map_err(crate::error::PutObjectError::unhandled)?;
        output.build()
    })
}
