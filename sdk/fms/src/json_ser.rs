// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_associate_admin_account_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateAdminAccountInput,
) {
    if let Some(var_1) = &input.admin_account {
        object.key("AdminAccount").string(var_1);
    }
}

pub fn serialize_structure_delete_apps_list_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAppsListInput,
) {
    if let Some(var_2) = &input.list_id {
        object.key("ListId").string(var_2);
    }
}

pub fn serialize_structure_delete_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePolicyInput,
) {
    if let Some(var_3) = &input.policy_id {
        object.key("PolicyId").string(var_3);
    }
    if input.delete_all_policy_resources {
        object
            .key("DeleteAllPolicyResources")
            .boolean(input.delete_all_policy_resources);
    }
}

pub fn serialize_structure_delete_protocols_list_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteProtocolsListInput,
) {
    if let Some(var_4) = &input.list_id {
        object.key("ListId").string(var_4);
    }
}

pub fn serialize_structure_get_apps_list_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAppsListInput,
) {
    if let Some(var_5) = &input.list_id {
        object.key("ListId").string(var_5);
    }
    if input.default_list {
        object.key("DefaultList").boolean(input.default_list);
    }
}

pub fn serialize_structure_get_compliance_detail_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetComplianceDetailInput,
) {
    if let Some(var_6) = &input.policy_id {
        object.key("PolicyId").string(var_6);
    }
    if let Some(var_7) = &input.member_account {
        object.key("MemberAccount").string(var_7);
    }
}

pub fn serialize_structure_get_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPolicyInput,
) {
    if let Some(var_8) = &input.policy_id {
        object.key("PolicyId").string(var_8);
    }
}

pub fn serialize_structure_get_protection_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetProtectionStatusInput,
) {
    if let Some(var_9) = &input.policy_id {
        object.key("PolicyId").string(var_9);
    }
    if let Some(var_10) = &input.member_account_id {
        object.key("MemberAccountId").string(var_10);
    }
    if let Some(var_11) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_11, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_12) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_12, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_13) = &input.next_token {
        object.key("NextToken").string(var_13);
    }
    if let Some(var_14) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
}

pub fn serialize_structure_get_protocols_list_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetProtocolsListInput,
) {
    if let Some(var_15) = &input.list_id {
        object.key("ListId").string(var_15);
    }
    if input.default_list {
        object.key("DefaultList").boolean(input.default_list);
    }
}

pub fn serialize_structure_get_violation_details_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetViolationDetailsInput,
) {
    if let Some(var_16) = &input.policy_id {
        object.key("PolicyId").string(var_16);
    }
    if let Some(var_17) = &input.member_account {
        object.key("MemberAccount").string(var_17);
    }
    if let Some(var_18) = &input.resource_id {
        object.key("ResourceId").string(var_18);
    }
    if let Some(var_19) = &input.resource_type {
        object.key("ResourceType").string(var_19);
    }
}

pub fn serialize_structure_list_apps_lists_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppsListsInput,
) {
    if input.default_lists {
        object.key("DefaultLists").boolean(input.default_lists);
    }
    if let Some(var_20) = &input.next_token {
        object.key("NextToken").string(var_20);
    }
    if let Some(var_21) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_21).into()),
        );
    }
}

pub fn serialize_structure_list_compliance_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListComplianceStatusInput,
) {
    if let Some(var_22) = &input.policy_id {
        object.key("PolicyId").string(var_22);
    }
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23);
    }
    if let Some(var_24) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_24).into()),
        );
    }
}

pub fn serialize_structure_list_member_accounts_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMemberAccountsInput,
) {
    if let Some(var_25) = &input.next_token {
        object.key("NextToken").string(var_25);
    }
    if let Some(var_26) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_26).into()),
        );
    }
}

pub fn serialize_structure_list_policies_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPoliciesInput,
) {
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27);
    }
    if let Some(var_28) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_28).into()),
        );
    }
}

pub fn serialize_structure_list_protocols_lists_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListProtocolsListsInput,
) {
    if input.default_lists {
        object.key("DefaultLists").boolean(input.default_lists);
    }
    if let Some(var_29) = &input.next_token {
        object.key("NextToken").string(var_29);
    }
    if let Some(var_30) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_30).into()),
        );
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_31) = &input.resource_arn {
        object.key("ResourceArn").string(var_31);
    }
}

