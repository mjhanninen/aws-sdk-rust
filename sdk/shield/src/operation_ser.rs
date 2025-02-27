// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_associate_drt_log_bucket(
    input: &crate::input::AssociateDrtLogBucketInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_associate_drt_log_bucket_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_associate_drt_role(
    input: &crate::input::AssociateDrtRoleInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_associate_drt_role_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_associate_health_check(
    input: &crate::input::AssociateHealthCheckInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_associate_health_check_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_associate_proactive_engagement_details(
    input: &crate::input::AssociateProactiveEngagementDetailsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_associate_proactive_engagement_details_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_protection(
    input: &crate::input::CreateProtectionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_protection_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_protection_group(
    input: &crate::input::CreateProtectionGroupInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_protection_group_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_subscription(
    _input: &crate::input::CreateSubscriptionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_delete_protection(
    input: &crate::input::DeleteProtectionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_delete_protection_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_protection_group(
    input: &crate::input::DeleteProtectionGroupInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_delete_protection_group_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_subscription(
    _input: &crate::input::DeleteSubscriptionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_describe_attack(
    input: &crate::input::DescribeAttackInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_describe_attack_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_attack_statistics(
    _input: &crate::input::DescribeAttackStatisticsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_describe_drt_access(
    _input: &crate::input::DescribeDrtAccessInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_describe_emergency_contact_settings(
    _input: &crate::input::DescribeEmergencyContactSettingsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_describe_protection(
    input: &crate::input::DescribeProtectionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_describe_protection_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_protection_group(
    input: &crate::input::DescribeProtectionGroupInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_describe_protection_group_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_subscription(
    _input: &crate::input::DescribeSubscriptionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_disable_proactive_engagement(
    _input: &crate::input::DisableProactiveEngagementInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_disassociate_drt_log_bucket(
    input: &crate::input::DisassociateDrtLogBucketInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_disassociate_drt_log_bucket_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_disassociate_drt_role(
    _input: &crate::input::DisassociateDrtRoleInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_disassociate_health_check(
    input: &crate::input::DisassociateHealthCheckInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_disassociate_health_check_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_enable_proactive_engagement(
    _input: &crate::input::EnableProactiveEngagementInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_get_subscription_state(
    _input: &crate::input::GetSubscriptionStateInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    Ok(smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_list_attacks(
    input: &crate::input::ListAttacksInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_attacks_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_protection_groups(
    input: &crate::input::ListProtectionGroupsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_protection_groups_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_protections(
    input: &crate::input::ListProtectionsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_protections_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_list_resources_in_protection_group(
    input: &crate::input::ListResourcesInProtectionGroupInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_list_resources_in_protection_group_input(
        &mut object,
        input,
    );
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

pub fn serialize_operation_update_emergency_contact_settings(
    input: &crate::input::UpdateEmergencyContactSettingsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_emergency_contact_settings_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_protection_group(
    input: &crate::input::UpdateProtectionGroupInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_protection_group_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_subscription(
    input: &crate::input::UpdateSubscriptionInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_subscription_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
