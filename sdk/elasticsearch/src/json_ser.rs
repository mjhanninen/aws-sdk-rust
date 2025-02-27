// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_add_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsInput,
) {
    if let Some(var_1) = &input.arn {
        object.key("ARN").string(var_1);
    }
    if let Some(var_2) = &input.tag_list {
        let mut array_3 = object.key("TagList").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_5, item_4);
                object_5.finish();
            }
        }
        array_3.finish();
    }
}

pub fn serialize_structure_cancel_elasticsearch_service_software_update_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelElasticsearchServiceSoftwareUpdateInput,
) {
    if let Some(var_6) = &input.domain_name {
        object.key("DomainName").string(var_6);
    }
}

pub fn serialize_structure_create_elasticsearch_domain_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateElasticsearchDomainInput,
) {
    if let Some(var_7) = &input.access_policies {
        object.key("AccessPolicies").string(var_7);
    }
    if let Some(var_8) = &input.advanced_options {
        let mut object_9 = object.key("AdvancedOptions").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10).string(value_11);
            }
        }
        object_9.finish();
    }
    if let Some(var_12) = &input.advanced_security_options {
        let mut object_13 = object.key("AdvancedSecurityOptions").start_object();
        crate::json_ser::serialize_structure_advanced_security_options_input(
            &mut object_13,
            var_12,
        );
        object_13.finish();
    }
    if let Some(var_14) = &input.auto_tune_options {
        let mut object_15 = object.key("AutoTuneOptions").start_object();
        crate::json_ser::serialize_structure_auto_tune_options_input(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.cognito_options {
        let mut object_17 = object.key("CognitoOptions").start_object();
        crate::json_ser::serialize_structure_cognito_options(&mut object_17, var_16);
        object_17.finish();
    }
    if let Some(var_18) = &input.domain_endpoint_options {
        let mut object_19 = object.key("DomainEndpointOptions").start_object();
        crate::json_ser::serialize_structure_domain_endpoint_options(&mut object_19, var_18);
        object_19.finish();
    }
    if let Some(var_20) = &input.domain_name {
        object.key("DomainName").string(var_20);
    }
    if let Some(var_21) = &input.ebs_options {
        let mut object_22 = object.key("EBSOptions").start_object();
        crate::json_ser::serialize_structure_ebs_options(&mut object_22, var_21);
        object_22.finish();
    }
    if let Some(var_23) = &input.elasticsearch_cluster_config {
        let mut object_24 = object.key("ElasticsearchClusterConfig").start_object();
        crate::json_ser::serialize_structure_elasticsearch_cluster_config(&mut object_24, var_23);
        object_24.finish();
    }
    if let Some(var_25) = &input.elasticsearch_version {
        object.key("ElasticsearchVersion").string(var_25);
    }
    if let Some(var_26) = &input.encryption_at_rest_options {
        let mut object_27 = object.key("EncryptionAtRestOptions").start_object();
        crate::json_ser::serialize_structure_encryption_at_rest_options(&mut object_27, var_26);
        object_27.finish();
    }
    if let Some(var_28) = &input.log_publishing_options {
        let mut object_29 = object.key("LogPublishingOptions").start_object();
        for (key_30, value_31) in var_28 {
            {
                let mut object_32 = object_29.key(key_30.as_str()).start_object();
                crate::json_ser::serialize_structure_log_publishing_option(
                    &mut object_32,
                    value_31,
                );
                object_32.finish();
            }
        }
        object_29.finish();
    }
    if let Some(var_33) = &input.node_to_node_encryption_options {
        let mut object_34 = object.key("NodeToNodeEncryptionOptions").start_object();
        crate::json_ser::serialize_structure_node_to_node_encryption_options(
            &mut object_34,
            var_33,
        );
        object_34.finish();
    }
    if let Some(var_35) = &input.snapshot_options {
        let mut object_36 = object.key("SnapshotOptions").start_object();
        crate::json_ser::serialize_structure_snapshot_options(&mut object_36, var_35);
        object_36.finish();
    }
    if let Some(var_37) = &input.tag_list {
        let mut array_38 = object.key("TagList").start_array();
        for item_39 in var_37 {
            {
                let mut object_40 = array_38.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_40, item_39);
                object_40.finish();
            }
        }
        array_38.finish();
    }
    if let Some(var_41) = &input.vpc_options {
        let mut object_42 = object.key("VPCOptions").start_object();
        crate::json_ser::serialize_structure_vpc_options(&mut object_42, var_41);
        object_42.finish();
    }
}

