use config;
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    // Initialize configuration reader
    let mut settings = config::Config::default();

    // Read configuration file named "configuration"
    settings.merge(config::File::with_name("configuration"))?;

    // (Try to) convert values from configuration file into resulting type
    settings.try_into()
}

impl DatabaseSettings {
    // Return connection string with database name
    pub fn cstring(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name,
        ))
    }

    // Return connection string without database name
    pub fn cstring_alt(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
