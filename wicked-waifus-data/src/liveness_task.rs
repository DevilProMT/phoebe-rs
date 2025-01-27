use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LivenessTaskData {
    pub task_id: i32,
    pub task_name: String,
    pub update_type: i32,
    pub task_reward: HashMap<String, i32>,
    pub task_func: Vec<String>,
    pub unlock_condition: i32,
    pub sort_rank: i32,
}