pub fn serialize_structure_create_outbound_cross_cluster_search_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateOutboundCrossClusterSearchConnectionInput,
) {
    if let Some(var_43) = &input.connection_alias {
        object.key("ConnectionAlias").string(var_43);
    }
    if let Some(var_44) = &input.destination_domain_info {
        let mut object_45 = object.key("DestinationDomainInfo").start_object();
        crate::json_ser::serialize_structure_domain_information(&mut object_45, var_44);
        object_45.finish();
    }
    if let Some(var_46) = &input.source_domain_info {
        let mut object_47 = object.key("SourceDomainInfo").start_object();
        crate::json_ser::serialize_structure_domain_information(&mut object_47, var_46);
        object_47.finish();
    }
}

pub fn serialize_structure_create_package_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePackageInput,
) {
    if let Some(var_48) = &input.package_description {
        object.key("PackageDescription").string(var_48);
    }
    if let Some(var_49) = &input.package_name {
        object.key("PackageName").string(var_49);
    }
    if let Some(var_50) = &input.package_source {
        let mut object_51 = object.key("PackageSource").start_object();
        crate::json_ser::serialize_structure_package_source(&mut object_51, var_50);
        object_51.finish();
    }
    if let Some(var_52) = &input.package_type {
        object.key("PackageType").string(var_52.as_str());
    }
}

pub fn serialize_structure_describe_domain_auto_tunes_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDomainAutoTunesInput,
) {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_53) = &input.next_token {
        object.key("NextToken").string(var_53);
    }
}

pub fn serialize_structure_describe_elasticsearch_domains_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeElasticsearchDomainsInput,
) {
    if let Some(var_54) = &input.domain_names {
        let mut array_55 = object.key("DomainNames").start_array();
        for item_56 in var_54 {
            {
                array_55.value().string(item_56);
            }
        }
        array_55.finish();
    }
}

pub fn serialize_structure_describe_inbound_cross_cluster_search_connections_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeInboundCrossClusterSearchConnectionsInput,
) {
    if let Some(var_57) = &input.filters {
        let mut array_58 = object.key("Filters").start_array();
        for item_59 in var_57 {
            {
                let mut object_60 = array_58.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_60, item_59);
                object_60.finish();
            }
        }
        array_58.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61);
    }
}

pub fn serialize_structure_describe_outbound_cross_cluster_search_connections_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeOutboundCrossClusterSearchConnectionsInput,
) {
    if let Some(var_62) = &input.filters {
        let mut array_63 = object.key("Filters").start_array();
        for item_64 in var_62 {
            {
                let mut object_65 = array_63.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_65, item_64);
                object_65.finish();
            }
        }
        array_63.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_66) = &input.next_token {
        object.key("NextToken").string(var_66);
    }
}

pub fn serialize_structure_describe_packages_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribePackagesInput,
) {
    if let Some(var_67) = &input.filters {
        let mut array_68 = object.key("Filters").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_describe_packages_filter(
                    &mut object_70,
                    item_69,
                );
                object_70.finish();
            }
        }
        array_68.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_71) = &input.next_token {
        object.key("NextToken").string(var_71);
    }
}

pub fn serialize_structure_purchase_reserved_elasticsearch_instance_offering_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PurchaseReservedElasticsearchInstanceOfferingInput,
) {
    if input.instance_count != 0 {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.instance_count).into()),
        );
    }
    if let Some(var_72) = &input.reservation_name {
        object.key("ReservationName").string(var_72);
    }
    if let Some(var_73) = &input.reserved_elasticsearch_instance_offering_id {
        object
            .key("ReservedElasticsearchInstanceOfferingId")
            .string(var_73);
    }
}

