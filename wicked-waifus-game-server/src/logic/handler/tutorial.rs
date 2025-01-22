use wicked_waifus_protocol::{ErrorCode, TutorialUnlockRequest, TutorialUnlockResponse};

use crate::logic::player::Player;

pub fn on_tutorial_unlock_request(
    player: &mut Player,
    request: TutorialUnlockRequest,
    response: &mut TutorialUnlockResponse,
) {
    // TODO: Implement this properly checking if guide exist in bindata
    response.error_code = ErrorCode::Success.into();
}
