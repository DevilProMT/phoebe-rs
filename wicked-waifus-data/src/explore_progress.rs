use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExploreProgressData {
    pub id: i32,
    pub area: i32,
    pub explore_type: i32,
    pub sub_type_score: HashMap<String, i32>,
    pub phantom_skill_id: i32,
    pub unlock_text_id: String,
    pub lock_text_id: String,
    pub unlock_condition: i32,
    pub special_player_map: HashMap<String, i32>,
    pub is_recommend: bool,
    pub is_show_progress: bool,
    pub is_show_track: bool,
    pub special_player_desc: String,
}