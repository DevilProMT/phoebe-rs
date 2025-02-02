use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub use in_world_player::InWorldPlayer;
use wicked_waifus_protocol_internal::{PlayerBasicData, PlayerRoleData, PlayerSaveData};
use wicked_waifus_protocol::{
    AdventreTask, AdventureManualData, AdventureUpdateNotify, AdviceSettingNotify, ControlInfoNotify, 
    EEntityType, EntityAddNotify, EntityConfigType, EntityPb, EntityRemoveInfo, EntityRemoveNotify, 
    EntityState, ERemoveEntityType, FightFormationNotifyInfo, FightRoleInfo, FightRoleInfos, FormationRoleInfo, 
    GroupFormation, InstDataNotify, ItemPkgOpenNotify, LivingStatus, MapUnlockFieldNotify, PbGetRoleListNotify, 
    PlayerFightFormations, ProtocolUnit, RoleChangeUnlockNotify, UpdateFormationNotify, UpdateGroupFormationNotify,
    CalabashMsgNotify, CalabashCfg, CalabashMsg, CalabashDevelopInfo, CalabashDevelopConditionState,
};
use wicked_waifus_protocol::message::Message;
use wicked_waifus_commons::time_util;
use wicked_waifus_data::{
    base_property_data,role_info_data,
    calabash_level_data, calabash_develop_reward_data,
};

use crate::{create_player_entity_pb, query_components};
use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, FightBuff, Movement, OwnerPlayer, PlayerEntityMarker,
        Position, Visibility, VisionSkill,
    },
    ecs::component::ComponentContainer,
};
use crate::logic::components::RoleSkin;
use crate::logic::ecs::world::WorldEntity;
use crate::logic::player::basic_info::PlayerBasicInfo;
use crate::logic::player::explore_tools::ExploreTools;
use crate::logic::player::location::PlayerLocation;
use crate::logic::player::player_chat::PlayerChat;
use crate::logic::player::player_func::PlayerFunc;
use crate::logic::player::guides::Guides;
use crate::session::Session;

use super::{
    ecs::world::World,
    role::{Role, RoleFormation},
};

mod basic_info;
mod explore_tools;
mod in_world_player;
mod location;
mod player_func;
mod player_chat;
mod guides;

pub struct Player {
    session: Option<Arc<Session>>,
    // Persistent
    pub basic_info: PlayerBasicInfo,
    pub role_list: HashMap<i32, Role>,
    pub formation_list: HashMap<i32, RoleFormation>,
    pub cur_formation_id: i32,
    pub location: PlayerLocation,
    pub func: PlayerFunc,
    pub explore_tools: ExploreTools,
    pub player_chat: PlayerChat,
    pub guides: Guides,
    // Runtime
    pub world: Rc<RefCell<World>>,
    pub last_save_time: u64,
    pub quadrant_id: u64,
}

impl Player {
    pub fn init(&mut self) {
        if self.role_list.is_empty() || self.formation_list.is_empty() {
            self.init_role_and_formation();
        }

        self.ensure_basic_unlock_func();
    }

    pub fn notify_general_data(&self) {
        self.notify(self.build_adventure_task_notify());
        self.notify(AdviceSettingNotify {
            is_show: true,
        });
        self.notify(self.basic_info.build_notify());
        self.notify(ControlInfoNotify {
            forbid_list: vec![], // Disable function prohibition
        });
        self.notify(self.build_calabash_msg_notify());
        // CalabashLevelRewardsNotify
        self.notify(self.func.build_func_open_notify());
        // RoleFavorListNotify
        self.notify(InstDataNotify {
            enter_infos: vec![], // TODO: No effect in normal world, to implement for dungeon::logic()
        });
        self.notify(ItemPkgOpenNotify {
            open_pkg: (0..8).collect(),
        });
        // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
        for i in 0..80 {
            self.notify(MapUnlockFieldNotify {
                field_id: i,
            });
        }
        self.notify(self.build_role_list_notify());
        self.notify(self.explore_tools.build_explore_tool_all_notify());
        self.notify(self.explore_tools.build_roulette_update_notify());
        // VisionExploreSkillNotify
        self.notify(RoleChangeUnlockNotify {
            unlock_role_ids: vec![
                Role::MAIN_CHARACTER_FEMALE_SPECTRO_ID,
                Role::MAIN_CHARACTER_MALE_SPECTRO_ID,
                Role::MAIN_CHARACTER_MALE_HAVOC_ID,
                Role::MAIN_CHARACTER_FEMALE_HAVOC_ID,
            ],
            next_allow_change_time: 0,
        });
        // EnergyUpdateNotify
        // LevelPlayInfoNotify
        // MailInfoNotify
        // PayShopInfoNotify
        // PlayerVarNotify
        // RoguelikeCurrencyNotify
        // RoleMotionListNotify
        // SettingNotify
    }

