use crate::logic::ecs::component::Component;

pub struct Visibility(pub bool);

impl Component for Visibility {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.is_visible = self.0;
    }
}
