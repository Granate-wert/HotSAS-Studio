use super::{
    NgspiceBinaryResolver, NgspiceControlBlockBuilder, NgspiceOutputParser, NgspiceProcessRunner,
};
use hotsas_core::{
    GraphSeries, NgspiceAvailability, NgspiceRunMetadata, SimulationAnalysisKind,
    SimulationProfile, SimulationResult, SimulationStatus, ValueWithUnit,
};
use hotsas_ports::{PortError, SimulationEnginePort};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct NgspiceSimulationAdapter {
    resolver: NgspiceBinaryResolver,
    runner: NgspiceProcessRunner,
    parser: NgspiceOutputParser,
    control_builder: NgspiceControlBlockBuilder,
    last_availability: Mutex<Option<NgspiceAvailability>>,
    temp_dir: PathBuf,
}

impl NgspiceSimulationAdapter {
    pub fn new() -> Self {
        Self {
            resolver: NgspiceBinaryResolver::new(),
            runner: NgspiceProcessRunner::new(),
            parser: NgspiceOutputParser::new(),
            control_builder: NgspiceControlBlockBuilder::new(),
            last_availability: Mutex::new(None),
            temp_dir: std::env::temp_dir().join("hotsas_ngspice"),
        }
    }

    fn ensure_temp_dir(&self) -> Result<(), PortError> {
        std::fs::create_dir_all(&self.temp_dir)
            .map_err(|e| PortError::Simulation(format!("cannot create temp dir: {e}")))
    }

    fn write_netlist(&self, run_id: &str, content: &str) -> Result<PathBuf, PortError> {
        let path = self.temp_dir.join(format!("{run_id}.cir"));
        std::fs::write(&path, content)
            .map_err(|e| PortError::Simulation(format!("cannot write netlist: {e}")))?;
        Ok(path)
    }

    fn write_log_file(
        &self,
        run_id: &str,
        suffix: &str,
        content: &str,
    ) -> Result<PathBuf, PortError> {
        let path = self.temp_dir.join(format!("{run_id}_{suffix}.txt"));
        std::fs::write(&path, content)
            .map_err(|e| PortError::Simulation(format!("cannot write log file: {e}")))?;
        Ok(path)
    }

    fn executable_path(&self) -> Result<String, PortError> {
        let availability = self.check_availability()?;
        if !availability.available {
            return Err(PortError::Simulation(
                availability
                    .message
                    .unwrap_or_else(|| "ngspice is not available".to_string()),
            ));
        }
        availability
            .executable_path
            .ok_or_else(|| PortError::Simulation("ngspice executable path not found".to_string()))
    }

