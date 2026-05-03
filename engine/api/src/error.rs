use hotsas_application::ApplicationError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum ApiError {
    Application(ApplicationError),
    Core(hotsas_core::CoreError),
    InvalidInput(String),
    State(String),
}

impl ApiError {
    pub fn to_dto(&self) -> ApiErrorDto {
        ApiErrorDto {
            code: self.code().to_string(),
            message: self.to_string(),
            details: self.details(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Application(ApplicationError::MissingProjectState(_)) => "missing_project_state",
            Self::Application(ApplicationError::FormulaNotFound(_)) => "formula_not_found",
            Self::Application(ApplicationError::DuplicateFormulaId(_)) => "duplicate_formula_id",
            Self::Application(ApplicationError::InvalidFormulaPack(_)) => "invalid_formula_pack",
            Self::Application(ApplicationError::InvalidBinding(_)) => "invalid_formula_binding",
            Self::Application(ApplicationError::Core(_)) | Self::Core(_) => "core_error",
            Self::Application(ApplicationError::Port(_)) => "port_error",
            Self::Application(ApplicationError::InvalidInput(_)) => "invalid_input",
            Self::Application(ApplicationError::State(_)) => "state_error",
            Self::Application(ApplicationError::NotFound(_)) => "not_found",
            Self::Application(ApplicationError::Storage(_)) => "storage_error",
            Self::Application(ApplicationError::Export(_)) => "export_error",
            Self::Application(ApplicationError::Simulation(_)) => "simulation_error",
            Self::InvalidInput(_) => "invalid_input",
            Self::State(_) => "state_error",
        }
    }

    fn details(&self) -> Option<String> {
        match self {
            Self::InvalidInput(message) | Self::State(message) => Some(message.clone()),
            Self::Application(error) => Some(error.to_string()),
            Self::Core(error) => Some(error.to_string()),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Application(error) => write!(f, "{error}"),
            Self::Core(error) => write!(f, "{error}"),
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::State(message) => write!(f, "state error: {message}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<ApplicationError> for ApiError {
    fn from(value: ApplicationError) -> Self {
        Self::Application(value)
    }
}

impl From<hotsas_core::CoreError> for ApiError {
    fn from(value: hotsas_core::CoreError) -> Self {
        Self::Core(value)
    }
}
