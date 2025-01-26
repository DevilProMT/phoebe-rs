use std::fs;
use std::sync::LazyLock;

use anyhow::Result;

use wicked_waifus_commons::config_util::{self, TomlConfig};
use serde::Deserialize;
use wicked_waifus_http::{
    config::{AesSettings, NetworkSettings},
    Application,
};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub network: NetworkSettings,
    pub encryption: AesSettings,
}

impl TomlConfig for ServerConfig {
    const DEFAULT_TOML: &str = include_str!("../configserver.default.toml");
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| config_util::load_or_create("configserver.toml"));

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init(::tracing::Level::DEBUG);

    Application::new()
        .get("/index.json", get_index)
        .with_encryption(&CONFIG.encryption)
        .serve(&CONFIG.network)
        .await?;

    Ok(())
}

async fn get_index() -> &'static str {
    static INDEX: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string("index.json").unwrap());

    &*INDEX
}
