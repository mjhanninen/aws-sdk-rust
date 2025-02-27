// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_associate_delegate_to_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateDelegateToResourceInput,
) {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1);
    }
    if let Some(var_2) = &input.resource_id {
        object.key("ResourceId").string(var_2);
    }
    if let Some(var_3) = &input.entity_id {
        object.key("EntityId").string(var_3);
    }
}

pub fn serialize_structure_associate_member_to_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateMemberToGroupInput,
) {
    if let Some(var_4) = &input.organization_id {
        object.key("OrganizationId").string(var_4);
    }
    if let Some(var_5) = &input.group_id {
        object.key("GroupId").string(var_5);
    }
    if let Some(var_6) = &input.member_id {
        object.key("MemberId").string(var_6);
    }
}

pub fn serialize_structure_cancel_mailbox_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelMailboxExportJobInput,
) {
    if let Some(var_7) = &input.client_token {
        object.key("ClientToken").string(var_7);
    }
    if let Some(var_8) = &input.job_id {
        object.key("JobId").string(var_8);
    }
    if let Some(var_9) = &input.organization_id {
        object.key("OrganizationId").string(var_9);
    }
}

pub fn serialize_structure_create_alias_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAliasInput,
) {
    if let Some(var_10) = &input.organization_id {
        object.key("OrganizationId").string(var_10);
    }
    if let Some(var_11) = &input.entity_id {
        object.key("EntityId").string(var_11);
    }
    if let Some(var_12) = &input.alias {
        object.key("Alias").string(var_12);
    }
}

pub fn serialize_structure_create_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGroupInput,
) {
    if let Some(var_13) = &input.organization_id {
        object.key("OrganizationId").string(var_13);
    }
    if let Some(var_14) = &input.name {
        object.key("Name").string(var_14);
    }
}

pub fn serialize_structure_create_mobile_device_access_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMobileDeviceAccessRuleInput,
) {
    if let Some(var_15) = &input.organization_id {
        object.key("OrganizationId").string(var_15);
    }
    if let Some(var_16) = &input.client_token {
        object.key("ClientToken").string(var_16);
    }
    if let Some(var_17) = &input.name {
        object.key("Name").string(var_17);
    }
    if let Some(var_18) = &input.description {
        object.key("Description").string(var_18);
    }
    if let Some(var_19) = &input.effect {
        object.key("Effect").string(var_19.as_str());
    }
    if let Some(var_20) = &input.device_types {
        let mut array_21 = object.key("DeviceTypes").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22);
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.not_device_types {
        let mut array_24 = object.key("NotDeviceTypes").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25);
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.device_models {
        let mut array_27 = object.key("DeviceModels").start_array();
        for item_28 in var_26 {
            {
                array_27.value().string(item_28);
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.not_device_models {
        let mut array_30 = object.key("NotDeviceModels").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31);
            }
        }
        array_30.finish();
    }
    if let Some(var_32) = &input.device_operating_systems {
        let mut array_33 = object.key("DeviceOperatingSystems").start_array();
        for item_34 in var_32 {
            {
                array_33.value().string(item_34);
            }
        }
        array_33.finish();
    }
    if let Some(var_35) = &input.not_device_operating_systems {
        let mut array_36 = object.key("NotDeviceOperatingSystems").start_array();
        for item_37 in var_35 {
            {
                array_36.value().string(item_37);
            }
        }
        array_36.finish();
    }
    if let Some(var_38) = &input.device_user_agents {
        let mut array_39 = object.key("DeviceUserAgents").start_array();
        for item_40 in var_38 {
            {
                array_39.value().string(item_40);
            }
        }
        array_39.finish();
    }
    if let Some(var_41) = &input.not_device_user_agents {
        let mut array_42 = object.key("NotDeviceUserAgents").start_array();
        for item_43 in var_41 {
            {
                array_42.value().string(item_43);
            }
        }
        array_42.finish();
    }
}

