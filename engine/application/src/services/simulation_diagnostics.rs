use crate::ApplicationError;
use hotsas_core::{
    CircuitProject, ComponentLibrary, NgspiceDiagnostics, SimulationDiagnosticEntityKind,
    SimulationDiagnosticEntityRef, SimulationDiagnosticMessage, SimulationDiagnosticSeverity,
    UserCircuitSimulationProfile, UserCircuitSimulationRun, UserCircuitSimulationStatus,
};
use hotsas_ports::SimulationEnginePort;
use std::sync::Arc;

#[derive(Clone)]
pub struct SimulationDiagnosticsService {
    ngspice_engine: Arc<dyn SimulationEnginePort>,
}

impl SimulationDiagnosticsService {
    pub fn new(ngspice_engine: Arc<dyn SimulationEnginePort>) -> Self {
        Self { ngspice_engine }
    }

    pub fn check_ngspice_diagnostics(&self) -> Result<NgspiceDiagnostics, ApplicationError> {
        let availability = self.ngspice_engine.check_availability()?;
        let warnings = vec![];
        let mut errors = vec![];

        if !availability.available {
            errors.push(SimulationDiagnosticMessage {
                code: "NGSPICE_UNAVAILABLE".to_string(),
                severity: SimulationDiagnosticSeverity::Warning,
                title: "ngspice not available".to_string(),
                message: availability.message.clone().unwrap_or_else(|| {
                    "ngspice executable not found. Simulation will fall back to mock engine in auto mode."
                        .to_string()
                }),
                related_entity: Some(SimulationDiagnosticEntityRef {
                    kind: SimulationDiagnosticEntityKind::Engine,
                    id: "ngspice".to_string(),
                }),
                related_model_id: None,
                suggested_fix: Some(
                    "Install ngspice and ensure it is on PATH, or continue using mock engine."
                        .to_string(),
                ),
            });
        }

        Ok(NgspiceDiagnostics {
            availability: availability.clone(),
            executable_path: availability.executable_path.clone(),
            version: availability.version.clone(),
            checked_at: format!("{:?}", std::time::SystemTime::now()),
            warnings,
            errors,
        })
    }

    pub fn diagnose_simulation_preflight(
        &self,
        project: &CircuitProject,
        profile: &UserCircuitSimulationProfile,
    ) -> Result<Vec<SimulationDiagnosticMessage>, ApplicationError> {
        self.diagnose_simulation_preflight_with_library(
            project,
            profile,
            &hotsas_core::built_in_component_library(),
        )
    }

