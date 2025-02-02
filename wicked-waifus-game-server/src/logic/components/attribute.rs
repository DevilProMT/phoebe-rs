use wicked_waifus_data::BasePropertyData;
use wicked_waifus_protocol::{
    entity_component_pb::ComponentPb, AttrData, AttributeComponentPb, EAttributeType,
    EntityComponentPb, LivingStatus, EEntityType,
};
use std::collections::HashMap;

use crate::logic::ecs::component::Component;
use crate::logic::utils::load_role_info::attribute_from_data;

pub struct Attribute {
    pub attr_map: HashMap<EAttributeType, (i32, i32)>,
}

impl Component for Attribute {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.living_status = (if self.is_alive() {
            LivingStatus::Alive
        } else {
            LivingStatus::Dead
        })
        .into();

        let mut local_attr_map = self.attr_map.clone();

        if pb.entity_type == EEntityType::Player as i32 {

            if let Some((base, _)) = local_attr_map.get_mut(&EAttributeType::ElementEnergy) {
                *base = 10000;
            }
            if let Some((base, _)) = local_attr_map.get_mut(&EAttributeType::Energy) {
                *base = 0;
            }
            if let Some((base, _)) = local_attr_map.get_mut(&EAttributeType::EnergyMax) {
                *base = 0;
            }
            // if let Some((base, _)) = local_attr_map.get_mut(&EAttributeType::CdReduse) {
            //     *base = 0;
            // }
        }

        let attr_map_ptr = self as *const _ as *mut Self;
        unsafe {
            (*attr_map_ptr).attr_map = local_attr_map;
        }

        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::AttributeComponent(
                self.build_entity_attribute(),
            )),
        });
    }
}

impl Attribute {
    pub fn is_alive(&self) -> bool {
        self.attr_map
            .get(&EAttributeType::Life)
            .copied()
            .unwrap_or_default()
            .0
            > 0
    }

    #[inline(always)]
    pub fn from_data(base_property: &BasePropertyData) -> Self {
        Self {
            attr_map: attribute_from_data(base_property),
        }
    }

    #[inline(always)]
    pub fn build_entity_attribute(&self) -> AttributeComponentPb {
        AttributeComponentPb {
            attr_data: self
                .attr_map
                .iter()
                .map(|(ty, (base, incr))| AttrData {
                    attribute_type: (*ty).into(),
                    current_value: *base,
                    value_increment: *incr,
                })
                .collect(),
            hardness_mode_id: 0,
            rage_mode_id: 0,
        }
    }
}
