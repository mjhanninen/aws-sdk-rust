// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_finalize_device_claim(
    input: &crate::input::FinalizeDeviceClaimInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_finalize_device_claim_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_invoke_device_method(
    input: &crate::input::InvokeDeviceMethodInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_invoke_device_method_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_tag_resource(
    input: &crate::input::TagResourceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_tag_resource_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_device_state(
    input: &crate::input::UpdateDeviceStateInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_device_state_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
