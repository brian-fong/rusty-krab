use config;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
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
    pub fn cstring(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name,
        )
    }
    pub fn cstring_alt(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
