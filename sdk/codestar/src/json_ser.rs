// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_associate_team_member_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateTeamMemberInput,
) {
    if let Some(var_1) = &input.project_id {
        object.key("projectId").string(var_1);
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("clientRequestToken").string(var_2);
    }
    if let Some(var_3) = &input.user_arn {
        object.key("userArn").string(var_3);
    }
    if let Some(var_4) = &input.project_role {
        object.key("projectRole").string(var_4);
    }
    if let Some(var_5) = &input.remote_access_allowed {
        object.key("remoteAccessAllowed").boolean(*var_5);
    }
}

pub fn serialize_structure_create_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProjectInput,
) {
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6);
    }
    if let Some(var_7) = &input.id {
        object.key("id").string(var_7);
    }
    if let Some(var_8) = &input.description {
        object.key("description").string(var_8);
    }
    if let Some(var_9) = &input.client_request_token {
        object.key("clientRequestToken").string(var_9);
    }
    if let Some(var_10) = &input.source_code {
        let mut array_11 = object.key("sourceCode").start_array();
        for item_12 in var_10 {
            {
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_code(&mut object_13, item_12);
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.toolchain {
        let mut object_15 = object.key("toolchain").start_object();
        crate::json_ser::serialize_structure_toolchain(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.tags {
        let mut object_17 = object.key("tags").start_object();
        for (key_18, value_19) in var_16 {
            {
                object_17.key(key_18).string(value_19);
            }
        }
        object_17.finish();
    }
}

pub fn serialize_structure_create_user_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserProfileInput,
) {
    if let Some(var_20) = &input.user_arn {
        object.key("userArn").string(var_20);
    }
    if let Some(var_21) = &input.display_name {
        object.key("displayName").string(var_21);
    }
    if let Some(var_22) = &input.email_address {
        object.key("emailAddress").string(var_22);
    }
    if let Some(var_23) = &input.ssh_public_key {
        object.key("sshPublicKey").string(var_23);
    }
}

pub fn serialize_structure_delete_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteProjectInput,
) {
    if let Some(var_24) = &input.id {
        object.key("id").string(var_24);
    }
    if let Some(var_25) = &input.client_request_token {
        object.key("clientRequestToken").string(var_25);
    }
    if input.delete_stack {
        object.key("deleteStack").boolean(input.delete_stack);
    }
}

pub fn serialize_structure_delete_user_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteUserProfileInput,
) {
    if let Some(var_26) = &input.user_arn {
        object.key("userArn").string(var_26);
    }
}

pub fn serialize_structure_describe_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeProjectInput,
) {
    if let Some(var_27) = &input.id {
        object.key("id").string(var_27);
    }
}

pub fn serialize_structure_describe_user_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeUserProfileInput,
) {
    if let Some(var_28) = &input.user_arn {
        object.key("userArn").string(var_28);
    }
}

pub fn serialize_structure_disassociate_team_member_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateTeamMemberInput,
) {
    if let Some(var_29) = &input.project_id {
        object.key("projectId").string(var_29);
    }
    if let Some(var_30) = &input.user_arn {
        object.key("userArn").string(var_30);
    }
}

pub fn serialize_structure_list_projects_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListProjectsInput,
) {
    if let Some(var_31) = &input.next_token {
        object.key("nextToken").string(var_31);
    }
    if let Some(var_32) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_32).into()),
        );
    }
}

pub fn serialize_structure_list_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResourcesInput,
) {
    if let Some(var_33) = &input.project_id {
        object.key("projectId").string(var_33);
    }
    if let Some(var_34) = &input.next_token {
        object.key("nextToken").string(var_34);
    }
    if let Some(var_35) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_35).into()),
        );
    }
}

pub fn serialize_structure_list_tags_for_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForProjectInput,
) {
    if let Some(var_36) = &input.id {
        object.key("id").string(var_36);
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37);
    }
    if let Some(var_38) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_38).into()),
        );
    }
}

pub fn serialize_structure_list_team_members_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTeamMembersInput,
) {
    if let Some(var_39) = &input.project_id {
        object.key("projectId").string(var_39);
    }
    if let Some(var_40) = &input.next_token {
        object.key("nextToken").string(var_40);
    }
    if let Some(var_41) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_41).into()),
        );
    }
}

pub fn serialize_structure_list_user_profiles_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListUserProfilesInput,
) {
    if let Some(var_42) = &input.next_token {
        object.key("nextToken").string(var_42);
    }
    if let Some(var_43) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_43).into()),
        );
    }
}

pub fn serialize_structure_tag_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagProjectInput,
) {
    if let Some(var_44) = &input.id {
        object.key("id").string(var_44);
    }
    if let Some(var_45) = &input.tags {
        let mut object_46 = object.key("tags").start_object();
        for (key_47, value_48) in var_45 {
            {
                object_46.key(key_47).string(value_48);
            }
        }
        object_46.finish();
    }
}