pub fn serialize_structure_remove_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsInput,
) {
    if let Some(var_74) = &input.arn {
        object.key("ARN").string(var_74);
    }
    if let Some(var_75) = &input.tag_keys {
        let mut array_76 = object.key("TagKeys").start_array();
        for item_77 in var_75 {
            {
                array_76.value().string(item_77);
            }
        }
        array_76.finish();
    }
}

pub fn serialize_structure_start_elasticsearch_service_software_update_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartElasticsearchServiceSoftwareUpdateInput,
) {
    if let Some(var_78) = &input.domain_name {
        object.key("DomainName").string(var_78);
    }
}

pub fn serialize_structure_update_elasticsearch_domain_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateElasticsearchDomainConfigInput,
) {
    if let Some(var_79) = &input.access_policies {
        object.key("AccessPolicies").string(var_79);
    }
    if let Some(var_80) = &input.advanced_options {
        let mut object_81 = object.key("AdvancedOptions").start_object();
        for (key_82, value_83) in var_80 {
            {
                object_81.key(key_82).string(value_83);
            }
        }
        object_81.finish();
    }
    if let Some(var_84) = &input.advanced_security_options {
        let mut object_85 = object.key("AdvancedSecurityOptions").start_object();
        crate::json_ser::serialize_structure_advanced_security_options_input(
            &mut object_85,
            var_84,
        );
        object_85.finish();
    }
    if let Some(var_86) = &input.auto_tune_options {
        let mut object_87 = object.key("AutoTuneOptions").start_object();
        crate::json_ser::serialize_structure_auto_tune_options(&mut object_87, var_86);
        object_87.finish();
    }
    if let Some(var_88) = &input.cognito_options {
        let mut object_89 = object.key("CognitoOptions").start_object();
        crate::json_ser::serialize_structure_cognito_options(&mut object_89, var_88);
        object_89.finish();
    }
    if let Some(var_90) = &input.domain_endpoint_options {
        let mut object_91 = object.key("DomainEndpointOptions").start_object();
        crate::json_ser::serialize_structure_domain_endpoint_options(&mut object_91, var_90);
        object_91.finish();
    }
    if let Some(var_92) = &input.ebs_options {
        let mut object_93 = object.key("EBSOptions").start_object();
        crate::json_ser::serialize_structure_ebs_options(&mut object_93, var_92);
        object_93.finish();
    }
    if let Some(var_94) = &input.elasticsearch_cluster_config {
        let mut object_95 = object.key("ElasticsearchClusterConfig").start_object();
        crate::json_ser::serialize_structure_elasticsearch_cluster_config(&mut object_95, var_94);
        object_95.finish();
    }
    if let Some(var_96) = &input.encryption_at_rest_options {
        let mut object_97 = object.key("EncryptionAtRestOptions").start_object();
        crate::json_ser::serialize_structure_encryption_at_rest_options(&mut object_97, var_96);
        object_97.finish();
    }
    if let Some(var_98) = &input.log_publishing_options {
        let mut object_99 = object.key("LogPublishingOptions").start_object();
        for (key_100, value_101) in var_98 {
            {
                let mut object_102 = object_99.key(key_100.as_str()).start_object();
                crate::json_ser::serialize_structure_log_publishing_option(
                    &mut object_102,
                    value_101,
                );
                object_102.finish();
            }
        }
        object_99.finish();
    }
    if let Some(var_103) = &input.node_to_node_encryption_options {
        let mut object_104 = object.key("NodeToNodeEncryptionOptions").start_object();
        crate::json_ser::serialize_structure_node_to_node_encryption_options(
            &mut object_104,
            var_103,
        );
        object_104.finish();
    }
    if let Some(var_105) = &input.snapshot_options {
        let mut object_106 = object.key("SnapshotOptions").start_object();
        crate::json_ser::serialize_structure_snapshot_options(&mut object_106, var_105);
        object_106.finish();
    }
    if let Some(var_107) = &input.vpc_options {
        let mut object_108 = object.key("VPCOptions").start_object();
        crate::json_ser::serialize_structure_vpc_options(&mut object_108, var_107);
        object_108.finish();
    }
}

