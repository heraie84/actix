use serde::Deserialize;
use config::ConfigError;
use tokio_postgres::NoTls;
use deadpool_postgres::Pool;
use slog::{Logger, Drain, o};
use slog_term;
use slog_async;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg:deadpool_postgres::Config
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    pub fn configure_pool(&self) -> Pool {
        self.pg.create_pool(NoTls).unwrap()
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("V" => env!("CARGO_PKG_VERSION")))
    }
}