    fn run_ngspice(
        &self,
        run_id: &str,
        netlist_with_control: &str,
        analysis_kind: &SimulationAnalysisKind,
        timeout_ms: u64,
    ) -> Result<(SimulationResult, NgspiceRunMetadata), PortError> {
        self.ensure_temp_dir()?;
        let executable = self.executable_path()?;
        let netlist_path = self.write_netlist(run_id, netlist_with_control)?;

        let args = vec![
            "-b".to_string(),
            "-o".to_string(),
            format!("{run_id}_stdout.log"),
            netlist_path.to_string_lossy().to_string(),
        ];

        let result = self
            .runner
            .run(Path::new(&executable), &args, &self.temp_dir, timeout_ms)?;

        let stdout_path = self
            .write_log_file(run_id, "stdout", &result.stdout)
            .ok()
            .map(|p| p.to_string_lossy().to_string());
        let stderr_path = self
            .write_log_file(run_id, "stderr", &result.stderr)
            .ok()
            .map(|p| p.to_string_lossy().to_string());

        let status = if result.timed_out {
            SimulationStatus::Failed
        } else if result.exit_code.unwrap_or(1) != 0 {
            SimulationStatus::Failed
        } else {
            SimulationStatus::Completed
        };

        let mut errors = vec![];
        let mut warnings = vec![];
        let mut graph_series = vec![];
        let mut measurements = BTreeMap::new();

        if result.timed_out {
            errors.push(format!("ngspice timed out after {timeout_ms} ms"));
        } else if result.exit_code.unwrap_or(1) != 0 {
            let stderr_summary = if result.stderr.len() > 500 {
                format!("{}...", &result.stderr[..500])
            } else {
                result.stderr.clone()
            };
            errors.push(format!(
                "ngspice exited with code {:?}. stderr: {stderr_summary}",
                result.exit_code
            ));
        }

        // Try to parse output files
        if status == SimulationStatus::Completed {
            match analysis_kind {
                SimulationAnalysisKind::AcSweep => {
                    let csv_path = self.temp_dir.join("ac_output.csv");
                    if csv_path.exists() {
                        match std::fs::read_to_string(&csv_path) {
                            Ok(content) => match self.parser.parse_wrdata_file(&content) {
                                Ok(parsed) => {
                                    for (idx, s) in parsed.series.into_iter().enumerate() {
                                        graph_series.push(GraphSeries {
                                            name: if idx == 0 {
                                                "V(out)".to_string()
                                            } else {
                                                format!("V(out{idx})")
                                            },
                                            x_unit: hotsas_core::EngineeringUnit::Hertz,
                                            y_unit: hotsas_core::EngineeringUnit::Unitless,
                                            points: s.points,
                                            metadata: BTreeMap::from([(
                                                "quantity".to_string(),
                                                if idx == 0 {
                                                    "dB".to_string()
                                                } else {
                                                    "V".to_string()
                                                },
                                            )]),
                                        });
                                    }
                                    warnings.extend(parsed.warnings);
                                }
                                Err(e) => {
                                    warnings.push(format!("parser error: {e}"));
                                }
                            },
                            Err(e) => warnings.push(format!("cannot read ac_output.csv: {e}")),
                        }
                    } else {
                        warnings.push("ac_output.csv not found after AC sweep".to_string());
                    }
                }
                SimulationAnalysisKind::Transient => {
                    let csv_path = self.temp_dir.join("tran_output.csv");
                    if csv_path.exists() {
                        match std::fs::read_to_string(&csv_path) {
                            Ok(content) => match self.parser.parse_wrdata_file(&content) {
                                Ok(parsed) => {
                                    for (idx, s) in parsed.series.into_iter().enumerate() {
                                        graph_series.push(GraphSeries {
                                            name: if idx == 0 {
                                                "V(in)".to_string()
                                            } else {
                                                "V(out)".to_string()
                                            },
                                            x_unit: hotsas_core::EngineeringUnit::Second,
                                            y_unit: hotsas_core::EngineeringUnit::Volt,
                                            points: s.points,
                                            metadata: BTreeMap::new(),
                                        });
                                    }
                                    warnings.extend(parsed.warnings);
                                }
                                Err(e) => {
                                    warnings.push(format!("parser error: {e}"));
                                }
                            },
                            Err(e) => warnings.push(format!("cannot read tran_output.csv: {e}")),
                        }
                    } else {
                        warnings.push("tran_output.csv not found after transient".to_string());
                    }
                }
                SimulationAnalysisKind::OperatingPoint => {
                    match self.parser.parse_operating_point_stdout(&result.stdout) {
                        Ok(vals) => {
                            for (name, val) in vals {
                                measurements.insert(
                                    name,
                                    ValueWithUnit::new_si(val, hotsas_core::EngineeringUnit::Volt),
                                );
                            }
                        }
                        Err(e) => warnings.push(format!("OP parser error: {e}")),
                    }
                }
                SimulationAnalysisKind::DcSweep => {
                    errors.push("DC sweep not supported in v1.8".to_string());
                }
            }
        }

        if graph_series.is_empty()
            && measurements.is_empty()
            && status == SimulationStatus::Completed
        {
            warnings.push(
                "ngspice completed but no graph series or measurements were parsed".to_string(),
            );
        }

        let metadata = NgspiceRunMetadata {
            run_id: run_id.to_string(),
            engine: "ngspice".to_string(),
            command: args,
            working_directory: self.temp_dir.to_string_lossy().to_string(),
            netlist_path: netlist_path.to_string_lossy().to_string(),
            stdout_path,
            stderr_path,
            raw_output_path: None,
            parsed_output_path: None,
            exit_code: result.exit_code,
            elapsed_ms: Some(result.elapsed_ms),
        };

        let mut metadata_map = BTreeMap::new();
        metadata_map.insert("run_id".to_string(), metadata.run_id.clone());
        metadata_map.insert("engine".to_string(), metadata.engine.clone());
        metadata_map.insert("elapsed_ms".to_string(), result.elapsed_ms.to_string());
        if let Some(code) = result.exit_code {
            metadata_map.insert("exit_code".to_string(), code.to_string());
        }

        let simulation_result = SimulationResult {
            id: run_id.to_string(),
            profile_id: run_id.to_string(),
            status,
            engine: "ngspice".to_string(),
            graph_series,
            measurements,
            warnings,
            errors,
            raw_data_path: Some(netlist_path.to_string_lossy().to_string()),
            metadata: metadata_map,
        };

        Ok((simulation_result, metadata))
    }
}

