use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebApiError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}
