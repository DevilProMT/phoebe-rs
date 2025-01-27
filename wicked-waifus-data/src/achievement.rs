use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AchievementData {
    pub id: i32,
    pub group_id: i32,
    pub level: i32,
    pub name: String,
    pub desc: String,
    pub icon_path: String,
    pub override_drop_id: i32,
    pub hidden: bool,
    pub next_link: i32,
    pub client_trigger: bool,
    pub third_party_trophy_id: i32,
}
