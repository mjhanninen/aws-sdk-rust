// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_add_notification_channel_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_1 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_1.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_add_notification_channel_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_2.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_2.len()
        )))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_header_describe_account_health_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_3.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_3.len()
        )))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_describe_account_health_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_4 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_4.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_4.len()
        )))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub fn deser_header_describe_account_overview_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_5 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_5.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_5.len()
        )))
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub fn deser_header_describe_account_overview_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_6 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_6.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_6.len()
        )))
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub fn deser_header_describe_anomaly_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_7 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_7.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_7.len()
        )))
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub fn deser_header_describe_anomaly_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_8 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_8.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_8.len()
        )))
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub fn deser_header_describe_feedback_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_9 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_9.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_9.len()
        )))
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub fn deser_header_describe_feedback_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_10 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_10.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_10.len()
        )))
    } else {
        let mut var_10 = var_10;
        Ok(var_10.pop())
    }
}

pub fn deser_header_describe_insight_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_11 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_11.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_11.len()
        )))
    } else {
        let mut var_11 = var_11;
        Ok(var_11.pop())
    }
}

pub fn deser_header_describe_insight_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_12 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_12.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_12.len()
        )))
    } else {
        let mut var_12 = var_12;
        Ok(var_12.pop())
    }
}

pub fn deser_header_describe_resource_collection_health_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_13 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_13.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_13.len()
        )))
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub fn deser_header_describe_resource_collection_health_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_14 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_14.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_14.len()
        )))
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}

pub fn deser_header_describe_service_integration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_15 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_15.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_15.len()
        )))
    } else {
        let mut var_15 = var_15;
        Ok(var_15.pop())
    }
}

pub fn deser_header_describe_service_integration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_16 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_16.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_16.len()
        )))
    } else {
        let mut var_16 = var_16;
        Ok(var_16.pop())
    }
}

pub fn deser_header_get_cost_estimation_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_17 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_17.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_17.len()
        )))
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}

pub fn deser_header_get_cost_estimation_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_18 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_18.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_18.len()
        )))
    } else {
        let mut var_18 = var_18;
        Ok(var_18.pop())
    }
}

pub fn deser_header_get_resource_collection_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_19 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_19.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_19.len()
        )))
    } else {
        let mut var_19 = var_19;
        Ok(var_19.pop())
    }
}

pub fn deser_header_get_resource_collection_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_20 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_20.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_20.len()
        )))
    } else {
        let mut var_20 = var_20;
        Ok(var_20.pop())
    }
}

pub fn deser_header_list_anomalies_for_insight_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_21 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_21.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_21.len()
        )))
    } else {
        let mut var_21 = var_21;
        Ok(var_21.pop())
    }
}

pub fn deser_header_list_anomalies_for_insight_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_22 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_22.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_22.len()
        )))
    } else {
        let mut var_22 = var_22;
        Ok(var_22.pop())
    }
}

pub fn deser_header_list_events_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_23 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_23.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_23.len()
        )))
    } else {
        let mut var_23 = var_23;
        Ok(var_23.pop())
    }
}

pub fn deser_header_list_events_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_24 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_24.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_24.len()
        )))
    } else {
        let mut var_24 = var_24;
        Ok(var_24.pop())
    }
}

pub fn deser_header_list_insights_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_25 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_25.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_25.len()
        )))
    } else {
        let mut var_25 = var_25;
        Ok(var_25.pop())
    }
}

pub fn deser_header_list_insights_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_26 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_26.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_26.len()
        )))
    } else {
        let mut var_26 = var_26;
        Ok(var_26.pop())
    }
}

pub fn deser_header_list_notification_channels_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_27 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_27.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_27.len()
        )))
    } else {
        let mut var_27 = var_27;
        Ok(var_27.pop())
    }
}

pub fn deser_header_list_notification_channels_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_28 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_28.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_28.len()
        )))
    } else {
        let mut var_28 = var_28;
        Ok(var_28.pop())
    }
}

pub fn deser_header_list_recommendations_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_29 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_29.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_29.len()
        )))
    } else {
        let mut var_29 = var_29;
        Ok(var_29.pop())
    }
}

pub fn deser_header_list_recommendations_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_30 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_30.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_30.len()
        )))
    } else {
        let mut var_30 = var_30;
        Ok(var_30.pop())
    }
}

pub fn deser_header_put_feedback_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_31 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_31.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_31.len()
        )))
    } else {
        let mut var_31 = var_31;
        Ok(var_31.pop())
    }
}

pub fn deser_header_put_feedback_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_32 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_32.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_32.len()
        )))
    } else {
        let mut var_32 = var_32;
        Ok(var_32.pop())
    }
}

pub fn deser_header_remove_notification_channel_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_33 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_33.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_33.len()
        )))
    } else {
        let mut var_33 = var_33;
        Ok(var_33.pop())
    }
}

pub fn deser_header_remove_notification_channel_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_34 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_34.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_34.len()
        )))
    } else {
        let mut var_34 = var_34;
        Ok(var_34.pop())
    }
}

pub fn deser_header_search_insights_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_35 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_35.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_35.len()
        )))
    } else {
        let mut var_35 = var_35;
        Ok(var_35.pop())
    }
}

pub fn deser_header_search_insights_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_36 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_36.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_36.len()
        )))
    } else {
        let mut var_36 = var_36;
        Ok(var_36.pop())
    }
}

pub fn deser_header_start_cost_estimation_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_37 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_37.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_37.len()
        )))
    } else {
        let mut var_37 = var_37;
        Ok(var_37.pop())
    }
}

pub fn deser_header_start_cost_estimation_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_38 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_38.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_38.len()
        )))
    } else {
        let mut var_38 = var_38;
        Ok(var_38.pop())
    }
}

pub fn deser_header_update_resource_collection_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_39 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_39.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_39.len()
        )))
    } else {
        let mut var_39 = var_39;
        Ok(var_39.pop())
    }
}

pub fn deser_header_update_resource_collection_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_40 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_40.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_40.len()
        )))
    } else {
        let mut var_40 = var_40;
        Ok(var_40.pop())
    }
}

pub fn deser_header_update_service_integration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_41 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_41.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_41.len()
        )))
    } else {
        let mut var_41 = var_41;
        Ok(var_41.pop())
    }
}

pub fn deser_header_update_service_integration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_42 = smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_42.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_42.len()
        )))
    } else {
        let mut var_42 = var_42;
        Ok(var_42.pop())
    }
}
