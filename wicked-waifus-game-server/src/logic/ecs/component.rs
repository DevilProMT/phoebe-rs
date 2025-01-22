use wicked_waifus_protocol::EntityPb;

use crate::logic::components::*;

macro_rules! impl_component_container {
    ($($comp:ident;)*) => {
        pub enum ComponentContainer {
        $(
            $comp($comp),
        )*
        }

        impl ComponentContainer {
            pub fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
                match self {
                $(
                    Self::$comp(comp) => comp.set_pb_data(pb),
                )*
                }
            }
        }
    };
}

impl_component_container! {
    Position;
    EntityConfig;
    OwnerPlayer;
    Visibility;
    Attribute;
    PlayerEntityMarker;
    Movement;
    Equip;
    VisionSkill;
    MonsterAi;
    Fsm;
    RoleSkin;
    FightBuff;
    StateTag;
    Tag;
    Autonomous;
    Interact;
}

pub trait Component {
    fn set_pb_data(&self, pb: &mut EntityPb);
}
