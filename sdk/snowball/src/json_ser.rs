// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_cancel_cluster_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelClusterInput,
) {
    if let Some(var_1) = &input.cluster_id {
        object.key("ClusterId").string(var_1);
    }
}

pub fn serialize_structure_cancel_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelJobInput,
) {
    if let Some(var_2) = &input.job_id {
        object.key("JobId").string(var_2);
    }
}

pub fn serialize_structure_create_address_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAddressInput,
) {
    if let Some(var_3) = &input.address {
        let mut object_4 = object.key("Address").start_object();
        crate::json_ser::serialize_structure_address(&mut object_4, var_3);
        object_4.finish();
    }
}

pub fn serialize_structure_create_cluster_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) {
    if let Some(var_5) = &input.job_type {
        object.key("JobType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.resources {
        let mut object_7 = object.key("Resources").start_object();
        crate::json_ser::serialize_structure_job_resource(&mut object_7, var_6);
        object_7.finish();
    }
    if let Some(var_8) = &input.on_device_service_configuration {
        let mut object_9 = object.key("OnDeviceServiceConfiguration").start_object();
        crate::json_ser::serialize_structure_on_device_service_configuration(&mut object_9, var_8);
        object_9.finish();
    }
    if let Some(var_10) = &input.description {
        object.key("Description").string(var_10);
    }
    if let Some(var_11) = &input.address_id {
        object.key("AddressId").string(var_11);
    }
    if let Some(var_12) = &input.kms_key_arn {
        object.key("KmsKeyARN").string(var_12);
    }
    if let Some(var_13) = &input.role_arn {
        object.key("RoleARN").string(var_13);
    }
    if let Some(var_14) = &input.snowball_type {
        object.key("SnowballType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.shipping_option {
        object.key("ShippingOption").string(var_15.as_str());
    }
    if let Some(var_16) = &input.notification {
        let mut object_17 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_notification(&mut object_17, var_16);
        object_17.finish();
    }
    if let Some(var_18) = &input.forwarding_address_id {
        object.key("ForwardingAddressId").string(var_18);
    }
    if let Some(var_19) = &input.tax_documents {
        let mut object_20 = object.key("TaxDocuments").start_object();
        crate::json_ser::serialize_structure_tax_documents(&mut object_20, var_19);
        object_20.finish();
    }
    if let Some(var_21) = &input.remote_management {
        object.key("RemoteManagement").string(var_21.as_str());
    }
}

pub fn serialize_structure_create_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateJobInput,
) {
    if let Some(var_22) = &input.job_type {
        object.key("JobType").string(var_22.as_str());
    }
    if let Some(var_23) = &input.resources {
        let mut object_24 = object.key("Resources").start_object();
        crate::json_ser::serialize_structure_job_resource(&mut object_24, var_23);
        object_24.finish();
    }
    if let Some(var_25) = &input.on_device_service_configuration {
        let mut object_26 = object.key("OnDeviceServiceConfiguration").start_object();
        crate::json_ser::serialize_structure_on_device_service_configuration(
            &mut object_26,
            var_25,
        );
        object_26.finish();
    }
    if let Some(var_27) = &input.description {
        object.key("Description").string(var_27);
    }
    if let Some(var_28) = &input.address_id {
        object.key("AddressId").string(var_28);
    }
    if let Some(var_29) = &input.kms_key_arn {
        object.key("KmsKeyARN").string(var_29);
    }
    if let Some(var_30) = &input.role_arn {
        object.key("RoleARN").string(var_30);
    }
    if let Some(var_31) = &input.snowball_capacity_preference {
        object
            .key("SnowballCapacityPreference")
            .string(var_31.as_str());
    }
    if let Some(var_32) = &input.shipping_option {
        object.key("ShippingOption").string(var_32.as_str());
    }
    if let Some(var_33) = &input.notification {
        let mut object_34 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_notification(&mut object_34, var_33);
        object_34.finish();
    }
    if let Some(var_35) = &input.cluster_id {
        object.key("ClusterId").string(var_35);
    }
    if let Some(var_36) = &input.snowball_type {
        object.key("SnowballType").string(var_36.as_str());
    }
    if let Some(var_37) = &input.forwarding_address_id {
        object.key("ForwardingAddressId").string(var_37);
    }
    if let Some(var_38) = &input.tax_documents {
        let mut object_39 = object.key("TaxDocuments").start_object();
        crate::json_ser::serialize_structure_tax_documents(&mut object_39, var_38);
        object_39.finish();
    }
    if let Some(var_40) = &input.device_configuration {
        let mut object_41 = object.key("DeviceConfiguration").start_object();
        crate::json_ser::serialize_structure_device_configuration(&mut object_41, var_40);
        object_41.finish();
    }
    if let Some(var_42) = &input.remote_management {
        object.key("RemoteManagement").string(var_42.as_str());
    }
    if let Some(var_43) = &input.long_term_pricing_id {
        object.key("LongTermPricingId").string(var_43);
    }
}

pub fn serialize_structure_create_long_term_pricing_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLongTermPricingInput,
) {
    if let Some(var_44) = &input.long_term_pricing_type {
        object.key("LongTermPricingType").string(var_44.as_str());
    }
    if let Some(var_45) = &input.is_long_term_pricing_auto_renew {
        object.key("IsLongTermPricingAutoRenew").boolean(*var_45);
    }
    if let Some(var_46) = &input.snowball_type {
        object.key("SnowballType").string(var_46.as_str());
    }
}

