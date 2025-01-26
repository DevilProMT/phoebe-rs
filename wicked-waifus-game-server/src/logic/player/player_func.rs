use std::collections::HashMap;

use wicked_waifus_data::function_condition_data;
use wicked_waifus_protocol::{FuncOpenNotify, Function};
use wicked_waifus_protocol_internal::PlayerFuncData;

pub struct PlayerFunc {
    pub func_map: HashMap<i32, i32>,
}

impl PlayerFunc {
    pub fn unlock(&mut self, id: i32) {
        self.func_map.insert(id, 2);
    }

    pub fn load_from_save(data: PlayerFuncData) -> Self {
        PlayerFunc {
            func_map: data.func_map,
        }
    }

    pub fn build_save_data(&self) -> PlayerFuncData {
        PlayerFuncData {
            func_map: self.func_map.clone(),
        }
    }

    pub fn build_func_open_notify(&self) -> FuncOpenNotify {
        let func_map = function_condition_data::iter();
        FuncOpenNotify {
            func: func_map
                .map(|func| Function {
                    id: func.function_id,
                    flag: 2,
                })
                .collect(),
        }
    }
}

impl Default for PlayerFunc {
    fn default() -> Self {
        Self {
            func_map: function_condition_data::iter()
                .filter(|fc| (fc.open_condition_id == 0 && fc.is_on) || (fc.function_id == 10009)) // TODO: remove this when required functions are implemented
                .map(|fc| (fc.function_id, 2))
                .collect(),
        }
    }
}
