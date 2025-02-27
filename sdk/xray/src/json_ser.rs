// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_batch_get_traces_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetTracesInput,
) {
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1);
    }
    if let Some(var_2) = &input.trace_ids {
        let mut array_3 = object.key("TraceIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4);
            }
        }
        array_3.finish();
    }
}

pub fn serialize_structure_create_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGroupInput,
) {
    if let Some(var_5) = &input.filter_expression {
        object.key("FilterExpression").string(var_5);
    }
    if let Some(var_6) = &input.group_name {
        object.key("GroupName").string(var_6);
    }
    if let Some(var_7) = &input.insights_configuration {
        let mut object_8 = object.key("InsightsConfiguration").start_object();
        crate::json_ser::serialize_structure_insights_configuration(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("Tags").start_array();
        for item_11 in var_9 {
            {
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_12, item_11);
                object_12.finish();
            }
        }
        array_10.finish();
    }
}

pub fn serialize_structure_create_sampling_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSamplingRuleInput,
) {
    if let Some(var_13) = &input.sampling_rule {
        let mut object_14 = object.key("SamplingRule").start_object();
        crate::json_ser::serialize_structure_sampling_rule(&mut object_14, var_13);
        object_14.finish();
    }
    if let Some(var_15) = &input.tags {
        let mut array_16 = object.key("Tags").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_18, item_17);
                object_18.finish();
            }
        }
        array_16.finish();
    }
}

pub fn serialize_structure_delete_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteGroupInput,
) {
    if let Some(var_19) = &input.group_arn {
        object.key("GroupARN").string(var_19);
    }
    if let Some(var_20) = &input.group_name {
        object.key("GroupName").string(var_20);
    }
}

pub fn serialize_structure_delete_sampling_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSamplingRuleInput,
) {
    if let Some(var_21) = &input.rule_arn {
        object.key("RuleARN").string(var_21);
    }
    if let Some(var_22) = &input.rule_name {
        object.key("RuleName").string(var_22);
    }
}

pub fn serialize_structure_get_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetGroupInput,
) {
    if let Some(var_23) = &input.group_arn {
        object.key("GroupARN").string(var_23);
    }
    if let Some(var_24) = &input.group_name {
        object.key("GroupName").string(var_24);
    }
}

pub fn serialize_structure_get_groups_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetGroupsInput,
) {
    if let Some(var_25) = &input.next_token {
        object.key("NextToken").string(var_25);
    }
}

pub fn serialize_structure_get_insight_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetInsightInput,
) {
    if let Some(var_26) = &input.insight_id {
        object.key("InsightId").string(var_26);
    }
}

pub fn serialize_structure_get_insight_events_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetInsightEventsInput,
) {
    if let Some(var_27) = &input.insight_id {
        object.key("InsightId").string(var_27);
    }
    if let Some(var_28) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_28).into()),
        );
    }
    if let Some(var_29) = &input.next_token {
        object.key("NextToken").string(var_29);
    }
}

pub fn serialize_structure_get_insight_impact_graph_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetInsightImpactGraphInput,
) {
    if let Some(var_30) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_30, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_31) = &input.insight_id {
        object.key("InsightId").string(var_31);
    }
    if let Some(var_32) = &input.next_token {
        object.key("NextToken").string(var_32);
    }
    if let Some(var_33) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_33, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_get_insight_summaries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetInsightSummariesInput,
) {
    if let Some(var_34) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_34, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_35) = &input.group_arn {
        object.key("GroupARN").string(var_35);
    }
    if let Some(var_36) = &input.group_name {
        object.key("GroupName").string(var_36);
    }
    if let Some(var_37) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.next_token {
        object.key("NextToken").string(var_38);
    }
    if let Some(var_39) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_39, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_40) = &input.states {
        let mut array_41 = object.key("States").start_array();
        for item_42 in var_40 {
            {
                array_41.value().string(item_42.as_str());
            }
        }
        array_41.finish();
    }
}

pub fn serialize_structure_get_sampling_rules_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSamplingRulesInput,
) {
    if let Some(var_43) = &input.next_token {
        object.key("NextToken").string(var_43);
    }
}

