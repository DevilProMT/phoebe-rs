use wicked_waifus_protocol::{
    ErrorCode, SceneLoadingFinishRequest, SceneLoadingFinishResponse, SceneTraceRequest,
    SceneTraceResponse, UpdateSceneDateRequest, UpdateSceneDateResponse,
    AccessPathTimeServerConfigRequest, AccessPathTimeServerConfigResponse,
    PlayerHeadDataRequest, PlayerHeadDataResponse, UnlockRoleSkinListRequest,
    UnlockRoleSkinListResponse
};

use crate::logic::player::Player;

pub fn on_scene_trace_request(
    _player: &Player,
    request: SceneTraceRequest,
    _: &mut SceneTraceResponse,
) {
    tracing::debug!("SceneTraceRequest: trace id {}", request.scene_trace_id);
}

pub fn on_scene_loading_finish_request(
    _player: &Player,
    _request: SceneLoadingFinishRequest,
    response: &mut SceneLoadingFinishResponse,
) {
    // TODO: Implement this if needed
    response.error_code = ErrorCode::Success.into();
}

pub fn on_update_scene_date_request(
    _player: &Player,
    _request: UpdateSceneDateRequest,
    response: &mut UpdateSceneDateResponse,
) {
    // TODO: port this from golang
    response.error_code = ErrorCode::Success.into();
}

pub fn on_access_path_time_server_config_request(
    _player: &Player,
    _request: AccessPathTimeServerConfigRequest,
    response: &mut AccessPathTimeServerConfigResponse,
) {
    // TODO: port this from golang
    response.access_path_time_server_config = vec![];
}

pub fn on_player_head_data_request(
    _player: &Player,
    _request: PlayerHeadDataRequest,
    response: &mut PlayerHeadDataResponse,
) {
    // TODO: port this from golang
    response.pi = vec![];
}

pub fn on_unlock_role_skin_list_request(
    _player: &Player,
    _request: UnlockRoleSkinListRequest,
    response: &mut UnlockRoleSkinListResponse,
) {
    // TODO: port this from golang
    response.phantom_skin_list = vec![];
}