use wicked_waifus_protocol::{FriendApplySendRequest, FriendApplySendResponse,
                           FriendRecentlyTeamRequest, FriendRecentlyTeamResponse,
                           PlayerBasicInfoGetRequest, PlayerBasicInfoGetResponse};
use crate::logic::player::Player;

pub fn on_friend_apply_send_request(
    _player: &Player,
    _request: FriendApplySendRequest,
    _response: &mut FriendApplySendResponse,
) {

}

pub fn on_friend_recently_team_request(
    _player: &Player,
    _request: FriendRecentlyTeamRequest,
    _response: &mut FriendRecentlyTeamResponse,
) {

}

pub fn on_player_basic_info_get_request(
    _player: &Player,
    _request: PlayerBasicInfoGetRequest,
    _response: &mut PlayerBasicInfoGetResponse,
) {

}