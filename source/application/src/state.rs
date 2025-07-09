use std::{error::Error, marker::PhantomData};

use crate::{
    services::{ApplicationServices, UserService},
    use_cases::UseCases,
};

trait ApplicationStateVariant {}

pub struct NewApplicationState {}
impl ApplicationStateVariant for NewApplicationState {}

pub struct InitialApplicationState {}
impl ApplicationStateVariant for InitialApplicationState {}

pub struct ApplicationStateWithServices {}
impl ApplicationStateVariant for ApplicationStateWithServices {}

pub struct ApplicationStateWithUseCases {}
impl ApplicationStateVariant for ApplicationStateWithUseCases {}

pub struct ApplicationState<'a, V: ApplicationStateVariant> {
    _state: PhantomData<V>,
    services: ApplicationServices<'a>,
    use_cases: UseCases,
}

pub type AppState<'a> = ApplicationState<'a, NewApplicationState>;

impl<'a> ApplicationState<'a, NewApplicationState> {
    pub fn init() -> ApplicationState<'a, InitialApplicationState> {
        ApplicationState::<InitialApplicationState> {
            _state: PhantomData,
            services: ApplicationServices::<'a>::init(),
            use_cases: UseCases::init(),
        }
    }
}

impl<'a> ApplicationState<'a, InitialApplicationState> {
    pub fn add_user_service(&mut self, service: impl UserService + 'a) {
        self.services.set_user_service(service);
    }

    pub fn build(
        self,
    ) -> Result<ApplicationState<'a, ApplicationStateWithServices>, Box<dyn Error>> {
        Result::Ok(ApplicationState::<'a, ApplicationStateWithServices> {
            _state: PhantomData,
            services: self.services,
            use_cases: self.use_cases,
        })
    }
}