pub fn serialize_structure_put_apps_list_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAppsListInput,
) {
    if let Some(var_32) = &input.apps_list {
        let mut object_33 = object.key("AppsList").start_object();
        crate::json_ser::serialize_structure_apps_list_data(&mut object_33, var_32);
        object_33.finish();
    }
    if let Some(var_34) = &input.tag_list {
        let mut array_35 = object.key("TagList").start_array();
        for item_36 in var_34 {
            {
                let mut object_37 = array_35.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_37, item_36);
                object_37.finish();
            }
        }
        array_35.finish();
    }
}

pub fn serialize_structure_put_notification_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutNotificationChannelInput,
) {
    if let Some(var_38) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_38);
    }
    if let Some(var_39) = &input.sns_role_name {
        object.key("SnsRoleName").string(var_39);
    }
}

pub fn serialize_structure_put_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPolicyInput,
) {
    if let Some(var_40) = &input.policy {
        let mut object_41 = object.key("Policy").start_object();
        crate::json_ser::serialize_structure_policy(&mut object_41, var_40);
        object_41.finish();
    }
    if let Some(var_42) = &input.tag_list {
        let mut array_43 = object.key("TagList").start_array();
        for item_44 in var_42 {
            {
                let mut object_45 = array_43.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_45, item_44);
                object_45.finish();
            }
        }
        array_43.finish();
    }
}

pub fn serialize_structure_put_protocols_list_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutProtocolsListInput,
) {
    if let Some(var_46) = &input.protocols_list {
        let mut object_47 = object.key("ProtocolsList").start_object();
        crate::json_ser::serialize_structure_protocols_list_data(&mut object_47, var_46);
        object_47.finish();
    }
    if let Some(var_48) = &input.tag_list {
        let mut array_49 = object.key("TagList").start_array();
        for item_50 in var_48 {
            {
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_51, item_50);
                object_51.finish();
            }
        }
        array_49.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_52) = &input.resource_arn {
        object.key("ResourceArn").string(var_52);
    }
    if let Some(var_53) = &input.tag_list {
        let mut array_54 = object.key("TagList").start_array();
        for item_55 in var_53 {
            {
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_56, item_55);
                object_56.finish();
            }
        }
        array_54.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_57) = &input.resource_arn {
        object.key("ResourceArn").string(var_57);
    }
    if let Some(var_58) = &input.tag_keys {
        let mut array_59 = object.key("TagKeys").start_array();
        for item_60 in var_58 {
            {
                array_59.value().string(item_60);
            }
        }
        array_59.finish();
    }
}

