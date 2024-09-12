impl Configurations {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"));

        // Allow to override settings from environment variables
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            builder = builder.set_override("logger.level", log_level)?;
        }

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }
}
