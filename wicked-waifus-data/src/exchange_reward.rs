
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeRewardData {
    pub id: i32,
    pub shared_id: i32,
    pub max_count: i32,
    pub cost: HashMap<String, i32>,
    pub reward_id: HashMap<String, i32>,
    pub preview_reward: HashMap<String, i32>,
}