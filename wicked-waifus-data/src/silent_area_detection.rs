use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SilentAreaDetectionData {
    pub id: i32,
    pub name: String,
    pub guide_id: i32,
    pub level_play_list: Vec<i32>,
    pub instance_sub_type_description: String,
    pub danger_type: i32,
    pub secondary: i32,
    pub type_description2: i32,
    pub mat_type: i32,
    pub attributes_description_lock: String,
    pub attributes_description_unlock: String,
    pub big_icon: String,
    pub icon: String,
    pub lock_big_icon: String,
    pub temporary_icon_un_lock: String,
    pub temporary_iconlock: String,
    pub show_reward: i32,
    pub show_reward_map: HashMap<String, i32>,
    pub begin_time_stamp: i32,
    pub pre_open_id: i32,
    pub mark_id: i32,
    pub lock_con: i32,
    pub phantom_id: Vec<Option<serde_json::Value>>,
    pub first_drop_id: i32,
    pub additional_id: i32,
    pub sort_id: i32,
    pub new_content: String,
}
