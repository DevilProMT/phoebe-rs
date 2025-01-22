use wicked_waifus_protocol::{ErrorCode, GuideInfoRequest, GuideInfoResponse, GuideTriggerRequest, GuideTriggerResponse};

use crate::logic::player::Player;

pub fn on_guide_info_request(
    player: &Player,
    _: GuideInfoRequest,
    response: &mut GuideInfoResponse,
) {
    response.guide_group_finish_list = player.guides.finished_guides.clone();
}

pub fn on_guide_trigger_request(
    player: &mut Player,
    request: GuideTriggerRequest,
    response: &mut GuideTriggerResponse,
) {
    // TODO: Implement this properly checking if guide exist in bindata
    player.guides.finished_guides.push(request.group_id);
    response.error_code = ErrorCode::Success.into();
}