pub fn serialize_structure_get_sampling_statistic_summaries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSamplingStatisticSummariesInput,
) {
    if let Some(var_44) = &input.next_token {
        object.key("NextToken").string(var_44);
    }
}

pub fn serialize_structure_get_sampling_targets_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSamplingTargetsInput,
) {
    if let Some(var_45) = &input.sampling_statistics_documents {
        let mut array_46 = object.key("SamplingStatisticsDocuments").start_array();
        for item_47 in var_45 {
            {
                let mut object_48 = array_46.value().start_object();
                crate::json_ser::serialize_structure_sampling_statistics_document(
                    &mut object_48,
                    item_47,
                );
                object_48.finish();
            }
        }
        array_46.finish();
    }
}

pub fn serialize_structure_get_service_graph_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceGraphInput,
) {
    if let Some(var_49) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_49, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_50) = &input.group_arn {
        object.key("GroupARN").string(var_50);
    }
    if let Some(var_51) = &input.group_name {
        object.key("GroupName").string(var_51);
    }
    if let Some(var_52) = &input.next_token {
        object.key("NextToken").string(var_52);
    }
    if let Some(var_53) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_53, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_get_time_series_service_statistics_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTimeSeriesServiceStatisticsInput,
) {
    if let Some(var_54) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_54, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_55) = &input.entity_selector_expression {
        object.key("EntitySelectorExpression").string(var_55);
    }
    if let Some(var_56) = &input.forecast_statistics {
        object.key("ForecastStatistics").boolean(*var_56);
    }
    if let Some(var_57) = &input.group_arn {
        object.key("GroupARN").string(var_57);
    }
    if let Some(var_58) = &input.group_name {
        object.key("GroupName").string(var_58);
    }
    if let Some(var_59) = &input.next_token {
        object.key("NextToken").string(var_59);
    }
    if let Some(var_60) = &input.period {
        object.key("Period").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_60).into()),
        );
    }
    if let Some(var_61) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_61, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_get_trace_graph_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTraceGraphInput,
) {
    if let Some(var_62) = &input.next_token {
        object.key("NextToken").string(var_62);
    }
    if let Some(var_63) = &input.trace_ids {
        let mut array_64 = object.key("TraceIds").start_array();
        for item_65 in var_63 {
            {
                array_64.value().string(item_65);
            }
        }
        array_64.finish();
    }
}

pub fn serialize_structure_get_trace_summaries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTraceSummariesInput,
) {
    if let Some(var_66) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_66, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_67) = &input.filter_expression {
        object.key("FilterExpression").string(var_67);
    }
    if let Some(var_68) = &input.next_token {
        object.key("NextToken").string(var_68);
    }
    if let Some(var_69) = &input.sampling {
        object.key("Sampling").boolean(*var_69);
    }
    if let Some(var_70) = &input.sampling_strategy {
        let mut object_71 = object.key("SamplingStrategy").start_object();
        crate::json_ser::serialize_structure_sampling_strategy(&mut object_71, var_70);
        object_71.finish();
    }
    if let Some(var_72) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_72, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_73) = &input.time_range_type {
        object.key("TimeRangeType").string(var_73.as_str());
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_74) = &input.next_token {
        object.key("NextToken").string(var_74);
    }
    if let Some(var_75) = &input.resource_arn {
        object.key("ResourceARN").string(var_75);
    }
}

pub fn serialize_structure_put_encryption_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutEncryptionConfigInput,
) {
    if let Some(var_76) = &input.key_id {
        object.key("KeyId").string(var_76);
    }
    if let Some(var_77) = &input.r#type {
        object.key("Type").string(var_77.as_str());
    }
}

pub fn serialize_structure_put_telemetry_records_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutTelemetryRecordsInput,
) {
    if let Some(var_78) = &input.ec2_instance_id {
        object.key("EC2InstanceId").string(var_78);
    }
    if let Some(var_79) = &input.hostname {
        object.key("Hostname").string(var_79);
    }
    if let Some(var_80) = &input.resource_arn {
        object.key("ResourceARN").string(var_80);
    }
    if let Some(var_81) = &input.telemetry_records {
        let mut array_82 = object.key("TelemetryRecords").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_telemetry_record(&mut object_84, item_83);
                object_84.finish();
            }
        }
        array_82.finish();
    }
}

