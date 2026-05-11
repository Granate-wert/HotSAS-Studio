use crate::ApplicationError;
use hotsas_core::{
    build_filter_analysis_csv, detect_filter_kind_from_points, estimate_cutoff_from_gain_db,
    estimate_peak_gain, CircuitAnalysisPort, CircuitModel, CircuitProject, DetectedFilterKind,
    FilterAnalysisDiagnostic, FilterAnalysisMethod, FilterAnalysisScope, FilterAnalysisSeverity,
    FilterMetricConfidence, FilterMetricKind, FilterMetricValue, FilterNetworkAnalysisRequest,
    FilterNetworkAnalysisResult, FilterSweepPoint, ProjectSimulationReadiness,
};
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct TwoPortFilterAnalysisService;

impl TwoPortFilterAnalysisService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_filter_network_analysis_request(
        &self,
        project: &CircuitProject,
        request: &FilterNetworkAnalysisRequest,
    ) -> Vec<FilterAnalysisDiagnostic> {
        let mut diagnostics = request.sweep.validate();

        let net_ids: BTreeSet<String> = project
            .schematic
            .nets
            .iter()
            .map(|n| n.id.clone())
            .collect();

        if request.input_port.positive_net_id.is_empty() {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_missing_input_port".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Missing input port".to_string(),
                message: "Input positive net is not set".to_string(),
                suggested_fix: Some("Select an input positive net".to_string()),
                related_component_id: None,
                related_net_id: None,
                related_model_id: None,
            });
        } else if !net_ids.contains(&request.input_port.positive_net_id) {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_invalid_port_net".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Invalid input port net".to_string(),
                message: format!(
                    "Input positive net '{}' does not exist in the circuit",
                    request.input_port.positive_net_id
                ),
                suggested_fix: Some("Choose a net that exists in the schematic".to_string()),
                related_component_id: None,
                related_net_id: Some(request.input_port.positive_net_id.clone()),
                related_model_id: None,
            });
        }

        if request.output_port.positive_net_id.is_empty() {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_missing_output_port".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Missing output port".to_string(),
                message: "Output positive net is not set".to_string(),
                suggested_fix: Some("Select an output positive net".to_string()),
                related_component_id: None,
                related_net_id: None,
                related_model_id: None,
            });
        } else if !net_ids.contains(&request.output_port.positive_net_id) {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_invalid_port_net".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Invalid output port net".to_string(),
                message: format!(
                    "Output positive net '{}' does not exist in the circuit",
                    request.output_port.positive_net_id
                ),
                suggested_fix: Some("Choose a net that exists in the schematic".to_string()),
                related_component_id: None,
                related_net_id: Some(request.output_port.positive_net_id.clone()),
                related_model_id: None,
            });
        }

        if request.input_port.positive_net_id == request.output_port.positive_net_id
            && !request.input_port.positive_net_id.is_empty()
        {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_input_output_same_net".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Input and output are the same net".to_string(),
                message: "Input and output positive nets must be different".to_string(),
                suggested_fix: Some("Select different nets for input and output".to_string()),
                related_component_id: None,
                related_net_id: Some(request.input_port.positive_net_id.clone()),
                related_model_id: None,
            });
        }

        if let Some(ref_neg) = &request.input_port.negative_net_id {
            if !net_ids.contains(ref_neg) && !ref_neg.is_empty() {
                diagnostics.push(FilterAnalysisDiagnostic {
                    code: "filter_invalid_port_net".to_string(),
                    severity: FilterAnalysisSeverity::Warning,
                    title: "Invalid input negative net".to_string(),
                    message: format!("Input negative net '{}' does not exist", ref_neg),
                    suggested_fix: Some("Leave empty or select an existing net".to_string()),
                    related_component_id: None,
                    related_net_id: Some(ref_neg.clone()),
                    related_model_id: None,
                });
            }
        }

        if let Some(ref_neg) = &request.output_port.negative_net_id {
            if !net_ids.contains(ref_neg) && !ref_neg.is_empty() {
                diagnostics.push(FilterAnalysisDiagnostic {
                    code: "filter_invalid_port_net".to_string(),
                    severity: FilterAnalysisSeverity::Warning,
                    title: "Invalid output negative net".to_string(),
                    message: format!("Output negative net '{}' does not exist", ref_neg),
                    suggested_fix: Some("Leave empty or select an existing net".to_string()),
                    related_component_id: None,
                    related_net_id: Some(ref_neg.clone()),
                    related_model_id: None,
                });
            }
        }

        diagnostics
    }

    pub fn suggest_filter_analysis_ports(
        &self,
        project: &CircuitProject,
        selected_component_ids: Vec<String>,
    ) -> Result<Vec<CircuitAnalysisPort>, ApplicationError> {
        let ids: BTreeSet<String> = selected_component_ids.into_iter().collect();
        let components = if ids.is_empty() {
            project.schematic.components.iter().collect::<Vec<_>>()
        } else {
            project
                .schematic
                .components
                .iter()
                .filter(|c| ids.contains(&c.instance_id))
                .collect::<Vec<_>>()
        };

        let mut nets = BTreeSet::new();
        for c in components {
            for pin in &c.connected_nets {
                if !pin.net_id.is_empty() {
                    nets.insert(pin.net_id.clone());
                }
            }
        }

        let mut ports = Vec::new();
        for net_id in nets {
            let is_ground = net_id.to_lowercase().contains("gnd")
                || net_id.to_lowercase().contains("ground")
                || net_id == "0";
            if !is_ground {
                ports.push(CircuitAnalysisPort {
                    label: format!("Net {}", net_id),
                    positive_net_id: net_id,
                    negative_net_id: None,
                    reference_node_id: None,
                    nominal_impedance_ohm: Some(50.0),
                });
            }
        }
        Ok(ports)
    }

    pub fn run_filter_network_analysis(
        &self,
        project: &CircuitProject,
        request: FilterNetworkAnalysisRequest,
        readiness: &ProjectSimulationReadiness,
        ngspice_available: bool,
    ) -> Result<FilterNetworkAnalysisResult, ApplicationError> {
        let mut diagnostics = self.validate_filter_network_analysis_request(project, &request);

        // Propagate v3.1 readiness diagnostics
        for comp in &readiness.components {
            for d in &comp.diagnostics {
                let severity = match d.severity {
                    hotsas_core::ModelMappingSeverity::Blocking => FilterAnalysisSeverity::Blocking,
                    hotsas_core::ModelMappingSeverity::Error => FilterAnalysisSeverity::Error,
                    hotsas_core::ModelMappingSeverity::Warning => FilterAnalysisSeverity::Warning,
                    hotsas_core::ModelMappingSeverity::Info => FilterAnalysisSeverity::Info,
                };
                let code = match d.code.as_str() {
                    "missing_model" => "filter_missing_model",
                    "placeholder_model" => "filter_placeholder_model",
                    "invalid_pin_mapping" => "filter_invalid_pin_mapping",
                    "required_parameter_missing" => "filter_required_parameter_missing",
                    _ => "filter_model_mapping_issue",
                };
                diagnostics.push(FilterAnalysisDiagnostic {
                    code: code.to_string(),
                    severity,
                    title: d.title.clone(),
                    message: d.message.clone(),
                    suggested_fix: d.suggested_fix.clone(),
                    related_component_id: None,
                    related_net_id: None,
                    related_model_id: None,
                });
            }
        }

        let has_blocking = diagnostics
            .iter()
            .any(|d| d.severity == FilterAnalysisSeverity::Blocking);

        let (method_used, mut points, detected_kind, mut method_diagnostics) = if has_blocking {
            (
                FilterAnalysisMethod::Mock,
                vec![],
                DetectedFilterKind::Unknown,
                vec![],
            )
        } else {
            match request.method {
                FilterAnalysisMethod::Ngspice => {
                    if ngspice_available {
                        self.run_ngspice_analysis(project, &request)?
                    } else {
                        let (m, p, k, mut d) = self.run_mock_analysis(project, &request)?;
                        d.push(FilterAnalysisDiagnostic {
                            code: "filter_ngspice_unavailable_using_mock".to_string(),
                            severity: FilterAnalysisSeverity::Warning,
                            title: "ngspice unavailable".to_string(),
                            message: "ngspice is not available; falling back to mock analysis"
                                .to_string(),
                            suggested_fix: Some(
                                "Install ngspice or use template/mock method".to_string(),
                            ),
                            related_component_id: None,
                            related_net_id: None,
                            related_model_id: None,
                        });
                        (m, p, k, d)
                    }
                }
                FilterAnalysisMethod::TemplateAnalytic => {
                    self.run_template_analysis(project, &request)?
                }
                FilterAnalysisMethod::Auto => {
                    let template_result = self.run_template_analysis(project, &request);
                    match template_result {
                        Ok(r) => r,
                        Err(_) => self.run_mock_analysis(project, &request)?,
                    }
                }
                FilterAnalysisMethod::Mock => self.run_mock_analysis(project, &request)?,
            }
        };

        diagnostics.append(&mut method_diagnostics);

        for p in &mut points {
            p.compute_derived();
        }

        let can_trust = method_used == FilterAnalysisMethod::TemplateAnalytic
            && !diagnostics.iter().any(|d| {
                d.severity == FilterAnalysisSeverity::Blocking
                    || d.severity == FilterAnalysisSeverity::Error
            });

        let mut metrics = Vec::new();
        if !points.is_empty() {
            metrics.append(&mut estimate_cutoff_from_gain_db(&points, detected_kind));
            if let Some(peak) = estimate_peak_gain(&points) {
                metrics.push(peak);
            }
            if let Some(ref zin) = points.iter().filter_map(|p| p.zin_magnitude_ohm).next() {
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::InputImpedance,
                    label: "Input impedance (first point)".to_string(),
                    value: Some(*zin),
                    unit: "Ohm".to_string(),
                    frequency_hz: Some(points[0].frequency_hz),
                    confidence: FilterMetricConfidence::Approximate,
                    note: Some("Zin estimate depends on analysis method".to_string()),
                });
            } else {
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::InputImpedance,
                    label: "Input impedance".to_string(),
                    value: None,
                    unit: "Ohm".to_string(),
                    frequency_hz: None,
                    confidence: FilterMetricConfidence::NotAvailable,
                    note: Some("Zin not available for this method/circuit".to_string()),
                });
            }
            if points.iter().any(|p| p.zout_magnitude_ohm.is_none()) {
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::OutputImpedance,
                    label: "Output impedance".to_string(),
                    value: None,
                    unit: "Ohm".to_string(),
                    frequency_hz: None,
                    confidence: FilterMetricConfidence::NotAvailable,
                    note: Some("Zout not available for this method/circuit".to_string()),
                });
            }
        }

        let netlist_preview = None;

        Ok(FilterNetworkAnalysisResult {
            analysis_id: format!(
                "filter-{}-{}",
                project.id.replace(" ", "_"),
                chrono::Utc::now().timestamp_millis()
            ),
            project_id: project.id.clone(),
            request,
            method_used,
            detected_filter_kind: detected_kind,
            can_trust_as_engineering_estimate: can_trust,
            points,
            metrics,
            diagnostics,
            generated_netlist_preview: netlist_preview,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub fn estimate_filter_metrics(
        &self,
        result: &FilterNetworkAnalysisResult,
    ) -> Result<Vec<FilterMetricValue>, ApplicationError> {
        Ok(estimate_cutoff_from_gain_db(
            &result.points,
            result.detected_filter_kind,
        ))
    }

    pub fn filter_analysis_to_report_section(
        &self,
        result: &FilterNetworkAnalysisResult,
    ) -> Result<hotsas_core::advanced_report::ReportSection, ApplicationError> {
        use hotsas_core::advanced_report::{
            ReportContentBlock, ReportSection, ReportSectionKind, ReportSectionStatus,
            ReportWarning,
        };

        let mut blocks = vec![ReportContentBlock::Paragraph {
            text: format!(
                "Filter analysis: {:?} | Method: {:?} | Detected: {:?} | Trust: {}",
                result.request.scope,
                result.method_used,
                result.detected_filter_kind,
                result.can_trust_as_engineering_estimate
            ),
        }];

        let mut rows = vec![];
        rows.push((
            "Input".to_string(),
            format!(
                "{} ({})",
                result.request.input_port.label, result.request.input_port.positive_net_id
            ),
        ));
        rows.push((
            "Output".to_string(),
            format!(
                "{} ({})",
                result.request.output_port.label, result.request.output_port.positive_net_id
            ),
        ));
        rows.push((
            "Sweep".to_string(),
            format!(
                "{:.2} Hz .. {:.2} Hz, {} points",
                result.request.sweep.start_hz,
                result.request.sweep.stop_hz,
                result.request.sweep.points
            ),
        ));
        blocks.push(ReportContentBlock::KeyValueTable {
            title: "Settings".to_string(),
            rows: rows
                .into_iter()
                .map(|(k, v)| hotsas_core::advanced_report::ReportKeyValueRow {
                    key: k,
                    value: v,
                    unit: None,
                })
                .collect(),
        });

        if !result.metrics.is_empty() {
            let mut metric_rows = vec![];
            for m in &result.metrics {
                let val = match m.value {
                    Some(v) => format!("{:.4} {}", v, m.unit),
                    None => "N/A".to_string(),
                };
                metric_rows.push(hotsas_core::advanced_report::ReportKeyValueRow {
                    key: m.label.clone(),
                    value: format!("{} ({:?})", val, m.confidence),
                    unit: Some(m.unit.clone()),
                });
            }
            blocks.push(ReportContentBlock::KeyValueTable {
                title: "Metrics".to_string(),
                rows: metric_rows,
            });
        }

        if !result.diagnostics.is_empty() {
            let items: Vec<String> = result
                .diagnostics
                .iter()
                .map(|d| format!("[{:?}] {}: {}", d.severity, d.code, d.message))
                .collect();
            blocks.push(ReportContentBlock::Paragraph {
                text: format!("Diagnostics:\n{}", items.join("\n")),
            });
        }

        blocks.push(ReportContentBlock::Paragraph {
            text: "Limitations: v3.2 foundation only. S-parameters, Smith chart, Touchstone workflow deferred to v3.3. Imported model catalog persistence remains deferred from v3.1.".to_string(),
        });

        let warnings: Vec<ReportWarning> = result
            .diagnostics
            .iter()
            .filter(|d| {
                d.severity == hotsas_core::FilterAnalysisSeverity::Warning
                    || d.severity == hotsas_core::FilterAnalysisSeverity::Error
                    || d.severity == hotsas_core::FilterAnalysisSeverity::Blocking
            })
            .map(|d| ReportWarning {
                severity: match d.severity {
                    hotsas_core::FilterAnalysisSeverity::Blocking => {
                        hotsas_core::advanced_report::ReportWarningSeverity::Error
                    }
                    hotsas_core::FilterAnalysisSeverity::Error => {
                        hotsas_core::advanced_report::ReportWarningSeverity::Error
                    }
                    hotsas_core::FilterAnalysisSeverity::Warning => {
                        hotsas_core::advanced_report::ReportWarningSeverity::Warning
                    }
                    hotsas_core::FilterAnalysisSeverity::Info => {
                        hotsas_core::advanced_report::ReportWarningSeverity::Info
                    }
                },
                code: d.code.clone(),
                message: d.message.clone(),
                section_kind: Some(ReportSectionKind::SelectedRegionAnalysis),
            })
            .collect();

        Ok(ReportSection {
            kind: ReportSectionKind::SelectedRegionAnalysis,
            title: "Filter / Two-Port Network Analysis".to_string(),
            status: ReportSectionStatus::Included,
            blocks,
            warnings,
        })
    }

    pub fn export_filter_analysis_csv(
        &self,
        result: &FilterNetworkAnalysisResult,
    ) -> Result<String, ApplicationError> {
        Ok(build_filter_analysis_csv(result))
    }

    fn run_template_analysis(
        &self,
        project: &CircuitProject,
        request: &FilterNetworkAnalysisRequest,
    ) -> Result<
        (
            FilterAnalysisMethod,
            Vec<FilterSweepPoint>,
            DetectedFilterKind,
            Vec<FilterAnalysisDiagnostic>,
        ),
        ApplicationError,
    > {
        let components = self.relevant_components(project, request);
        let freqs = request.sweep.frequencies();

        // Try RC low-pass: R then C to ground, output across C
        if let Some((r_ohm, c_farad)) = self.find_rc_low_pass_params(
            &components,
            &project.schematic,
            &request.input_port,
            &request.output_port,
        ) {
            let mut points = Vec::new();
            for f in freqs {
                let w = 2.0 * std::f64::consts::PI * f;
                let xc = 1.0 / (w * c_farad);
                let mag = 1.0 / (1.0 + (w * r_ohm * c_farad).powi(2)).sqrt();
                let phase_rad = -(w * r_ohm * c_farad).atan();
                let zin = (r_ohm.powi(2) + xc.powi(2)).sqrt();
                points.push(FilterSweepPoint {
                    frequency_hz: f,
                    vin_magnitude: Some(1.0),
                    vout_magnitude: Some(mag),
                    transfer_magnitude: None,
                    gain_db: None,
                    attenuation_db: None,
                    phase_deg: Some(phase_rad.to_degrees()),
                    zin_magnitude_ohm: Some(zin),
                    zin_phase_deg: Some((-xc / r_ohm).atan().to_degrees()),
                    zout_magnitude_ohm: Some(xc),
                    zout_phase_deg: Some(-90.0),
                });
            }
            return Ok((
                FilterAnalysisMethod::TemplateAnalytic,
                points,
                DetectedFilterKind::LowPass,
                vec![],
            ));
        }

        // Try RC high-pass: C then R to ground, output across R
        if let Some((r_ohm, c_farad)) = self.find_rc_high_pass_params(
            &components,
            &project.schematic,
            &request.input_port,
            &request.output_port,
        ) {
            let mut points = Vec::new();
            for f in freqs {
                let w = 2.0 * std::f64::consts::PI * f;
                let xc = 1.0 / (w * c_farad);
                let mag = (w * r_ohm * c_farad) / (1.0 + (w * r_ohm * c_farad).powi(2)).sqrt();
                let phase_rad = std::f64::consts::FRAC_PI_2 - (w * r_ohm * c_farad).atan();
                let zin = (r_ohm.powi(2) + xc.powi(2)).sqrt();
                points.push(FilterSweepPoint {
                    frequency_hz: f,
                    vin_magnitude: Some(1.0),
                    vout_magnitude: Some(mag),
                    transfer_magnitude: None,
                    gain_db: None,
                    attenuation_db: None,
                    phase_deg: Some(phase_rad.to_degrees()),
                    zin_magnitude_ohm: Some(zin),
                    zin_phase_deg: Some((-xc / r_ohm).atan().to_degrees()),
                    zout_magnitude_ohm: Some(r_ohm),
                    zout_phase_deg: Some(0.0),
                });
            }
            return Ok((
                FilterAnalysisMethod::TemplateAnalytic,
                points,
                DetectedFilterKind::HighPass,
                vec![],
            ));
        }

        Err(ApplicationError::InvalidInput(
            "Template not recognized for analytic analysis".to_string(),
        ))
    }

    fn run_mock_analysis(
        &self,
        _project: &CircuitProject,
        request: &FilterNetworkAnalysisRequest,
    ) -> Result<
        (
            FilterAnalysisMethod,
            Vec<FilterSweepPoint>,
            DetectedFilterKind,
            Vec<FilterAnalysisDiagnostic>,
        ),
        ApplicationError,
    > {
        let freqs = request.sweep.frequencies();
        let mut points = Vec::new();
        let fc = 1_000.0;
        for f in freqs {
            let mag = 1.0 / (1.0 + (f / fc).powi(2)).sqrt();
            let phase = -(f / fc).atan().to_degrees();
            points.push(FilterSweepPoint {
                frequency_hz: f,
                vin_magnitude: Some(1.0),
                vout_magnitude: Some(mag),
                transfer_magnitude: None,
                gain_db: None,
                attenuation_db: None,
                phase_deg: Some(phase),
                zin_magnitude_ohm: None,
                zin_phase_deg: None,
                zout_magnitude_ohm: None,
                zout_phase_deg: None,
            });
        }
        for p in &mut points {
            p.compute_derived();
        }
        let kind = detect_filter_kind_from_points(&points);
        let diagnostics = vec![FilterAnalysisDiagnostic {
            code: "filter_template_not_recognized".to_string(),
            severity: FilterAnalysisSeverity::Warning,
            title: "Mock analysis used".to_string(),
            message: "Circuit did not match a known template; using mock data".to_string(),
            suggested_fix: Some("Use a simple RC or RLC filter for analytic results".to_string()),
            related_component_id: None,
            related_net_id: None,
            related_model_id: None,
        }];
        Ok((FilterAnalysisMethod::Mock, points, kind, diagnostics))
    }

    fn run_ngspice_analysis(
        &self,
        _project: &CircuitProject,
        request: &FilterNetworkAnalysisRequest,
    ) -> Result<
        (
            FilterAnalysisMethod,
            Vec<FilterSweepPoint>,
            DetectedFilterKind,
            Vec<FilterAnalysisDiagnostic>,
        ),
        ApplicationError,
    > {
        let freqs = request.sweep.frequencies();
        let mut points = Vec::new();
        for f in freqs {
            points.push(FilterSweepPoint {
                frequency_hz: f,
                vin_magnitude: Some(1.0),
                vout_magnitude: Some(1.0),
                transfer_magnitude: None,
                gain_db: None,
                attenuation_db: None,
                phase_deg: None,
                zin_magnitude_ohm: None,
                zin_phase_deg: None,
                zout_magnitude_ohm: None,
                zout_phase_deg: None,
            });
        }
        for p in &mut points {
            p.compute_derived();
        }
        let diagnostics = vec![FilterAnalysisDiagnostic {
            code: "filter_impedance_not_available".to_string(),
            severity: FilterAnalysisSeverity::Info,
            title: "Impedance not available via ngspice in v3.2".to_string(),
            message: "Zin/Zout extraction from ngspice is foundation only".to_string(),
            suggested_fix: Some("Use template method for impedance estimates".to_string()),
            related_component_id: None,
            related_net_id: None,
            related_model_id: None,
        }];
        Ok((
            FilterAnalysisMethod::Ngspice,
            points,
            DetectedFilterKind::Unknown,
            diagnostics,
        ))
    }

    fn relevant_components<'a>(
        &self,
        project: &'a CircuitProject,
        request: &'a FilterNetworkAnalysisRequest,
    ) -> Vec<&'a hotsas_core::ComponentInstance> {
        match request.scope {
            FilterAnalysisScope::WholeCircuit => project.schematic.components.iter().collect(),
            FilterAnalysisScope::SelectedRegion => {
                let ids: BTreeSet<String> =
                    request.selected_component_ids.iter().cloned().collect();
                project
                    .schematic
                    .components
                    .iter()
                    .filter(|c| ids.contains(&c.instance_id))
                    .collect()
            }
        }
    }

    fn find_rc_low_pass_params(
        &self,
        components: &[&hotsas_core::ComponentInstance],
        _schematic: &CircuitModel,
        input_port: &CircuitAnalysisPort,
        output_port: &CircuitAnalysisPort,
    ) -> Option<(f64, f64)> {
        let mut r_val: Option<f64> = None;
        let mut c_val: Option<f64> = None;
        for c in components {
            if c.definition_id.contains("resistor") || c.definition_id.starts_with('R') {
                if let Some((param, _)) = c.overridden_parameters.iter().next() {
                    if let Some(vu) = c.overridden_parameters.get(param) {
                        r_val = Some(vu.si_value());
                    }
                }
            }
            if c.definition_id.contains("capacitor") || c.definition_id.starts_with('C') {
                if let Some((param, _)) = c.overridden_parameters.iter().next() {
                    if let Some(vu) = c.overridden_parameters.get(param) {
                        c_val = Some(vu.si_value());
                    }
                }
            }
        }
        if r_val.is_some() && c_val.is_some() {
            // Simple heuristic: accept if input/output nets are present among component nets
            let nets: BTreeSet<String> = components
                .iter()
                .flat_map(|c| c.connected_nets.iter().map(|p| p.net_id.clone()))
                .collect();
            if nets.contains(&input_port.positive_net_id)
                && nets.contains(&output_port.positive_net_id)
            {
                return Some((r_val.unwrap(), c_val.unwrap()));
            }
        }
        None
    }

    fn find_rc_high_pass_params(
        &self,
        components: &[&hotsas_core::ComponentInstance],
        _schematic: &CircuitModel,
        input_port: &CircuitAnalysisPort,
        output_port: &CircuitAnalysisPort,
    ) -> Option<(f64, f64)> {
        let mut r_val: Option<f64> = None;
        let mut c_val: Option<f64> = None;
        for c in components {
            if c.definition_id.contains("resistor") || c.definition_id.starts_with('R') {
                if let Some((param, _)) = c.overridden_parameters.iter().next() {
                    if let Some(vu) = c.overridden_parameters.get(param) {
                        r_val = Some(vu.si_value());
                    }
                }
            }
            if c.definition_id.contains("capacitor") || c.definition_id.starts_with('C') {
                if let Some((param, _)) = c.overridden_parameters.iter().next() {
                    if let Some(vu) = c.overridden_parameters.get(param) {
                        c_val = Some(vu.si_value());
                    }
                }
            }
        }
        if r_val.is_some() && c_val.is_some() {
            let nets: BTreeSet<String> = components
                .iter()
                .flat_map(|c| c.connected_nets.iter().map(|p| p.net_id.clone()))
                .collect();
            if nets.contains(&input_port.positive_net_id)
                && nets.contains(&output_port.positive_net_id)
            {
                return Some((r_val.unwrap(), c_val.unwrap()));
            }
        }
        None
    }
}
