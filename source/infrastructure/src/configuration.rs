pub struct InfrastructureSettings {
    database_url: String,
}

impl InfrastructureSettings {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

pub struct InfrastructureConfiguration {
    settings: InfrastructureSettings,
}

impl InfrastructureConfiguration {
    pub fn new(settings: InfrastructureSettings) -> Self {
        Self { settings }
    }

    pub fn settings(&self) -> &InfrastructureSettings {
        &self.settings
    }
}
