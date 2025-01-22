use wicked_waifus_protocol::{DarkCoastDeliveryRequest, DarkCoastDeliveryResponse, DragonPoolDropItems, ErrorCode, ItemDict, ItemEntry, MapUnlockFieldInfoRequest, MapUnlockFieldInfoResponse};
use crate::logic::player::Player;

pub fn on_dark_coast_delivery_request(
    _player: &mut Player,
    request: DarkCoastDeliveryRequest,
    response: &mut DarkCoastDeliveryResponse,
) {
    // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
    match wicked_waifus_data::dragon_pool_data::get(&request.dragon_pool_id) {
        None => response.error_code = ErrorCode::ErrDragonPoolConf.into(),
        Some(value) => {
            response.error_code = ErrorCode::Success.into();
            response.level_gain = value.core_id;
            response.defeated_guard = value.goal.clone();
            // response.received_guard_reward =
            response.dragon_pool_drop_items = Some(DragonPoolDropItems {
                dragon_pool_id: request.dragon_pool_id,
                q_ss: value.dark_coast_delivery_list.clone(),
                drop_items: vec![ItemDict {
                    items: value.drop_ids.iter()
                        .map(|id| ItemEntry {
                            item_id: *id,
                            item_count: 1,
                        })
                        .collect::<Vec<_>>(),
                }],
            })
        }
    }
}

pub fn on_map_unlock_field_info_request(
    _player: &mut Player,
    _: MapUnlockFieldInfoRequest,
    response: &mut MapUnlockFieldInfoResponse,
) {
    // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
    response.error_code = ErrorCode::Success.into();
    response.field_id = wicked_waifus_data::area_data::iter()
        .map(|area| area.area_id)
        .collect::<Vec<_>>();
}