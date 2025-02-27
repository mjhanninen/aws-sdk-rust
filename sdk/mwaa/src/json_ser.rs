// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentInput,
) {
    if let Some(var_1) = &input.airflow_configuration_options {
        let mut object_2 = object.key("AirflowConfigurationOptions").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3).string(value_4);
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.airflow_version {
        object.key("AirflowVersion").string(var_5);
    }
    if let Some(var_6) = &input.dag_s3_path {
        object.key("DagS3Path").string(var_6);
    }
    if let Some(var_7) = &input.environment_class {
        object.key("EnvironmentClass").string(var_7);
    }
    if let Some(var_8) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_8);
    }
    if let Some(var_9) = &input.kms_key {
        object.key("KmsKey").string(var_9);
    }
    if let Some(var_10) = &input.logging_configuration {
        let mut object_11 = object.key("LoggingConfiguration").start_object();
        crate::json_ser::serialize_structure_logging_configuration_input(&mut object_11, var_10);
        object_11.finish();
    }
    if let Some(var_12) = &input.max_workers {
        object.key("MaxWorkers").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.min_workers {
        object.key("MinWorkers").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.network_configuration {
        let mut object_15 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_network_configuration(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.plugins_s3_object_version {
        object.key("PluginsS3ObjectVersion").string(var_16);
    }
    if let Some(var_17) = &input.plugins_s3_path {
        object.key("PluginsS3Path").string(var_17);
    }
    if let Some(var_18) = &input.requirements_s3_object_version {
        object.key("RequirementsS3ObjectVersion").string(var_18);
    }
    if let Some(var_19) = &input.requirements_s3_path {
        object.key("RequirementsS3Path").string(var_19);
    }
    if let Some(var_20) = &input.schedulers {
        object.key("Schedulers").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.source_bucket_arn {
        object.key("SourceBucketArn").string(var_21);
    }
    if let Some(var_22) = &input.tags {
        let mut object_23 = object.key("Tags").start_object();
        for (key_24, value_25) in var_22 {
            {
                object_23.key(key_24).string(value_25);
            }
        }
        object_23.finish();
    }
    if let Some(var_26) = &input.webserver_access_mode {
        object.key("WebserverAccessMode").string(var_26.as_str());
    }
    if let Some(var_27) = &input.weekly_maintenance_window_start {
        object.key("WeeklyMaintenanceWindowStart").string(var_27);
    }
}

pub fn serialize_structure_publish_metrics_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishMetricsInput,
) {
    if let Some(var_28) = &input.metric_data {
        let mut array_29 = object.key("MetricData").start_array();
        for item_30 in var_28 {
            {
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_metric_datum(&mut object_31, item_30);
                object_31.finish();
            }
        }
        array_29.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_32) = &input.tags {
        let mut object_33 = object.key("Tags").start_object();
        for (key_34, value_35) in var_32 {
            {
                object_33.key(key_34).string(value_35);
            }
        }
        object_33.finish();
    }
}

pub fn serialize_structure_update_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentInput,
) {
    if let Some(var_36) = &input.airflow_configuration_options {
        let mut object_37 = object.key("AirflowConfigurationOptions").start_object();
        for (key_38, value_39) in var_36 {
            {
                object_37.key(key_38).string(value_39);
            }
        }
        object_37.finish();
    }
    if let Some(var_40) = &input.airflow_version {
        object.key("AirflowVersion").string(var_40);
    }
    if let Some(var_41) = &input.dag_s3_path {
        object.key("DagS3Path").string(var_41);
    }
    if let Some(var_42) = &input.environment_class {
        object.key("EnvironmentClass").string(var_42);
    }
    if let Some(var_43) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_43);
    }
    if let Some(var_44) = &input.logging_configuration {
        let mut object_45 = object.key("LoggingConfiguration").start_object();
        crate::json_ser::serialize_structure_logging_configuration_input(&mut object_45, var_44);
        object_45.finish();
    }
    if let Some(var_46) = &input.max_workers {
        object.key("MaxWorkers").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_46).into()),
        );
    }
    if let Some(var_47) = &input.min_workers {
        object.key("MinWorkers").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_47).into()),
        );
    }
    if let Some(var_48) = &input.network_configuration {
        let mut object_49 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_update_network_configuration_input(
            &mut object_49,
            var_48,
        );
        object_49.finish();
    }
    if let Some(var_50) = &input.plugins_s3_object_version {
        object.key("PluginsS3ObjectVersion").string(var_50);
    }
    if let Some(var_51) = &input.plugins_s3_path {
        object.key("PluginsS3Path").string(var_51);
    }
    if let Some(var_52) = &input.requirements_s3_object_version {
        object.key("RequirementsS3ObjectVersion").string(var_52);
    }
    if let Some(var_53) = &input.requirements_s3_path {
        object.key("RequirementsS3Path").string(var_53);
    }
    if let Some(var_54) = &input.schedulers {
        object.key("Schedulers").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    if let Some(var_55) = &input.source_bucket_arn {
        object.key("SourceBucketArn").string(var_55);
    }
    if let Some(var_56) = &input.webserver_access_mode {
        object.key("WebserverAccessMode").string(var_56.as_str());
    }
    if let Some(var_57) = &input.weekly_maintenance_window_start {
        object.key("WeeklyMaintenanceWindowStart").string(var_57);
    }
}

