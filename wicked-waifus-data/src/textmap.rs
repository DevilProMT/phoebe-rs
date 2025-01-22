use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TextMapData {
    pub id: String,
    pub content: String,
}
