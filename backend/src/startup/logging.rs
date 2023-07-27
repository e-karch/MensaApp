use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// 
pub struct LogInfo {
    /// 
    pub log_config: String,
}

/// Class for initializing the logging.
pub struct Logger;

impl Logger {
    // Initializes the logger.
    pub fn init(info: LogInfo) {
        // setup logging
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .with_env_filter(EnvFilter::builder().from_env().unwrap())
            .pretty()
            // .with_env_filter(EnvFilter::default())
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}