pub fn serialize_structure_put_trace_segments_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutTraceSegmentsInput,
) {
    if let Some(var_85) = &input.trace_segment_documents {
        let mut array_86 = object.key("TraceSegmentDocuments").start_array();
        for item_87 in var_85 {
            {
                array_86.value().string(item_87);
            }
        }
        array_86.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_88) = &input.resource_arn {
        object.key("ResourceARN").string(var_88);
    }
    if let Some(var_89) = &input.tags {
        let mut array_90 = object.key("Tags").start_array();
        for item_91 in var_89 {
            {
                let mut object_92 = array_90.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_92, item_91);
                object_92.finish();
            }
        }
        array_90.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_93) = &input.resource_arn {
        object.key("ResourceARN").string(var_93);
    }
    if let Some(var_94) = &input.tag_keys {
        let mut array_95 = object.key("TagKeys").start_array();
        for item_96 in var_94 {
            {
                array_95.value().string(item_96);
            }
        }
        array_95.finish();
    }
}

pub fn serialize_structure_update_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGroupInput,
) {
    if let Some(var_97) = &input.filter_expression {
        object.key("FilterExpression").string(var_97);
    }
    if let Some(var_98) = &input.group_arn {
        object.key("GroupARN").string(var_98);
    }
    if let Some(var_99) = &input.group_name {
        object.key("GroupName").string(var_99);
    }
    if let Some(var_100) = &input.insights_configuration {
        let mut object_101 = object.key("InsightsConfiguration").start_object();
        crate::json_ser::serialize_structure_insights_configuration(&mut object_101, var_100);
        object_101.finish();
    }
}

pub fn serialize_structure_update_sampling_rule_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSamplingRuleInput,
) {
    if let Some(var_102) = &input.sampling_rule_update {
        let mut object_103 = object.key("SamplingRuleUpdate").start_object();
        crate::json_ser::serialize_structure_sampling_rule_update(&mut object_103, var_102);
        object_103.finish();
    }
}

pub fn serialize_structure_insights_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InsightsConfiguration,
) {
    if let Some(var_104) = &input.insights_enabled {
        object.key("InsightsEnabled").boolean(*var_104);
    }
    if let Some(var_105) = &input.notifications_enabled {
        object.key("NotificationsEnabled").boolean(*var_105);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_106) = &input.key {
        object.key("Key").string(var_106);
    }
    if let Some(var_107) = &input.value {
        object.key("Value").string(var_107);
    }
}

pub fn serialize_structure_sampling_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamplingRule,
) {
    if let Some(var_108) = &input.rule_name {
        object.key("RuleName").string(var_108);
    }
    if let Some(var_109) = &input.rule_arn {
        object.key("RuleARN").string(var_109);
    }
    if let Some(var_110) = &input.resource_arn {
        object.key("ResourceARN").string(var_110);
    }
    {
        object.key("Priority").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.priority).into()),
        );
    }
    {
        object.key("FixedRate").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.fixed_rate).into()),
        );
    }
    {
        object.key("ReservoirSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.reservoir_size).into()),
        );
    }
    if let Some(var_111) = &input.service_name {
        object.key("ServiceName").string(var_111);
    }
    if let Some(var_112) = &input.service_type {
        object.key("ServiceType").string(var_112);
    }
    if let Some(var_113) = &input.host {
        object.key("Host").string(var_113);
    }
    if let Some(var_114) = &input.http_method {
        object.key("HTTPMethod").string(var_114);
    }
    if let Some(var_115) = &input.url_path {
        object.key("URLPath").string(var_115);
    }
    {
        object.key("Version").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.version).into()),
        );
    }
    if let Some(var_116) = &input.attributes {
        let mut object_117 = object.key("Attributes").start_object();
        for (key_118, value_119) in var_116 {
            {
                object_117.key(key_118).string(value_119);
            }
        }
        object_117.finish();
    }
}

