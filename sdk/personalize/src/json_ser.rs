// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_batch_inference_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBatchInferenceJobInput,
) {
    if let Some(var_1) = &input.job_name {
        object.key("jobName").string(var_1);
    }
    if let Some(var_2) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_2);
    }
    if let Some(var_3) = &input.filter_arn {
        object.key("filterArn").string(var_3);
    }
    if let Some(var_4) = &input.num_results {
        object.key("numResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.job_input {
        let mut object_6 = object.key("jobInput").start_object();
        crate::json_ser::serialize_structure_batch_inference_job_input(&mut object_6, var_5);
        object_6.finish();
    }
    if let Some(var_7) = &input.job_output {
        let mut object_8 = object.key("jobOutput").start_object();
        crate::json_ser::serialize_structure_batch_inference_job_output(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.role_arn {
        object.key("roleArn").string(var_9);
    }
    if let Some(var_10) = &input.batch_inference_job_config {
        let mut object_11 = object.key("batchInferenceJobConfig").start_object();
        crate::json_ser::serialize_structure_batch_inference_job_config(&mut object_11, var_10);
        object_11.finish();
    }
}

pub fn serialize_structure_create_campaign_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCampaignInput,
) {
    if let Some(var_12) = &input.name {
        object.key("name").string(var_12);
    }
    if let Some(var_13) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_13);
    }
    if let Some(var_14) = &input.min_provisioned_tps {
        object.key("minProvisionedTPS").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.campaign_config {
        let mut object_16 = object.key("campaignConfig").start_object();
        crate::json_ser::serialize_structure_campaign_config(&mut object_16, var_15);
        object_16.finish();
    }
}

pub fn serialize_structure_create_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetInput,
) {
    if let Some(var_17) = &input.name {
        object.key("name").string(var_17);
    }
    if let Some(var_18) = &input.schema_arn {
        object.key("schemaArn").string(var_18);
    }
    if let Some(var_19) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_19);
    }
    if let Some(var_20) = &input.dataset_type {
        object.key("datasetType").string(var_20);
    }
}

pub fn serialize_structure_create_dataset_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetExportJobInput,
) {
    if let Some(var_21) = &input.job_name {
        object.key("jobName").string(var_21);
    }
    if let Some(var_22) = &input.dataset_arn {
        object.key("datasetArn").string(var_22);
    }
    if let Some(var_23) = &input.ingestion_mode {
        object.key("ingestionMode").string(var_23.as_str());
    }
    if let Some(var_24) = &input.role_arn {
        object.key("roleArn").string(var_24);
    }
    if let Some(var_25) = &input.job_output {
        let mut object_26 = object.key("jobOutput").start_object();
        crate::json_ser::serialize_structure_dataset_export_job_output(&mut object_26, var_25);
        object_26.finish();
    }
}

pub fn serialize_structure_create_dataset_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetGroupInput,
) {
    if let Some(var_27) = &input.name {
        object.key("name").string(var_27);
    }
    if let Some(var_28) = &input.role_arn {
        object.key("roleArn").string(var_28);
    }
    if let Some(var_29) = &input.kms_key_arn {
        object.key("kmsKeyArn").string(var_29);
    }
}

pub fn serialize_structure_create_dataset_import_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetImportJobInput,
) {
    if let Some(var_30) = &input.job_name {
        object.key("jobName").string(var_30);
    }
    if let Some(var_31) = &input.dataset_arn {
        object.key("datasetArn").string(var_31);
    }
    if let Some(var_32) = &input.data_source {
        let mut object_33 = object.key("dataSource").start_object();
        crate::json_ser::serialize_structure_data_source(&mut object_33, var_32);
        object_33.finish();
    }
    if let Some(var_34) = &input.role_arn {
        object.key("roleArn").string(var_34);
    }
}

pub fn serialize_structure_create_event_tracker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEventTrackerInput,
) {
    if let Some(var_35) = &input.name {
        object.key("name").string(var_35);
    }
    if let Some(var_36) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_36);
    }
}

pub fn serialize_structure_create_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFilterInput,
) {
    if let Some(var_37) = &input.name {
        object.key("name").string(var_37);
    }
    if let Some(var_38) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_38);
    }
    if let Some(var_39) = &input.filter_expression {
        object.key("filterExpression").string(var_39);
    }
}

pub fn serialize_structure_create_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSchemaInput,
) {
    if let Some(var_40) = &input.name {
        object.key("name").string(var_40);
    }
    if let Some(var_41) = &input.schema {
        object.key("schema").string(var_41);
    }
}

