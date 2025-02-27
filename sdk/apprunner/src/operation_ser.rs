// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_associate_custom_domain(
    input: &crate::input::AssociateCustomDomainInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_associate_custom_domain_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_auto_scaling_configuration(
    input: &crate::input::CreateAutoScalingConfigurationInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_auto_scaling_configuration_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_connection(
    input: &crate::input::CreateConnectionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_connection_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_service(
    input: &crate::input::CreateServiceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_service_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_auto_scaling_configuration(
    input: &crate::input::DeleteAutoScalingConfigurationInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_delete_auto_scaling_configuration_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_connection(
    input: &crate::input::DeleteConnectionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_delete_connection_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_service(
    input: &crate::input::DeleteServiceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_delete_service_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_auto_scaling_configuration(
    input: &crate::input::DescribeAutoScalingConfigurationInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_describe_auto_scaling_configuration_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_custom_domains(
    input: &crate::input::DescribeCustomDomainsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_describe_custom_domains_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_service(
    input: &crate::input::DescribeServiceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_describe_service_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_disassociate_custom_domain(
    input: &crate::input::DisassociateCustomDomainInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_disassociate_custom_domain_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_auto_scaling_configurations(
    input: &crate::input::ListAutoScalingConfigurationsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_auto_scaling_configurations_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_connections(
    input: &crate::input::ListConnectionsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_connections_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_operations(
    input: &crate::input::ListOperationsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_operations_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_services(
    input: &crate::input::ListServicesInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_services_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_tags_for_resource(
    input: &crate::input::ListTagsForResourceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_tags_for_resource_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_pause_service(
    input: &crate::input::PauseServiceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_pause_service_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_resume_service(
    input: &crate::input::ResumeServiceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_resume_service_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_start_deployment(
    input: &crate::input::StartDeploymentInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_start_deployment_input(&mut object, input);
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

pub fn serialize_operation_untag_resource(
    input: &crate::input::UntagResourceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_untag_resource_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_service(
    input: &crate::input::UpdateServiceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_service_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
