use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeSharedData {
    pub id: i32,
    pub max_count: i32,
    pub cost: HashMap<String, i32>,
    pub reset_time_id: i32,
}