    fn init_role_and_formation(&mut self) {
        self.role_list.clear();
        let mut role = match self.basic_info.sex {
            0 => Role::new(Role::MAIN_CHARACTER_FEMALE_SPECTRO_ID),
            1 => Role::new(Role::MAIN_CHARACTER_MALE_SPECTRO_ID),
            _ => unreachable!(),
        };

        role.name = self.basic_info.name.clone();

        self.role_list.insert(role.role_id, role);

        let required_role_ids: Vec<i32> = role_info_data::iter()
            .filter(|role_info| role_info.role_type == 1)
            .map(|role_info| role_info.id)
            .collect();
        let formation = vec![1506, 1206, 1505];

        required_role_ids.iter().for_each(|&role_id| {
            if !self.role_list.keys().any(|&k| k == role_id) {
                self.role_list.insert(role_id, Role::new(role_id));
            }
        });

        self.formation_list.insert(
            1,
            RoleFormation {
                id: 1,
                cur_role: *formation.iter().next().unwrap(),
                role_ids: formation,
                is_current: true,
            },
        );
        self.cur_formation_id = 1;

        self.formation_list.values_mut().for_each(|formation| {
            if formation.is_current && formation.id != 1 {
                formation.is_current = false;
            }
        });

        self.ensure_current_formation();
    }

    // Ensure basic functionality is unlocked
    // Should be handled by quest progression,
    // but as of right now, just unlock what we need
    fn ensure_basic_unlock_func(&mut self) {
        self.func.unlock(10026); // explore tools
    }

    fn ensure_current_formation(&mut self) {
        // If the list off formation is empty, add a default formation
        if self.formation_list.is_empty() {
            let mut role_list_clone = self.role_list.iter().clone();

            self.formation_list.insert(
                1,
                RoleFormation {
                    id: 1,
                    cur_role: role_list_clone.next().unwrap().1.role_id,
                    role_ids: role_list_clone
                        .take(3)
                        .map(|(&role_id, _)| role_id)
                        .collect(),
                    is_current: true,
                },
            );
        }

        // If there is no current formation, set the first formation as the current formation
        if !self.formation_list.values().any(|rf| rf.is_current) {
            self.formation_list.get_mut(&1).unwrap().is_current = true;
        }

        // Ensure that the set of character IDs for the current formation is not empty and that the current character ID is in the set
        if let Some(rf) = self.formation_list.values_mut().find(|rf| rf.is_current) {
            if rf.role_ids.is_empty() {
                rf.role_ids
                    .push(self.role_list.iter().next().unwrap().1.role_id);
            }

            if !rf.role_ids.contains(&rf.cur_role) {
                rf.cur_role = *rf.role_ids.iter().nth(0).unwrap();
            }
        }
    }

    pub fn build_in_world_player(&self) -> InWorldPlayer {
        InWorldPlayer {
            player_id: self.basic_info.id,
            player_name: self.basic_info.name.clone(),
            player_icon: 0,
            level: self.basic_info.level,
            group_type: 1,
        }
    }

    pub fn build_adventure_task_notify(&self) -> AdventureUpdateNotify {
        // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
        let adventure_task_data = wicked_waifus_data::adventure_task_data::iter();

        AdventureUpdateNotify {
            adventure_manual_data: vec![
                AdventureManualData {
                    adventre_task: adventure_task_data
                        .map(|task| AdventreTask {
                            id:task.id,
                            adventre_progress: 1,
                            state:2,
                        })
                        .collect(),
                    now_chapter:9,
                    received_chapter:9,
                }
            ],
        }
    }

    pub fn build_player_entity_add_notify(
        &self,
        role_list: Vec<Role>,
        world: &mut WorldEntity,
    ) -> EntityAddNotify {
        create_player_entity_pb!(
            role_list,
            self.basic_info.cur_map_id,
            world,
            self.basic_info.id,
            self.location.position.clone(),
            self.explore_tools
        )
    }

