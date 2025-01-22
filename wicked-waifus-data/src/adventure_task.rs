use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AdventureTaskData {
    pub id: i32,
    pub chapter_id: i32,
    pub task_text: String,
    pub record_id: Vec<i32>,
    pub need_progress: i32,
    pub drop_ids: i32,
    pub path_id: i32,
    pub jump_to: HashMap<String, String>,
}