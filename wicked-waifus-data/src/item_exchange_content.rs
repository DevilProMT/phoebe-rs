use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ItemExchangeContentData {
    pub item_id: i32,
    pub times: i32,
    pub gain_count: i32,
    pub consume: HashMap<String, i32>,
}