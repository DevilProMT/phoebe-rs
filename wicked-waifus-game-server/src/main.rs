use std::sync::{Arc, LazyLock};

use anyhow::Result;
use wicked_waifus_commons::config_util;
use config::ServiceConfig;
use session::SessionManager;

mod config;
mod gateway_connection;
mod logic;
mod player_save_task;
mod service_message_handler;
mod session;

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServiceConfig> =
        LazyLock::new(|| config_util::load_or_create("gameserver.toml"));
    static SESSION_MGR: LazyLock<SessionManager> = LazyLock::new(SessionManager::default);

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init(::tracing::Level::DEBUG);
    wicked_waifus_data::load_all_json_data("data/assets/game-data/BinData")?;
    wicked_waifus_data::text_map_data::load_textmaps("data/assets/game-data/Textmap")?;
    logic::utils::quadrant_util::initialize_quadrant_system();

    let database = Arc::new(wicked_waifus_database::connect_to(&CONFIG.database).await?);
    wicked_waifus_database::run_migrations(database.as_ref()).await?;

    logic::thread_mgr::start_logic_threads(1);

    player_save_task::start(database.clone());
    gateway_connection::init(CONFIG.service_id, &CONFIG.gateway_end_point);
    service_message_handler::run(&CONFIG.service_end_point, &SESSION_MGR, database).await?;

    Ok(())
}
