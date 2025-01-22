use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropValueData {
    pub id: i32,
    pub value: f32,
    pub is_ratio: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VectorData([f32; 3]);

impl VectorData {
    pub fn get_x(&self) -> f32 {
        self.0[0]
    }

    pub fn get_y(&self) -> f32 {
        self.0[1]
    }

    pub fn get_z(&self) -> f32 {
        self.0[2]
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawVectorData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RawVectorData {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntranceEntityData {
    pub dungeon_id: i32,
    pub entrance_entity_id: i32,
}

#[derive(Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i32)]
pub enum GachaViewTypeInfoId {
    NoviceConvene = 1,
    FeaturedResonatorConvene = 2,
    FeaturedWeaponConvene = 3,
    StandardResonatorConvene = 4,
    StandardWeaponConvene = 5,
    BeginnersChoiceConvene = 6,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Category {
    pub main_type: Option<String>,
    pub monster_match_type: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ScanFunction {
    pub scan_id: Option<i32>,
    pub is_concealed: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WorldLevelBonusType {
    pub r#type: Option<i32>,
    pub world_level_bonus_id: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BaseInfoComponent {
    pub tid_name: Option<String>,
    pub category: Option<Category>,
    pub camp: Option<i32>,
    pub online_interact_type: Option<i32>,
    pub scan_function: Option<ScanFunction>,
    pub aoi_layer: Option<i32>,
    pub entity_property_id: Option<i32>,
    pub focus_priority: Option<i32>,
    pub aoi_zradius: Option<i32>,
    // TODO: Add more
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AiComponent {
    pub disabled: Option<bool>,
    pub ai_id: Option<i32>,
    // TODO: Add more
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AttributeComponent {
    pub property_id: Option<i32>,
    pub level: Option<i32>,
    pub world_level_bonus_type: Option<WorldLevelBonusType>,
    pub rage_mode_id: Option<i32>,
    pub hardness_mode_id: Option<i32>,
    pub monster_prop_extra_rate_id: Option<i32>,
    pub world_level_bonus_id: Option<i32>,
    pub fight_music: Option<String>,
    // TODO: Add more
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TeleportPosition {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub a: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TeleportComponent {
    pub disabled: Option<bool>,
    pub teleporter_id: Option<i32>,
    #[serde(rename = "TeleportPos")]
    pub teleport_position: Option<TeleportPosition>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ComponentsData {
    pub base_info_component: Option<BaseInfoComponent>,
    pub ai_component: Option<AiComponent>,
    pub attribute_component: Option<AttributeComponent>,
    pub teleport_component: Option<TeleportComponent>,
    // TODO: Implement this ones
    pub scene_actor_ref_component: Option<serde_json::Value>,
    pub effect_area_component: Option<serde_json::Value>,
    pub entity_state_component: Option<serde_json::Value>,
    pub condition_listener_component: Option<serde_json::Value>,
    pub interact_component: Option<serde_json::Value>,
    pub npc_perform_component: Option<serde_json::Value>,
    pub var_component: Option<serde_json::Value>,
    pub entity_visible_component: Option<serde_json::Value>,
    pub level_ai_component: Option<serde_json::Value>,
    pub trigger_component: Option<serde_json::Value>,
    pub range_component: Option<serde_json::Value>,
    pub spline_component: Option<serde_json::Value>,
    pub bubble_component: Option<serde_json::Value>,
    pub reward_component: Option<serde_json::Value>,
    pub refresh_component: Option<serde_json::Value>,
    pub passerby_npc_spawn_component: Option<serde_json::Value>,
    pub vision_capture_component: Option<serde_json::Value>,
    pub refresh_group_component: Option<serde_json::Value>,
    pub collect_component: Option<serde_json::Value>,
    pub target_gear_component: Option<serde_json::Value>,
    pub fight_interact_component: Option<serde_json::Value>,
    pub guide_line_creator_component: Option<serde_json::Value>,
    pub photo_target_component: Option<serde_json::Value>,
    pub model_component: Option<serde_json::Value>,
    pub entity_group_component: Option<serde_json::Value>,
    pub scene_item_life_cycle_component: Option<serde_json::Value>,
    pub entity_state_audio_component: Option<serde_json::Value>,
    pub animal_component: Option<serde_json::Value>,
    pub monster_component: Option<serde_json::Value>,
    pub nearby_tracking_component: Option<serde_json::Value>,
    pub follow_track_component: Option<serde_json::Value>,
    pub jigsaw_foundation: Option<serde_json::Value>,
    pub treasure_box_component: Option<serde_json::Value>,
    pub hook_lock_point: Option<serde_json::Value>,
    pub explore_skill_interact_component: Option<serde_json::Value>,
    pub attach_target_component: Option<serde_json::Value>,
    pub target_gear_group_component: Option<serde_json::Value>,
    pub spawn_monster_component: Option<serde_json::Value>,
    pub skybox_component: Option<serde_json::Value>,
    pub destructible_item: Option<serde_json::Value>,
    pub fan_component: Option<serde_json::Value>,
    pub state_hint_component: Option<serde_json::Value>,
    pub buff_consumer_component: Option<serde_json::Value>,
    pub reset_entities_pos_component: Option<serde_json::Value>,
    pub group_ai_component: Option<serde_json::Value>,
    pub pulling_object_foundation: Option<serde_json::Value>,
    pub lift_component: Option<serde_json::Value>,
    pub scene_item_movement_component: Option<serde_json::Value>,
    pub reset_self_pos_component: Option<serde_json::Value>,
    pub jigsaw_item: Option<serde_json::Value>,
    pub level_play_component: Option<serde_json::Value>,
    pub interact_gear_component: Option<serde_json::Value>,
    pub ai_gear_strategy_component: Option<serde_json::Value>,
    pub pick_interact_component: Option<serde_json::Value>,
    pub level_sequence_frame_event_component: Option<serde_json::Value>,
    pub air_wall_spawner_component: Option<serde_json::Value>,
    pub progress_bar_control_component: Option<serde_json::Value>,
    pub batch_bullet_caster_component: Option<serde_json::Value>,
    pub client_trigger_component: Option<serde_json::Value>,
    pub enrichment_area_component: Option<serde_json::Value>,
    pub vehicle_component: Option<serde_json::Value>,
    pub item_foundation2: Option<serde_json::Value>,
    pub tele_control2: Option<serde_json::Value>,
    pub interact_audio_component: Option<serde_json::Value>,
    pub level_qte_component: Option<serde_json::Value>,
    pub resurrection_component: Option<serde_json::Value>,
    pub ai_alert_notify_component: Option<serde_json::Value>,
    pub trample_component: Option<serde_json::Value>,
    pub dungeon_entry_component: Option<serde_json::Value>,
    pub level_prefab_perform_component: Option<serde_json::Value>,
    pub render_specified_range_component: Option<serde_json::Value>,
    pub walking_pattern_component: Option<serde_json::Value>,
    pub no_render_portal_component: Option<serde_json::Value>,
    pub adsorb_component: Option<serde_json::Value>,
    pub beam_cast_component: Option<serde_json::Value>,
    pub beam_receive_component: Option<serde_json::Value>,
    pub timeline_track_control_component: Option<serde_json::Value>,
    pub scene_bullet_component: Option<serde_json::Value>,
    pub edit_custom_aoi_component: Option<serde_json::Value>,
    pub combat_component: Option<serde_json::Value>,
    pub location_safety_component: Option<serde_json::Value>,
    pub turntable_control_component: Option<serde_json::Value>,
    pub scene_item_ai_component: Option<serde_json::Value>,
    pub buff_producer_component: Option<serde_json::Value>,
    pub portal_component: Option<serde_json::Value>,
    pub inhalation_ability_component: Option<serde_json::Value>,
    pub inhaled_item_component: Option<serde_json::Value>,
    pub monster_gacha_base_component: Option<serde_json::Value>,
    pub monster_gacha_item_component: Option<serde_json::Value>,
    pub time_stop_component: Option<serde_json::Value>,
    pub hit_component: Option<serde_json::Value>,
    pub levitate_magnet_component: Option<serde_json::Value>,
    pub rebound_component: Option<serde_json::Value>,
    pub rotator_component2: Option<serde_json::Value>,
    pub conveyor_belt_component: Option<serde_json::Value>,
    pub dynamic_portal_creator_component: Option<serde_json::Value>,
    pub connector_component: Option<serde_json::Value>,
    pub monitor_component: Option<serde_json::Value>,
}