    pub fn diagnose_simulation_preflight_with_library(
        &self,
        project: &CircuitProject,
        profile: &UserCircuitSimulationProfile,
        library: &ComponentLibrary,
    ) -> Result<Vec<SimulationDiagnosticMessage>, ApplicationError> {
        let mut diagnostics = vec![];

        if project.schematic.components.is_empty() {
            diagnostics.push(SimulationDiagnosticMessage {
                code: "NO_COMPONENTS".to_string(),
                severity: SimulationDiagnosticSeverity::Blocking,
                title: "Schematic has no components".to_string(),
                message: "Add at least one component to the schematic before running simulation."
                    .to_string(),
                related_entity: None,
                related_model_id: None,
                suggested_fix: Some(
                    "Place a voltage source and at least one passive component.".to_string(),
                ),
            });
        }

        if project.schematic.nets.is_empty() {
            diagnostics.push(SimulationDiagnosticMessage {
                code: "NO_NETS".to_string(),
                severity: SimulationDiagnosticSeverity::Blocking,
                title: "Schematic has no nets".to_string(),
                message: "Nets are required for SPICE simulation.".to_string(),
                related_entity: None,
                related_model_id: None,
                suggested_fix: Some("Connect component pins to create nets.".to_string()),
            });
        }

        let has_ground = project
            .schematic
            .components
            .iter()
            .any(|c| c.definition_id.contains("ground"));
        if !has_ground {
            diagnostics.push(SimulationDiagnosticMessage {
                code: "NO_GROUND".to_string(),
                severity: SimulationDiagnosticSeverity::Warning,
                title: "No ground reference found".to_string(),
                message: "Simulation may be unstable without a ground reference.".to_string(),
                related_entity: None,
                related_model_id: None,
                suggested_fix: Some(
                    "Add a ground component and connect it to your circuit.".to_string(),
                ),
            });
        }

        if profile.probes.is_empty() {
            diagnostics.push(SimulationDiagnosticMessage {
                code: "NO_PROBES".to_string(),
                severity: SimulationDiagnosticSeverity::Info,
                title: "No probes selected".to_string(),
                message: "No output probes are configured. Results may be minimal.".to_string(),
                related_entity: None,
                related_model_id: None,
                suggested_fix: Some(
                    "Select node voltage probes for nets you want to observe.".to_string(),
                ),
            });
        }

        for probe in &profile.probes {
            match &probe.target {
                hotsas_core::SimulationProbeTarget::Net { net_id } => {
                    if !project.schematic.nets.iter().any(|n| n.id == *net_id) {
                        diagnostics.push(SimulationDiagnosticMessage {
                            code: "INVALID_PROBE_NET".to_string(),
                            severity: SimulationDiagnosticSeverity::Blocking,
                            title: format!("Probe references missing net: {}", net_id),
                            message: format!(
                                "The probe '{}' targets net '{}', which does not exist.",
                                probe.label, net_id
                            ),
                            related_entity: Some(SimulationDiagnosticEntityRef {
                                kind: SimulationDiagnosticEntityKind::Probe,
                                id: probe.id.clone(),
                            }),
                            related_model_id: None,
                            suggested_fix: Some(
                                "Remove or reconfigure the probe to target an existing net."
                                    .to_string(),
                            ),
                        });
                    }
                }
                _ => {}
            }
        }

        // v3.1: Model mapping diagnostics
        let mapping_service = crate::services::ComponentModelMappingService::new();
        let readiness = mapping_service.evaluate_project_simulation_readiness(project, &library);
        for comp in &readiness.components {
            for d in &comp.diagnostics {
                diagnostics.push(SimulationDiagnosticMessage {
                    code: d.code.clone(),
                    severity: match d.severity {
                        hotsas_core::ModelMappingSeverity::Info => {
                            SimulationDiagnosticSeverity::Info
                        }
                        hotsas_core::ModelMappingSeverity::Warning => {
                            SimulationDiagnosticSeverity::Warning
                        }
                        hotsas_core::ModelMappingSeverity::Error => {
                            SimulationDiagnosticSeverity::Error
                        }
                        hotsas_core::ModelMappingSeverity::Blocking => {
                            SimulationDiagnosticSeverity::Blocking
                        }
                    },
                    title: d.title.clone(),
                    message: d.message.clone(),
                    related_entity: Some(SimulationDiagnosticEntityRef {
                        kind: SimulationDiagnosticEntityKind::Component,
                        id: comp
                            .component_instance_id
                            .clone()
                            .unwrap_or_else(|| comp.component_definition_id.clone()),
                    })
                    .or_else(|| {
                        d.related_component_id
                            .as_ref()
                            .map(|id| SimulationDiagnosticEntityRef {
                                kind: SimulationDiagnosticEntityKind::Component,
                                id: id.clone(),
                            })
                    }),
                    related_model_id: d.related_model_id.clone(),
                    suggested_fix: d.suggested_fix.clone(),
                });
            }
        }

        Ok(diagnostics)
    }

