use hotsas_core::CoreError;
use hotsas_ports::PortError;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    Core(CoreError),
    Port(PortError),
    MissingProjectState(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(error) => write!(f, "{error}"),
            Self::Port(error) => write!(f, "{error}"),
            Self::MissingProjectState(message) => write!(f, "missing project state: {message}"),
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
