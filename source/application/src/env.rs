use std::{error::Error, marker::PhantomData};

use crate::services::{self, ApplicationServices};

pub struct ApplicationEnvironment {
    services: ApplicationServices,
}

impl ApplicationEnvironment {
    pub fn init(services: ApplicationServices) -> Self {
        ApplicationEnvironment { services }
    }
}
