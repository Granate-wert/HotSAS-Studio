use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    InvalidEngineeringValue(String),
    InvalidUnit(String),
    UnsupportedSeries(String),
    ValueOutOfRange(String),
    MissingParameter {
        component_id: String,
        parameter: String,
    },
    InvalidCircuit(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEngineeringValue(value) => {
                write!(f, "invalid engineering value: {value}")
            }
            Self::InvalidUnit(unit) => write!(f, "invalid engineering unit: {unit}"),
            Self::UnsupportedSeries(series) => {
                write!(f, "unsupported preferred value series: {series}")
            }
            Self::ValueOutOfRange(message) => write!(f, "value out of range: {message}"),
            Self::MissingParameter {
                component_id,
                parameter,
            } => write!(
                f,
                "missing parameter {parameter} on component {component_id}"
            ),
            Self::InvalidCircuit(message) => write!(f, "invalid circuit: {message}"),
        }
    }
}

impl std::error::Error for CoreError {}
