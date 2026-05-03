use crate::ValueWithUnit;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineeringNotebook {
    pub id: String,
    pub title: String,
    pub blocks: Vec<NotebookBlock>,
    pub variables: BTreeMap<String, ValueWithUnit>,
    pub history: Vec<NotebookHistoryEntry>,
}

impl EngineeringNotebook {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            blocks: Vec::new(),
            variables: BTreeMap::new(),
            history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookBlock {
    pub id: String,
    pub kind: NotebookBlockKind,
    pub input: String,
    pub result: Option<NotebookEvaluationResult>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotebookBlockKind {
    Text,
    Expression,
    FormulaCall,
    PreferredValue,
    Assignment,
    Result,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookHistoryEntry {
    pub id: String,
    pub input: String,
    pub result_summary: String,
    pub status: NotebookEvaluationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotebookEvaluationStatus {
    Success,
    Error,
    Unsupported,
}

impl NotebookEvaluationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotebookEvaluationStatus::Success => "success",
            NotebookEvaluationStatus::Error => "error",
            NotebookEvaluationStatus::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookEvaluationRequest {
    pub input: String,
    pub variable_scope: BTreeMap<String, ValueWithUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookEvaluationResult {
    pub input: String,
    pub status: NotebookEvaluationStatus,
    pub kind: NotebookBlockKind,
    pub outputs: BTreeMap<String, ValueWithUnit>,
    pub variables: BTreeMap<String, ValueWithUnit>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
}

impl NotebookEvaluationResult {
    pub fn unsupported(input: &str, message: &str) -> Self {
        Self {
            input: input.to_string(),
            status: NotebookEvaluationStatus::Unsupported,
            kind: NotebookBlockKind::Text,
            outputs: BTreeMap::new(),
            variables: BTreeMap::new(),
            message: Some(message.to_string()),
            warnings: Vec::new(),
        }
    }

    pub fn error(input: &str, message: &str) -> Self {
        Self {
            input: input.to_string(),
            status: NotebookEvaluationStatus::Error,
            kind: NotebookBlockKind::Text,
            outputs: BTreeMap::new(),
            variables: BTreeMap::new(),
            message: Some(message.to_string()),
            warnings: Vec::new(),
        }
    }
}