pub fn serialize_structure_create_return_shipping_label_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateReturnShippingLabelInput,
) {
    if let Some(var_47) = &input.job_id {
        object.key("JobId").string(var_47);
    }
    if let Some(var_48) = &input.shipping_option {
        object.key("ShippingOption").string(var_48.as_str());
    }
}

pub fn serialize_structure_describe_address_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAddressInput,
) {
    if let Some(var_49) = &input.address_id {
        object.key("AddressId").string(var_49);
    }
}

pub fn serialize_structure_describe_addresses_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAddressesInput,
) {
    if let Some(var_50) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    if let Some(var_51) = &input.next_token {
        object.key("NextToken").string(var_51);
    }
}

pub fn serialize_structure_describe_cluster_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeClusterInput,
) {
    if let Some(var_52) = &input.cluster_id {
        object.key("ClusterId").string(var_52);
    }
}

pub fn serialize_structure_describe_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeJobInput,
) {
    if let Some(var_53) = &input.job_id {
        object.key("JobId").string(var_53);
    }
}

pub fn serialize_structure_describe_return_shipping_label_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeReturnShippingLabelInput,
) {
    if let Some(var_54) = &input.job_id {
        object.key("JobId").string(var_54);
    }
}

pub fn serialize_structure_get_job_manifest_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetJobManifestInput,
) {
    if let Some(var_55) = &input.job_id {
        object.key("JobId").string(var_55);
    }
}

pub fn serialize_structure_get_job_unlock_code_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetJobUnlockCodeInput,
) {
    if let Some(var_56) = &input.job_id {
        object.key("JobId").string(var_56);
    }
}

pub fn serialize_structure_get_software_updates_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSoftwareUpdatesInput,
) {
    if let Some(var_57) = &input.job_id {
        object.key("JobId").string(var_57);
    }
}

pub fn serialize_structure_list_cluster_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListClusterJobsInput,
) {
    if let Some(var_58) = &input.cluster_id {
        object.key("ClusterId").string(var_58);
    }
    if let Some(var_59) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_59).into()),
        );
    }
    if let Some(var_60) = &input.next_token {
        object.key("NextToken").string(var_60);
    }
}

pub fn serialize_structure_list_clusters_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListClustersInput,
) {
    if let Some(var_61) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.next_token {
        object.key("NextToken").string(var_62);
    }
}

pub fn serialize_structure_list_compatible_images_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCompatibleImagesInput,
) {
    if let Some(var_63) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_63).into()),
        );
    }
    if let Some(var_64) = &input.next_token {
        object.key("NextToken").string(var_64);
    }
}