pub fn serialize_structure_sampling_statistics_document(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamplingStatisticsDocument,
) {
    if let Some(var_120) = &input.rule_name {
        object.key("RuleName").string(var_120);
    }
    if let Some(var_121) = &input.client_id {
        object.key("ClientID").string(var_121);
    }
    if let Some(var_122) = &input.timestamp {
        object
            .key("Timestamp")
            .instant(var_122, smithy_types::instant::Format::EpochSeconds);
    }
    {
        object.key("RequestCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.request_count).into()),
        );
    }
    {
        object.key("SampledCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.sampled_count).into()),
        );
    }
    if input.borrow_count != 0 {
        object.key("BorrowCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.borrow_count).into()),
        );
    }
}

pub fn serialize_structure_sampling_strategy(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamplingStrategy,
) {
    if let Some(var_123) = &input.name {
        object.key("Name").string(var_123.as_str());
    }
    if let Some(var_124) = &input.value {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_124).into()),
        );
    }
}

pub fn serialize_structure_telemetry_record(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TelemetryRecord,
) {
    if let Some(var_125) = &input.timestamp {
        object
            .key("Timestamp")
            .instant(var_125, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_126) = &input.segments_received_count {
        object.key("SegmentsReceivedCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_126).into()),
        );
    }
    if let Some(var_127) = &input.segments_sent_count {
        object.key("SegmentsSentCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_127).into()),
        );
    }
    if let Some(var_128) = &input.segments_spillover_count {
        object.key("SegmentsSpilloverCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_128).into()),
        );
    }
    if let Some(var_129) = &input.segments_rejected_count {
        object.key("SegmentsRejectedCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_129).into()),
        );
    }
    if let Some(var_130) = &input.backend_connection_errors {
        let mut object_131 = object.key("BackendConnectionErrors").start_object();
        crate::json_ser::serialize_structure_backend_connection_errors(&mut object_131, var_130);
        object_131.finish();
    }
}

pub fn serialize_structure_sampling_rule_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamplingRuleUpdate,
) {
    if let Some(var_132) = &input.rule_name {
        object.key("RuleName").string(var_132);
    }
    if let Some(var_133) = &input.rule_arn {
        object.key("RuleARN").string(var_133);
    }
    if let Some(var_134) = &input.resource_arn {
        object.key("ResourceARN").string(var_134);
    }
    if let Some(var_135) = &input.priority {
        object.key("Priority").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_135).into()),
        );
    }
    if let Some(var_136) = &input.fixed_rate {
        object.key("FixedRate").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_136).into()),
        );
    }
    if let Some(var_137) = &input.reservoir_size {
        object.key("ReservoirSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_137).into()),
        );
    }
    if let Some(var_138) = &input.host {
        object.key("Host").string(var_138);
    }
    if let Some(var_139) = &input.service_name {
        object.key("ServiceName").string(var_139);
    }
    if let Some(var_140) = &input.service_type {
        object.key("ServiceType").string(var_140);
    }
    if let Some(var_141) = &input.http_method {
        object.key("HTTPMethod").string(var_141);
    }
    if let Some(var_142) = &input.url_path {
        object.key("URLPath").string(var_142);
    }
    if let Some(var_143) = &input.attributes {
        let mut object_144 = object.key("Attributes").start_object();
        for (key_145, value_146) in var_143 {
            {
                object_144.key(key_145).string(value_146);
            }
        }
        object_144.finish();
    }
}

pub fn serialize_structure_backend_connection_errors(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendConnectionErrors,
) {
    if let Some(var_147) = &input.timeout_count {
        object.key("TimeoutCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_147).into()),
        );
    }
    if let Some(var_148) = &input.connection_refused_count {
        object.key("ConnectionRefusedCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_148).into()),
        );
    }
    if let Some(var_149) = &input.http_code4_xx_count {
        object.key("HTTPCode4XXCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_149).into()),
        );
    }
    if let Some(var_150) = &input.http_code5_xx_count {
        object.key("HTTPCode5XXCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_150).into()),
        );
    }
    if let Some(var_151) = &input.unknown_host_count {
        object.key("UnknownHostCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_151).into()),
        );
    }
    if let Some(var_152) = &input.other_count {
        object.key("OtherCount").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_152).into()),
        );
    }
}
