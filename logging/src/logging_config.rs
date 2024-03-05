use env_logger;
use std::io::Write;

pub fn logging_config() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}    {}:{}    {}",
                record.level(),
                record.args(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            )
        })
        .parse_env("RUST_LOG")
        .init();
}