pub fn serialize_structure_create_organization_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateOrganizationInput,
) {
    if let Some(var_44) = &input.directory_id {
        object.key("DirectoryId").string(var_44);
    }
    if let Some(var_45) = &input.alias {
        object.key("Alias").string(var_45);
    }
    if let Some(var_46) = &input.client_token {
        object.key("ClientToken").string(var_46);
    }
    if let Some(var_47) = &input.domains {
        let mut array_48 = object.key("Domains").start_array();
        for item_49 in var_47 {
            {
                let mut object_50 = array_48.value().start_object();
                crate::json_ser::serialize_structure_domain(&mut object_50, item_49);
                object_50.finish();
            }
        }
        array_48.finish();
    }
    if let Some(var_51) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_51);
    }
    if input.enable_interoperability {
        object
            .key("EnableInteroperability")
            .boolean(input.enable_interoperability);
    }
}

pub fn serialize_structure_create_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResourceInput,
) {
    if let Some(var_52) = &input.organization_id {
        object.key("OrganizationId").string(var_52);
    }
    if let Some(var_53) = &input.name {
        object.key("Name").string(var_53);
    }
    if let Some(var_54) = &input.r#type {
        object.key("Type").string(var_54.as_str());
    }
}

pub fn serialize_structure_create_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) {
    if let Some(var_55) = &input.organization_id {
        object.key("OrganizationId").string(var_55);
    }
    if let Some(var_56) = &input.name {
        object.key("Name").string(var_56);
    }
    if let Some(var_57) = &input.display_name {
        object.key("DisplayName").string(var_57);
    }
    if let Some(var_58) = &input.password {
        object.key("Password").string(var_58);
    }
}

pub fn serialize_structure_delete_access_control_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAccessControlRuleInput,
) {
    if let Some(var_59) = &input.organization_id {
        object.key("OrganizationId").string(var_59);
    }
    if let Some(var_60) = &input.name {
        object.key("Name").string(var_60);
    }
}

pub fn serialize_structure_delete_alias_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAliasInput,
) {
    if let Some(var_61) = &input.organization_id {
        object.key("OrganizationId").string(var_61);
    }
    if let Some(var_62) = &input.entity_id {
        object.key("EntityId").string(var_62);
    }
    if let Some(var_63) = &input.alias {
        object.key("Alias").string(var_63);
    }
}

pub fn serialize_structure_delete_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteGroupInput,
) {
    if let Some(var_64) = &input.organization_id {
        object.key("OrganizationId").string(var_64);
    }
    if let Some(var_65) = &input.group_id {
        object.key("GroupId").string(var_65);
    }
}

pub fn serialize_structure_delete_mailbox_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteMailboxPermissionsInput,
) {
    if let Some(var_66) = &input.organization_id {
        object.key("OrganizationId").string(var_66);
    }
    if let Some(var_67) = &input.entity_id {
        object.key("EntityId").string(var_67);
    }
    if let Some(var_68) = &input.grantee_id {
        object.key("GranteeId").string(var_68);
    }
}

pub fn serialize_structure_delete_mobile_device_access_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteMobileDeviceAccessRuleInput,
) {
    if let Some(var_69) = &input.organization_id {
        object.key("OrganizationId").string(var_69);
    }
    if let Some(var_70) = &input.mobile_device_access_rule_id {
        object.key("MobileDeviceAccessRuleId").string(var_70);
    }
}

pub fn serialize_structure_delete_organization_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteOrganizationInput,
) {
    if let Some(var_71) = &input.client_token {
        object.key("ClientToken").string(var_71);
    }
    if let Some(var_72) = &input.organization_id {
        object.key("OrganizationId").string(var_72);
    }
    {
        object
            .key("DeleteDirectory")
            .boolean(input.delete_directory);
    }
}

pub fn serialize_structure_delete_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResourceInput,
) {
    if let Some(var_73) = &input.organization_id {
        object.key("OrganizationId").string(var_73);
    }
    if let Some(var_74) = &input.resource_id {
        object.key("ResourceId").string(var_74);
    }
}

