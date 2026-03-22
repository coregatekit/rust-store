#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub time_out: u64,
    pub allow_origins: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}
