use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductWorkflowStatus {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub build_profile: String,
    pub current_project: Option<ProjectSummary>,
    pub workflow_steps: Vec<WorkflowStepStatus>,
    pub module_statuses: Vec<WorkflowModuleStatus>,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

impl ProductWorkflowStatus {
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
            current_project: None,
            workflow_steps: Vec::new(),
            module_statuses: Vec::new(),
            blockers: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_project(mut self, project: ProjectSummary) -> Self {
        self.current_project = Some(project);
        self
    }

    pub fn with_step(mut self, step: WorkflowStepStatus) -> Self {
        self.workflow_steps.push(step);
        self
    }

    pub fn with_module(mut self, module: WorkflowModuleStatus) -> Self {
        self.module_statuses.push(module);
        self
    }

    pub fn with_blocker(mut self, blocker: &str) -> Self {
        self.blockers.push(blocker.to_string());
        self
    }

    pub fn with_warning(mut self, warning: &str) -> Self {
        self.warnings.push(warning.to_string());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub project_id: String,
    pub project_name: String,
    pub format_version: String,
    pub component_count: usize,
    pub net_count: usize,
    pub simulation_profile_count: usize,
}

impl ProjectSummary {
    pub fn from_project(project: &crate::CircuitProject) -> Self {
        Self {
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            format_version: project.format_version.clone(),
            component_count: project.schematic.components.len(),
            net_count: project.schematic.nets.len(),
            simulation_profile_count: project.simulation_profiles.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStepStatus {
    pub id: String,
    pub title: String,
    pub status: WorkflowStatusKind,
    pub screen_id: String,
    pub description: String,
    pub warnings: Vec<String>,
}

impl WorkflowStepStatus {
    pub fn new(
        id: &str,
        title: &str,
        status: WorkflowStatusKind,
        screen_id: &str,
        description: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            status,
            screen_id: screen_id.to_string(),
            description: description.to_string(),
            warnings: Vec::new(),
        }
    }

    pub fn with_warning(mut self, warning: &str) -> Self {
        self.warnings.push(warning.to_string());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowModuleStatus {
    pub id: String,
    pub title: String,
    pub status: WorkflowStatusKind,
    pub details: BTreeMap<String, String>,
}

impl WorkflowModuleStatus {
    pub fn new(id: &str, title: &str, status: WorkflowStatusKind) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            status,
            details: BTreeMap::new(),
        }
    }

    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatusKind {
    Ready,
    Limited,
    Unavailable,
    NotConfigured,
    Error,
}

impl WorkflowStatusKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkflowStatusKind::Ready => "ready",
            WorkflowStatusKind::Limited => "limited",
            WorkflowStatusKind::Unavailable => "unavailable",
            WorkflowStatusKind::NotConfigured => "not_configured",
            WorkflowStatusKind::Error => "error",
        }
    }
}

impl From<crate::ModuleStatus> for WorkflowStatusKind {
    fn from(status: crate::ModuleStatus) -> Self {
        match status {
            crate::ModuleStatus::Ready => WorkflowStatusKind::Ready,
            crate::ModuleStatus::Limited => WorkflowStatusKind::Limited,
            crate::ModuleStatus::Unavailable => WorkflowStatusKind::Unavailable,
            crate::ModuleStatus::Unknown => WorkflowStatusKind::NotConfigured,
        }
    }
}
