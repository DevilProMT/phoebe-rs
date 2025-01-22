use serde::Deserialize;
use crate::GachaViewTypeInfoId;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GachaViewInfoData {
    pub id: i32,
    pub r#type: GachaViewTypeInfoId,
    pub summary_title: String,
    pub summary_describe: String,
    // TODO: Effect path
    pub theme_color: String,
    pub content_texture_path: String,
    pub content_texture_bg_path: String,
    pub under_bg_texture_path: String,
    pub tag_not_selected_sprite_path: String,
    pub tag_selected_sprite_path: String,
    pub weapon_prefab_path: String,
    pub up_list: Vec<i32>,
    pub show_id_list: Vec<i32>,
    pub preview_id_list: Vec<i32>,
}