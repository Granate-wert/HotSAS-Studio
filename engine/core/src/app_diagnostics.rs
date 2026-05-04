use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppDiagnosticsReport {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub build_profile: String,
    pub modules: Vec<ModuleDiagnostics>,
    pub checks: Vec<ReadinessCheck>,
    pub warnings: Vec<String>,
}

impl AppDiagnosticsReport {
    pub fn new(
        app_name: &str,
        app_version: &str,
        roadmap_stage: &str,
        build_profile: &str,
    ) -> Self {
        Self {
            app_name: app_name.to_string(),
            app_version: app_version.to_string(),
            roadmap_stage: roadmap_stage.to_string(),
            build_profile: build_profile.to_string(),
            modules: Vec::new(),
            checks: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleDiagnostics {
    pub id: String,
    pub title: String,
    pub status: ModuleStatus,
    pub summary: String,
    pub details: BTreeMap<String, String>,
}

impl ModuleDiagnostics {
    pub fn new(id: &str, title: &str, status: ModuleStatus, summary: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            status,
            summary: summary.to_string(),
            details: BTreeMap::new(),
        }
    }

    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleStatus {
    Ready,
    Limited,
    Unavailable,
    Unknown,
}

impl ModuleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleStatus::Ready => "ready",
            ModuleStatus::Limited => "limited",
            ModuleStatus::Unavailable => "unavailable",
            ModuleStatus::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessCheck {
    pub id: String,
    pub title: String,
    pub status: ReadinessStatus,
    pub message: String,
}

impl ReadinessCheck {
    pub fn new(id: &str, title: &str, status: ReadinessStatus, message: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            status,
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadinessStatus {
    Pass,
    Warn,
    Fail,
    NotRun,
}

impl ReadinessStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReadinessStatus::Pass => "pass",
            ReadinessStatus::Warn => "warn",
            ReadinessStatus::Fail => "fail",
            ReadinessStatus::NotRun => "not_run",
        }
    }
}