pub fn serialize_structure_delete_retention_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRetentionPolicyInput,
) {
    if let Some(var_75) = &input.organization_id {
        object.key("OrganizationId").string(var_75);
    }
    if let Some(var_76) = &input.id {
        object.key("Id").string(var_76);
    }
}

pub fn serialize_structure_delete_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteUserInput,
) {
    if let Some(var_77) = &input.organization_id {
        object.key("OrganizationId").string(var_77);
    }
    if let Some(var_78) = &input.user_id {
        object.key("UserId").string(var_78);
    }
}

pub fn serialize_structure_deregister_from_work_mail_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeregisterFromWorkMailInput,
) {
    if let Some(var_79) = &input.organization_id {
        object.key("OrganizationId").string(var_79);
    }
    if let Some(var_80) = &input.entity_id {
        object.key("EntityId").string(var_80);
    }
}

pub fn serialize_structure_describe_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeGroupInput,
) {
    if let Some(var_81) = &input.organization_id {
        object.key("OrganizationId").string(var_81);
    }
    if let Some(var_82) = &input.group_id {
        object.key("GroupId").string(var_82);
    }
}

pub fn serialize_structure_describe_mailbox_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeMailboxExportJobInput,
) {
    if let Some(var_83) = &input.job_id {
        object.key("JobId").string(var_83);
    }
    if let Some(var_84) = &input.organization_id {
        object.key("OrganizationId").string(var_84);
    }
}

pub fn serialize_structure_describe_organization_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeOrganizationInput,
) {
    if let Some(var_85) = &input.organization_id {
        object.key("OrganizationId").string(var_85);
    }
}

pub fn serialize_structure_describe_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeResourceInput,
) {
    if let Some(var_86) = &input.organization_id {
        object.key("OrganizationId").string(var_86);
    }
    if let Some(var_87) = &input.resource_id {
        object.key("ResourceId").string(var_87);
    }
}

pub fn serialize_structure_describe_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeUserInput,
) {
    if let Some(var_88) = &input.organization_id {
        object.key("OrganizationId").string(var_88);
    }
    if let Some(var_89) = &input.user_id {
        object.key("UserId").string(var_89);
    }
}

pub fn serialize_structure_disassociate_delegate_from_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateDelegateFromResourceInput,
) {
    if let Some(var_90) = &input.organization_id {
        object.key("OrganizationId").string(var_90);
    }
    if let Some(var_91) = &input.resource_id {
        object.key("ResourceId").string(var_91);
    }
    if let Some(var_92) = &input.entity_id {
        object.key("EntityId").string(var_92);
    }
}

pub fn serialize_structure_disassociate_member_from_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateMemberFromGroupInput,
) {
    if let Some(var_93) = &input.organization_id {
        object.key("OrganizationId").string(var_93);
    }
    if let Some(var_94) = &input.group_id {
        object.key("GroupId").string(var_94);
    }
    if let Some(var_95) = &input.member_id {
        object.key("MemberId").string(var_95);
    }
}

pub fn serialize_structure_get_access_control_effect_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAccessControlEffectInput,
) {
    if let Some(var_96) = &input.organization_id {
        object.key("OrganizationId").string(var_96);
    }
    if let Some(var_97) = &input.ip_address {
        object.key("IpAddress").string(var_97);
    }
    if let Some(var_98) = &input.action {
        object.key("Action").string(var_98);
    }
    if let Some(var_99) = &input.user_id {
        object.key("UserId").string(var_99);
    }
}

pub fn serialize_structure_get_default_retention_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDefaultRetentionPolicyInput,
) {
    if let Some(var_100) = &input.organization_id {
        object.key("OrganizationId").string(var_100);
    }
}

