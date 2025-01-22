use serde::Deserialize;
use crate::ComponentsData;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TemplateConfigData {
    pub id: i32,
    pub blueprint_type: String,
    pub name: String,
    pub components_data: ComponentsData,
}