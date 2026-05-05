use std::fmt;

#[derive(Debug)]
pub enum CliError {
    Io(String),
    Validation(String),
    Input(String),
    Unsupported(String),
    Api(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io(msg) => write!(f, "IO error: {msg}"),
            CliError::Validation(msg) => write!(f, "Validation error: {msg}"),
            CliError::Input(msg) => write!(f, "Input error: {msg}"),
            CliError::Unsupported(msg) => write!(f, "Unsupported: {msg}"),
            CliError::Api(msg) => write!(f, "API error: {msg}"),
        }
    }
}

impl std::error::Error for CliError {}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Io(_) => 1,
            CliError::Validation(_) => 2,
            CliError::Input(_) => 3,
            CliError::Unsupported(_) => 4,
            CliError::Api(_) => 1,
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e.to_string())
    }
}

impl From<hotsas_api::ApiError> for CliError {
    fn from(e: hotsas_api::ApiError) -> Self {
        let msg = e.to_string();
        // Heuristic: if message contains "validation", treat as validation error
        if msg.to_lowercase().contains("validation")
            || msg.to_lowercase().contains("invalid")
            || msg.to_lowercase().contains("missing")
        {
            CliError::Validation(msg)
        } else if msg.to_lowercase().contains("not found")
            || msg.to_lowercase().contains("unsupported")
        {
            CliError::Unsupported(msg)
        } else {
            CliError::Api(msg)
        }
    }
}
