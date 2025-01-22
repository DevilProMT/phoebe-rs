use std::collections::HashMap;

use crate::logic::utils::load_role_info::load_key_value;
use wicked_waifus_commons::time_util;
pub use formation::RoleFormation;
use wicked_waifus_data::{base_property_data, role_info_data};
use wicked_waifus_protocol::{ArrayIntInt, RoleInfo};
use wicked_waifus_protocol_internal::RoleData;

mod formation;
pub struct Role {
    pub role_id: i32,
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub breakthrough: i32,
    pub skill_map: HashMap<i32, i32>,
    pub star: i32,
    pub favor: i32,
    pub create_time: u32,
    pub equip_weapon: i32,
    pub skin_id: i32,
}

impl Role {
    pub const MAIN_CHARACTER_MALE_SPECTRO_ID: i32 = 1501;
    pub const MAIN_CHARACTER_FEMALE_SPECTRO_ID: i32 = 1502;
    pub const MAIN_CHARACTER_MALE_HAVOC_ID: i32 = 1605;
    pub const MAIN_CHARACTER_FEMALE_HAVOC_ID: i32 = 1604;

    pub fn new(role_id: i32) -> Self {
        let data = role_info_data::iter().find(|d| d.id == role_id).unwrap();

        Self {
            role_id,
            name: String::with_capacity(0),
            level: data.max_level,
            exp: 0,
            breakthrough: 0,
            skill_map: HashMap::new(), // TODO!
            star: 0,
            favor: 0,
            create_time: time_util::unix_timestamp() as u32,
            equip_weapon: data.init_weapon_item_id,
            skin_id: data.skin_id
        }
    }

    pub fn to_protobuf(&self) -> RoleInfo {
        let base_prop: HashMap<i32, i32> = load_key_value(
            base_property_data::iter()
                .find(|d| d.id == self.role_id)
                .unwrap_or_else(|| {
                    base_property_data::iter().find(|d| d.id == 1102).unwrap()
                }),
        );

        RoleInfo {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            create_time: self.create_time,
            skills: self
                .skill_map
                .iter()
                .map(|(k, v)| ArrayIntInt { key: *k, value: *v })
                .collect(),
            star: self.star,
            favor: self.favor,
            base_prop: base_prop
                .iter()
                .map(|(&k, &v)| ArrayIntInt { key: k, value: v })
                .collect(),
            skin_id: self.skin_id,
            ..Default::default()
        }
    }

    pub fn load_from_save(data: RoleData) -> (i32, Self) {
        (
            data.role_id,
            Self {
                role_id: data.role_id,
                name: data.name,
                level: data.level,
                exp: data.exp,
                breakthrough: data.breakthrough,
                skill_map: data.skill_map,
                star: data.star,
                favor: data.favor,
                create_time: data.create_time,
                equip_weapon: data.equip_weapon,
                skin_id: data.skin_id
            },
        )
    }

    pub fn build_save_data(&self) -> RoleData {
        RoleData {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            skill_map: self.skill_map.clone(),
            star: self.star,
            favor: self.favor,
            create_time: self.create_time,
            equip_weapon: self.equip_weapon,
            skin_id: self.skin_id,
            ..Default::default()
        }
    }
}
