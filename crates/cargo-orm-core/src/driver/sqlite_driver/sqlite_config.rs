use crate::driver::connection_config::ConnectionConfig;

pub struct SqliteConfig {
    pub url: String,
    pub max_connections: usize,
    pub min_connections: usize,
    pub connection_timeout_ms: u64,
}

impl ConnectionConfig for SqliteConfig {
    fn url(&self) -> &str {
        &self.url
    }
    fn max_connections(&self) -> usize {
        self.max_connections
    }
    fn min_connections(&self) -> usize {
        self.min_connections
    }
    fn connection_timeout_ms(&self) -> u64 {
        self.connection_timeout_ms
    }
}
