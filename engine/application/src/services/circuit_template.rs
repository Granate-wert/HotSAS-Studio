use hotsas_core::{rc_low_pass_project, CircuitProject};

#[derive(Debug, Clone, Default)]
pub struct CircuitTemplateService;

impl CircuitTemplateService {
    pub fn create_rc_low_pass_demo_project(&self) -> CircuitProject {
        rc_low_pass_project()
    }
}
