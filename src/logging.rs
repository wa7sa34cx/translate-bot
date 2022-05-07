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
    log_level: LevelFilter,
}

impl Logging {
    /// Creates config from environment variables
    pub fn from_env() -> LoggingConfig {
        let time_level =
            LevelFilter::from_str(dotenv::var("LOG_TIME_LEVEL").unwrap().as_str()).unwrap();
        let log_level =
            LevelFilter::from_str(dotenv::var("LOG_LOG_LEVEL").unwrap().as_str()).unwrap();

        LoggingConfig {
            time_level,
            log_level,
        }
    }
}

impl LoggingConfig {
    /// Initializes logging
    pub fn init(&self) -> Result<(), SetLoggerError> {
        let config = ConfigBuilder::new()
            .set_time_level(self.time_level)
            .set_target_level(LevelFilter::Debug)
            .build();

        TermLogger::init(
            self.log_level,
            config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
    }
}
