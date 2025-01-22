use wicked_waifus_protocol::{BuffEffectCd, EntityComponentPb, FightBuffComponentPb, FightBuffInformation};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;
use crate::logic::utils::buff_util::add_buff_to_manager;

#[derive(Default, Clone)]
pub struct FightBuff {
    pub fight_buff_infos: Vec<FightBuffInformation>,
    pub permanent_fight_buff_infos: Vec<FightBuffInformation>,
    pub list_buff_effect_cd: Vec<BuffEffectCd>,
}

impl FightBuff {
    fn get_pb_data(pb: &wicked_waifus_protocol::EntityPb) -> Vec<&FightBuffComponentPb> {
        pb.component_pbs.iter()
            .filter_map(|pb| {
                if let Some(value) = &pb.component_pb {
                    match value {
                        ComponentPb::FightBuffComponent(result) => Some(result),
                        _ => None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn from_pb_data(pb: &wicked_waifus_protocol::EntityPb) -> Self {
        let pb_data = Self::get_pb_data(pb);
        match pb_data.get(0) {
            None => Self::default(),
            Some(pb) => {
                Self {
                    fight_buff_infos: pb.fight_buff_infos.clone(),
                    list_buff_effect_cd: pb.list_buff_effect_cd.clone(),
                    ..Default::default()
                }
            }
        }
    }

    pub fn add_permanent_buff(&mut self, id: i64) {
        self.permanent_fight_buff_infos.push(FightBuffInformation {
            handle_id: 0, // TODO: Although permanent buffs dont require a handle it would be best to have them
            buff_id: id,
            level: 1,
            stack_count: 1,
            instigator_id: 0,
            entity_id: 0,
            apply_type: 0,
            duration: -1f32,
            left_duration: -1f32,
            context: vec![],
            is_active: true,
            server_id: 0,
            message_id: 0,
        })
    }

    pub fn add_generic_permanent_buffs(&mut self) {
        self.add_permanent_buff(3003); // Remove wall run prohibition
        self.add_permanent_buff(3004); // Remove gliding prohibition
        self.add_permanent_buff(1213); // Reduce stamina while flying
        self.add_permanent_buff(1214); // Reduce stamina while flying in sprint
        self.add_permanent_buff(1215); // Reduce stamina while flying up in sprint
        self.add_permanent_buff(1216); // Reduce stamina while flying down in sprint
        self.add_permanent_buff(640012051); // Allow flying
    }
}

impl Component for FightBuff {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        // Remove existing FightBuffComponent
        pb.component_pbs.retain(|component_pb| {
            if let Some(value) = &component_pb.component_pb {
                match value {
                    ComponentPb::FightBuffComponent(_) => false,
                    _ => true
                }
            } else {
                true
            }
        });

        // Fix Instigator and Entity Id for permanent buffs
        let mut fight_buff_infos = self.fight_buff_infos.clone();
        let mut permanent_buffs = self.permanent_fight_buff_infos.clone();
        for buf in &mut permanent_buffs {
            buf.instigator_id = pb.id;
            buf.entity_id = pb.id;
            add_buff_to_manager(buf);
            fight_buff_infos.push(buf.clone());
        }

        // Add new FightBuffComponent
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::FightBuffComponent(FightBuffComponentPb {
                fight_buff_infos,
                list_buff_effect_cd: self.list_buff_effect_cd.clone(),
            })),
        });
    }
}