pub fn serialize_structure_get_mailbox_details_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMailboxDetailsInput,
) {
    if let Some(var_101) = &input.organization_id {
        object.key("OrganizationId").string(var_101);
    }
    if let Some(var_102) = &input.user_id {
        object.key("UserId").string(var_102);
    }
}

pub fn serialize_structure_get_mobile_device_access_effect_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMobileDeviceAccessEffectInput,
) {
    if let Some(var_103) = &input.organization_id {
        object.key("OrganizationId").string(var_103);
    }
    if let Some(var_104) = &input.device_type {
        object.key("DeviceType").string(var_104);
    }
    if let Some(var_105) = &input.device_model {
        object.key("DeviceModel").string(var_105);
    }
    if let Some(var_106) = &input.device_operating_system {
        object.key("DeviceOperatingSystem").string(var_106);
    }
    if let Some(var_107) = &input.device_user_agent {
        object.key("DeviceUserAgent").string(var_107);
    }
}

pub fn serialize_structure_list_access_control_rules_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAccessControlRulesInput,
) {
    if let Some(var_108) = &input.organization_id {
        object.key("OrganizationId").string(var_108);
    }
}

pub fn serialize_structure_list_aliases_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAliasesInput,
) {
    if let Some(var_109) = &input.organization_id {
        object.key("OrganizationId").string(var_109);
    }
    if let Some(var_110) = &input.entity_id {
        object.key("EntityId").string(var_110);
    }
    if let Some(var_111) = &input.next_token {
        object.key("NextToken").string(var_111);
    }
    if let Some(var_112) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_112).into()),
        );
    }
}

pub fn serialize_structure_list_group_members_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGroupMembersInput,
) {
    if let Some(var_113) = &input.organization_id {
        object.key("OrganizationId").string(var_113);
    }
    if let Some(var_114) = &input.group_id {
        object.key("GroupId").string(var_114);
    }
    if let Some(var_115) = &input.next_token {
        object.key("NextToken").string(var_115);
    }
    if let Some(var_116) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_116).into()),
        );
    }
}

pub fn serialize_structure_list_groups_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGroupsInput,
) {
    if let Some(var_117) = &input.organization_id {
        object.key("OrganizationId").string(var_117);
    }
    if let Some(var_118) = &input.next_token {
        object.key("NextToken").string(var_118);
    }
    if let Some(var_119) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_119).into()),
        );
    }
}

pub fn serialize_structure_list_mailbox_export_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMailboxExportJobsInput,
) {
    if let Some(var_120) = &input.organization_id {
        object.key("OrganizationId").string(var_120);
    }
    if let Some(var_121) = &input.next_token {
        object.key("NextToken").string(var_121);
    }
    if let Some(var_122) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_122).into()),
        );
    }
}

pub fn serialize_structure_list_mailbox_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMailboxPermissionsInput,
) {
    if let Some(var_123) = &input.organization_id {
        object.key("OrganizationId").string(var_123);
    }
    if let Some(var_124) = &input.entity_id {
        object.key("EntityId").string(var_124);
    }
    if let Some(var_125) = &input.next_token {
        object.key("NextToken").string(var_125);
    }
    if let Some(var_126) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_126).into()),
        );
    }
}

pub fn serialize_structure_list_mobile_device_access_rules_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMobileDeviceAccessRulesInput,
) {
    if let Some(var_127) = &input.organization_id {
        object.key("OrganizationId").string(var_127);
    }
}

pub fn serialize_structure_list_organizations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListOrganizationsInput,
) {
    if let Some(var_128) = &input.next_token {
        object.key("NextToken").string(var_128);
    }
    if let Some(var_129) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_129).into()),
        );
    }
}

pub fn serialize_structure_list_resource_delegates_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResourceDelegatesInput,
) {
    if let Some(var_130) = &input.organization_id {
        object.key("OrganizationId").string(var_130);
    }
    if let Some(var_131) = &input.resource_id {
        object.key("ResourceId").string(var_131);
    }
    if let Some(var_132) = &input.next_token {
        object.key("NextToken").string(var_132);
    }
    if let Some(var_133) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_133).into()),
        );
    }
}

