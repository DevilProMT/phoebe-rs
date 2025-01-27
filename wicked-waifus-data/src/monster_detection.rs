use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MonsterDetectionData {
    pub id: i32,
    pub blueprint_type: String,
    pub mark_id: i32,
    pub name: String,
    pub show_reward: i32,
    pub entity_config_id: i32,
    pub danger_type: i32,
    pub type_description2: i32,
    pub attributes_description_lock: String,
    pub attributes_description_unlock: String,
    pub big_icon: String,
    pub icon: String,
    pub temporary_icon_un_lock: String,
    pub temporary_iconlock: String,
    pub begin_time_stamp: i32,
    pub lock_con: i32,
    pub monster_info_id: i32,
}
