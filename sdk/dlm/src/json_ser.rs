// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_lifecycle_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLifecyclePolicyInput,
) {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1);
    }
    if let Some(var_2) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_2);
    }
    if let Some(var_3) = &input.policy_details {
        let mut object_4 = object.key("PolicyDetails").start_object();
        crate::json_ser::serialize_structure_policy_details(&mut object_4, var_3);
        object_4.finish();
    }
    if let Some(var_5) = &input.state {
        object.key("State").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9);
            }
        }
        object_7.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("Tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
}

pub fn serialize_structure_update_lifecycle_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLifecyclePolicyInput,
) {
    if let Some(var_14) = &input.description {
        object.key("Description").string(var_14);
    }
    if let Some(var_15) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_15);
    }
    if let Some(var_16) = &input.policy_details {
        let mut object_17 = object.key("PolicyDetails").start_object();
        crate::json_ser::serialize_structure_policy_details(&mut object_17, var_16);
        object_17.finish();
    }
    if let Some(var_18) = &input.state {
        object.key("State").string(var_18.as_str());
    }
}

pub fn serialize_structure_policy_details(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyDetails,
) {
    if let Some(var_19) = &input.policy_type {
        object.key("PolicyType").string(var_19.as_str());
    }
    if let Some(var_20) = &input.resource_types {
        let mut array_21 = object.key("ResourceTypes").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.resource_locations {
        let mut array_24 = object.key("ResourceLocations").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.target_tags {
        let mut array_27 = object.key("TargetTags").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_29, item_28);
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.schedules {
        let mut array_31 = object.key("Schedules").start_array();
        for item_32 in var_30 {
            {
                let mut object_33 = array_31.value().start_object();
                crate::json_ser::serialize_structure_schedule(&mut object_33, item_32);
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.parameters {
        let mut object_35 = object.key("Parameters").start_object();
        crate::json_ser::serialize_structure_parameters(&mut object_35, var_34);
        object_35.finish();
    }
    if let Some(var_36) = &input.event_source {
        let mut object_37 = object.key("EventSource").start_object();
        crate::json_ser::serialize_structure_event_source(&mut object_37, var_36);
        object_37.finish();
    }
    if let Some(var_38) = &input.actions {
        let mut array_39 = object.key("Actions").start_array();
        for item_40 in var_38 {
            {
                let mut object_41 = array_39.value().start_object();
                crate::json_ser::serialize_structure_action(&mut object_41, item_40);
                object_41.finish();
            }
        }
        array_39.finish();
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_42) = &input.key {
        object.key("Key").string(var_42);
    }
    if let Some(var_43) = &input.value {
        object.key("Value").string(var_43);
    }
}

pub fn serialize_structure_schedule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Schedule,
) {
    if let Some(var_44) = &input.name {
        object.key("Name").string(var_44);
    }
    if input.copy_tags {
        object.key("CopyTags").boolean(input.copy_tags);
    }
    if let Some(var_45) = &input.tags_to_add {
        let mut array_46 = object.key("TagsToAdd").start_array();
        for item_47 in var_45 {
            {
                let mut object_48 = array_46.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_48, item_47);
                object_48.finish();
            }
        }
        array_46.finish();
    }
    if let Some(var_49) = &input.variable_tags {
        let mut array_50 = object.key("VariableTags").start_array();
        for item_51 in var_49 {
            {
                let mut object_52 = array_50.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_52, item_51);
                object_52.finish();
            }
        }
        array_50.finish();
    }
    if let Some(var_53) = &input.create_rule {
        let mut object_54 = object.key("CreateRule").start_object();
        crate::json_ser::serialize_structure_create_rule(&mut object_54, var_53);
        object_54.finish();
    }
    if let Some(var_55) = &input.retain_rule {
        let mut object_56 = object.key("RetainRule").start_object();
        crate::json_ser::serialize_structure_retain_rule(&mut object_56, var_55);
        object_56.finish();
    }
    if let Some(var_57) = &input.fast_restore_rule {
        let mut object_58 = object.key("FastRestoreRule").start_object();
        crate::json_ser::serialize_structure_fast_restore_rule(&mut object_58, var_57);
        object_58.finish();
    }
    if let Some(var_59) = &input.cross_region_copy_rules {
        let mut array_60 = object.key("CrossRegionCopyRules").start_array();
        for item_61 in var_59 {
            {
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_cross_region_copy_rule(
                    &mut object_62,
                    item_61,
                );
                object_62.finish();
            }
        }
        array_60.finish();
    }
    if let Some(var_63) = &input.share_rules {
        let mut array_64 = object.key("ShareRules").start_array();
        for item_65 in var_63 {
            {
                let mut object_66 = array_64.value().start_object();
                crate::json_ser::serialize_structure_share_rule(&mut object_66, item_65);
                object_66.finish();
            }
        }
        array_64.finish();
    }
    if let Some(var_67) = &input.deprecate_rule {
        let mut object_68 = object.key("DeprecateRule").start_object();
        crate::json_ser::serialize_structure_deprecate_rule(&mut object_68, var_67);
        object_68.finish();
    }
}

pub fn serialize_structure_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Parameters,
) {
    if let Some(var_69) = &input.exclude_boot_volume {
        object.key("ExcludeBootVolume").boolean(*var_69);
    }
    if let Some(var_70) = &input.no_reboot {
        object.key("NoReboot").boolean(*var_70);
    }
}

pub fn serialize_structure_event_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventSource,
) {
    if let Some(var_71) = &input.r#type {
        object.key("Type").string(var_71.as_str());
    }
    if let Some(var_72) = &input.parameters {
        let mut object_73 = object.key("Parameters").start_object();
        crate::json_ser::serialize_structure_event_parameters(&mut object_73, var_72);
        object_73.finish();
    }
}

