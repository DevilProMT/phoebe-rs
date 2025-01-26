use wicked_waifus_protocol::{ErrorCode, GuideInfoRequest, GuideInfoResponse, GuideTriggerRequest, GuideTriggerResponse};

use crate::logic::player::Player;

use wicked_waifus_data::{guide_group_data};

pub fn on_guide_info_request(
    _: &Player,
    _: GuideInfoRequest,
    response: &mut GuideInfoResponse,
) {
    let guide_group_data = guide_group_data::iter();
    response.guide_group_finish_list = guide_group_data
        .map(|group| group.id)
        .collect();
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
