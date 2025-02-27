// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_environment_ec2_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentEc2Input,
) {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1);
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2);
    }
    if let Some(var_3) = &input.client_request_token {
        object.key("clientRequestToken").string(var_3);
    }
    if let Some(var_4) = &input.instance_type {
        object.key("instanceType").string(var_4);
    }
    if let Some(var_5) = &input.subnet_id {
        object.key("subnetId").string(var_5);
    }
    if let Some(var_6) = &input.image_id {
        object.key("imageId").string(var_6);
    }
    if let Some(var_7) = &input.automatic_stop_time_minutes {
        object.key("automaticStopTimeMinutes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.owner_arn {
        object.key("ownerArn").string(var_8);
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("tags").start_array();
        for item_11 in var_9 {
            {
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_12, item_11);
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.connection_type {
        object.key("connectionType").string(var_13.as_str());
    }
    if let Some(var_14) = &input.dry_run {
        object.key("dryRun").boolean(*var_14);
    }
}

pub fn serialize_structure_create_environment_membership_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentMembershipInput,
) {
    if let Some(var_15) = &input.environment_id {
        object.key("environmentId").string(var_15);
    }
    if let Some(var_16) = &input.user_arn {
        object.key("userArn").string(var_16);
    }
    if let Some(var_17) = &input.permissions {
        object.key("permissions").string(var_17.as_str());
    }
}

pub fn serialize_structure_delete_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEnvironmentInput,
) {
    if let Some(var_18) = &input.environment_id {
        object.key("environmentId").string(var_18);
    }
}

pub fn serialize_structure_delete_environment_membership_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEnvironmentMembershipInput,
) {
    if let Some(var_19) = &input.environment_id {
        object.key("environmentId").string(var_19);
    }
    if let Some(var_20) = &input.user_arn {
        object.key("userArn").string(var_20);
    }
}

pub fn serialize_structure_describe_environment_memberships_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEnvironmentMembershipsInput,
) {
    if let Some(var_21) = &input.user_arn {
        object.key("userArn").string(var_21);
    }
    if let Some(var_22) = &input.environment_id {
        object.key("environmentId").string(var_22);
    }
    if let Some(var_23) = &input.permissions {
        let mut array_24 = object.key("permissions").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.next_token {
        object.key("nextToken").string(var_26);
    }
    if let Some(var_27) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_27).into()),
        );
    }
}

pub fn serialize_structure_describe_environments_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEnvironmentsInput,
) {
    if let Some(var_28) = &input.environment_ids {
        let mut array_29 = object.key("environmentIds").start_array();
        for item_30 in var_28 {
            {
                array_29.value().string(item_30);
            }
        }
        array_29.finish();
    }
}

pub fn serialize_structure_describe_environment_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEnvironmentStatusInput,
) {
    if let Some(var_31) = &input.environment_id {
        object.key("environmentId").string(var_31);
    }
}

pub fn serialize_structure_list_environments_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEnvironmentsInput,
) {
    if let Some(var_32) = &input.next_token {
        object.key("nextToken").string(var_32);
    }
    if let Some(var_33) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_33).into()),
        );
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_34) = &input.resource_arn {
        object.key("ResourceARN").string(var_34);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_35) = &input.resource_arn {
        object.key("ResourceARN").string(var_35);
    }
    if let Some(var_36) = &input.tags {
        let mut array_37 = object.key("Tags").start_array();
        for item_38 in var_36 {
            {
                let mut object_39 = array_37.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_39, item_38);
                object_39.finish();
            }
        }
        array_37.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_40) = &input.resource_arn {
        object.key("ResourceARN").string(var_40);
    }
    if let Some(var_41) = &input.tag_keys {
        let mut array_42 = object.key("TagKeys").start_array();
        for item_43 in var_41 {
            {
                array_42.value().string(item_43);
            }
        }
        array_42.finish();
    }
}

pub fn serialize_structure_update_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentInput,
) {
    if let Some(var_44) = &input.environment_id {
        object.key("environmentId").string(var_44);
    }
    if let Some(var_45) = &input.name {
        object.key("name").string(var_45);
    }
    if let Some(var_46) = &input.description {
        object.key("description").string(var_46);
    }
    if let Some(var_47) = &input.managed_credentials_action {
        object
            .key("managedCredentialsAction")
            .string(var_47.as_str());
    }
}

pub fn serialize_structure_update_environment_membership_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentMembershipInput,
) {
    if let Some(var_48) = &input.environment_id {
        object.key("environmentId").string(var_48);
    }
    if let Some(var_49) = &input.user_arn {
        object.key("userArn").string(var_49);
    }
    if let Some(var_50) = &input.permissions {
        object.key("permissions").string(var_50.as_str());
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_51) = &input.key {
        object.key("Key").string(var_51);
    }
    if let Some(var_52) = &input.value {
        object.key("Value").string(var_52);
    }
}
