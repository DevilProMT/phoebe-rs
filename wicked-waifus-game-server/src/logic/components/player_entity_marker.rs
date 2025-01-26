use wicked_waifus_protocol::EEntityType;

use crate::logic::ecs::component::Component;

pub struct PlayerEntityMarker {
    pub entity_type: EEntityType,
}

impl Component for PlayerEntityMarker {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.entity_type = self.entity_type.into();
    }
}
