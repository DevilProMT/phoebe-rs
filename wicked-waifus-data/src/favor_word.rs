use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FavorWordData {
    pub id: i32,
    pub role_id: i32,
    pub r#type: i32,
    pub sort: i32,
    pub title: String,
    pub content: String,
    pub voice: String,
    #[serde(rename = "CVCn")]
    pub cv_cn: String,
    #[serde(rename = "CVJp")]
    pub cv_jp: String,
    #[serde(rename = "CVEn")]
    pub cv_en: String,
    #[serde(rename = "CVKo")]
    pub cv_ko: String,
    pub cond_group_id: i32,
    pub motion_img: String,
    pub ani_blueprint: String,
    pub ani_montage: String,
}