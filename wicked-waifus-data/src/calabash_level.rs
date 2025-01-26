use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CalabashLevelData {
    pub level: i32,
    pub level_up_exp: i32,
    pub level_up_condition: i32,
    pub temp_catch_gain: i32,
    pub buff_ids: Vec<i32>,
    pub buff_description: String,
    pub level_up_description: String,
    pub quality_description: String,
    pub buff_description_map: HashMap<i32,i32>,
    pub cost: i32,
    pub reward_id: i32,
    pub quality_drop_weight: HashMap<i32, i32>,
}