    pub fn build_player_entity_remove_notify(
        &self,
        entities: Vec<i64>,
        remove_type: ERemoveEntityType,
    ) -> EntityRemoveNotify {
        EntityRemoveNotify {
            remove_infos: entities
                .iter()
                .map(|&entity_id| EntityRemoveInfo {
                    entity_id,
                    r#type: remove_type.into(),
                })
                .collect(),
            is_remove: true,
        }
    }

    pub fn build_update_group_formation_notify(
        &self,
        cur_formation: RoleFormation,
        world: &mut WorldEntity,
    ) -> UpdateGroupFormationNotify {
        let group_type = 1;
        UpdateGroupFormationNotify {
            group_formation: vec![GroupFormation {
                player_id: self.basic_info.id,
                fight_role_infos: vec![FightRoleInfos {
                    group_type,
                    fight_role_infos: cur_formation
                        .role_ids
                        .iter()
                        .map(|&role_id| {
                            let entity_id = world.get_entity_id(role_id);
                            let role_skin = query_components!(world, entity_id, RoleSkin).0.unwrap();
                            FightRoleInfo {
                                role_id,
                                entity_id: world.get_entity_id(role_id),
                                on_stage_without_control: false,
                                role_skin_id: role_skin.skin_id,
                            }
                        })
                        .collect(),
                    cur_role: cur_formation.cur_role,
                    // is_retain: false,
                    living_status: LivingStatus::Alive.into(),
                    is_fixed_location: false,
                }],
                current_group_type: group_type,
            }],
        }
    }

    pub fn build_update_formation_notify(&self) -> UpdateFormationNotify {
        let role_map: HashMap<i32, &Role> = self
            .role_list
            .values()
            .map(|role| (role.role_id, role))
            .collect();

        UpdateFormationNotify {
            players_formations: vec![PlayerFightFormations {
                player_id: self.basic_info.id,
                formations: self
                    .formation_list
                    .iter()
                    .map(|(&formation_id, formation)| FightFormationNotifyInfo {
                        formation_id,
                        cur_role: formation.cur_role,
                        role_infos: formation
                            .role_ids
                            .iter()
                            .map(|role_id| {
                                if !role_map.contains_key(role_id) {
                                    tracing::warn!("Role {} not found in use role list", role_id);
                                    return Default::default();
                                }
                                let role = *role_map.get(&role_id).unwrap();
                                FormationRoleInfo {
                                    role_id: role.role_id,
                                    max_hp: 0,
                                    cur_hp: 0,
                                    level: role.level,
                                    role_skin_id: role.skin_id,
                                }
                            })
                            .collect(),
                        is_current: formation.is_current,
                    })
                    .collect(),
            }],
        }
    }

    pub fn load_from_save(save_data: PlayerSaveData) -> Self {
        let role_data = save_data.role_data.unwrap_or_default();

        Self {
            session: None,
            basic_info: PlayerBasicInfo::load_from_save(
                save_data.basic_data.clone().unwrap_or_default(),
            ),
            role_list: role_data
                .role_list
                .into_iter()
                .map(Role::load_from_save)
                .collect::<HashMap<i32, Role>>(),
            formation_list: role_data
                .role_formation_list
                .into_iter()
                .map(|(k, v)| (k, RoleFormation::load_from_save(v)))
                .collect(),
            cur_formation_id: role_data.cur_formation_id,
            location: save_data
                .location_data
                .map(PlayerLocation::load_from_save)
                .unwrap_or_default(),
            func: save_data
                .func_data
                .map(PlayerFunc::load_from_save)
                .unwrap_or_default(),
            explore_tools: save_data
                .explore_tools_data
                .map(ExploreTools::load_from_save)
                .unwrap_or_default(),
            player_chat: save_data
                .chat_data
                .map(PlayerChat::load_from_save)
                .unwrap_or_default(),
            guides: save_data
                .guides
                .map(Guides::load_from_save)
                .unwrap_or_default(),
            world: Rc::new(RefCell::new(World::new())),
            last_save_time: time_util::unix_timestamp(),
            quadrant_id: 0,
        }
    }

    pub fn build_save_data(&self) -> PlayerSaveData {
        PlayerSaveData {
            basic_data: Some(self.basic_info.build_save_data()),
            role_data: Some(PlayerRoleData {
                role_list: self
                    .role_list
                    .iter()
                    .map(|(_, role)| role.build_save_data())
                    .collect(),
                role_formation_list: self
                    .formation_list
                    .iter()
                    .map(|(&k, v)| (k, v.build_save_data()))
                    .collect(),
                cur_formation_id: self.cur_formation_id,
            }),
            location_data: Some(self.location.build_save_data()),
            func_data: Some(self.func.build_save_data()),
            explore_tools_data: Some(self.explore_tools.build_save_data()),
            chat_data: Some(self.player_chat.build_save_data()),
            guides: Some(self.guides.build_save_data()),
        }
    }

