use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CalabashDevelopRewardData {
    pub monster_id: i32,
    pub develop_condition: Vec<i32>,
    pub monster_info_id: i32,
    pub all_exp: i32,
    pub sort_id: i32,
    pub monster_probe_id: i32,
    pub hand_book_bp: String,
    pub monster_body_type: i32,
    pub hand_book_camera: String,
    pub monster_number: String,
    pub interaction_radius: i32,
    pub is_show: bool,
}