pub fn serialize_structure_list_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListJobsInput,
) {
    if let Some(var_65) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_65).into()),
        );
    }
    if let Some(var_66) = &input.next_token {
        object.key("NextToken").string(var_66);
    }
}

pub fn serialize_structure_list_long_term_pricing_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListLongTermPricingInput,
) {
    if let Some(var_67) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_67).into()),
        );
    }
    if let Some(var_68) = &input.next_token {
        object.key("NextToken").string(var_68);
    }
}

pub fn serialize_structure_update_cluster_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterInput,
) {
    if let Some(var_69) = &input.cluster_id {
        object.key("ClusterId").string(var_69);
    }
    if let Some(var_70) = &input.role_arn {
        object.key("RoleARN").string(var_70);
    }
    if let Some(var_71) = &input.description {
        object.key("Description").string(var_71);
    }
    if let Some(var_72) = &input.resources {
        let mut object_73 = object.key("Resources").start_object();
        crate::json_ser::serialize_structure_job_resource(&mut object_73, var_72);
        object_73.finish();
    }
    if let Some(var_74) = &input.on_device_service_configuration {
        let mut object_75 = object.key("OnDeviceServiceConfiguration").start_object();
        crate::json_ser::serialize_structure_on_device_service_configuration(
            &mut object_75,
            var_74,
        );
        object_75.finish();
    }
    if let Some(var_76) = &input.address_id {
        object.key("AddressId").string(var_76);
    }
    if let Some(var_77) = &input.shipping_option {
        object.key("ShippingOption").string(var_77.as_str());
    }
    if let Some(var_78) = &input.notification {
        let mut object_79 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_notification(&mut object_79, var_78);
        object_79.finish();
    }
    if let Some(var_80) = &input.forwarding_address_id {
        object.key("ForwardingAddressId").string(var_80);
    }
}

pub fn serialize_structure_update_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateJobInput,
) {
    if let Some(var_81) = &input.job_id {
        object.key("JobId").string(var_81);
    }
    if let Some(var_82) = &input.role_arn {
        object.key("RoleARN").string(var_82);
    }
    if let Some(var_83) = &input.notification {
        let mut object_84 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_notification(&mut object_84, var_83);
        object_84.finish();
    }
    if let Some(var_85) = &input.resources {
        let mut object_86 = object.key("Resources").start_object();
        crate::json_ser::serialize_structure_job_resource(&mut object_86, var_85);
        object_86.finish();
    }
    if let Some(var_87) = &input.on_device_service_configuration {
        let mut object_88 = object.key("OnDeviceServiceConfiguration").start_object();
        crate::json_ser::serialize_structure_on_device_service_configuration(
            &mut object_88,
            var_87,
        );
        object_88.finish();
    }
    if let Some(var_89) = &input.address_id {
        object.key("AddressId").string(var_89);
    }
    if let Some(var_90) = &input.shipping_option {
        object.key("ShippingOption").string(var_90.as_str());
    }
    if let Some(var_91) = &input.description {
        object.key("Description").string(var_91);
    }
    if let Some(var_92) = &input.snowball_capacity_preference {
        object
            .key("SnowballCapacityPreference")
            .string(var_92.as_str());
    }
    if let Some(var_93) = &input.forwarding_address_id {
        object.key("ForwardingAddressId").string(var_93);
    }
}

pub fn serialize_structure_update_job_shipment_state_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateJobShipmentStateInput,
) {
    if let Some(var_94) = &input.job_id {
        object.key("JobId").string(var_94);
    }
    if let Some(var_95) = &input.shipment_state {
        object.key("ShipmentState").string(var_95.as_str());
    }
}

pub fn serialize_structure_update_long_term_pricing_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLongTermPricingInput,
) {
    if let Some(var_96) = &input.long_term_pricing_id {
        object.key("LongTermPricingId").string(var_96);
    }
    if let Some(var_97) = &input.replacement_job {
        object.key("ReplacementJob").string(var_97);
    }
    if let Some(var_98) = &input.is_long_term_pricing_auto_renew {
        object.key("IsLongTermPricingAutoRenew").boolean(*var_98);
    }
}

