use serde::Deserialize;
use config as config_crate;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub ws_port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub max_connections: u32,
    pub cache_ttl_sec: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CalculationConfig {
    pub interval_sec: u64,
    pub payment_interval_sec: u64,
    pub max_symbols: u32,
    pub target_calculation_time_ms: u64,
    pub parallel_tasks: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FundingConfig {
    pub interest_rate_daily: f64,
    pub rate_min: f64,
    pub rate_max: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OracleConfig {
    pub timeout_ms: u64,
    pub max_staleness_sec: u64,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub enable_alerts: bool,
    pub metrics_port: u16,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub calculation: CalculationConfig,
    pub funding: FundingConfig,
    pub oracle: OracleConfig,
    pub monitoring: MonitoringConfig,
}

pub fn load() -> anyhow::Result<AppConfig> {
    let cfg = config_crate::Config::builder()
        .add_source(config_crate::File::with_name("config/development"))
        .add_source(config_crate::Environment::with_prefix("APP").separator("__"))
        .build()?;
    cfg.try_deserialize::<AppConfig>().map_err(Into::into)
}

