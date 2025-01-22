use std::sync::OnceLock;

use wicked_waifus_protocol::FightBuffInformation;

static mut BUFF_MANAGER: OnceLock<Vec<FightBuffInformation>> = OnceLock::new();

pub fn add_buff_to_manager(buff: &mut FightBuffInformation) {
    let _ = unsafe { BUFF_MANAGER.get_or_init(|| vec![]) };
    let manager = unsafe { BUFF_MANAGER.get_mut() }.unwrap();
    let handle = rand::random::<i32>(); // TODO: rework this
    buff.handle_id = handle;
    buff.server_id = handle;
    buff.message_id= handle as i64;
    manager.push(buff.clone());
}