pub fn serialize_structure_create_solution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSolutionInput,
) {
    if let Some(var_42) = &input.name {
        object.key("name").string(var_42);
    }
    if let Some(var_43) = &input.perform_hpo {
        object.key("performHPO").boolean(*var_43);
    }
    if input.perform_auto_ml {
        object.key("performAutoML").boolean(input.perform_auto_ml);
    }
    if let Some(var_44) = &input.recipe_arn {
        object.key("recipeArn").string(var_44);
    }
    if let Some(var_45) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_45);
    }
    if let Some(var_46) = &input.event_type {
        object.key("eventType").string(var_46);
    }
    if let Some(var_47) = &input.solution_config {
        let mut object_48 = object.key("solutionConfig").start_object();
        crate::json_ser::serialize_structure_solution_config(&mut object_48, var_47);
        object_48.finish();
    }
}

pub fn serialize_structure_create_solution_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSolutionVersionInput,
) {
    if let Some(var_49) = &input.solution_arn {
        object.key("solutionArn").string(var_49);
    }
    if let Some(var_50) = &input.training_mode {
        object.key("trainingMode").string(var_50.as_str());
    }
}

pub fn serialize_structure_delete_campaign_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCampaignInput,
) {
    if let Some(var_51) = &input.campaign_arn {
        object.key("campaignArn").string(var_51);
    }
}

pub fn serialize_structure_delete_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDatasetInput,
) {
    if let Some(var_52) = &input.dataset_arn {
        object.key("datasetArn").string(var_52);
    }
}

pub fn serialize_structure_delete_dataset_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDatasetGroupInput,
) {
    if let Some(var_53) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_53);
    }
}

pub fn serialize_structure_delete_event_tracker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEventTrackerInput,
) {
    if let Some(var_54) = &input.event_tracker_arn {
        object.key("eventTrackerArn").string(var_54);
    }
}

pub fn serialize_structure_delete_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteFilterInput,
) {
    if let Some(var_55) = &input.filter_arn {
        object.key("filterArn").string(var_55);
    }
}

pub fn serialize_structure_delete_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSchemaInput,
) {
    if let Some(var_56) = &input.schema_arn {
        object.key("schemaArn").string(var_56);
    }
}

pub fn serialize_structure_delete_solution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSolutionInput,
) {
    if let Some(var_57) = &input.solution_arn {
        object.key("solutionArn").string(var_57);
    }
}

pub fn serialize_structure_describe_algorithm_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAlgorithmInput,
) {
    if let Some(var_58) = &input.algorithm_arn {
        object.key("algorithmArn").string(var_58);
    }
}

pub fn serialize_structure_describe_batch_inference_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBatchInferenceJobInput,
) {
    if let Some(var_59) = &input.batch_inference_job_arn {
        object.key("batchInferenceJobArn").string(var_59);
    }
}

pub fn serialize_structure_describe_campaign_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCampaignInput,
) {
    if let Some(var_60) = &input.campaign_arn {
        object.key("campaignArn").string(var_60);
    }
}

pub fn serialize_structure_describe_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDatasetInput,
) {
    if let Some(var_61) = &input.dataset_arn {
        object.key("datasetArn").string(var_61);
    }
}

pub fn serialize_structure_describe_dataset_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDatasetExportJobInput,
) {
    if let Some(var_62) = &input.dataset_export_job_arn {
        object.key("datasetExportJobArn").string(var_62);
    }
}

pub fn serialize_structure_describe_dataset_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDatasetGroupInput,
) {
    if let Some(var_63) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_63);
    }
}

pub fn serialize_structure_describe_dataset_import_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDatasetImportJobInput,
) {
    if let Some(var_64) = &input.dataset_import_job_arn {
        object.key("datasetImportJobArn").string(var_64);
    }
}

pub fn serialize_structure_describe_event_tracker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEventTrackerInput,
) {
    if let Some(var_65) = &input.event_tracker_arn {
        object.key("eventTrackerArn").string(var_65);
    }
}

pub fn serialize_structure_describe_feature_transformation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFeatureTransformationInput,
) {
    if let Some(var_66) = &input.feature_transformation_arn {
        object.key("featureTransformationArn").string(var_66);
    }
}

pub fn serialize_structure_describe_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFilterInput,
) {
    if let Some(var_67) = &input.filter_arn {
        object.key("filterArn").string(var_67);
    }
}

pub fn serialize_structure_describe_recipe_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRecipeInput,
) {
    if let Some(var_68) = &input.recipe_arn {
        object.key("recipeArn").string(var_68);
    }
}

