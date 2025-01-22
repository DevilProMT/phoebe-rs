use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GachaData {
    pub id: i32,
    pub rule_group_id: i32,
    pub sort: i32,
}