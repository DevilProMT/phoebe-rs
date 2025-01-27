use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ForgeFormulaData {
    pub id: i32,
    pub formula_item_id: i32,
    pub item_id: i32,
    pub type_id: i32,
    pub unlock: bool,
    pub sort_id: i32,
    pub name: String,
    pub consume_items: Vec<ConsumeItem>,
    pub forge_content: String,
    pub background: String,
    pub role_list: Vec<i32>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConsumeItem {
    pub item_id: i32,
    pub count: i32,
}