pub fn serialize_structure_update_package_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePackageInput,
) {
    if let Some(var_109) = &input.commit_message {
        object.key("CommitMessage").string(var_109);
    }
    if let Some(var_110) = &input.package_description {
        object.key("PackageDescription").string(var_110);
    }
    if let Some(var_111) = &input.package_id {
        object.key("PackageID").string(var_111);
    }
    if let Some(var_112) = &input.package_source {
        let mut object_113 = object.key("PackageSource").start_object();
        crate::json_ser::serialize_structure_package_source(&mut object_113, var_112);
        object_113.finish();
    }
}

pub fn serialize_structure_upgrade_elasticsearch_domain_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpgradeElasticsearchDomainInput,
) {
    if let Some(var_114) = &input.domain_name {
        object.key("DomainName").string(var_114);
    }
    if let Some(var_115) = &input.perform_check_only {
        object.key("PerformCheckOnly").boolean(*var_115);
    }
    if let Some(var_116) = &input.target_version {
        object.key("TargetVersion").string(var_116);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_117) = &input.key {
        object.key("Key").string(var_117);
    }
    if let Some(var_118) = &input.value {
        object.key("Value").string(var_118);
    }
}

pub fn serialize_structure_advanced_security_options_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdvancedSecurityOptionsInput,
) {
    if let Some(var_119) = &input.enabled {
        object.key("Enabled").boolean(*var_119);
    }
    if let Some(var_120) = &input.internal_user_database_enabled {
        object.key("InternalUserDatabaseEnabled").boolean(*var_120);
    }
    if let Some(var_121) = &input.master_user_options {
        let mut object_122 = object.key("MasterUserOptions").start_object();
        crate::json_ser::serialize_structure_master_user_options(&mut object_122, var_121);
        object_122.finish();
    }
    if let Some(var_123) = &input.saml_options {
        let mut object_124 = object.key("SAMLOptions").start_object();
        crate::json_ser::serialize_structure_saml_options_input(&mut object_124, var_123);
        object_124.finish();
    }
}

pub fn serialize_structure_auto_tune_options_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoTuneOptionsInput,
) {
    if let Some(var_125) = &input.desired_state {
        object.key("DesiredState").string(var_125.as_str());
    }
    if let Some(var_126) = &input.maintenance_schedules {
        let mut array_127 = object.key("MaintenanceSchedules").start_array();
        for item_128 in var_126 {
            {
                let mut object_129 = array_127.value().start_object();
                crate::json_ser::serialize_structure_auto_tune_maintenance_schedule(
                    &mut object_129,
                    item_128,
                );
                object_129.finish();
            }
        }
        array_127.finish();
    }
}

pub fn serialize_structure_cognito_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CognitoOptions,
) {
    if let Some(var_130) = &input.enabled {
        object.key("Enabled").boolean(*var_130);
    }
    if let Some(var_131) = &input.user_pool_id {
        object.key("UserPoolId").string(var_131);
    }
    if let Some(var_132) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_132);
    }
    if let Some(var_133) = &input.role_arn {
        object.key("RoleArn").string(var_133);
    }
}

pub fn serialize_structure_domain_endpoint_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DomainEndpointOptions,
) {
    if let Some(var_134) = &input.enforce_https {
        object.key("EnforceHTTPS").boolean(*var_134);
    }
    if let Some(var_135) = &input.tls_security_policy {
        object.key("TLSSecurityPolicy").string(var_135.as_str());
    }
    if let Some(var_136) = &input.custom_endpoint_enabled {
        object.key("CustomEndpointEnabled").boolean(*var_136);
    }
    if let Some(var_137) = &input.custom_endpoint {
        object.key("CustomEndpoint").string(var_137);
    }
    if let Some(var_138) = &input.custom_endpoint_certificate_arn {
        object.key("CustomEndpointCertificateArn").string(var_138);
    }
}

pub fn serialize_structure_ebs_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EbsOptions,
) {
    if let Some(var_139) = &input.ebs_enabled {
        object.key("EBSEnabled").boolean(*var_139);
    }
    if let Some(var_140) = &input.volume_type {
        object.key("VolumeType").string(var_140.as_str());
    }
    if let Some(var_141) = &input.volume_size {
        object.key("VolumeSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_141).into()),
        );
    }
    if let Some(var_142) = &input.iops {
        object.key("Iops").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_142).into()),
        );
    }
}

