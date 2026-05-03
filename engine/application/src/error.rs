use hotsas_core::CoreError;
use hotsas_ports::PortError;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    Core(CoreError),
    Port(PortError),
    MissingProjectState(String),
    FormulaNotFound(String),
    DuplicateFormulaId(String),
    InvalidFormulaPack(String),
    InvalidBinding(String),
    InvalidInput(String),
    State(String),
    NotFound(String),
    Storage(String),
    Export(String),
    Simulation(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(error) => write!(f, "{error}"),
            Self::Port(error) => write!(f, "{error}"),
            Self::MissingProjectState(message) => write!(f, "missing project state: {message}"),
            Self::FormulaNotFound(id) => write!(f, "formula not found: {id}"),
            Self::DuplicateFormulaId(id) => write!(f, "duplicate formula id: {id}"),
            Self::InvalidFormulaPack(message) => write!(f, "invalid formula pack: {message}"),
            Self::InvalidBinding(message) => write!(f, "invalid formula binding: {message}"),
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::State(message) => write!(f, "state error: {message}"),
            Self::NotFound(message) => write!(f, "not found: {message}"),
            Self::Storage(message) => write!(f, "storage error: {message}"),
            Self::Export(message) => write!(f, "export error: {message}"),
            Self::Simulation(message) => write!(f, "simulation error: {message}"),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<CoreError> for ApplicationError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}

impl From<PortError> for ApplicationError {
    fn from(value: PortError) -> Self {
        Self::Port(value)
    }
}
