use thiserror::Error;

#[derive(Debug, Error)]
pub enum UseCasesBuilderError {
    #[error("Service '{service_name}' is missing")]
    ServiceIsMissing { service_name: String },
}
