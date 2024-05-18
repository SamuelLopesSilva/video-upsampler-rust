use colored::*;
use log::info;

pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }

    pub fn set_log_level(&self, level: &str) {
        match level {
            "error" => env_logger::builder()
                .filter_level(log::LevelFilter::Error)
                .init(),
            "warn" => env_logger::builder()
                .filter_level(log::LevelFilter::Warn)
                .init(),
            "info" => env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .init(),
            "debug" => env_logger::builder()
                .filter_level(log::LevelFilter::Debug)
                .init(),
            "trace" => env_logger::builder()
                .filter_level(log::LevelFilter::Trace)
                .init(),
            _ => env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .init(),
        };
    }

    pub fn info(&self, message: &str) {
        info!("{}", message.to_string().green());
    }

    // pub fn error(&self, message: &str) {
    //     error!("{}", message.to_string().red());
    // }
}