pub fn serialize_structure_list_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResourcesInput,
) {
    if let Some(var_134) = &input.organization_id {
        object.key("OrganizationId").string(var_134);
    }
    if let Some(var_135) = &input.next_token {
        object.key("NextToken").string(var_135);
    }
    if let Some(var_136) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_136).into()),
        );
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_137) = &input.resource_arn {
        object.key("ResourceARN").string(var_137);
    }
}

pub fn serialize_structure_list_users_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListUsersInput,
) {
    if let Some(var_138) = &input.organization_id {
        object.key("OrganizationId").string(var_138);
    }
    if let Some(var_139) = &input.next_token {
        object.key("NextToken").string(var_139);
    }
    if let Some(var_140) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_140).into()),
        );
    }
}

pub fn serialize_structure_put_access_control_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAccessControlRuleInput,
) {
    if let Some(var_141) = &input.name {
        object.key("Name").string(var_141);
    }
    if let Some(var_142) = &input.effect {
        object.key("Effect").string(var_142.as_str());
    }
    if let Some(var_143) = &input.description {
        object.key("Description").string(var_143);
    }
    if let Some(var_144) = &input.ip_ranges {
        let mut array_145 = object.key("IpRanges").start_array();
        for item_146 in var_144 {
            {
                array_145.value().string(item_146);
            }
        }
        array_145.finish();
    }
    if let Some(var_147) = &input.not_ip_ranges {
        let mut array_148 = object.key("NotIpRanges").start_array();
        for item_149 in var_147 {
            {
                array_148.value().string(item_149);
            }
        }
        array_148.finish();
    }
    if let Some(var_150) = &input.actions {
        let mut array_151 = object.key("Actions").start_array();
        for item_152 in var_150 {
            {
                array_151.value().string(item_152);
            }
        }
        array_151.finish();
    }
    if let Some(var_153) = &input.not_actions {
        let mut array_154 = object.key("NotActions").start_array();
        for item_155 in var_153 {
            {
                array_154.value().string(item_155);
            }
        }
        array_154.finish();
    }
    if let Some(var_156) = &input.user_ids {
        let mut array_157 = object.key("UserIds").start_array();
        for item_158 in var_156 {
            {
                array_157.value().string(item_158);
            }
        }
        array_157.finish();
    }
    if let Some(var_159) = &input.not_user_ids {
        let mut array_160 = object.key("NotUserIds").start_array();
        for item_161 in var_159 {
            {
                array_160.value().string(item_161);
            }
        }
        array_160.finish();
    }
    if let Some(var_162) = &input.organization_id {
        object.key("OrganizationId").string(var_162);
    }
}

pub fn serialize_structure_put_mailbox_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutMailboxPermissionsInput,
) {
    if let Some(var_163) = &input.organization_id {
        object.key("OrganizationId").string(var_163);
    }
    if let Some(var_164) = &input.entity_id {
        object.key("EntityId").string(var_164);
    }
    if let Some(var_165) = &input.grantee_id {
        object.key("GranteeId").string(var_165);
    }
    if let Some(var_166) = &input.permission_values {
        let mut array_167 = object.key("PermissionValues").start_array();
        for item_168 in var_166 {
            {
                array_167.value().string(item_168.as_str());
            }
        }
        array_167.finish();
    }
}

pub fn serialize_structure_put_retention_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRetentionPolicyInput,
) {
    if let Some(var_169) = &input.organization_id {
        object.key("OrganizationId").string(var_169);
    }
    if let Some(var_170) = &input.id {
        object.key("Id").string(var_170);
    }
    if let Some(var_171) = &input.name {
        object.key("Name").string(var_171);
    }
    if let Some(var_172) = &input.description {
        object.key("Description").string(var_172);
    }
    if let Some(var_173) = &input.folder_configurations {
        let mut array_174 = object.key("FolderConfigurations").start_array();
        for item_175 in var_173 {
            {
                let mut object_176 = array_174.value().start_object();
                crate::json_ser::serialize_structure_folder_configuration(
                    &mut object_176,
                    item_175,
                );
                object_176.finish();
            }
        }
        array_174.finish();
    }
}

