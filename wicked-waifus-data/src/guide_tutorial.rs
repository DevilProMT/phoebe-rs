use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GuideTutorialData {
        pub id: i32,
        pub tutorial_type: i32,
        pub tutorial_order: i32,
        pub page_id: Vec<i32>,
        pub tutorial_tip: bool,
    }