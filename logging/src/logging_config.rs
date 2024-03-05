use colored::*;
use env_logger;
use std::io::Write;

const VAR_NAME: &str = "LOG_LEVEL";

pub fn logging_config() {
    env_logger::Builder::new()
        .format(|buf, record| {
            let level_str = match record.level() {
                log::Level::Trace => "TRACE".purple(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Info => "INFO".green(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Error => "ERROR".red(),
            };

            writeln!(
                buf,
                "{}: {}    {}:{}    {}",
                level_str,
                record.args(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            )
        })
        .parse_env(VAR_NAME)
        .init();

    println!(
        "{}={}", 
        VAR_NAME.blue(),
        std::env::var(VAR_NAME).unwrap_or("Не определена!!!".red().to_string()),
    );
}