pub fn serialize_structure_action(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Action,
) {
    if let Some(var_74) = &input.name {
        object.key("Name").string(var_74);
    }
    if let Some(var_75) = &input.cross_region_copy {
        let mut array_76 = object.key("CrossRegionCopy").start_array();
        for item_77 in var_75 {
            {
                let mut object_78 = array_76.value().start_object();
                crate::json_ser::serialize_structure_cross_region_copy_action(
                    &mut object_78,
                    item_77,
                );
                object_78.finish();
            }
        }
        array_76.finish();
    }
}

pub fn serialize_structure_create_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateRule,
) {
    if let Some(var_79) = &input.location {
        object.key("Location").string(var_79.as_str());
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_80) = &input.interval_unit {
        object.key("IntervalUnit").string(var_80.as_str());
    }
    if let Some(var_81) = &input.times {
        let mut array_82 = object.key("Times").start_array();
        for item_83 in var_81 {
            {
                array_82.value().string(item_83);
            }
        }
        array_82.finish();
    }
    if let Some(var_84) = &input.cron_expression {
        object.key("CronExpression").string(var_84);
    }
}

pub fn serialize_structure_retain_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RetainRule,
) {
    if input.count != 0 {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.count).into()),
        );
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_85) = &input.interval_unit {
        object.key("IntervalUnit").string(var_85.as_str());
    }
}

pub fn serialize_structure_fast_restore_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FastRestoreRule,
) {
    if input.count != 0 {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.count).into()),
        );
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_86) = &input.interval_unit {
        object.key("IntervalUnit").string(var_86.as_str());
    }
    if let Some(var_87) = &input.availability_zones {
        let mut array_88 = object.key("AvailabilityZones").start_array();
        for item_89 in var_87 {
            {
                array_88.value().string(item_89);
            }
        }
        array_88.finish();
    }
}

pub fn serialize_structure_cross_region_copy_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyRule,
) {
    if let Some(var_90) = &input.target_region {
        object.key("TargetRegion").string(var_90);
    }
    if let Some(var_91) = &input.target {
        object.key("Target").string(var_91);
    }
    if let Some(var_92) = &input.encrypted {
        object.key("Encrypted").boolean(*var_92);
    }
    if let Some(var_93) = &input.cmk_arn {
        object.key("CmkArn").string(var_93);
    }
    if let Some(var_94) = &input.copy_tags {
        object.key("CopyTags").boolean(*var_94);
    }
    if let Some(var_95) = &input.retain_rule {
        let mut object_96 = object.key("RetainRule").start_object();
        crate::json_ser::serialize_structure_cross_region_copy_retain_rule(&mut object_96, var_95);
        object_96.finish();
    }
    if let Some(var_97) = &input.deprecate_rule {
        let mut object_98 = object.key("DeprecateRule").start_object();
        crate::json_ser::serialize_structure_cross_region_copy_deprecate_rule(
            &mut object_98,
            var_97,
        );
        object_98.finish();
    }
}

pub fn serialize_structure_share_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ShareRule,
) {
    if let Some(var_99) = &input.target_accounts {
        let mut array_100 = object.key("TargetAccounts").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101);
            }
        }
        array_100.finish();
    }
    if input.unshare_interval != 0 {
        object.key("UnshareInterval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.unshare_interval).into()),
        );
    }
    if let Some(var_102) = &input.unshare_interval_unit {
        object.key("UnshareIntervalUnit").string(var_102.as_str());
    }
}

pub fn serialize_structure_deprecate_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeprecateRule,
) {
    if input.count != 0 {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.count).into()),
        );
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_103) = &input.interval_unit {
        object.key("IntervalUnit").string(var_103.as_str());
    }
}

pub fn serialize_structure_event_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventParameters,
) {
    if let Some(var_104) = &input.event_type {
        object.key("EventType").string(var_104.as_str());
    }
    if let Some(var_105) = &input.snapshot_owner {
        let mut array_106 = object.key("SnapshotOwner").start_array();
        for item_107 in var_105 {
            {
                array_106.value().string(item_107);
            }
        }
        array_106.finish();
    }
    if let Some(var_108) = &input.description_regex {
        object.key("DescriptionRegex").string(var_108);
    }
}

pub fn serialize_structure_cross_region_copy_action(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyAction,
) {
    if let Some(var_109) = &input.target {
        object.key("Target").string(var_109);
    }
    if let Some(var_110) = &input.encryption_configuration {
        let mut object_111 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_encryption_configuration(&mut object_111, var_110);
        object_111.finish();
    }
    if let Some(var_112) = &input.retain_rule {
        let mut object_113 = object.key("RetainRule").start_object();
        crate::json_ser::serialize_structure_cross_region_copy_retain_rule(
            &mut object_113,
            var_112,
        );
        object_113.finish();
    }
}

pub fn serialize_structure_cross_region_copy_retain_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyRetainRule,
) {
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_114) = &input.interval_unit {
        object.key("IntervalUnit").string(var_114.as_str());
    }
}

pub fn serialize_structure_cross_region_copy_deprecate_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyDeprecateRule,
) {
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_115) = &input.interval_unit {
        object.key("IntervalUnit").string(var_115.as_str());
    }
}

pub fn serialize_structure_encryption_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionConfiguration,
) {
    if let Some(var_116) = &input.encrypted {
        object.key("Encrypted").boolean(*var_116);
    }
    if let Some(var_117) = &input.cmk_arn {
        object.key("CmkArn").string(var_117);
    }
}
