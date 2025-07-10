use std::error::Error;

mod repositories;

pub use repositories::UserRepository;

pub struct ApplicationServices {
    user_repository: Box<dyn UserRepository>,
}

pub struct ApplicationServicesBuilder {
    user_repository: Option<Box<dyn UserRepository>>,
}

impl ApplicationServices {
    pub fn user_repository(&self) -> &dyn UserRepository {
        &*self.user_repository
    }
}

impl ApplicationServicesBuilder {
    pub fn init() -> Self {
        ApplicationServicesBuilder {
            user_repository: None,
        }
    }

    pub fn with_user_repository(mut self, service: Box<dyn UserRepository>) -> Self {
        self.user_repository = Some(service);
        self
    }

    pub fn build(self) -> Result<ApplicationServices, Box<dyn Error>> {
        let user_repository = self
            .user_repository
            .ok_or("User repository is not set yet")?;

        Ok(ApplicationServices { user_repository })
    }
}
