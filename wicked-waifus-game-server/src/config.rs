use serde::Deserialize;

use wicked_waifus_commons::config_util::TomlConfig;
use wicked_waifus_database::DatabaseSettings;
use wicked_waifus_network::config::ServiceEndPoint;

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub service_id: u32,
    pub database: DatabaseSettings,
    pub service_end_point: ServiceEndPoint,
    pub gateway_end_point: ServiceEndPoint,
}

impl TomlConfig for ServiceConfig {
    const DEFAULT_TOML: &str = include_str!("../gameserver.default.toml");
}
