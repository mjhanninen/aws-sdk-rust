// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_internal_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::internal_exception::Builder,
) -> Result<crate::error::internal_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "errorCode" => {
                        builder = builder.set_error_code(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_invalid_input_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::invalid_input_exception::Builder,
) -> Result<crate::error::invalid_input_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "errorCode" => {
                        builder = builder.set_error_code(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "fieldName" => {
                        builder = builder.set_field_name(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_limit_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::limit_exceeded_exception::Builder,
) -> Result<crate::error::limit_exceeded_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "errorCode" => {
                        builder = builder.set_error_code(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "resourceType" => {
                        builder = builder.set_resource_type(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_access_denied_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::access_denied_exception::Builder,
) -> Result<crate::error::access_denied_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "resourceType" => {
                        builder = builder.set_resource_type(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_associate_s3_resources(
    input: &[u8],
    mut builder: crate::output::associate_s3_resources_output::Builder,
) -> Result<crate::output::associate_s3_resources_output::Builder, smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "failedS3Resources" => {
                        builder = builder.set_failed_s3_resources(
                            crate::json_deser::deser_list_failed_s3_resources(tokens)?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_disassociate_s3_resources(
    input: &[u8],
    mut builder: crate::output::disassociate_s3_resources_output::Builder,
) -> Result<crate::output::disassociate_s3_resources_output::Builder, smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "failedS3Resources" => {
                        builder = builder.set_failed_s3_resources(
                            crate::json_deser::deser_list_failed_s3_resources(tokens)?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_list_member_accounts(
    input: &[u8],
    mut builder: crate::output::list_member_accounts_output::Builder,
) -> Result<crate::output::list_member_accounts_output::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "memberAccounts" => {
                        builder = builder.set_member_accounts(
                            crate::json_deser::deser_list_member_accounts(tokens)?,
                        );
                    }
                    "nextToken" => {
                        builder = builder.set_next_token(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_list_s3_resources(
    input: &[u8],
    mut builder: crate::output::list_s3_resources_output::Builder,
) -> Result<crate::output::list_s3_resources_output::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "s3Resources" => {
                        builder = builder.set_s3_resources(
                            crate::json_deser::deser_list_s3_resources_classification(tokens)?,
                        );
                    }
                    "nextToken" => {
                        builder = builder.set_next_token(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_update_s3_resources(
    input: &[u8],
    mut builder: crate::output::update_s3_resources_output::Builder,
) -> Result<crate::output::update_s3_resources_output::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "failedS3Resources" => {
                        builder = builder.set_failed_s3_resources(
                            crate::json_deser::deser_list_failed_s3_resources(tokens)?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_failed_s3_resources<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::FailedS3Resource>>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value = crate::json_deser::deser_structure_failed_s3_resource(tokens)?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_member_accounts<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::MemberAccount>>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value = crate::json_deser::deser_structure_member_account(tokens)?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_s3_resources_classification<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<std::vec::Vec<crate::model::S3ResourceClassification>>,
    smithy_json::deserialize::Error,
>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value =
                            crate::json_deser::deser_structure_s3_resource_classification(tokens)?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

pub fn deser_structure_failed_s3_resource<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::FailedS3Resource>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::FailedS3Resource::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "failedItem" => {
                                builder = builder.set_failed_item(
                                    crate::json_deser::deser_structure_s3_resource(tokens)?,
                                );
                            }
                            "errorCode" => {
                                builder = builder.set_error_code(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "errorMessage" => {
                                builder = builder.set_error_message(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_member_account<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::MemberAccount>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::MemberAccount::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "accountId" => {
                                builder = builder.set_account_id(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_s3_resource_classification<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::S3ResourceClassification>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::S3ResourceClassification::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "bucketName" => {
                                builder = builder.set_bucket_name(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "prefix" => {
                                builder = builder.set_prefix(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "classificationType" => {
                                builder = builder.set_classification_type(
                                    crate::json_deser::deser_structure_classification_type(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_s3_resource<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::S3Resource>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::S3Resource::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "bucketName" => {
                                builder = builder.set_bucket_name(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "prefix" => {
                                builder = builder.set_prefix(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_classification_type<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::ClassificationType>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ClassificationType::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "oneTime" => {
                                builder = builder.set_one_time(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::model::S3OneTimeClassificationType::from(
                                                u.as_ref(),
                                            )
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "continuous" => {
                                builder = builder.set_continuous(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::model::S3ContinuousClassificationType::from(
                                                u.as_ref(),
                                            )
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}
