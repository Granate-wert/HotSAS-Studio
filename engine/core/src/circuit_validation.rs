use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitValidationIssue {
    pub code: String,
    pub message: String,
    pub component_id: Option<String>,
    pub net_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitValidationReport {
    pub valid: bool,
    pub warnings: Vec<CircuitValidationIssue>,
    pub errors: Vec<CircuitValidationIssue>,
}

impl CircuitValidationReport {
    pub fn new() -> Self {
        Self {
            valid: true,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_error(
        &mut self,
        code: &str,
        message: &str,
        component_id: Option<String>,
        net_id: Option<String>,
    ) {
        self.valid = false;
        self.errors.push(CircuitValidationIssue {
            code: code.to_string(),
            message: message.to_string(),
            component_id,
            net_id,
        });
    }

    pub fn add_warning(
        &mut self,
        code: &str,
        message: &str,
        component_id: Option<String>,
        net_id: Option<String>,
    ) {
        self.warnings.push(CircuitValidationIssue {
            code: code.to_string(),
            message: message.to_string(),
            component_id,
            net_id,
        });
    }
}

impl Default for CircuitValidationReport {
    fn default() -> Self {
        Self::new()
    }
}
