// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_batch_get_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetChannelInput,
) {
    if let Some(var_1) = &input.arns {
        let mut array_2 = object.key("arns").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3);
            }
        }
        array_2.finish();
    }
}

pub fn serialize_structure_batch_get_stream_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetStreamKeyInput,
) {
    if let Some(var_4) = &input.arns {
        let mut array_5 = object.key("arns").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6);
            }
        }
        array_5.finish();
    }
}

pub fn serialize_structure_create_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) {
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if let Some(var_7) = &input.latency_mode {
        object.key("latencyMode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8);
    }
    if let Some(var_9) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_9);
    }
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
    if let Some(var_14) = &input.r#type {
        object.key("type").string(var_14.as_str());
    }
}

pub fn serialize_structure_create_recording_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRecordingConfigurationInput,
) {
    if let Some(var_15) = &input.destination_configuration {
        let mut object_16 = object.key("destinationConfiguration").start_object();
        crate::json_ser::serialize_structure_destination_configuration(&mut object_16, var_15);
        object_16.finish();
    }
    if let Some(var_17) = &input.name {
        object.key("name").string(var_17);
    }
    if let Some(var_18) = &input.tags {
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20).string(value_21);
            }
        }
        object_19.finish();
    }
}

pub fn serialize_structure_create_stream_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamKeyInput,
) {
    if let Some(var_22) = &input.channel_arn {
        object.key("channelArn").string(var_22);
    }
    if let Some(var_23) = &input.tags {
        let mut object_24 = object.key("tags").start_object();
        for (key_25, value_26) in var_23 {
            {
                object_24.key(key_25).string(value_26);
            }
        }
        object_24.finish();
    }
}

pub fn serialize_structure_delete_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteChannelInput,
) {
    if let Some(var_27) = &input.arn {
        object.key("arn").string(var_27);
    }
}

pub fn serialize_structure_delete_playback_key_pair_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePlaybackKeyPairInput,
) {
    if let Some(var_28) = &input.arn {
        object.key("arn").string(var_28);
    }
}

pub fn serialize_structure_delete_recording_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRecordingConfigurationInput,
) {
    if let Some(var_29) = &input.arn {
        object.key("arn").string(var_29);
    }
}

pub fn serialize_structure_delete_stream_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteStreamKeyInput,
) {
    if let Some(var_30) = &input.arn {
        object.key("arn").string(var_30);
    }
}

pub fn serialize_structure_get_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetChannelInput,
) {
    if let Some(var_31) = &input.arn {
        object.key("arn").string(var_31);
    }
}

pub fn serialize_structure_get_playback_key_pair_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPlaybackKeyPairInput,
) {
    if let Some(var_32) = &input.arn {
        object.key("arn").string(var_32);
    }
}

pub fn serialize_structure_get_recording_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRecordingConfigurationInput,
) {
    if let Some(var_33) = &input.arn {
        object.key("arn").string(var_33);
    }
}

pub fn serialize_structure_get_stream_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetStreamInput,
) {
    if let Some(var_34) = &input.channel_arn {
        object.key("channelArn").string(var_34);
    }
}

pub fn serialize_structure_get_stream_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetStreamKeyInput,
) {
    if let Some(var_35) = &input.arn {
        object.key("arn").string(var_35);
    }
}

pub fn serialize_structure_import_playback_key_pair_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportPlaybackKeyPairInput,
) {
    if let Some(var_36) = &input.name {
        object.key("name").string(var_36);
    }
    if let Some(var_37) = &input.public_key_material {
        object.key("publicKeyMaterial").string(var_37);
    }
    if let Some(var_38) = &input.tags {
        let mut object_39 = object.key("tags").start_object();
        for (key_40, value_41) in var_38 {
            {
                object_39.key(key_40).string(value_41);
            }
        }
        object_39.finish();
    }
}

pub fn serialize_structure_list_channels_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListChannelsInput,
) {
    if let Some(var_42) = &input.filter_by_name {
        object.key("filterByName").string(var_42);
    }
    if let Some(var_43) = &input.filter_by_recording_configuration_arn {
        object
            .key("filterByRecordingConfigurationArn")
            .string(var_43);
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_44) = &input.next_token {
        object.key("nextToken").string(var_44);
    }
}

pub fn serialize_structure_list_playback_key_pairs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPlaybackKeyPairsInput,
) {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_45) = &input.next_token {
        object.key("nextToken").string(var_45);
    }
}

pub fn serialize_structure_list_recording_configurations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRecordingConfigurationsInput,
) {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("nextToken").string(var_46);
    }
}

pub fn serialize_structure_list_stream_keys_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamKeysInput,
) {
    if let Some(var_47) = &input.channel_arn {
        object.key("channelArn").string(var_47);
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48);
    }
}

pub fn serialize_structure_list_streams_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamsInput,
) {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_49) = &input.next_token {
        object.key("nextToken").string(var_49);
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_50) = &input.next_token {
        object.key("nextToken").string(var_50);
    }
}

pub fn serialize_structure_put_metadata_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutMetadataInput,
) {
    if let Some(var_51) = &input.channel_arn {
        object.key("channelArn").string(var_51);
    }
    if let Some(var_52) = &input.metadata {
        object.key("metadata").string(var_52);
    }
}

pub fn serialize_structure_stop_stream_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopStreamInput,
) {
    if let Some(var_53) = &input.channel_arn {
        object.key("channelArn").string(var_53);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_54) = &input.tags {
        let mut object_55 = object.key("tags").start_object();
        for (key_56, value_57) in var_54 {
            {
                object_55.key(key_56).string(value_57);
            }
        }
        object_55.finish();
    }
}

pub fn serialize_structure_update_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) {
    if let Some(var_58) = &input.arn {
        object.key("arn").string(var_58);
    }
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if let Some(var_59) = &input.latency_mode {
        object.key("latencyMode").string(var_59.as_str());
    }
    if let Some(var_60) = &input.name {
        object.key("name").string(var_60);
    }
    if let Some(var_61) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_61);
    }
    if let Some(var_62) = &input.r#type {
        object.key("type").string(var_62.as_str());
    }
}

pub fn serialize_structure_destination_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DestinationConfiguration,
) {
    if let Some(var_63) = &input.s3 {
        let mut object_64 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_s3_destination_configuration(&mut object_64, var_63);
        object_64.finish();
    }
}

pub fn serialize_structure_s3_destination_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3DestinationConfiguration,
) {
    if let Some(var_65) = &input.bucket_name {
        object.key("bucketName").string(var_65);
    }
}