pub fn serialize_structure_address(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Address,
) {
    if let Some(var_99) = &input.address_id {
        object.key("AddressId").string(var_99);
    }
    if let Some(var_100) = &input.name {
        object.key("Name").string(var_100);
    }
    if let Some(var_101) = &input.company {
        object.key("Company").string(var_101);
    }
    if let Some(var_102) = &input.street1 {
        object.key("Street1").string(var_102);
    }
    if let Some(var_103) = &input.street2 {
        object.key("Street2").string(var_103);
    }
    if let Some(var_104) = &input.street3 {
        object.key("Street3").string(var_104);
    }
    if let Some(var_105) = &input.city {
        object.key("City").string(var_105);
    }
    if let Some(var_106) = &input.state_or_province {
        object.key("StateOrProvince").string(var_106);
    }
    if let Some(var_107) = &input.prefecture_or_district {
        object.key("PrefectureOrDistrict").string(var_107);
    }
    if let Some(var_108) = &input.landmark {
        object.key("Landmark").string(var_108);
    }
    if let Some(var_109) = &input.country {
        object.key("Country").string(var_109);
    }
    if let Some(var_110) = &input.postal_code {
        object.key("PostalCode").string(var_110);
    }
    if let Some(var_111) = &input.phone_number {
        object.key("PhoneNumber").string(var_111);
    }
    if input.is_restricted {
        object.key("IsRestricted").boolean(input.is_restricted);
    }
}

pub fn serialize_structure_job_resource(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobResource,
) {
    if let Some(var_112) = &input.s3_resources {
        let mut array_113 = object.key("S3Resources").start_array();
        for item_114 in var_112 {
            {
                let mut object_115 = array_113.value().start_object();
                crate::json_ser::serialize_structure_s3_resource(&mut object_115, item_114);
                object_115.finish();
            }
        }
        array_113.finish();
    }
    if let Some(var_116) = &input.lambda_resources {
        let mut array_117 = object.key("LambdaResources").start_array();
        for item_118 in var_116 {
            {
                let mut object_119 = array_117.value().start_object();
                crate::json_ser::serialize_structure_lambda_resource(&mut object_119, item_118);
                object_119.finish();
            }
        }
        array_117.finish();
    }
    if let Some(var_120) = &input.ec2_ami_resources {
        let mut array_121 = object.key("Ec2AmiResources").start_array();
        for item_122 in var_120 {
            {
                let mut object_123 = array_121.value().start_object();
                crate::json_ser::serialize_structure_ec2_ami_resource(&mut object_123, item_122);
                object_123.finish();
            }
        }
        array_121.finish();
    }
}

pub fn serialize_structure_on_device_service_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnDeviceServiceConfiguration,
) {
    if let Some(var_124) = &input.nfs_on_device_service {
        let mut object_125 = object.key("NFSOnDeviceService").start_object();
        crate::json_ser::serialize_structure_nfs_on_device_service_configuration(
            &mut object_125,
            var_124,
        );
        object_125.finish();
    }
}

pub fn serialize_structure_notification(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Notification,
) {
    if let Some(var_126) = &input.sns_topic_arn {
        object.key("SnsTopicARN").string(var_126);
    }
    if let Some(var_127) = &input.job_states_to_notify {
        let mut array_128 = object.key("JobStatesToNotify").start_array();
        for item_129 in var_127 {
            {
                array_128.value().string(item_129.as_str());
            }
        }
        array_128.finish();
    }
    if input.notify_all {
        object.key("NotifyAll").boolean(input.notify_all);
    }
}

pub fn serialize_structure_tax_documents(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TaxDocuments,
) {
    if let Some(var_130) = &input.ind {
        let mut object_131 = object.key("IND").start_object();
        crate::json_ser::serialize_structure_ind_tax_documents(&mut object_131, var_130);
        object_131.finish();
    }
}

