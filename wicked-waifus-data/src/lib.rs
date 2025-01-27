use paste::paste;

pub use misc_data::*;

mod misc_data;
#[derive(thiserror::Error, Debug)]
pub enum LoadDataError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse json: {0}")]
    Json(#[from] serde_json::Error),
}

macro_rules! json_data {
    ($($table_type:ident;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::sync::OnceLock;
                type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<Vec<Data>> = OnceLock::new();

                pub fn iter() -> std::slice::Iter<'static, Data> {
                    TABLE.get().unwrap().iter()
                }
            }
        })*

        fn load_json_data(base_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let json_content = std::fs::read_to_string(&format!("{}/{}.json", base_path, stringify!($table_type)))?;
                let _ = [<$table_type:snake _data>]::TABLE.set(serde_json::from_str(&json_content)?);
            })*

            Ok(())
        }
    };
}

macro_rules! json_hash_table_data {
    ($($table_type:ident, $key_param:expr, $key_type:ty;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::collections::HashMap;
                use std::sync::OnceLock;

                pub(crate) type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<HashMap<$key_type, Data>> = OnceLock::new();

                pub fn iter() -> std::collections::hash_map::Iter<'static, $key_type, Data> {
                    TABLE.get().unwrap().iter()
                }

                pub fn get(k: &$key_type) -> Option<&Data> {
                    TABLE.get().unwrap().get(k)
                }
            }
        })*

        fn load_json_hash_table_data(base_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let json_content = std::fs::read_to_string(&format!("{}/{}.json", base_path, stringify!($table_type)))?;
                let _ = [<$table_type:snake _data>]::TABLE.set(
                    serde_json::from_str::<Vec<[<$table_type:snake _data>]::Data>>(&json_content)?
                        .iter()
                        .cloned()
                        .map(|element| (element.$key_param, element))
                        .collect::<std::collections::HashMap<_, _>>()
                );
            })*

            Ok(())
        }
    };
}

macro_rules! custom_json_data {
    ($($table_type:ident;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::sync::OnceLock;
                type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<Vec<Data>> = OnceLock::new();

                pub fn iter() -> std::slice::Iter<'static, Data> {
                    TABLE.get().unwrap().iter()
                }
            }
        })*

        fn load_custom_json_data(custom_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let json_content = std::fs::read_to_string(&format!("{}/{}.json", custom_path, stringify!($table_type)))?;
                let _ = [<$table_type:snake _data>]::TABLE.set(serde_json::from_str(&json_content)?);
            })*

            Ok(())
        }
    };
}

pub fn load_all_json_data(base_path: &str) -> Result<(), LoadDataError> {
    load_json_data(base_path)?;
    load_json_hash_table_data(base_path)?;

    // Custom Data
    let custom_path = base_path.replace("BinData", "CustomData");
    load_custom_json_data(&custom_path)?;
    Ok(())
}

json_data! {
    Achievement;
    AdventureTask;
    Area;
    BaseProperty;
    CalabashLevel;
    CalabashDevelopReward;
    Damage;
    ExploreTools;
    FavorWord;
    FavorGoods;
    FavorStory;
    FunctionCondition;
    Gacha;
    GachaPool;
    GachaViewInfo;
    GuideGroup;
    GuideTutorial;
    InstanceDungeon;
    LordGym;
    RoleInfo;
    Teleporter;
    WeaponConf;
}

json_hash_table_data! {
    DragonPool, id, i32;
    LevelEntityConfig, entity_id, i64;
    // TemplateConfig, blueprint_type, String;
}

custom_json_data! {
    Resonator;
}

mod textmap;

pub mod text_map_data {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    use crate::LoadDataError;
    use crate::textmap::TextMapData;

    static EMPTY: OnceLock<HashMap<String, String>> = OnceLock::new();
    static TABLE: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();

    pub fn load_textmaps(base_path: &str) -> Result<(), LoadDataError> {
        let _ = EMPTY.set(HashMap::new());
        let languages = std::fs::read_dir(base_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .collect::<Vec<_>>();
        let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
        for language in languages {
            let lang_id = language.file_name().to_str().unwrap().to_string();
            let json_content = std::fs::read_to_string(
                &format!("{base_path}/{lang_id}/multi_text/MultiText.json")
            )?;
            result.insert(
                lang_id,
                serde_json::from_str::<Vec<TextMapData>>(&json_content)?
                    .iter()
                    .cloned()
                    .map(|element| (element.id, element.content))
                    .collect::<HashMap<_, _>>(),
            );
        }
        let _ = TABLE.set(result);
        Ok(())
    }

    pub fn get_textmap(language: i32) -> &'static HashMap<String, String> {
        let (text_code, _audio_code) = get_language_from_i32(language);
        TABLE.get().unwrap().get(text_code).unwrap_or(EMPTY.get().unwrap())
    }

    fn get_language_from_i32(language: i32) -> (&'static str, &'static str) {
        match language {
            0 => ("zh-Hans", "zh"),
            1 => ("en", "en"),
            2 => ("ja", "ja"),
            3 => ("ko", "ko"),
            4 => ("ru", "en"),
            5 => ("zh-Hant", "zh"),
            6 => ("de", "en"),
            7 => ("es", "en"),
            8 => ("pt", "en"),
            9 => ("id", "en"),
            10 => ("fr", "en"),
            11 => ("vi", "en"),
            12 => ("th", "en"),
            _ => ("en", "en"),
        }
    }
}