pub fn serialize_structure_elasticsearch_cluster_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ElasticsearchClusterConfig,
) {
    if let Some(var_143) = &input.instance_type {
        object.key("InstanceType").string(var_143.as_str());
    }
    if let Some(var_144) = &input.instance_count {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_144).into()),
        );
    }
    if let Some(var_145) = &input.dedicated_master_enabled {
        object.key("DedicatedMasterEnabled").boolean(*var_145);
    }
    if let Some(var_146) = &input.zone_awareness_enabled {
        object.key("ZoneAwarenessEnabled").boolean(*var_146);
    }
    if let Some(var_147) = &input.zone_awareness_config {
        let mut object_148 = object.key("ZoneAwarenessConfig").start_object();
        crate::json_ser::serialize_structure_zone_awareness_config(&mut object_148, var_147);
        object_148.finish();
    }
    if let Some(var_149) = &input.dedicated_master_type {
        object.key("DedicatedMasterType").string(var_149.as_str());
    }
    if let Some(var_150) = &input.dedicated_master_count {
        object.key("DedicatedMasterCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_150).into()),
        );
    }
    if let Some(var_151) = &input.warm_enabled {
        object.key("WarmEnabled").boolean(*var_151);
    }
    if let Some(var_152) = &input.warm_type {
        object.key("WarmType").string(var_152.as_str());
    }
    if let Some(var_153) = &input.warm_count {
        object.key("WarmCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_153).into()),
        );
    }
    if let Some(var_154) = &input.cold_storage_options {
        let mut object_155 = object.key("ColdStorageOptions").start_object();
        crate::json_ser::serialize_structure_cold_storage_options(&mut object_155, var_154);
        object_155.finish();
    }
}

pub fn serialize_structure_encryption_at_rest_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionAtRestOptions,
) {
    if let Some(var_156) = &input.enabled {
        object.key("Enabled").boolean(*var_156);
    }
    if let Some(var_157) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_157);
    }
}

pub fn serialize_structure_log_publishing_option(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogPublishingOption,
) {
    if let Some(var_158) = &input.cloud_watch_logs_log_group_arn {
        object.key("CloudWatchLogsLogGroupArn").string(var_158);
    }
    if let Some(var_159) = &input.enabled {
        object.key("Enabled").boolean(*var_159);
    }
}

pub fn serialize_structure_node_to_node_encryption_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodeToNodeEncryptionOptions,
) {
    if let Some(var_160) = &input.enabled {
        object.key("Enabled").boolean(*var_160);
    }
}

pub fn serialize_structure_snapshot_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SnapshotOptions,
) {
    if let Some(var_161) = &input.automated_snapshot_start_hour {
        object.key("AutomatedSnapshotStartHour").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_161).into()),
        );
    }
}

pub fn serialize_structure_vpc_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcOptions,
) {
    if let Some(var_162) = &input.subnet_ids {
        let mut array_163 = object.key("SubnetIds").start_array();
        for item_164 in var_162 {
            {
                array_163.value().string(item_164);
            }
        }
        array_163.finish();
    }
    if let Some(var_165) = &input.security_group_ids {
        let mut array_166 = object.key("SecurityGroupIds").start_array();
        for item_167 in var_165 {
            {
                array_166.value().string(item_167);
            }
        }
        array_166.finish();
    }
}

pub fn serialize_structure_domain_information(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DomainInformation,
) {
    if let Some(var_168) = &input.owner_id {
        object.key("OwnerId").string(var_168);
    }
    if let Some(var_169) = &input.domain_name {
        object.key("DomainName").string(var_169);
    }
    if let Some(var_170) = &input.region {
        object.key("Region").string(var_170);
    }
}

pub fn serialize_structure_package_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PackageSource,
) {
    if let Some(var_171) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_171);
    }
    if let Some(var_172) = &input.s3_key {
        object.key("S3Key").string(var_172);
    }
}

pub fn serialize_structure_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) {
    if let Some(var_173) = &input.name {
        object.key("Name").string(var_173);
    }
    if let Some(var_174) = &input.values {
        let mut array_175 = object.key("Values").start_array();
        for item_176 in var_174 {
            {
                array_175.value().string(item_176);
            }
        }
        array_175.finish();
    }
}