pub fn serialize_structure_logging_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoggingConfigurationInput,
) {
    if let Some(var_58) = &input.dag_processing_logs {
        let mut object_59 = object.key("DagProcessingLogs").start_object();
        crate::json_ser::serialize_structure_module_logging_configuration_input(
            &mut object_59,
            var_58,
        );
        object_59.finish();
    }
    if let Some(var_60) = &input.scheduler_logs {
        let mut object_61 = object.key("SchedulerLogs").start_object();
        crate::json_ser::serialize_structure_module_logging_configuration_input(
            &mut object_61,
            var_60,
        );
        object_61.finish();
    }
    if let Some(var_62) = &input.webserver_logs {
        let mut object_63 = object.key("WebserverLogs").start_object();
        crate::json_ser::serialize_structure_module_logging_configuration_input(
            &mut object_63,
            var_62,
        );
        object_63.finish();
    }
    if let Some(var_64) = &input.worker_logs {
        let mut object_65 = object.key("WorkerLogs").start_object();
        crate::json_ser::serialize_structure_module_logging_configuration_input(
            &mut object_65,
            var_64,
        );
        object_65.finish();
    }
    if let Some(var_66) = &input.task_logs {
        let mut object_67 = object.key("TaskLogs").start_object();
        crate::json_ser::serialize_structure_module_logging_configuration_input(
            &mut object_67,
            var_66,
        );
        object_67.finish();
    }
}

pub fn serialize_structure_network_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NetworkConfiguration,
) {
    if let Some(var_68) = &input.subnet_ids {
        let mut array_69 = object.key("SubnetIds").start_array();
        for item_70 in var_68 {
            {
                array_69.value().string(item_70);
            }
        }
        array_69.finish();
    }
    if let Some(var_71) = &input.security_group_ids {
        let mut array_72 = object.key("SecurityGroupIds").start_array();
        for item_73 in var_71 {
            {
                array_72.value().string(item_73);
            }
        }
        array_72.finish();
    }
}

pub fn serialize_structure_metric_datum(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricDatum,
) {
    if let Some(var_74) = &input.metric_name {
        object.key("MetricName").string(var_74);
    }
    if let Some(var_75) = &input.timestamp {
        object
            .key("Timestamp")
            .instant(var_75, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_76) = &input.dimensions {
        let mut array_77 = object.key("Dimensions").start_array();
        for item_78 in var_76 {
            {
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_dimension(&mut object_79, item_78);
                object_79.finish();
            }
        }
        array_77.finish();
    }
    if let Some(var_80) = &input.value {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_80).into()),
        );
    }
    if let Some(var_81) = &input.unit {
        object.key("Unit").string(var_81.as_str());
    }
    if let Some(var_82) = &input.statistic_values {
        let mut object_83 = object.key("StatisticValues").start_object();
        crate::json_ser::serialize_structure_statistic_set(&mut object_83, var_82);
        object_83.finish();
    }
}

pub fn serialize_structure_update_network_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateNetworkConfigurationInput,
) {
    if let Some(var_84) = &input.security_group_ids {
        let mut array_85 = object.key("SecurityGroupIds").start_array();
        for item_86 in var_84 {
            {
                array_85.value().string(item_86);
            }
        }
        array_85.finish();
    }
}

pub fn serialize_structure_module_logging_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ModuleLoggingConfigurationInput,
) {
    if let Some(var_87) = &input.enabled {
        object.key("Enabled").boolean(*var_87);
    }
    if let Some(var_88) = &input.log_level {
        object.key("LogLevel").string(var_88.as_str());
    }
}

pub fn serialize_structure_dimension(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Dimension,
) {
    if let Some(var_89) = &input.name {
        object.key("Name").string(var_89);
    }
    if let Some(var_90) = &input.value {
        object.key("Value").string(var_90);
    }
}

pub fn serialize_structure_statistic_set(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StatisticSet,
) {
    if let Some(var_91) = &input.sample_count {
        object.key("SampleCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_91).into()),
        );
    }
    if let Some(var_92) = &input.sum {
        object.key("Sum").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_92).into()),
        );
    }
    if let Some(var_93) = &input.minimum {
        object.key("Minimum").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_93).into()),
        );
    }
    if let Some(var_94) = &input.maximum {
        object.key("Maximum").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_94).into()),
        );
    }
}
