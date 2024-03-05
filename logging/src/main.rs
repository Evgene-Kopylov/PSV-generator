use logging::logging_config::logging_config;
use logging::logging_config::VAR_NAME;


fn main() {
    std::env::set_var(VAR_NAME, "debug");

    logging_config();

    log::debug!("LOG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
    log::info!("long INFO ===================== ==================== =========================----------------------====================");
}