pub fn serialize_structure_describe_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSchemaInput,
) {
    if let Some(var_69) = &input.schema_arn {
        object.key("schemaArn").string(var_69);
    }
}

pub fn serialize_structure_describe_solution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSolutionInput,
) {
    if let Some(var_70) = &input.solution_arn {
        object.key("solutionArn").string(var_70);
    }
}

pub fn serialize_structure_describe_solution_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSolutionVersionInput,
) {
    if let Some(var_71) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_71);
    }
}

pub fn serialize_structure_get_solution_metrics_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSolutionMetricsInput,
) {
    if let Some(var_72) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_72);
    }
}

pub fn serialize_structure_list_batch_inference_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListBatchInferenceJobsInput,
) {
    if let Some(var_73) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_73);
    }
    if let Some(var_74) = &input.next_token {
        object.key("nextToken").string(var_74);
    }
    if let Some(var_75) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_75).into()),
        );
    }
}

pub fn serialize_structure_list_campaigns_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCampaignsInput,
) {
    if let Some(var_76) = &input.solution_arn {
        object.key("solutionArn").string(var_76);
    }
    if let Some(var_77) = &input.next_token {
        object.key("nextToken").string(var_77);
    }
    if let Some(var_78) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_78).into()),
        );
    }
}

pub fn serialize_structure_list_dataset_export_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatasetExportJobsInput,
) {
    if let Some(var_79) = &input.dataset_arn {
        object.key("datasetArn").string(var_79);
    }
    if let Some(var_80) = &input.next_token {
        object.key("nextToken").string(var_80);
    }
    if let Some(var_81) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_81).into()),
        );
    }
}

pub fn serialize_structure_list_dataset_groups_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatasetGroupsInput,
) {
    if let Some(var_82) = &input.next_token {
        object.key("nextToken").string(var_82);
    }
    if let Some(var_83) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_83).into()),
        );
    }
}

pub fn serialize_structure_list_dataset_import_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatasetImportJobsInput,
) {
    if let Some(var_84) = &input.dataset_arn {
        object.key("datasetArn").string(var_84);
    }
    if let Some(var_85) = &input.next_token {
        object.key("nextToken").string(var_85);
    }
    if let Some(var_86) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_86).into()),
        );
    }
}

pub fn serialize_structure_list_datasets_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatasetsInput,
) {
    if let Some(var_87) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_87);
    }
    if let Some(var_88) = &input.next_token {
        object.key("nextToken").string(var_88);
    }
    if let Some(var_89) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_89).into()),
        );
    }
}

pub fn serialize_structure_list_event_trackers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEventTrackersInput,
) {
    if let Some(var_90) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_90);
    }
    if let Some(var_91) = &input.next_token {
        object.key("nextToken").string(var_91);
    }
    if let Some(var_92) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_92).into()),
        );
    }
}

pub fn serialize_structure_list_filters_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListFiltersInput,
) {
    if let Some(var_93) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_93);
    }
    if let Some(var_94) = &input.next_token {
        object.key("nextToken").string(var_94);
    }
    if let Some(var_95) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_95).into()),
        );
    }
}

pub fn serialize_structure_list_recipes_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRecipesInput,
) {
    if let Some(var_96) = &input.recipe_provider {
        object.key("recipeProvider").string(var_96.as_str());
    }
    if let Some(var_97) = &input.next_token {
        object.key("nextToken").string(var_97);
    }
    if let Some(var_98) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_98).into()),
        );
    }
}

pub fn serialize_structure_list_schemas_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSchemasInput,
) {
    if let Some(var_99) = &input.next_token {
        object.key("nextToken").string(var_99);
    }
    if let Some(var_100) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_100).into()),
        );
    }
}

pub fn serialize_structure_list_solutions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSolutionsInput,
) {
    if let Some(var_101) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_101);
    }
    if let Some(var_102) = &input.next_token {
        object.key("nextToken").string(var_102);
    }
    if let Some(var_103) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_103).into()),
        );
    }
}

pub fn serialize_structure_list_solution_versions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSolutionVersionsInput,
) {
    if let Some(var_104) = &input.solution_arn {
        object.key("solutionArn").string(var_104);
    }
    if let Some(var_105) = &input.next_token {
        object.key("nextToken").string(var_105);
    }
    if let Some(var_106) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_106).into()),
        );
    }
}

pub fn serialize_structure_stop_solution_version_creation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopSolutionVersionCreationInput,
) {
    if let Some(var_107) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_107);
    }
}

