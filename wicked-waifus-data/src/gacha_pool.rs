use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GachaPoolData {
    pub id: i32,
    pub gacha_id: i32,
    pub sort: i32,
}