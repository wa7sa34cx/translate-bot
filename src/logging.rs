//! Logging initialization module

use log::SetLoggerError;
use simplelog::*;
use std::str::FromStr;

/// Logging struct
#[derive(Debug)]
pub struct Logging {}

/// Logging config struct
#[derive(Debug)]
pub struct LoggingConfig {
    time_level: LevelFilter,
    level: LevelFilter,
}

impl Logging {
    /// Creates config from environment variables
    pub fn from_env() -> LoggingConfig {
        let time_level =
            LevelFilter::from_str(dotenv::var("LOG_TIME_LEVEL").unwrap().as_str()).unwrap();
        let level = LevelFilter::from_str(dotenv::var("LOG_LEVEL").unwrap().as_str()).unwrap();

        LoggingConfig { time_level, level }
    }
}

impl LoggingConfig {
    /// Initializes logging
    pub fn init(&self) -> Result<(), SetLoggerError> {
        let config = ConfigBuilder::new()
            .set_time_level(self.time_level)
            .set_target_level(LevelFilter::Debug)
            .build();

        TermLogger::init(self.level, config, TerminalMode::Mixed, ColorChoice::Auto)
    }
}