pub fn serialize_structure_update_campaign_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCampaignInput,
) {
    if let Some(var_108) = &input.campaign_arn {
        object.key("campaignArn").string(var_108);
    }
    if let Some(var_109) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_109);
    }
    if let Some(var_110) = &input.min_provisioned_tps {
        object.key("minProvisionedTPS").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_110).into()),
        );
    }
    if let Some(var_111) = &input.campaign_config {
        let mut object_112 = object.key("campaignConfig").start_object();
        crate::json_ser::serialize_structure_campaign_config(&mut object_112, var_111);
        object_112.finish();
    }
}

pub fn serialize_structure_batch_inference_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BatchInferenceJobInput,
) {
    if let Some(var_113) = &input.s3_data_source {
        let mut object_114 = object.key("s3DataSource").start_object();
        crate::json_ser::serialize_structure_s3_data_config(&mut object_114, var_113);
        object_114.finish();
    }
}

pub fn serialize_structure_batch_inference_job_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BatchInferenceJobOutput,
) {
    if let Some(var_115) = &input.s3_data_destination {
        let mut object_116 = object.key("s3DataDestination").start_object();
        crate::json_ser::serialize_structure_s3_data_config(&mut object_116, var_115);
        object_116.finish();
    }
}

pub fn serialize_structure_batch_inference_job_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BatchInferenceJobConfig,
) {
    if let Some(var_117) = &input.item_exploration_config {
        let mut object_118 = object.key("itemExplorationConfig").start_object();
        for (key_119, value_120) in var_117 {
            {
                object_118.key(key_119).string(value_120);
            }
        }
        object_118.finish();
    }
}

pub fn serialize_structure_campaign_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CampaignConfig,
) {
    if let Some(var_121) = &input.item_exploration_config {
        let mut object_122 = object.key("itemExplorationConfig").start_object();
        for (key_123, value_124) in var_121 {
            {
                object_122.key(key_123).string(value_124);
            }
        }
        object_122.finish();
    }
}

pub fn serialize_structure_dataset_export_job_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetExportJobOutput,
) {
    if let Some(var_125) = &input.s3_data_destination {
        let mut object_126 = object.key("s3DataDestination").start_object();
        crate::json_ser::serialize_structure_s3_data_config(&mut object_126, var_125);
        object_126.finish();
    }
}

pub fn serialize_structure_data_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataSource,
) {
    if let Some(var_127) = &input.data_location {
        object.key("dataLocation").string(var_127);
    }
}

pub fn serialize_structure_solution_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SolutionConfig,
) {
    if let Some(var_128) = &input.event_value_threshold {
        object.key("eventValueThreshold").string(var_128);
    }
    if let Some(var_129) = &input.hpo_config {
        let mut object_130 = object.key("hpoConfig").start_object();
        crate::json_ser::serialize_structure_hpo_config(&mut object_130, var_129);
        object_130.finish();
    }
    if let Some(var_131) = &input.algorithm_hyper_parameters {
        let mut object_132 = object.key("algorithmHyperParameters").start_object();
        for (key_133, value_134) in var_131 {
            {
                object_132.key(key_133).string(value_134);
            }
        }
        object_132.finish();
    }
    if let Some(var_135) = &input.feature_transformation_parameters {
        let mut object_136 = object.key("featureTransformationParameters").start_object();
        for (key_137, value_138) in var_135 {
            {
                object_136.key(key_137).string(value_138);
            }
        }
        object_136.finish();
    }
    if let Some(var_139) = &input.auto_ml_config {
        let mut object_140 = object.key("autoMLConfig").start_object();
        crate::json_ser::serialize_structure_auto_ml_config(&mut object_140, var_139);
        object_140.finish();
    }
    if let Some(var_141) = &input.optimization_objective {
        let mut object_142 = object.key("optimizationObjective").start_object();
        crate::json_ser::serialize_structure_optimization_objective(&mut object_142, var_141);
        object_142.finish();
    }
}

pub fn serialize_structure_s3_data_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3DataConfig,
) {
    if let Some(var_143) = &input.path {
        object.key("path").string(var_143);
    }
    if let Some(var_144) = &input.kms_key_arn {
        object.key("kmsKeyArn").string(var_144);
    }
}

