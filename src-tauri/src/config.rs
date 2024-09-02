use std::{
    fs::OpenOptions,
    hash::Hash,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use dco3::DracoonClientError;
use moka::future::Cache;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
pub const DEFAULT_CACHE_TTL: u64 = 60 * 5; // 5 minutes
pub const APPLICATION_NAME: &str = "dcadmin";
pub const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB
pub const DEFAULT_NO_DEBUG_MESSAGE: &str = "No details";

pub fn get_client_credentials() -> (String, String) {
    let client_id = include_str!("../.env")
        .split('\n')
        .next()
        .expect("env file has more than one line")
        .split("DCADMIN_CLIENT_ID=")
        .nth(1)
        .expect("CLIENT_ID MUST be provided");
    let client_secret = include_str!("../.env")
        .split('\n')
        .nth(1)
        .expect("env file has more than one line")
        .split("DCADMIN_CLIENT_SECRET=")
        .nth(1)
        .expect("CLIENT_SECRET MUST be provided");

    (client_id.to_string(), client_secret.to_string())
}

pub fn setup_logging(config_dir: &Path, debug: bool) {
    let log_file_path = config_dir.join("dcadmin.log");

    // get file size
    let size = if let Ok(metadata) = std::fs::metadata(&log_file_path) {
        metadata.len()
    } else {
        0
    };

    // set up env filter
    let env_filter = if debug {
        EnvFilter::from_default_env()
            .add_directive(LevelFilter::DEBUG.into())
            .add_directive("hyper_utils=warn".parse().expect("invalid crate setup"))
    } else {
        EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into())
    };

    // set up log format
    let log_format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_thread_names(false)
        .with_target(true)
        .with_ansi(false)
        .compact();

    // rotate log file if bigger than 10MB
    let log_file = if size > MAX_LOG_SIZE {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        let rotated_log_file_path = config_dir.join(format!("dcadmin.log.{}", timestamp));
        std::fs::rename(&log_file_path, &rotated_log_file_path).expect("Failed to rotate log file");
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .expect("Failed to open log file")
    } else {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .expect("Failed to open log file")
    };

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .event_format(log_format)
        .with_writer(std::sync::Mutex::new(log_file))
        .init();
}

pub fn get_or_create_config_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        let config_dir = config_dir.join(APPLICATION_NAME);
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        }
        config_dir
    } else {
        // config_dir might be None on any non Linux, MacOS or Windows platform
        panic!("unsupported platform");
    }
}

pub fn setup_cache<K: Hash + Eq + Send + Sync + 'static, V: Clone + Send + Sync + 'static>(
    max_capacity: u64,
    ttl: Option<Duration>,
) -> Cache<K, V> {
    Cache::builder()
        .time_to_live(ttl.unwrap_or(Duration::from_secs(DEFAULT_CACHE_TTL)))
        .max_capacity(max_capacity)
        .build()
}

pub fn log_dracoon_error(e: &DracoonClientError, msg: Option<&str>) {
    if let Some(msg) = msg {
        tracing::error!("{msg}: {e}");
    } else {
        tracing::error!("Error: {e}");
    }

    if let Some(e) = e.get_http_error() {
        tracing::error!(
            "HTTP error: {} - {} ({})",
            e.code(),
            e.error_message(),
            e.debug_info().unwrap_or(DEFAULT_NO_DEBUG_MESSAGE.into())
        );
    }
}
