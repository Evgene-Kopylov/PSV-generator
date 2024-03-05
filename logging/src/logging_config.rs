use colored::*;
use env_logger;
use std::io::Write;

pub const VAR_NAME: &str = "LOG_LEVEL";

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

            let level_str = format!("{:<width$}", level_str, width = 5).dimmed();

            writeln!(
                buf,
                "{}  {}    {}    {}\n",
                level_str,
                format!("{:<30}", record.args().to_string()),
                format!("{}:{}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0)).blue(),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string().dimmed(),
            )
        })
        .parse_env(VAR_NAME)
        .init();

    println!(
        "{}={}", 
        VAR_NAME.blue(),
        std::env::var(VAR_NAME).unwrap_or("<Перемпенная не определена.>".to_string()).green(),
    );
}
