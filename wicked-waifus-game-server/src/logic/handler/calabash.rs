use wicked_waifus_protocol::{
    IllustratedInfoRequest, IllustratedInfoResponse,
    IllustratedClass, IllustratedEntry,
    ErrorCode

};
use crate::logic::player::Player;

use wicked_waifus_data::{calabash_develop_reward_data};


pub fn on_illustrated_info_request(
    _player: &Player,
    _request: IllustratedInfoRequest,
    response: &mut IllustratedInfoResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.illustrated_class_list= vec![
        IllustratedClass {
            r#type: 1,
            illustrated_entry_list: calabash_develop_reward_data::iter()
                .map(|monsters| IllustratedEntry {
                    id: monsters.monster_id,
                    is_read: false,
                    num: 36,
                    create_time: 1716442781,
                    ..Default::default()
                })
                .collect(),
        },
    ];
}