pub fn serialize_structure_register_to_work_mail_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterToWorkMailInput,
) {
    if let Some(var_177) = &input.organization_id {
        object.key("OrganizationId").string(var_177);
    }
    if let Some(var_178) = &input.entity_id {
        object.key("EntityId").string(var_178);
    }
    if let Some(var_179) = &input.email {
        object.key("Email").string(var_179);
    }
}

pub fn serialize_structure_reset_password_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResetPasswordInput,
) {
    if let Some(var_180) = &input.organization_id {
        object.key("OrganizationId").string(var_180);
    }
    if let Some(var_181) = &input.user_id {
        object.key("UserId").string(var_181);
    }
    if let Some(var_182) = &input.password {
        object.key("Password").string(var_182);
    }
}

pub fn serialize_structure_start_mailbox_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartMailboxExportJobInput,
) {
    if let Some(var_183) = &input.client_token {
        object.key("ClientToken").string(var_183);
    }
    if let Some(var_184) = &input.organization_id {
        object.key("OrganizationId").string(var_184);
    }
    if let Some(var_185) = &input.entity_id {
        object.key("EntityId").string(var_185);
    }
    if let Some(var_186) = &input.description {
        object.key("Description").string(var_186);
    }
    if let Some(var_187) = &input.role_arn {
        object.key("RoleArn").string(var_187);
    }
    if let Some(var_188) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_188);
    }
    if let Some(var_189) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_189);
    }
    if let Some(var_190) = &input.s3_prefix {
        object.key("S3Prefix").string(var_190);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_191) = &input.resource_arn {
        object.key("ResourceARN").string(var_191);
    }
    if let Some(var_192) = &input.tags {
        let mut array_193 = object.key("Tags").start_array();
        for item_194 in var_192 {
            {
                let mut object_195 = array_193.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_195, item_194);
                object_195.finish();
            }
        }
        array_193.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_196) = &input.resource_arn {
        object.key("ResourceARN").string(var_196);
    }
    if let Some(var_197) = &input.tag_keys {
        let mut array_198 = object.key("TagKeys").start_array();
        for item_199 in var_197 {
            {
                array_198.value().string(item_199);
            }
        }
        array_198.finish();
    }
}

pub fn serialize_structure_update_mailbox_quota_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMailboxQuotaInput,
) {
    if let Some(var_200) = &input.organization_id {
        object.key("OrganizationId").string(var_200);
    }
    if let Some(var_201) = &input.user_id {
        object.key("UserId").string(var_201);
    }
    if let Some(var_202) = &input.mailbox_quota {
        object.key("MailboxQuota").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_202).into()),
        );
    }
}