pub fn serialize_structure_device_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeviceConfiguration,
) {
    if let Some(var_132) = &input.snowcone_device_configuration {
        let mut object_133 = object.key("SnowconeDeviceConfiguration").start_object();
        crate::json_ser::serialize_structure_snowcone_device_configuration(
            &mut object_133,
            var_132,
        );
        object_133.finish();
    }
}

pub fn serialize_structure_s3_resource(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Resource,
) {
    if let Some(var_134) = &input.bucket_arn {
        object.key("BucketArn").string(var_134);
    }
    if let Some(var_135) = &input.key_range {
        let mut object_136 = object.key("KeyRange").start_object();
        crate::json_ser::serialize_structure_key_range(&mut object_136, var_135);
        object_136.finish();
    }
    if let Some(var_137) = &input.target_on_device_services {
        let mut array_138 = object.key("TargetOnDeviceServices").start_array();
        for item_139 in var_137 {
            {
                let mut object_140 = array_138.value().start_object();
                crate::json_ser::serialize_structure_target_on_device_service(
                    &mut object_140,
                    item_139,
                );
                object_140.finish();
            }
        }
        array_138.finish();
    }
}

pub fn serialize_structure_lambda_resource(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaResource,
) {
    if let Some(var_141) = &input.lambda_arn {
        object.key("LambdaArn").string(var_141);
    }
    if let Some(var_142) = &input.event_triggers {
        let mut array_143 = object.key("EventTriggers").start_array();
        for item_144 in var_142 {
            {
                let mut object_145 = array_143.value().start_object();
                crate::json_ser::serialize_structure_event_trigger_definition(
                    &mut object_145,
                    item_144,
                );
                object_145.finish();
            }
        }
        array_143.finish();
    }
}

pub fn serialize_structure_ec2_ami_resource(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Ec2AmiResource,
) {
    if let Some(var_146) = &input.ami_id {
        object.key("AmiId").string(var_146);
    }
    if let Some(var_147) = &input.snowball_ami_id {
        object.key("SnowballAmiId").string(var_147);
    }
}

pub fn serialize_structure_nfs_on_device_service_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NfsOnDeviceServiceConfiguration,
) {
    if input.storage_limit != 0 {
        object.key("StorageLimit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.storage_limit).into()),
        );
    }
    if let Some(var_148) = &input.storage_unit {
        object.key("StorageUnit").string(var_148.as_str());
    }
}

pub fn serialize_structure_ind_tax_documents(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IndTaxDocuments,
) {
    if let Some(var_149) = &input.gstin {
        object.key("GSTIN").string(var_149);
    }
}

pub fn serialize_structure_snowcone_device_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SnowconeDeviceConfiguration,
) {
    if let Some(var_150) = &input.wireless_connection {
        let mut object_151 = object.key("WirelessConnection").start_object();
        crate::json_ser::serialize_structure_wireless_connection(&mut object_151, var_150);
        object_151.finish();
    }
}

pub fn serialize_structure_key_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KeyRange,
) {
    if let Some(var_152) = &input.begin_marker {
        object.key("BeginMarker").string(var_152);
    }
    if let Some(var_153) = &input.end_marker {
        object.key("EndMarker").string(var_153);
    }
}

pub fn serialize_structure_target_on_device_service(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TargetOnDeviceService,
) {
    if let Some(var_154) = &input.service_name {
        object.key("ServiceName").string(var_154.as_str());
    }
    if let Some(var_155) = &input.transfer_option {
        object.key("TransferOption").string(var_155.as_str());
    }
}

pub fn serialize_structure_event_trigger_definition(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventTriggerDefinition,
) {
    if let Some(var_156) = &input.event_resource_arn {
        object.key("EventResourceARN").string(var_156);
    }
}

pub fn serialize_structure_wireless_connection(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WirelessConnection,
) {
    if input.is_wifi_enabled {
        object.key("IsWifiEnabled").boolean(input.is_wifi_enabled);
    }
}
