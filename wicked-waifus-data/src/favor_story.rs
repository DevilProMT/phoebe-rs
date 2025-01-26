use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FavorStoryData {
    pub id: i32,
    pub role_id: i32,
    pub sort: i32,
    pub title: String,
    pub content: String,
    pub cond_group_id: i32,
    pub motion_img: String,
    pub ani_blueprint: String,
    pub ani_montage: String,
}