pub fn serialize_structure_hpo_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HpoConfig,
) {
    if let Some(var_145) = &input.hpo_objective {
        let mut object_146 = object.key("hpoObjective").start_object();
        crate::json_ser::serialize_structure_hpo_objective(&mut object_146, var_145);
        object_146.finish();
    }
    if let Some(var_147) = &input.hpo_resource_config {
        let mut object_148 = object.key("hpoResourceConfig").start_object();
        crate::json_ser::serialize_structure_hpo_resource_config(&mut object_148, var_147);
        object_148.finish();
    }
    if let Some(var_149) = &input.algorithm_hyper_parameter_ranges {
        let mut object_150 = object.key("algorithmHyperParameterRanges").start_object();
        crate::json_ser::serialize_structure_hyper_parameter_ranges(&mut object_150, var_149);
        object_150.finish();
    }
}

pub fn serialize_structure_auto_ml_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoMlConfig,
) {
    if let Some(var_151) = &input.metric_name {
        object.key("metricName").string(var_151);
    }
    if let Some(var_152) = &input.recipe_list {
        let mut array_153 = object.key("recipeList").start_array();
        for item_154 in var_152 {
            {
                array_153.value().string(item_154);
            }
        }
        array_153.finish();
    }
}

pub fn serialize_structure_optimization_objective(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OptimizationObjective,
) {
    if let Some(var_155) = &input.item_attribute {
        object.key("itemAttribute").string(var_155);
    }
    if let Some(var_156) = &input.objective_sensitivity {
        object.key("objectiveSensitivity").string(var_156.as_str());
    }
}

pub fn serialize_structure_hpo_objective(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HpoObjective,
) {
    if let Some(var_157) = &input.r#type {
        object.key("type").string(var_157);
    }
    if let Some(var_158) = &input.metric_name {
        object.key("metricName").string(var_158);
    }
    if let Some(var_159) = &input.metric_regex {
        object.key("metricRegex").string(var_159);
    }
}

pub fn serialize_structure_hpo_resource_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HpoResourceConfig,
) {
    if let Some(var_160) = &input.max_number_of_training_jobs {
        object.key("maxNumberOfTrainingJobs").string(var_160);
    }
    if let Some(var_161) = &input.max_parallel_training_jobs {
        object.key("maxParallelTrainingJobs").string(var_161);
    }
}

pub fn serialize_structure_hyper_parameter_ranges(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HyperParameterRanges,
) {
    if let Some(var_162) = &input.integer_hyper_parameter_ranges {
        let mut array_163 = object.key("integerHyperParameterRanges").start_array();
        for item_164 in var_162 {
            {
                let mut object_165 = array_163.value().start_object();
                crate::json_ser::serialize_structure_integer_hyper_parameter_range(
                    &mut object_165,
                    item_164,
                );
                object_165.finish();
            }
        }
        array_163.finish();
    }
    if let Some(var_166) = &input.continuous_hyper_parameter_ranges {
        let mut array_167 = object.key("continuousHyperParameterRanges").start_array();
        for item_168 in var_166 {
            {
                let mut object_169 = array_167.value().start_object();
                crate::json_ser::serialize_structure_continuous_hyper_parameter_range(
                    &mut object_169,
                    item_168,
                );
                object_169.finish();
            }
        }
        array_167.finish();
    }
    if let Some(var_170) = &input.categorical_hyper_parameter_ranges {
        let mut array_171 = object.key("categoricalHyperParameterRanges").start_array();
        for item_172 in var_170 {
            {
                let mut object_173 = array_171.value().start_object();
                crate::json_ser::serialize_structure_categorical_hyper_parameter_range(
                    &mut object_173,
                    item_172,
                );
                object_173.finish();
            }
        }
        array_171.finish();
    }
}

pub fn serialize_structure_integer_hyper_parameter_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IntegerHyperParameterRange,
) {
    if let Some(var_174) = &input.name {
        object.key("name").string(var_174);
    }
    if input.min_value != 0 {
        object.key("minValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.min_value).into()),
        );
    }
    if input.max_value != 0 {
        object.key("maxValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_value).into()),
        );
    }
}

pub fn serialize_structure_continuous_hyper_parameter_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ContinuousHyperParameterRange,
) {
    if let Some(var_175) = &input.name {
        object.key("name").string(var_175);
    }
    if input.min_value != 0.0 {
        object.key("minValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.min_value).into()),
        );
    }
    if input.max_value != 0.0 {
        object.key("maxValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.max_value).into()),
        );
    }
}

pub fn serialize_structure_categorical_hyper_parameter_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CategoricalHyperParameterRange,
) {
    if let Some(var_176) = &input.name {
        object.key("name").string(var_176);
    }
    if let Some(var_177) = &input.values {
        let mut array_178 = object.key("values").start_array();
        for item_179 in var_177 {
            {
                array_178.value().string(item_179);
            }
        }
        array_178.finish();
    }
}
