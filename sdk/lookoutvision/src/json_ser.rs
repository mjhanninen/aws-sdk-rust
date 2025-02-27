// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_dataset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetInput,
) {
    if let Some(var_1) = &input.dataset_source {
        let mut object_2 = object.key("DatasetSource").start_object();
        crate::json_ser::serialize_structure_dataset_source(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.dataset_type {
        object.key("DatasetType").string(var_3);
    }
}

pub fn serialize_structure_create_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateModelInput,
) {
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4);
    }
    if let Some(var_5) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_5);
    }
    if let Some(var_6) = &input.output_config {
        let mut object_7 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_output_config(&mut object_7, var_6);
        object_7.finish();
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_11, item_10);
                object_11.finish();
            }
        }
        array_9.finish();
    }
}

pub fn serialize_structure_create_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProjectInput,
) {
    if let Some(var_12) = &input.project_name {
        object.key("ProjectName").string(var_12);
    }
}

pub fn serialize_structure_start_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartModelInput,
) {
    if let Some(var_13) = &input.min_inference_units {
        object.key("MinInferenceUnits").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_13).into()),
        );
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
            {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_17, item_16);
                object_17.finish();
            }
        }
        array_15.finish();
    }
}

pub fn serialize_structure_update_dataset_entries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDatasetEntriesInput,
) {
    if let Some(var_18) = &input.changes {
        object
            .key("Changes")
            .string_unchecked(&smithy_types::base64::encode(var_18));
    }
}

pub fn serialize_structure_dataset_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetSource,
) {
    if let Some(var_19) = &input.ground_truth_manifest {
        let mut object_20 = object.key("GroundTruthManifest").start_object();
        crate::json_ser::serialize_structure_dataset_ground_truth_manifest(&mut object_20, var_19);
        object_20.finish();
    }
}

pub fn serialize_structure_output_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputConfig,
) {
    if let Some(var_21) = &input.s3_location {
        let mut object_22 = object.key("S3Location").start_object();
        crate::json_ser::serialize_structure_s3_location(&mut object_22, var_21);
        object_22.finish();
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_23) = &input.key {
        object.key("Key").string(var_23);
    }
    if let Some(var_24) = &input.value {
        object.key("Value").string(var_24);
    }
}

pub fn serialize_structure_dataset_ground_truth_manifest(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetGroundTruthManifest,
) {
    if let Some(var_25) = &input.s3_object {
        let mut object_26 = object.key("S3Object").start_object();
        crate::json_ser::serialize_structure_input_s3_object(&mut object_26, var_25);
        object_26.finish();
    }
}

pub fn serialize_structure_s3_location(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Location,
) {
    if let Some(var_27) = &input.bucket {
        object.key("Bucket").string(var_27);
    }
    if let Some(var_28) = &input.prefix {
        object.key("Prefix").string(var_28);
    }
}

pub fn serialize_structure_input_s3_object(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputS3Object,
) {
    if let Some(var_29) = &input.bucket {
        object.key("Bucket").string(var_29);
    }
    if let Some(var_30) = &input.key {
        object.key("Key").string(var_30);
    }
    if let Some(var_31) = &input.version_id {
        object.key("VersionId").string(var_31);
    }
}