    pub fn diagnose_failed_run(
        &self,
        run: &UserCircuitSimulationRun,
    ) -> Result<Vec<SimulationDiagnosticMessage>, ApplicationError> {
        let mut diagnostics = vec![];

        if run.status == UserCircuitSimulationStatus::Failed {
            diagnostics.push(SimulationDiagnosticMessage {
                code: "RUN_FAILED".to_string(),
                severity: SimulationDiagnosticSeverity::Error,
                title: "Simulation run failed".to_string(),
                message: format!(
                    "The simulation using profile '{}' and engine '{}' failed.",
                    run.profile.name, run.engine_used
                ),
                related_entity: Some(SimulationDiagnosticEntityRef {
                    kind: SimulationDiagnosticEntityKind::Profile,
                    id: run.profile.id.clone(),
                }),
                related_model_id: None,
                suggested_fix: Some(
                    "Check schematic validity, probe targets, and ngspice availability."
                        .to_string(),
                ),
            });
        }

        if run.engine_used == "mock"
            && run.profile.engine == hotsas_core::UserCircuitSimulationEngine::Auto
        {
            diagnostics.push(SimulationDiagnosticMessage {
                code: "AUTO_MOCK_FALLBACK".to_string(),
                severity: SimulationDiagnosticSeverity::Info,
                title: "Auto mode fell back to mock engine".to_string(),
                message: "ngspice was unavailable; simulation ran with mock engine instead."
                    .to_string(),
                related_entity: Some(SimulationDiagnosticEntityRef {
                    kind: SimulationDiagnosticEntityKind::Engine,
                    id: "mock".to_string(),
                }),
                related_model_id: None,
                suggested_fix: Some(
                    "Install ngspice to use the real SPICE engine in auto mode.".to_string(),
                ),
            });
        }

        for err in &run.errors {
            diagnostics.push(SimulationDiagnosticMessage {
                code: err.code.clone(),
                severity: SimulationDiagnosticSeverity::Error,
                title: err.message.clone(),
                message: err.message.clone(),
                related_entity: None,
                related_model_id: None,
                suggested_fix: None,
            });
        }

        for warn in &run.warnings {
            diagnostics.push(SimulationDiagnosticMessage {
                code: warn.code.clone(),
                severity: SimulationDiagnosticSeverity::Warning,
                title: warn.message.clone(),
                message: warn.message.clone(),
                related_entity: None,
                related_model_id: None,
                suggested_fix: None,
            });
        }

        Ok(diagnostics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{
        EngineeringUnit, EngineeringValue, NgspiceAvailability, SimulationMeasurement,
        SimulationPoint, SimulationProbe, SimulationProbeKind, SimulationProbeTarget,
        SimulationSeries, SimulationWorkflowError, SimulationWorkflowWarning,
        UserCircuitAnalysisType, UserCircuitSimulationEngine, UserCircuitSimulationProfile,
        UserCircuitSimulationResult, UserCircuitSimulationStatus, ValueWithUnit,
    };

    fn mock_project() -> CircuitProject {
        hotsas_core::rc_low_pass_project()
    }

    fn empty_project() -> CircuitProject {
        CircuitProject {
            id: "empty".to_string(),
            name: "Empty".to_string(),
            format_version: "1.0".to_string(),
            engine_version: "0.1.4".to_string(),
            project_type: "circuit".to_string(),
            created_at: "now".to_string(),
            updated_at: "now".to_string(),
            schematic: hotsas_core::CircuitModel {
                id: "sch-empty".to_string(),
                title: "Empty".to_string(),
                components: vec![],
                wires: vec![],
                nets: vec![],
                labels: vec![],
                probes: vec![],
                annotations: vec![],
            },
            simulation_profiles: vec![],
            linked_libraries: vec![],
            reports: vec![],
        }
    }

    fn mock_profile() -> UserCircuitSimulationProfile {
        UserCircuitSimulationProfile {
            id: "mock".to_string(),
            name: "Mock".to_string(),
            analysis_type: UserCircuitAnalysisType::AcSweep,
            engine: UserCircuitSimulationEngine::Mock,
            probes: vec![],
            ac: None,
            transient: None,
            op: None,
        }
    }

    struct MockNgspiceAvailable;
    impl SimulationEnginePort for MockNgspiceAvailable {
        fn engine_name(&self) -> &str {
            "mock-ngspice"
        }
        fn check_availability(&self) -> Result<NgspiceAvailability, hotsas_ports::PortError> {
            Ok(NgspiceAvailability {
                available: true,
                executable_path: Some("/bin/ngspice".to_string()),
                version: Some("42".to_string()),
                message: None,
                warnings: vec![],
            })
        }
        fn run_ac_sweep(
            &self,
            _project: &CircuitProject,
            _profile: &hotsas_core::SimulationProfile,
        ) -> Result<hotsas_core::SimulationResult, hotsas_ports::PortError> {
            unimplemented!()
        }
    }

    struct MockNgspiceUnavailable;
    impl SimulationEnginePort for MockNgspiceUnavailable {
        fn engine_name(&self) -> &str {
            "mock-ngspice"
        }
        fn check_availability(&self) -> Result<NgspiceAvailability, hotsas_ports::PortError> {
            Ok(NgspiceAvailability {
                available: false,
                executable_path: None,
                version: None,
                message: Some("not found".to_string()),
                warnings: vec![],
            })
        }
        fn run_ac_sweep(
            &self,
            _project: &CircuitProject,
            _profile: &hotsas_core::SimulationProfile,
        ) -> Result<hotsas_core::SimulationResult, hotsas_ports::PortError> {
            unimplemented!()
        }
    }

    #[test]
    fn check_ngspice_diagnostics_when_available() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let diag = svc.check_ngspice_diagnostics().unwrap();
        assert!(diag.availability.available);
        assert_eq!(diag.errors.len(), 0);
    }

    #[test]
    fn check_ngspice_diagnostics_when_unavailable() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceUnavailable));
        let diag = svc.check_ngspice_diagnostics().unwrap();
        assert!(!diag.availability.available);
        assert_eq!(diag.errors.len(), 1);
        assert_eq!(diag.errors[0].code, "NGSPICE_UNAVAILABLE");
    }

    #[test]
    fn preflight_empty_project_has_blocking_errors() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let project = empty_project();
        let profile = mock_profile();
        let diagnostics = svc
            .diagnose_simulation_preflight(&project, &profile)
            .unwrap();
        let blocking: Vec<_> = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, SimulationDiagnosticSeverity::Blocking))
            .collect();
        assert_eq!(blocking.len(), 2);
        assert!(blocking.iter().any(|d| d.code == "NO_COMPONENTS"));
        assert!(blocking.iter().any(|d| d.code == "NO_NETS"));
    }

    #[test]
    fn preflight_no_ground_warning() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let mut project = mock_project();
        project
            .schematic
            .components
            .retain(|c| !c.definition_id.contains("ground"));
        let profile = mock_profile();
        let diagnostics = svc
            .diagnose_simulation_preflight(&project, &profile)
            .unwrap();
        assert!(diagnostics.iter().any(|d| d.code == "NO_GROUND"));
    }

    #[test]
    fn preflight_no_probes_info() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let project = mock_project();
        let profile = mock_profile();
        let diagnostics = svc
            .diagnose_simulation_preflight(&project, &profile)
            .unwrap();
        assert!(diagnostics.iter().any(|d| d.code == "NO_PROBES"));
    }

    #[test]
    fn preflight_invalid_probe_net_blocking() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let project = mock_project();
        let mut profile = mock_profile();
        profile.probes.push(SimulationProbe {
            id: "p1".to_string(),
            label: "V(out)".to_string(),
            kind: SimulationProbeKind::NodeVoltage,
            target: SimulationProbeTarget::Net {
                net_id: "nonexistent".to_string(),
            },
            unit: None,
        });
        let diagnostics = svc
            .diagnose_simulation_preflight(&project, &profile)
            .unwrap();
        assert!(diagnostics.iter().any(|d| d.code == "INVALID_PROBE_NET"));
    }

    #[test]
    fn diagnose_failed_run_returns_error() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let run = UserCircuitSimulationRun {
            id: "run-1".to_string(),
            project_id: "proj".to_string(),
            profile: mock_profile(),
            generated_netlist: "".to_string(),
            status: UserCircuitSimulationStatus::Failed,
            engine_used: "mock".to_string(),
            warnings: vec![SimulationWorkflowWarning {
                code: "W1".to_string(),
                message: "warn".to_string(),
            }],
            errors: vec![SimulationWorkflowError {
                code: "E1".to_string(),
                message: "err".to_string(),
            }],
            result: None,
            created_at: "now".to_string(),
        };
        let diagnostics = svc.diagnose_failed_run(&run).unwrap();
        assert!(diagnostics.iter().any(|d| d.code == "RUN_FAILED"));
        assert!(diagnostics.iter().any(|d| d.code == "E1"));
        assert!(diagnostics.iter().any(|d| d.code == "W1"));
    }

    #[test]
    fn diagnose_auto_fallback_info() {
        let svc = SimulationDiagnosticsService::new(Arc::new(MockNgspiceAvailable));
        let mut profile = mock_profile();
        profile.engine = UserCircuitSimulationEngine::Auto;
        let run = UserCircuitSimulationRun {
            id: "run-1".to_string(),
            project_id: "proj".to_string(),
            profile,
            generated_netlist: "".to_string(),
            status: UserCircuitSimulationStatus::Succeeded,
            engine_used: "mock".to_string(),
            warnings: vec![],
            errors: vec![],
            result: None,
            created_at: "now".to_string(),
        };
        let diagnostics = svc.diagnose_failed_run(&run).unwrap();
        assert!(diagnostics.iter().any(|d| d.code == "AUTO_MOCK_FALLBACK"));
    }
}
