mod traits;
pub use traits::*;

pub struct ApplicationServices<'a> {
    pub user_service: Option<Box<dyn UserService + 'a>>,
}

impl<'a> ApplicationServices<'a> {
    pub fn init() -> Self {
        Self { user_service: None }
    }

    pub fn set_user_service(&mut self, service: impl UserService + 'a) -> &mut Self {
        self.user_service = Some(Box::new(service));
        self
    }
}