pub fn serialize_structure_update_mobile_device_access_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMobileDeviceAccessRuleInput,
) {
    if let Some(var_203) = &input.organization_id {
        object.key("OrganizationId").string(var_203);
    }
    if let Some(var_204) = &input.mobile_device_access_rule_id {
        object.key("MobileDeviceAccessRuleId").string(var_204);
    }
    if let Some(var_205) = &input.name {
        object.key("Name").string(var_205);
    }
    if let Some(var_206) = &input.description {
        object.key("Description").string(var_206);
    }
    if let Some(var_207) = &input.effect {
        object.key("Effect").string(var_207.as_str());
    }
    if let Some(var_208) = &input.device_types {
        let mut array_209 = object.key("DeviceTypes").start_array();
        for item_210 in var_208 {
            {
                array_209.value().string(item_210);
            }
        }
        array_209.finish();
    }
    if let Some(var_211) = &input.not_device_types {
        let mut array_212 = object.key("NotDeviceTypes").start_array();
        for item_213 in var_211 {
            {
                array_212.value().string(item_213);
            }
        }
        array_212.finish();
    }
    if let Some(var_214) = &input.device_models {
        let mut array_215 = object.key("DeviceModels").start_array();
        for item_216 in var_214 {
            {
                array_215.value().string(item_216);
            }
        }
        array_215.finish();
    }
    if let Some(var_217) = &input.not_device_models {
        let mut array_218 = object.key("NotDeviceModels").start_array();
        for item_219 in var_217 {
            {
                array_218.value().string(item_219);
            }
        }
        array_218.finish();
    }
    if let Some(var_220) = &input.device_operating_systems {
        let mut array_221 = object.key("DeviceOperatingSystems").start_array();
        for item_222 in var_220 {
            {
                array_221.value().string(item_222);
            }
        }
        array_221.finish();
    }
    if let Some(var_223) = &input.not_device_operating_systems {
        let mut array_224 = object.key("NotDeviceOperatingSystems").start_array();
        for item_225 in var_223 {
            {
                array_224.value().string(item_225);
            }
        }
        array_224.finish();
    }
    if let Some(var_226) = &input.device_user_agents {
        let mut array_227 = object.key("DeviceUserAgents").start_array();
        for item_228 in var_226 {
            {
                array_227.value().string(item_228);
            }
        }
        array_227.finish();
    }
    if let Some(var_229) = &input.not_device_user_agents {
        let mut array_230 = object.key("NotDeviceUserAgents").start_array();
        for item_231 in var_229 {
            {
                array_230.value().string(item_231);
            }
        }
        array_230.finish();
    }
}

pub fn serialize_structure_update_primary_email_address_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePrimaryEmailAddressInput,
) {
    if let Some(var_232) = &input.organization_id {
        object.key("OrganizationId").string(var_232);
    }
    if let Some(var_233) = &input.entity_id {
        object.key("EntityId").string(var_233);
    }
    if let Some(var_234) = &input.email {
        object.key("Email").string(var_234);
    }
}

pub fn serialize_structure_update_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResourceInput,
) {
    if let Some(var_235) = &input.organization_id {
        object.key("OrganizationId").string(var_235);
    }
    if let Some(var_236) = &input.resource_id {
        object.key("ResourceId").string(var_236);
    }
    if let Some(var_237) = &input.name {
        object.key("Name").string(var_237);
    }
    if let Some(var_238) = &input.booking_options {
        let mut object_239 = object.key("BookingOptions").start_object();
        crate::json_ser::serialize_structure_booking_options(&mut object_239, var_238);
        object_239.finish();
    }
}

pub fn serialize_structure_domain(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Domain,
) {
    if let Some(var_240) = &input.domain_name {
        object.key("DomainName").string(var_240);
    }
    if let Some(var_241) = &input.hosted_zone_id {
        object.key("HostedZoneId").string(var_241);
    }
}

pub fn serialize_structure_folder_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FolderConfiguration,
) {
    if let Some(var_242) = &input.name {
        object.key("Name").string(var_242.as_str());
    }
    if let Some(var_243) = &input.action {
        object.key("Action").string(var_243.as_str());
    }
    if let Some(var_244) = &input.period {
        object.key("Period").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_244).into()),
        );
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_245) = &input.key {
        object.key("Key").string(var_245);
    }
    if let Some(var_246) = &input.value {
        object.key("Value").string(var_246);
    }
}

pub fn serialize_structure_booking_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BookingOptions,
) {
    if input.auto_accept_requests {
        object
            .key("AutoAcceptRequests")
            .boolean(input.auto_accept_requests);
    }
    if input.auto_decline_recurring_requests {
        object
            .key("AutoDeclineRecurringRequests")
            .boolean(input.auto_decline_recurring_requests);
    }
    if input.auto_decline_conflicting_requests {
        object
            .key("AutoDeclineConflictingRequests")
            .boolean(input.auto_decline_conflicting_requests);
    }
}