    pub fn set_session(&mut self, session: Arc<Session>) {
        self.session = Some(session);
    }

    pub fn build_role_list_notify(&self) -> PbGetRoleListNotify {
        PbGetRoleListNotify {
            role_list: self
                .role_list
                .iter()
                .map(|(_, role)| role.to_protobuf())
                .take(3) // TODO: There is a bug we are investigating with several resonators, this is a workaround
                .collect(),
        }
    }

    pub fn build_calabash_msg_notify(&self) -> CalabashMsgNotify {

        let calabash_level_data = calabash_level_data::iter();
    
        let calabash_max_lvl = calabash_level_data
            .clone()
            .map(|data| data.level)
            .max()
            .unwrap_or(0);
    
        let calabash_condition = calabash_level_data
            .clone()
            .find(|data| data.level == calabash_max_lvl)
            .map(|data| data.level_up_condition)
            .unwrap_or(0);
    
        let calabash_max_exp = calabash_level_data
            .clone()
            .find(|data| data.level == calabash_max_lvl)
            .map(|data| data.level_up_exp)
            .unwrap_or(0);
    
        let catch_gain: std::collections::HashMap<i32, i32> = calabash_level_data
            .clone()
            .map(|data| (data.level, data.temp_catch_gain))
            .collect();
    
        let calabash_develop_rewards: Vec<CalabashDevelopInfo> = calabash_develop_reward_data::iter()
            .filter(|dev_reward| dev_reward.is_show)
            .map(|dev_reward| CalabashDevelopInfo {
                monster_id: dev_reward.monster_id,
                unlock_conditions: dev_reward
                    .develop_condition
                    .iter()
                    .map(|&condition_id| CalabashDevelopConditionState {
                        condition_id,
                        rewarded: true,
                    })
                    .collect(),
            })
            .collect();
    
        CalabashMsgNotify {
            calabash_cfg: Some(CalabashCfg {
                level_up_exp: calabash_max_exp,
                level_up_condition: calabash_condition,
                catch_gain,
            }),
            calabash_msg: Some(CalabashMsg {
                level: calabash_max_lvl,
                exp: calabash_max_exp,
                unlocked_levels: calabash_level_data.clone().map(|data| data.level).collect(),
                unlocked_develop_rewards: calabash_develop_rewards,
                identify_guarantee_count: 0,
            }),
        }
    }

    pub fn notify(&self, content: impl ProtocolUnit) {
        if let Some(session) = self.session.as_ref() {
            session.forward_to_gateway(Message::Push {
                sequence_number: 0,
                message_id: content.get_message_id(),
                payload: Some(content.encode_to_vec().into_boxed_slice()),
            });
        }
    }

    pub fn respond(&self, content: impl ProtocolUnit, rpc_id: u16) {
        if let Some(session) = self.session.as_ref() {
            let data = content.encode_to_vec().into_boxed_slice();
            // tracing::debug!("Push to gateway MSG_ID: {}, LEN: {}, DATA: {}", content.get_message_id(), data.len(), hex::encode(&data));
            let response = Message::Response {
                sequence_number: 0,
                message_id: content.get_message_id(),
                rpc_id,
                payload: Some(data),
            };
            // let (name, value) = ("PbGetRoleListNotify", serde_json::to_string_pretty(&list).unwrap());
            // tracing::debug!("trying to log unhandled data for message {name} with:\n{value}");
            session.forward_to_gateway(response);
        }
    }

    pub fn create_default_save_data(id: i32, name: String, sex: i32) -> PlayerSaveData {
        let role_id = match sex {
            0 => Role::MAIN_CHARACTER_FEMALE_SPECTRO_ID, // 1502
            1 => Role::MAIN_CHARACTER_MALE_SPECTRO_ID,   // 1501
            _ => Role::MAIN_CHARACTER_MALE_SPECTRO_ID,   // Default to male
        };

        PlayerSaveData {
            basic_data: Some(PlayerBasicData {
                id,
                name,
                sex,
                level: 1,
                head_photo: 1603,
                head_frame: 80060009,
                cur_map_id: 8,
                role_show_list: vec![role_id],
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
