use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResonatorData {
    pub id: i32,
    pub buffs: Vec<i64>,
    pub passive_skills: Vec<i64>,
    pub concomitant: Option<Concomitant>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Concomitant {
    pub id: i64,
    pub range: i32,
    pub property_id: i64,
    pub buffs: Vec<i64>,
    pub passive_skills: Vec<i64>,
    pub summoner_component: Option<SummonerComponent>,
}

#[derive(Deserialize)]
pub struct SummonerComponent {
    #[serde(rename = "SummonCfgId")]
    pub summon_cfg_id: i32,
    #[serde(rename = "SummonSkillId")]
    pub summon_skill_id: Option<i32>,
    #[serde(rename = "type")]
    pub summoner_component_type: i32,
}