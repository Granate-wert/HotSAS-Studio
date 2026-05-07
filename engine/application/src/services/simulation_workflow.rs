use crate::{ApplicationError, NgspiceSimulationService, SimulationEngineChoice};
use hotsas_core::{
    AcSweepSettings, CircuitProject, EngineeringUnit, OperatingPointSettings,
    SimulationMeasurement, SimulationPoint, SimulationPreflightResult, SimulationProbe,
    SimulationProbeKind, SimulationProbeTarget, SimulationSeries, SimulationWorkflowError,
    SimulationWorkflowWarning, TransientSettings, UserCircuitAnalysisType,
    UserCircuitSimulationEngine, UserCircuitSimulationProfile, UserCircuitSimulationResult,
    UserCircuitSimulationRun, UserCircuitSimulationStatus, ValueWithUnit,
};
use hotsas_ports::NetlistExporterPort;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SimulationWorkflowService {
    netlist_exporter: Arc<dyn NetlistExporterPort>,
    ngspice_service: NgspiceSimulationService,
    last_runs: Arc<Mutex<BTreeMap<String, UserCircuitSimulationRun>>>,
}

impl SimulationWorkflowService {
    pub fn new(
        netlist_exporter: Arc<dyn NetlistExporterPort>,
        ngspice_service: NgspiceSimulationService,
    ) -> Self {
        Self {
            netlist_exporter,
            ngspice_service,
            last_runs: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn list_default_simulation_profiles(
        &self,
        _project: &CircuitProject,
    ) -> Result<Vec<UserCircuitSimulationProfile>, ApplicationError> {
        Ok(vec![
            UserCircuitSimulationProfile {
                id: "mock-op".to_string(),
                name: "Operating Point (Mock)".to_string(),
                analysis_type: UserCircuitAnalysisType::OperatingPoint,
                engine: UserCircuitSimulationEngine::Mock,
                probes: vec![],
                ac: None,
                transient: None,
                op: Some(OperatingPointSettings {
                    include_node_voltages: true,
                    include_branch_currents: true,
                }),
            },
            UserCircuitSimulationProfile {
                id: "mock-ac".to_string(),
                name: "AC Sweep (Mock)".to_string(),
                analysis_type: UserCircuitAnalysisType::AcSweep,
                engine: UserCircuitSimulationEngine::Mock,
                probes: vec![],
                ac: Some(AcSweepSettings {
                    start_hz: 10.0,
                    stop_hz: 1_000_000.0,
                    points_per_decade: 100,
                }),
                transient: None,
                op: None,
            },
            UserCircuitSimulationProfile {
                id: "mock-transient".to_string(),
                name: "Transient (Mock)".to_string(),
                analysis_type: UserCircuitAnalysisType::Transient,
                engine: UserCircuitSimulationEngine::Mock,
                probes: vec![],
                ac: None,
                transient: Some(TransientSettings {
                    step_seconds: 1e-6,
                    stop_seconds: 1e-3,
                }),
                op: None,
            },
            UserCircuitSimulationProfile {
                id: "auto-ac".to_string(),
                name: "AC Sweep (Auto)".to_string(),
                analysis_type: UserCircuitAnalysisType::AcSweep,
                engine: UserCircuitSimulationEngine::Auto,
                probes: vec![],
                ac: Some(AcSweepSettings {
                    start_hz: 10.0,
                    stop_hz: 1_000_000.0,
                    points_per_decade: 100,
                }),
                transient: None,
                op: None,
            },
        ])
    }

    pub fn suggest_simulation_probes(
        &self,
        project: &CircuitProject,
    ) -> Result<Vec<SimulationProbe>, ApplicationError> {
        let mut probes = vec![];
        for net in &project.schematic.nets {
            probes.push(SimulationProbe {
                id: format!("probe-v-{}", net.id),
                label: format!("V({})", net.name),
                kind: SimulationProbeKind::NodeVoltage,
                target: SimulationProbeTarget::Net {
                    net_id: net.id.clone(),
                },
                unit: Some(EngineeringUnit::Volt),
            });
        }
        Ok(probes)
    }

    pub fn validate_circuit_for_simulation(
        &self,
        project: &CircuitProject,
        profile: &UserCircuitSimulationProfile,
    ) -> Result<SimulationPreflightResult, ApplicationError> {
        let mut errors = vec![];
        let mut warnings = vec![];

        if project.schematic.components.is_empty() {
            errors.push(SimulationWorkflowError {
                code: "NO_COMPONENTS".to_string(),
                message: "Schematic has no components".to_string(),
            });
        }

        if project.schematic.nets.is_empty() {
            errors.push(SimulationWorkflowError {
                code: "NO_NETS".to_string(),
                message: "Schematic has no nets".to_string(),
            });
        }

        // Check that every non-ground component has at least one connection
        for comp in &project.schematic.components {
            if comp.definition_id.contains("ground") {
                continue;
            }
            if comp.connected_nets.is_empty() {
                warnings.push(SimulationWorkflowWarning {
                    code: "UNCONNECTED_COMPONENT".to_string(),
                    message: format!("{} has no connections", comp.instance_id),
                });
            }
        }

        // Probe validation
        for probe in &profile.probes {
            match &probe.target {
                SimulationProbeTarget::Net { net_id } => {
                    if !project.schematic.nets.iter().any(|n| n.id == *net_id) {
                        errors.push(SimulationWorkflowError {
                            code: "INVALID_PROBE_NET".to_string(),
                            message: format!("Probe references non-existent net: {}", net_id),
                        });
                    }
                }
                SimulationProbeTarget::ComponentPin {
                    component_id,
                    pin_id,
                } => {
                    if let Some(comp) = project
                        .schematic
                        .components
                        .iter()
                        .find(|c| c.instance_id == *component_id)
                    {
                        if !comp.connected_nets.iter().any(|cn| cn.pin_id == *pin_id) {
                            warnings.push(SimulationWorkflowWarning {
                                code: "PROBE_PIN_UNCONNECTED".to_string(),
                                message: format!(
                                    "Probe targets unconnected pin {} on {}",
                                    pin_id, component_id
                                ),
                            });
                        }
                    } else {
                        errors.push(SimulationWorkflowError {
                            code: "INVALID_PROBE_COMPONENT".to_string(),
                            message: format!(
                                "Probe references non-existent component: {}",
                                component_id
                            ),
                        });
                    }
                }
                _ => {}
            }
        }

        // Ground check
        let has_ground = project
            .schematic
            .components
            .iter()
            .any(|c| c.definition_id.contains("ground"));
        if !has_ground {
            warnings.push(SimulationWorkflowWarning {
                code: "NO_GROUND".to_string(),
                message: "No ground reference found; simulation may be unstable".to_string(),
            });
        }

        let mut netlist_preview = None;
        if errors.is_empty() {
            match self.generate_netlist(project, profile) {
                Ok(netlist) => netlist_preview = Some(netlist),
                Err(e) => errors.push(SimulationWorkflowError {
                    code: "NETLIST_ERROR".to_string(),
                    message: format!("Failed to generate netlist: {e}"),
                }),
            }
        }

        Ok(SimulationPreflightResult {
            can_run: errors.is_empty() && netlist_preview.is_some(),
            blocking_errors: errors,
            warnings,
            generated_netlist_preview: netlist_preview,
        })
    }

    pub fn run_user_circuit_simulation(
        &self,
        project: &CircuitProject,
        profile: UserCircuitSimulationProfile,
    ) -> Result<UserCircuitSimulationRun, ApplicationError> {
        let netlist = self.generate_netlist(project, &profile)?;
        let sim_profile = to_simulation_profile(&profile);
        let choice = to_engine_choice(&profile.engine);

        let sim_result =
            match profile.analysis_type {
                UserCircuitAnalysisType::AcSweep => self.ngspice_service.run_ac_sweep_with_profile(
                    project,
                    &sim_profile,
                    choice.clone(),
                ),
                UserCircuitAnalysisType::OperatingPoint => self
                    .ngspice_service
                    .run_operating_point_with_profile(project, &sim_profile, choice.clone()),
                UserCircuitAnalysisType::Transient => self
                    .ngspice_service
                    .run_transient_with_profile(project, &sim_profile, choice.clone()),
            };

        let (status, engine_used, result, warnings, errors) = match sim_result {
            Ok(sr) => {
                let engine_used = sr.engine.clone();
                let warnings = sr.warnings.clone();
                let user_result = to_user_result(&sr);
                (
                    UserCircuitSimulationStatus::Succeeded,
                    engine_used,
                    Some(user_result),
                    warnings,
                    sr.errors,
                )
            }
            Err(e) => {
                let engine_used = match choice {
                    SimulationEngineChoice::Mock => "mock".to_string(),
                    SimulationEngineChoice::Ngspice => "ngspice".to_string(),
                    SimulationEngineChoice::Auto => "auto".to_string(),
                };
                let errors = vec![format!("{e}")];
                (
                    UserCircuitSimulationStatus::Failed,
                    engine_used,
                    None,
                    vec![],
                    errors,
                )
            }
        };

        // Add ngspice unavailable warning for auto mode if fallback occurred
        let warnings = if matches!(profile.engine, UserCircuitSimulationEngine::Auto)
            && engine_used == "mock"
            && status == UserCircuitSimulationStatus::Succeeded
        {
            let mut w = warnings;
            w.push("ngspice unavailable in auto mode; fallback to mock engine".to_string());
            w
        } else {
            warnings
        };

        let run = UserCircuitSimulationRun {
            id: format!("run-{}", project.id),
            project_id: project.id.clone(),
            profile,
            generated_netlist: netlist,
            status,
            engine_used,
            warnings: warnings
                .into_iter()
                .map(|m| SimulationWorkflowWarning {
                    code: "SIM_WARNING".to_string(),
                    message: m,
                })
                .collect(),
            errors: errors
                .into_iter()
                .map(|m| SimulationWorkflowError {
                    code: "SIM_ERROR".to_string(),
                    message: m,
                })
                .collect(),
            result,
            created_at: format!("{:?}", std::time::SystemTime::now()),
        };

        if let Ok(mut guard) = self.last_runs.lock() {
            guard.insert(run.project_id.clone(), run.clone());
        }

        Ok(run)
    }

    pub fn get_last_user_circuit_simulation(
        &self,
        project_id: &str,
    ) -> Option<UserCircuitSimulationRun> {
        self.last_runs
            .lock()
            .ok()
            .and_then(|guard| guard.get(project_id).cloned())
    }

    pub fn clear_last_user_circuit_simulation(
        &self,
        project_id: &str,
    ) -> Result<(), ApplicationError> {
        if let Ok(mut guard) = self.last_runs.lock() {
            guard.remove(project_id);
        }
        Ok(())
    }

    pub fn simulation_result_to_report_section(
        &self,
        run: &UserCircuitSimulationRun,
    ) -> Result<hotsas_core::advanced_report::ReportSection, ApplicationError> {
        use hotsas_core::advanced_report::{ReportContentBlock, ReportSection};

        let mut blocks = vec![ReportContentBlock::Paragraph {
            text: format!(
                "Simulation profile: {} ({})",
                run.profile.name, run.engine_used
            ),
        }];

        if let Some(result) = &run.result {
            if !result.summary.is_empty() {
                let mut rows: Vec<Vec<String>> = vec![];
                for m in &result.summary {
                    rows.push(vec![
                        m.name.clone(),
                        format!("{:.6} {}", m.value.si_value(), m.unit_symbol),
                    ]);
                }
                blocks.push(ReportContentBlock::DataTable {
                    title: "Measurements".to_string(),
                    columns: vec!["Measurement".to_string(), "Value".to_string()],
                    rows,
                });
            }
            if !result.series.is_empty() {
                blocks.push(ReportContentBlock::Paragraph {
                    text: format!("Series count: {}", result.series.len()),
                });
            }
        }

        let warnings: Vec<hotsas_core::advanced_report::ReportWarning> = run
            .warnings
            .iter()
            .map(|w| hotsas_core::advanced_report::ReportWarning {
                severity: hotsas_core::advanced_report::ReportWarningSeverity::Warning,
                code: w.code.clone(),
                message: w.message.clone(),
                section_kind: Some(
                    hotsas_core::advanced_report::ReportSectionKind::SimulationResults,
                ),
            })
            .collect();

        Ok(ReportSection {
            kind: hotsas_core::advanced_report::ReportSectionKind::SimulationResults,
            title: "Simulation Results".to_string(),
            status: hotsas_core::advanced_report::ReportSectionStatus::Included,
            blocks,
            warnings,
        })
    }

    fn generate_netlist(
        &self,
        project: &CircuitProject,
        profile: &UserCircuitSimulationProfile,
    ) -> Result<String, ApplicationError> {
        let base = self.netlist_exporter.export_spice_netlist(project)?;
        let mut lines: Vec<String> = base.lines().map(|s| s.to_string()).collect();

        // Remove the trailing .end, append analysis directive, then .end
        if let Some(last) = lines.last() {
            if last.trim() == ".end" {
                lines.pop();
            }
        }

        match profile.analysis_type {
            UserCircuitAnalysisType::AcSweep => {
                if let Some(ac) = &profile.ac {
                    lines.push(format!(
                        ".ac dec {} {} {}",
                        ac.points_per_decade,
                        format_hz(ac.start_hz),
                        format_hz(ac.stop_hz)
                    ));
                }
            }
            UserCircuitAnalysisType::OperatingPoint => {
                lines.push(".op".to_string());
            }
            UserCircuitAnalysisType::Transient => {
                if let Some(tr) = &profile.transient {
                    lines.push(format!(
                        ".tran {} {}",
                        format_time(tr.step_seconds),
                        format_time(tr.stop_seconds)
                    ));
                }
            }
        }

        lines.push(".end".to_string());
        Ok(lines.join("\n"))
    }
}

fn to_simulation_profile(profile: &UserCircuitSimulationProfile) -> hotsas_core::SimulationProfile {
    let mut parameters = BTreeMap::new();
    match profile.analysis_type {
        UserCircuitAnalysisType::AcSweep => {
            if let Some(ac) = &profile.ac {
                parameters.insert(
                    "start".to_string(),
                    ValueWithUnit::new_si(ac.start_hz, EngineeringUnit::Hertz),
                );
                parameters.insert(
                    "stop".to_string(),
                    ValueWithUnit::new_si(ac.stop_hz, EngineeringUnit::Hertz),
                );
                parameters.insert(
                    "points_per_decade".to_string(),
                    ValueWithUnit::new_si(ac.points_per_decade as f64, EngineeringUnit::Unitless),
                );
            }
        }
        UserCircuitAnalysisType::Transient => {
            if let Some(tr) = &profile.transient {
                parameters.insert(
                    "step".to_string(),
                    ValueWithUnit::new_si(tr.step_seconds, EngineeringUnit::Second),
                );
                parameters.insert(
                    "stop".to_string(),
                    ValueWithUnit::new_si(tr.stop_seconds, EngineeringUnit::Second),
                );
            }
        }
        UserCircuitAnalysisType::OperatingPoint => {}
    }

    hotsas_core::SimulationProfile {
        id: profile.id.clone(),
        simulation_type: match profile.analysis_type {
            UserCircuitAnalysisType::OperatingPoint => hotsas_core::SimulationType::OperatingPoint,
            UserCircuitAnalysisType::AcSweep => hotsas_core::SimulationType::AcSweep,
            UserCircuitAnalysisType::Transient => hotsas_core::SimulationType::Transient,
        },
        parameters,
        requested_outputs: profile.probes.iter().map(|p| p.id.clone()).collect(),
    }
}

fn to_engine_choice(engine: &UserCircuitSimulationEngine) -> SimulationEngineChoice {
    match engine {
        UserCircuitSimulationEngine::Mock => SimulationEngineChoice::Mock,
        UserCircuitSimulationEngine::Ngspice => SimulationEngineChoice::Ngspice,
        UserCircuitSimulationEngine::Auto => SimulationEngineChoice::Auto,
    }
}

fn to_user_result(result: &hotsas_core::SimulationResult) -> UserCircuitSimulationResult {
    UserCircuitSimulationResult {
        summary: result
            .measurements
            .iter()
            .map(|(name, value)| SimulationMeasurement {
                name: name.clone(),
                value: value.clone(),
                unit_symbol: value.unit.symbol().to_string(),
            })
            .collect(),
        series: result
            .graph_series
            .iter()
            .map(|gs| SimulationSeries {
                id: gs.name.clone(),
                label: gs.name.clone(),
                x_unit: Some(gs.x_unit),
                y_unit: Some(gs.y_unit),
                points: gs
                    .points
                    .iter()
                    .map(|p| SimulationPoint { x: p.x, y: p.y })
                    .collect(),
            })
            .collect(),
        raw_output_excerpt: None,
        netlist_hash: None,
    }
}

fn format_hz(value: f64) -> String {
    if value.abs() >= 1e4 || value.abs() < 1e-3 {
        format!("{value:.6e}")
    } else {
        let s = format!("{value:.6}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn format_time(value: f64) -> String {
    if value.abs() < 1e-6 {
        format!("{value:.6e}")
    } else {
        let s = format!("{value:.6}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