pub fn serialize_structure_untag_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagProjectInput,
) {
    if let Some(var_49) = &input.id {
        object.key("id").string(var_49);
    }
    if let Some(var_50) = &input.tags {
        let mut array_51 = object.key("tags").start_array();
        for item_52 in var_50 {
            {
                array_51.value().string(item_52);
            }
        }
        array_51.finish();
    }
}

pub fn serialize_structure_update_project_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateProjectInput,
) {
    if let Some(var_53) = &input.id {
        object.key("id").string(var_53);
    }
    if let Some(var_54) = &input.name {
        object.key("name").string(var_54);
    }
    if let Some(var_55) = &input.description {
        object.key("description").string(var_55);
    }
}

pub fn serialize_structure_update_team_member_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTeamMemberInput,
) {
    if let Some(var_56) = &input.project_id {
        object.key("projectId").string(var_56);
    }
    if let Some(var_57) = &input.user_arn {
        object.key("userArn").string(var_57);
    }
    if let Some(var_58) = &input.project_role {
        object.key("projectRole").string(var_58);
    }
    if let Some(var_59) = &input.remote_access_allowed {
        object.key("remoteAccessAllowed").boolean(*var_59);
    }
}

pub fn serialize_structure_update_user_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserProfileInput,
) {
    if let Some(var_60) = &input.user_arn {
        object.key("userArn").string(var_60);
    }
    if let Some(var_61) = &input.display_name {
        object.key("displayName").string(var_61);
    }
    if let Some(var_62) = &input.email_address {
        object.key("emailAddress").string(var_62);
    }
    if let Some(var_63) = &input.ssh_public_key {
        object.key("sshPublicKey").string(var_63);
    }
}

pub fn serialize_structure_code(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Code,
) {
    if let Some(var_64) = &input.source {
        let mut object_65 = object.key("source").start_object();
        crate::json_ser::serialize_structure_code_source(&mut object_65, var_64);
        object_65.finish();
    }
    if let Some(var_66) = &input.destination {
        let mut object_67 = object.key("destination").start_object();
        crate::json_ser::serialize_structure_code_destination(&mut object_67, var_66);
        object_67.finish();
    }
}

pub fn serialize_structure_toolchain(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Toolchain,
) {
    if let Some(var_68) = &input.source {
        let mut object_69 = object.key("source").start_object();
        crate::json_ser::serialize_structure_toolchain_source(&mut object_69, var_68);
        object_69.finish();
    }
    if let Some(var_70) = &input.role_arn {
        object.key("roleArn").string(var_70);
    }
    if let Some(var_71) = &input.stack_parameters {
        let mut object_72 = object.key("stackParameters").start_object();
        for (key_73, value_74) in var_71 {
            {
                object_72.key(key_73).string(value_74);
            }
        }
        object_72.finish();
    }
}

pub fn serialize_structure_code_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeSource,
) {
    if let Some(var_75) = &input.s3 {
        let mut object_76 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_s3_location(&mut object_76, var_75);
        object_76.finish();
    }
}

pub fn serialize_structure_code_destination(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeDestination,
) {
    if let Some(var_77) = &input.code_commit {
        let mut object_78 = object.key("codeCommit").start_object();
        crate::json_ser::serialize_structure_code_commit_code_destination(&mut object_78, var_77);
        object_78.finish();
    }
    if let Some(var_79) = &input.git_hub {
        let mut object_80 = object.key("gitHub").start_object();
        crate::json_ser::serialize_structure_git_hub_code_destination(&mut object_80, var_79);
        object_80.finish();
    }
}

pub fn serialize_structure_toolchain_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ToolchainSource,
) {
    if let Some(var_81) = &input.s3 {
        let mut object_82 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_s3_location(&mut object_82, var_81);
        object_82.finish();
    }
}

pub fn serialize_structure_s3_location(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Location,
) {
    if let Some(var_83) = &input.bucket_name {
        object.key("bucketName").string(var_83);
    }
    if let Some(var_84) = &input.bucket_key {
        object.key("bucketKey").string(var_84);
    }
}

pub fn serialize_structure_code_commit_code_destination(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeCommitCodeDestination,
) {
    if let Some(var_85) = &input.name {
        object.key("name").string(var_85);
    }
}

pub fn serialize_structure_git_hub_code_destination(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::GitHubCodeDestination,
) {
    if let Some(var_86) = &input.name {
        object.key("name").string(var_86);
    }
    if let Some(var_87) = &input.description {
        object.key("description").string(var_87);
    }
    if let Some(var_88) = &input.r#type {
        object.key("type").string(var_88);
    }
    if let Some(var_89) = &input.owner {
        object.key("owner").string(var_89);
    }
    {
        object
            .key("privateRepository")
            .boolean(input.private_repository);
    }
    {
        object.key("issuesEnabled").boolean(input.issues_enabled);
    }
    if let Some(var_90) = &input.token {
        object.key("token").string(var_90);
    }
}
