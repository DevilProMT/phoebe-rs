use std::collections::HashMap;

use crate::logic::player::Player;
use crate::logic::utils::load_role_info::load_key_value;
use crate::{query_components};

use wicked_waifus_protocol::combat_message::{
    combat_receive_data, combat_request_data, combat_response_data, combat_send_data,
    CombatReceiveData, CombatRequestData, CombatResponseData, CombatSendPackRequest,
    CombatSendPackResponse,
};
use wicked_waifus_protocol::{
    ErrorCode, SwitchRoleRequest, SwitchRoleResponse, DamageExecuteRequest, DamageExecuteResponse,
    EAttributeType
};

use wicked_waifus_data::{base_property_data, damage_data};

use crate::logic::{
    components::{
        Attribute
    },
    ecs::component::ComponentContainer,
};

#[inline(always)]
fn create_combat_response(
    combat_request: &CombatRequestData,
    message: combat_response_data::Message,
) -> CombatReceiveData {
    CombatReceiveData {
        message: Some(combat_receive_data::Message::CombatResponseData(
            CombatResponseData {
                combat_common: combat_request.combat_common,
                request_id: combat_request.request_id,
                message: Some(message),
            },
        )),
    }
}

pub fn on_combat_message_combat_send_pack_request(
    player: &mut Player,
    request: CombatSendPackRequest,
    response: &mut CombatSendPackResponse,
) {
    for data in request.data.iter() {
        if let Some(combat_send_data::Message::Request(ref request_data)) = data.message {
            if let Some(ref request_message) = request_data.message {
                match request_message {
                    combat_request_data::Message::SwitchRoleRequest(ref request) => {
                        handle_switch_role_request(player, request_data, request, response);
                    }
                    combat_request_data::Message::DamageExecuteRequest(ref request) => {
                        handle_damage_execute_request(player, request_data, request, response);
                    }
                    _ => {}
                }
            }
        }
    }
    response.error_code = ErrorCode::Success.into();
}

fn handle_switch_role_request(
    player: &mut Player,
    combat_request: &CombatRequestData,
    request: &SwitchRoleRequest,
    response: &mut CombatSendPackResponse,
) {
    // Find current formation and update current role
    if let Some(formation) = player.formation_list.values_mut().find(|f| f.is_current) {
        formation.cur_role = request.role_id;

        let receive_pack = response
            .receive_pack_notify
            .get_or_insert_with(Default::default);

        receive_pack.data.push(create_combat_response(
            combat_request,
            combat_response_data::Message::SwitchRoleResponse(SwitchRoleResponse {
                error_code: ErrorCode::Success.into(),
                role_id: request.role_id,
            }),
        ));
    } else {
        tracing::error!("Role with id {} not found", request.role_id);
        response.error_code = ErrorCode::ErrSwitchRoleEntityNotExist.into();
        return;
    }

    response.error_code = ErrorCode::Success.into();
}

fn handle_damage_execute_request(
    player: &mut Player,
    combat_request: &CombatRequestData,
    request: &DamageExecuteRequest,
    response: &mut CombatSendPackResponse,
) {
    
    let receive_pack = response
        .receive_pack_notify
        .get_or_insert_with(Default::default);
    
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();
    let config_id = world.get_config_id(request.attacker_entity_id.try_into().unwrap());
    let mut damage = 1;

    if config_id.to_string().len() == 4 {
        if let Some(damage_data) = damage_data::iter()
            .find(|d| d.id == request.damage_id)
        {
            let attribute = query_components!(world, request.attacker_entity_id, Attribute).0.unwrap();
            if let Ok(related_attribute) = EAttributeType::try_from(damage_data.related_property) {
                if let Some((value, _)) = attribute.attr_map.get(&related_attribute) {
                    let rate_lv = damage_data.rate_lv[request.skill_level as usize];
                    let hardness_lv = damage_data.hardness_lv[0];
                    tracing::info!(
                        "atk: {}, damage_id: {}, role_id: {}, rate_lv: {}, hardness_lv: {}",
                        value,
                        request.damage_id,
                        config_id,
                        rate_lv,
                        hardness_lv
                    );
                    damage = if hardness_lv == 0 || rate_lv <= 0 {
                        1
                    } else {
                        ((rate_lv as f32 / hardness_lv as f32) * 100.0 + (*value as f32)) as i32
                    };
                }
            };
        }
    } 

    receive_pack.data.push(create_combat_response(
        combat_request,
        combat_response_data::Message::DamageExecuteResponse(DamageExecuteResponse {
            error_code: ErrorCode::Success.into(),
            attacker_entity_id: request.attacker_entity_id,
            target_entity_id: request.target_entity_id,
            part_index: request.part_index,
            damage: damage,
            ..Default::default()
        }),
    ));
    

    response.error_code = ErrorCode::Success.into();
}