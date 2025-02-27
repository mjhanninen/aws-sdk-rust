// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_get_media_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMediaInput,
) {
    if let Some(var_1) = &input.start_selector {
        let mut object_2 = object.key("StartSelector").start_object();
        crate::json_ser::serialize_structure_start_selector(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.stream_arn {
        object.key("StreamARN").string(var_3);
    }
    if let Some(var_4) = &input.stream_name {
        object.key("StreamName").string(var_4);
    }
}

pub fn serialize_structure_start_selector(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StartSelector,
) {
    if let Some(var_5) = &input.start_selector_type {
        object.key("StartSelectorType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.after_fragment_number {
        object.key("AfterFragmentNumber").string(var_6);
    }
    if let Some(var_7) = &input.start_timestamp {
        object
            .key("StartTimestamp")
            .instant(var_7, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_8) = &input.continuation_token {
        object.key("ContinuationToken").string(var_8);
    }
}