pub fn serialize_structure_describe_packages_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DescribePackagesFilter,
) {
    if let Some(var_177) = &input.name {
        object.key("Name").string(var_177.as_str());
    }
    if let Some(var_178) = &input.value {
        let mut array_179 = object.key("Value").start_array();
        for item_180 in var_178 {
            {
                array_179.value().string(item_180);
            }
        }
        array_179.finish();
    }
}

pub fn serialize_structure_auto_tune_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoTuneOptions,
) {
    if let Some(var_181) = &input.desired_state {
        object.key("DesiredState").string(var_181.as_str());
    }
    if let Some(var_182) = &input.rollback_on_disable {
        object.key("RollbackOnDisable").string(var_182.as_str());
    }
    if let Some(var_183) = &input.maintenance_schedules {
        let mut array_184 = object.key("MaintenanceSchedules").start_array();
        for item_185 in var_183 {
            {
                let mut object_186 = array_184.value().start_object();
                crate::json_ser::serialize_structure_auto_tune_maintenance_schedule(
                    &mut object_186,
                    item_185,
                );
                object_186.finish();
            }
        }
        array_184.finish();
    }
}

pub fn serialize_structure_master_user_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MasterUserOptions,
) {
    if let Some(var_187) = &input.master_user_arn {
        object.key("MasterUserARN").string(var_187);
    }
    if let Some(var_188) = &input.master_user_name {
        object.key("MasterUserName").string(var_188);
    }
    if let Some(var_189) = &input.master_user_password {
        object.key("MasterUserPassword").string(var_189);
    }
}

pub fn serialize_structure_saml_options_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamlOptionsInput,
) {
    if let Some(var_190) = &input.enabled {
        object.key("Enabled").boolean(*var_190);
    }
    if let Some(var_191) = &input.idp {
        let mut object_192 = object.key("Idp").start_object();
        crate::json_ser::serialize_structure_saml_idp(&mut object_192, var_191);
        object_192.finish();
    }
    if let Some(var_193) = &input.master_user_name {
        object.key("MasterUserName").string(var_193);
    }
    if let Some(var_194) = &input.master_backend_role {
        object.key("MasterBackendRole").string(var_194);
    }
    if let Some(var_195) = &input.subject_key {
        object.key("SubjectKey").string(var_195);
    }
    if let Some(var_196) = &input.roles_key {
        object.key("RolesKey").string(var_196);
    }
    if let Some(var_197) = &input.session_timeout_minutes {
        object.key("SessionTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_197).into()),
        );
    }
}

pub fn serialize_structure_auto_tune_maintenance_schedule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoTuneMaintenanceSchedule,
) {
    if let Some(var_198) = &input.start_at {
        object
            .key("StartAt")
            .instant(var_198, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_199) = &input.duration {
        let mut object_200 = object.key("Duration").start_object();
        crate::json_ser::serialize_structure_duration(&mut object_200, var_199);
        object_200.finish();
    }
    if let Some(var_201) = &input.cron_expression_for_recurrence {
        object.key("CronExpressionForRecurrence").string(var_201);
    }
}

pub fn serialize_structure_zone_awareness_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ZoneAwarenessConfig,
) {
    if let Some(var_202) = &input.availability_zone_count {
        object.key("AvailabilityZoneCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_202).into()),
        );
    }
}

pub fn serialize_structure_cold_storage_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ColdStorageOptions,
) {
    if let Some(var_203) = &input.enabled {
        object.key("Enabled").boolean(*var_203);
    }
}

pub fn serialize_structure_saml_idp(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamlIdp,
) {
    if let Some(var_204) = &input.metadata_content {
        object.key("MetadataContent").string(var_204);
    }
    if let Some(var_205) = &input.entity_id {
        object.key("EntityId").string(var_205);
    }
}

pub fn serialize_structure_duration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Duration,
) {
    if input.value != 0 {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.value).into()),
        );
    }
    if let Some(var_206) = &input.unit {
        object.key("Unit").string(var_206.as_str());
    }
}
