use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LordGymData {
    pub id: i32,
    pub difficulty: i32,
    pub reward_id: i32,
    pub play_id: i32,
    pub gym_title: String,
    pub icon_path: String,
    pub play_description: String,
    pub help_id: i32,
    pub monster_list: Vec<i32>,
    pub monster_level: i32,
    pub lock_con: i32,
    pub lock_description: String,
    pub filter_type: i32,
    pub is_debug: bool,
}