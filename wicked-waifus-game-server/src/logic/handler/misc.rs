use wicked_waifus_protocol::{
    ErrorCode, InputSettingRequest, InputSettingResponse, InputSettingUpdateRequest,
    InputSettingUpdateResponse, LanguageSettingUpdateRequest, LanguageSettingUpdateResponse,
    ServerPlayStationPlayOnlyStateRequest, ServerPlayStationPlayOnlyStateResponse, VersionInfoPush,
    AdventureManualRequest, AdventureManualResponse, AdventureManualData, AdventreTask, DetectionTarget,
    DetectionUnlock, ExchangeRewardRequest, ExchangeRewardResponse, ForgeInfoRequest, ForgeInfoResponse,
    OneForgeInfo, GetDetectionLabelInfoRequest, GetDetectionLabelInfoResponse, UnlockDetectionLabelInfo,
    ItemExchangeInfoRequest, ItemExchangeInfoResponse, ItemExchangeInfo, LivenessRequest, LivenessResponse,
    LivenessInfo, LivenessTask, SynthesisInfoRequest, SynthesisInfoResponse, OneSynthesisInfo, OneSynthesisConfig,
    SynthesisLevelInfo, 
};

use crate::logic::player::Player;

use wicked_waifus_data::{
    adventure_task_data,dungeon_detection_data,monster_detection_data,
    silent_area_detection_data, exchange_reward_data, exchange_shared_data,
    forge_formula_data, item_exchange_content_data, liveness_task_data, 
    synthesis_formula_data, 
};

pub fn on_input_setting_request(
    _: &Player,
    _: InputSettingRequest,
    _: &mut InputSettingResponse,
) {}

pub fn on_input_setting_update_request(
    _: &Player,
    _: InputSettingUpdateRequest,
    response: &mut InputSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_language_setting_update_request(
    _: &Player,
    _: LanguageSettingUpdateRequest,
    response: &mut LanguageSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_server_play_station_play_only_state_request(
    _: &Player,
    _: ServerPlayStationPlayOnlyStateRequest,
    response: &mut ServerPlayStationPlayOnlyStateResponse,
) {
    response.cross_play_enabled = false;
}

pub fn on_adventure_manual_request(
    _: &Player,
    _: AdventureManualRequest,
    response: &mut AdventureManualResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.adventure_manual_data = Some(
        AdventureManualData {
            adventre_task: adventure_task_data::iter()
                .map(|task| AdventreTask {
                    adventre_progress: 1,
                    id: task.id,
                    state: 2,
                })
                .collect(),
            now_chapter: 9,
            received_chapter: 9,
        },
    );
    response.detection_target = dungeon_detection_data::iter()
        .map(|detect| DetectionTarget {
            detection_id: detect.id,
            id: detect.dungeon_id,
            is_trace: 8,
            unlock_state: true,
            r#type: 1,
            ..Default::default()
        })
        .collect();
    response.detection_unlocks = Some(DetectionUnlock {
        dungeon_detection_ids: dungeon_detection_data::iter()
            .map(|detect| detect.id)
            .collect(),
        monster_detection_ids: monster_detection_data::iter()
            .map(|detect| detect.id)
            .collect(),
        silent_area_detection_ids: silent_area_detection_data::iter()
            .map(|detect| detect.id)
            .collect(),
    });
}

pub fn on_exchange_reward_request(
    _: &Player,
    _: ExchangeRewardRequest,
    response: &mut ExchangeRewardResponse,
) {
    response.exchange_reward_data = exchange_reward_data::iter()
        .map(|exc| (exc.id, exc.max_count))
        .collect::<std::collections::HashMap<_, _>>();
        
    response.exchange_share_data = exchange_shared_data::iter()
    .map(|exc| (exc.id, exc.max_count))
    .collect::<std::collections::HashMap<_, _>>();
}

pub fn on_forge_info_request(
    _: &Player,
    _: ForgeInfoRequest,
    response: &mut ForgeInfoResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.forge_info_list = forge_formula_data::iter()
        .map(|formula| OneForgeInfo {
            id: formula.id,
            ..Default::default()
        })
        .collect::<Vec<_>>();
}

pub fn on_get_detection_label_info_request(
    _: &Player,
    _: GetDetectionLabelInfoRequest,
    response: &mut GetDetectionLabelInfoResponse,
) {
    // TODO
    response.unlock_label_info = Some(UnlockDetectionLabelInfo {
        unlocked_detection_text_ids: vec![0,1,2,3,14,15,16,18,19,6,62,8,9,10,11,12,13,17,21,7,5,22,4,61],
        unlocked_guide_ids: vec![0,1,2,3,4],
    });
}

pub fn on_item_exchange_info_request(
    _: &Player,
    _: ItemExchangeInfoRequest,
    response: &mut ItemExchangeInfoResponse,
) {
    response.item_exchange_infos = item_exchange_content_data::iter()
        .map(|item| ItemExchangeInfo {
            item_id: item.item_id,
            total_times: 50,
            ..Default::default()
        })
        .collect::<Vec<_>>();
}

pub fn on_liveness_request(
    _: &Player,
    _: LivenessRequest,
    response: &mut LivenessResponse,
) {
    response.liveness_info = Some(LivenessInfo {
        area_id: -1,
        day_end: 1761382800,
        tasks: liveness_task_data::iter()
            .map(|task| LivenessTask {
                id: task.task_id,
                target: 1,
                bxs: true,
                ..Default::default()
            })
            .collect::<Vec<_>>(),
        ..Default::default()
    });
}

pub fn on_synthesis_info_request(
    _: &Player,
    _: SynthesisInfoRequest,
    response: &mut SynthesisInfoResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.synthesis_info_list = synthesis_formula_data::iter()
        .map(|item| OneSynthesisInfo {
            id: item.id,
            count: 20,
            limit_total_count: if [6, 7].contains(&item.id) {
                10
            } else if [8, 9].contains(&item.id) {
                8
            } else {
                0
            },
            limit_synthesis_count: 0,
            exist_start_time: 0,
            exist_end_time: 1752000000,
            ..Default::default()
        })
        .collect();
    response.synthesis_configs = synthesis_formula_data::iter()
        .map(|item| OneSynthesisConfig {
            id: item.id,
            exist_start_time: 0,
            exist_end_time: 1752000000,
        })
        .collect();
    response.level_info = Some(SynthesisLevelInfo {
        level: 5,
        total_proficiency: 6000,
    });
    response.limit_refresh_time = 1752000000;
}

pub fn on_version_info_push(_player: &Player, push: VersionInfoPush) {
    // TODO: Shall we do safety check and ensure we have compatible versions?
    tracing::debug!(
        "Client versions: launcher: {}, app: {}, resources: {}",
        push.launcher_version,
        push.app_version,
        push.resource_version
    );
}