pub fn serialize_structure_apps_list_data(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AppsListData,
) {
    if let Some(var_61) = &input.list_id {
        object.key("ListId").string(var_61);
    }
    if let Some(var_62) = &input.list_name {
        object.key("ListName").string(var_62);
    }
    if let Some(var_63) = &input.list_update_token {
        object.key("ListUpdateToken").string(var_63);
    }
    if let Some(var_64) = &input.create_time {
        object
            .key("CreateTime")
            .instant(var_64, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_65) = &input.last_update_time {
        object
            .key("LastUpdateTime")
            .instant(var_65, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_66) = &input.apps_list {
        let mut array_67 = object.key("AppsList").start_array();
        for item_68 in var_66 {
            {
                let mut object_69 = array_67.value().start_object();
                crate::json_ser::serialize_structure_app(&mut object_69, item_68);
                object_69.finish();
            }
        }
        array_67.finish();
    }
    if let Some(var_70) = &input.previous_apps_list {
        let mut object_71 = object.key("PreviousAppsList").start_object();
        for (key_72, value_73) in var_70 {
            {
                let mut array_74 = object_71.key(key_72).start_array();
                for item_75 in value_73 {
                    {
                        let mut object_76 = array_74.value().start_object();
                        crate::json_ser::serialize_structure_app(&mut object_76, item_75);
                        object_76.finish();
                    }
                }
                array_74.finish();
            }
        }
        object_71.finish();
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_77) = &input.key {
        object.key("Key").string(var_77);
    }
    if let Some(var_78) = &input.value {
        object.key("Value").string(var_78);
    }
}

pub fn serialize_structure_policy(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Policy,
) {
    if let Some(var_79) = &input.policy_id {
        object.key("PolicyId").string(var_79);
    }
    if let Some(var_80) = &input.policy_name {
        object.key("PolicyName").string(var_80);
    }
    if let Some(var_81) = &input.policy_update_token {
        object.key("PolicyUpdateToken").string(var_81);
    }
    if let Some(var_82) = &input.security_service_policy_data {
        let mut object_83 = object.key("SecurityServicePolicyData").start_object();
        crate::json_ser::serialize_structure_security_service_policy_data(&mut object_83, var_82);
        object_83.finish();
    }
    if let Some(var_84) = &input.resource_type {
        object.key("ResourceType").string(var_84);
    }
    if let Some(var_85) = &input.resource_type_list {
        let mut array_86 = object.key("ResourceTypeList").start_array();
        for item_87 in var_85 {
            {
                array_86.value().string(item_87);
            }
        }
        array_86.finish();
    }
    if let Some(var_88) = &input.resource_tags {
        let mut array_89 = object.key("ResourceTags").start_array();
        for item_90 in var_88 {
            {
                let mut object_91 = array_89.value().start_object();
                crate::json_ser::serialize_structure_resource_tag(&mut object_91, item_90);
                object_91.finish();
            }
        }
        array_89.finish();
    }
    {
        object
            .key("ExcludeResourceTags")
            .boolean(input.exclude_resource_tags);
    }
    {
        object
            .key("RemediationEnabled")
            .boolean(input.remediation_enabled);
    }
    if input.delete_unused_fm_managed_resources {
        object
            .key("DeleteUnusedFMManagedResources")
            .boolean(input.delete_unused_fm_managed_resources);
    }
    if let Some(var_92) = &input.include_map {
        let mut object_93 = object.key("IncludeMap").start_object();
        for (key_94, value_95) in var_92 {
            {
                let mut array_96 = object_93.key(key_94.as_str()).start_array();
                for item_97 in value_95 {
                    {
                        array_96.value().string(item_97);
                    }
                }
                array_96.finish();
            }
        }
        object_93.finish();
    }
    if let Some(var_98) = &input.exclude_map {
        let mut object_99 = object.key("ExcludeMap").start_object();
        for (key_100, value_101) in var_98 {
            {
                let mut array_102 = object_99.key(key_100.as_str()).start_array();
                for item_103 in value_101 {
                    {
                        array_102.value().string(item_103);
                    }
                }
                array_102.finish();
            }
        }
        object_99.finish();
    }
}

pub fn serialize_structure_protocols_list_data(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ProtocolsListData,
) {
    if let Some(var_104) = &input.list_id {
        object.key("ListId").string(var_104);
    }
    if let Some(var_105) = &input.list_name {
        object.key("ListName").string(var_105);
    }
    if let Some(var_106) = &input.list_update_token {
        object.key("ListUpdateToken").string(var_106);
    }
    if let Some(var_107) = &input.create_time {
        object
            .key("CreateTime")
            .instant(var_107, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_108) = &input.last_update_time {
        object
            .key("LastUpdateTime")
            .instant(var_108, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_109) = &input.protocols_list {
        let mut array_110 = object.key("ProtocolsList").start_array();
        for item_111 in var_109 {
            {
                array_110.value().string(item_111);
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.previous_protocols_list {
        let mut object_113 = object.key("PreviousProtocolsList").start_object();
        for (key_114, value_115) in var_112 {
            {
                let mut array_116 = object_113.key(key_114).start_array();
                for item_117 in value_115 {
                    {
                        array_116.value().string(item_117);
                    }
                }
                array_116.finish();
            }
        }
        object_113.finish();
    }
}

pub fn serialize_structure_app(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::App,
) {
    if let Some(var_118) = &input.app_name {
        object.key("AppName").string(var_118);
    }
    if let Some(var_119) = &input.protocol {
        object.key("Protocol").string(var_119);
    }
    if let Some(var_120) = &input.port {
        object.key("Port").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_120).into()),
        );
    }
}

pub fn serialize_structure_security_service_policy_data(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SecurityServicePolicyData,
) {
    if let Some(var_121) = &input.r#type {
        object.key("Type").string(var_121.as_str());
    }
    if let Some(var_122) = &input.managed_service_data {
        object.key("ManagedServiceData").string(var_122);
    }
}

pub fn serialize_structure_resource_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourceTag,
) {
    if let Some(var_123) = &input.key {
        object.key("Key").string(var_123);
    }
    if let Some(var_124) = &input.value {
        object.key("Value").string(var_124);
    }
}
