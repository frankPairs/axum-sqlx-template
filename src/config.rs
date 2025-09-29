use std::{
    fmt::Display,
    net::{AddrParseError, SocketAddr},
};

#[derive(Debug)]
pub enum SettingsError {
    EnvironmentLoad,
    EnvironmentVariableMissing(String),
    InvalidVariableFormat(String),
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::EnvironmentLoad => write!(f, "Failed to load environment variables."),
            SettingsError::EnvironmentVariableMissing(key) => {
                write!(f, "Failed to find environment variable: {}", key)
            }
            SettingsError::InvalidVariableFormat(key) => {
                write!(f, "Failed to parse environment variable: {}", key)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub db: DbSettings,
}

#[derive(Debug, Clone)]
pub struct ApplicationSettings {
    port: u32,
    host: String,
}

impl ApplicationSettings {
    pub fn new() -> Result<Self, SettingsError> {
        let port: u32 = get_env_value("PORT")?
            .parse()
            .map_err(|_| SettingsError::InvalidVariableFormat("PORT".to_string()))?;
        let host = get_env_value("HOST")?;

        Ok(Self { port, host })
    }

    pub fn get_addr(&self) -> Result<SocketAddr, AddrParseError> {
        format!("{}:{}", self.host, self.port).parse()
    }
}

#[derive(Debug, Clone)]
pub struct DbSettings {
    url: String,
}

impl DbSettings {
    pub fn new() -> Result<Self, SettingsError> {
        let url = get_env_value("DATABASE_URL")?;

        Ok(Self { url })
    }

    pub fn get_db_url(&self) -> String {
        self.url.clone()
    }
}

pub fn get_app_settings() -> Result<Settings, SettingsError> {
    dotenv::dotenv().map_err(|_| SettingsError::EnvironmentLoad)?;

    Ok(Settings {
        application: ApplicationSettings::new()?,
        db: DbSettings::new()?,
    })
}

fn get_env_value(key: &str) -> Result<String, SettingsError> {
    std::env::var(key).map_err(|_| SettingsError::EnvironmentVariableMissing(key.to_string()))
}