impl Default for NgspiceSimulationAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationEnginePort for NgspiceSimulationAdapter {
    fn engine_name(&self) -> &str {
        "ngspice"
    }

    fn check_availability(&self) -> Result<NgspiceAvailability, PortError> {
        let availability = self.resolver.resolve()?;
        if let Ok(mut guard) = self.last_availability.lock() {
            *guard = Some(availability.clone());
        }
        Ok(availability)
    }

    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        let run_id = format!(
            "ngspice-ac-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        let netlist = hotsas_ports::NetlistExporterPort::export_spice_netlist(
            &SpiceNetlistExporterForNgspice,
            _project,
        )?;
        let netlist_with_control = self.control_builder.build_control_block(
            &SimulationAnalysisKind::AcSweep,
            &netlist,
            &profile.requested_outputs,
        )?;
        let timeout_ms = 30_000;
        let (result, _metadata) = self.run_ngspice(
            &run_id,
            &netlist_with_control,
            &SimulationAnalysisKind::AcSweep,
            timeout_ms,
        )?;
        Ok(result)
    }

    fn run_operating_point(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        let run_id = format!(
            "ngspice-op-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        let netlist = hotsas_ports::NetlistExporterPort::export_spice_netlist(
            &SpiceNetlistExporterForNgspice,
            _project,
        )?;
        let netlist_with_control = self.control_builder.build_control_block(
            &SimulationAnalysisKind::OperatingPoint,
            &netlist,
            &[],
        )?;
        let timeout_ms = 10_000;
        let (result, _metadata) = self.run_ngspice(
            &run_id,
            &netlist_with_control,
            &SimulationAnalysisKind::OperatingPoint,
            timeout_ms,
        )?;
        Ok(result)
    }

    fn run_transient(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        let run_id = format!(
            "ngspice-tran-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        let netlist = hotsas_ports::NetlistExporterPort::export_spice_netlist(
            &SpiceNetlistExporterForNgspice,
            _project,
        )?;
        let netlist_with_control = self.control_builder.build_control_block(
            &SimulationAnalysisKind::Transient,
            &netlist,
            &[],
        )?;
        let timeout_ms = 30_000;
        let (result, _metadata) = self.run_ngspice(
            &run_id,
            &netlist_with_control,
            &SimulationAnalysisKind::Transient,
            timeout_ms,
        )?;
        Ok(result)
    }
}

struct SpiceNetlistExporterForNgspice;

impl hotsas_ports::NetlistExporterPort for SpiceNetlistExporterForNgspice {
    fn export_spice_netlist(
        &self,
        project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        use hotsas_core::CircuitQueryService;
        let resistance =
            CircuitQueryService::require_component_parameter(project, "R1", "resistance")
                .map_err(|e| PortError::Export(e.to_string()))?;
        let capacitance =
            CircuitQueryService::require_component_parameter(project, "C1", "capacitance")
                .map_err(|e| PortError::Export(e.to_string()))?;

        fn format_si(value: f64) -> String {
            if value.abs() >= 1e4 || value.abs() < 1e-3 {
                format!("{value:.9e}")
            } else {
                format!("{value:.9}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        }

        Ok(format!(
            "* HotSAS Studio - RC Low-Pass Demo\n* Source of truth: CircuitModel\nV1 net_in 0 AC 1\nR1 net_in net_out {}\nC1 net_out 0 {}\n.ac dec 100 10 1e6\n.end",
            format_si(resistance.si_value()),
            format_si(capacitance.si_value())
        ))
    }
}
