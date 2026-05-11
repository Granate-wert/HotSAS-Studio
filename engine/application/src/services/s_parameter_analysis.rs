use crate::ApplicationError;
use hotsas_core::{
    magnitude_db, phase_deg, return_loss_db, vswr, SParameterAnalysisResult,
    SParameterAnalysisSource, SParameterCurvePoint, SParameterDataPoint, SParameterDataset,
    SParameterDiagnostic, SParameterMetric, SParameterMetricConfidence, SParameterSeverity,
    TouchstoneImportReport, TouchstoneNetworkData,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SParameterAnalysisService {
    last_result: Arc<Mutex<Option<SParameterAnalysisResult>>>,
}

impl SParameterAnalysisService {
    pub fn new() -> Self {
        Self {
            last_result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn analyze_touchstone_report(
        &self,
        report: TouchstoneImportReport,
        source_name: Option<String>,
    ) -> Result<SParameterAnalysisResult, ApplicationError> {
        let network = report
            .network
            .ok_or_else(|| ApplicationError::InvalidInput("No network data in Touchstone report".to_string()))?;

        let dataset = Self::network_to_dataset(&network, source_name)?;
        let mut result = Self::derive_result(dataset)?;

        for w in report.warnings {
            result.diagnostics.push(SParameterDiagnostic {
                code: "touchstone_parser_warning".to_string(),
                severity: SParameterSeverity::Warning,
                title: "Touchstone parse warning".to_string(),
                message: w,
                suggested_fix: None,
            });
        }
        for e in report.errors {
            result.diagnostics.push(SParameterDiagnostic {
                code: "touchstone_parser_error".to_string(),
                severity: SParameterSeverity::Error,
                title: "Touchstone parse error".to_string(),
                message: e,
                suggested_fix: None,
            });
        }

        let mut guard = self
            .last_result
            .lock()
            .map_err(|_| ApplicationError::State("s-parameter result lock poisoned".to_string()))?;
        *guard = Some(result.clone());
        Ok(result)
    }

    pub fn analyze_imported_touchstone_dataset(
        &self,
        network: &TouchstoneNetworkData,
    ) -> Result<SParameterAnalysisResult, ApplicationError> {
        let dataset = Self::network_to_dataset(network, Some(network.name.clone()))?;
        let result = Self::derive_result(dataset)?;
        let mut guard = self
            .last_result
            .lock()
            .map_err(|_| ApplicationError::State("s-parameter result lock poisoned".to_string()))?;
        *guard = Some(result.clone());
        Ok(result)
    }

    fn network_to_dataset(
        network: &TouchstoneNetworkData,
        source_name: Option<String>,
    ) -> Result<SParameterDataset, ApplicationError> {
        let port_count = network.port_count;
        if port_count > 2 {
            return Err(ApplicationError::InvalidInput(format!(
                "Unsupported port count {port_count}; v3.3 supports 1- and 2-port datasets only"
            )));
        }

        let mut points: Vec<SParameterDataPoint> = Vec::with_capacity(network.points.len());
        for p in &network.points {
            let s11 = p.values.get(0).cloned();
            let s21 = if port_count >= 2 {
                p.values.get(1).cloned()
            } else {
                None
            };
            let s12 = if port_count >= 2 {
                p.values.get(2).cloned()
            } else {
                None
            };
            let s22 = if port_count >= 2 {
                p.values.get(3).cloned()
            } else {
                None
            };
            points.push(SParameterDataPoint {
                frequency_hz: p.frequency_hz,
                s11,
                s21,
                s12,
                s22,
            });
        }

        Ok(SParameterDataset {
            id: network.id.clone(),
            name: source_name.unwrap_or_else(|| network.name.clone()),
            source: SParameterAnalysisSource::ImportedTouchstone,
            port_count,
            reference_impedance_ohm: network.reference_impedance_ohm,
            frequency_unit: format!("{:?}", network.frequency_unit),
            parameter_format: format!("{:?}", network.parameter_format),
            points,
            warnings: Vec::new(),
        })
    }

    fn derive_result(dataset: SParameterDataset) -> Result<SParameterAnalysisResult, ApplicationError> {
        let mut curve_points = Vec::with_capacity(dataset.points.len());
        let mut can_plot_s11 = false;
        let mut can_plot_s21 = false;
        let mut can_plot_s12 = false;
        let mut can_plot_s22 = false;

        for p in &dataset.points {
            let s11_db = p.s11.as_ref().and_then(magnitude_db);
            let s21_db = p.s21.as_ref().and_then(magnitude_db);
            let s12_db = p.s12.as_ref().and_then(magnitude_db);
            let s22_db = p.s22.as_ref().and_then(magnitude_db);

            let s11_phase = p.s11.as_ref().and_then(phase_deg);
            let s21_phase = p.s21.as_ref().and_then(phase_deg);
            let s12_phase = p.s12.as_ref().and_then(phase_deg);
            let s22_phase = p.s22.as_ref().and_then(phase_deg);

            let s11_mag = p.s11.as_ref().map(|c| (c.re * c.re + c.im * c.im).sqrt());
            let s22_mag = p.s22.as_ref().map(|c| (c.re * c.re + c.im * c.im).sqrt());

            let rl_s11 = s11_mag.and_then(return_loss_db);
            let rl_s22 = s22_mag.and_then(return_loss_db);
            let il_s21 = s21_db.map(|db| -db);
            let vswr_s11 = s11_mag.and_then(vswr);
            let vswr_s22 = s22_mag.and_then(vswr);

            if s11_db.is_some() { can_plot_s11 = true; }
            if s21_db.is_some() { can_plot_s21 = true; }
            if s12_db.is_some() { can_plot_s12 = true; }
            if s22_db.is_some() { can_plot_s22 = true; }

            curve_points.push(SParameterCurvePoint {
                frequency_hz: p.frequency_hz,
                s11_db,
                s21_db,
                s12_db,
                s22_db,
                s11_phase_deg: s11_phase,
                s21_phase_deg: s21_phase,
                s12_phase_deg: s12_phase,
                s22_phase_deg: s22_phase,
                return_loss_s11_db: rl_s11,
                return_loss_s22_db: rl_s22,
                insertion_loss_s21_db: il_s21,
                vswr_s11,
                vswr_s22,
            });
        }

        let mut diagnostics = Vec::new();
        if dataset.reference_impedance_ohm != 50.0 {
            diagnostics.push(SParameterDiagnostic {
                code: "sparam_non_50_ohm_ref".to_string(),
                severity: SParameterSeverity::Warning,
                title: "Non-50 ohm reference impedance".to_string(),
                message: format!(
                    "Reference impedance is {} Ω; calculations assume 50 Ω unless otherwise noted",
                    dataset.reference_impedance_ohm
                ),
                suggested_fix: Some("Verify that your measurement setup uses the expected reference impedance".to_string()),
            });
        }
        if dataset.points.len() < 2 {
            diagnostics.push(SParameterDiagnostic {
                code: "sparam_sparse_data".to_string(),
                severity: SParameterSeverity::Warning,
                title: "Sparse dataset".to_string(),
                message: "Dataset has fewer than 2 points; metrics and bandwidth estimates may be unreliable".to_string(),
                suggested_fix: Some("Use a dataset with more frequency points".to_string()),
            });
        }
        if dataset.port_count > 2 {
            diagnostics.push(SParameterDiagnostic {
                code: "sparam_unsupported_ports".to_string(),
                severity: SParameterSeverity::Blocking,
                title: "Unsupported port count".to_string(),
                message: format!("Port count {} is not supported in v3.3", dataset.port_count),
                suggested_fix: Some("Use a 1-port or 2-port Touchstone file".to_string()),
            });
        }

        let metrics = Self::compute_metrics(&curve_points, &dataset);

        let summary = format!(
            "{}-port S-parameter dataset with {} points, reference {} Ω",
            dataset.port_count,
            dataset.points.len(),
            dataset.reference_impedance_ohm
        );

        let id = format!("spa-{}", chrono::Utc::now().timestamp_millis());

        Ok(SParameterAnalysisResult {
            id,
            dataset,
            curve_points,
            metrics,
            diagnostics,
            can_plot_s11,
            can_plot_s21,
            can_plot_s12,
            can_plot_s22,
            summary,
        })
    }

    fn compute_metrics(
        curve_points: &[SParameterCurvePoint],
        dataset: &SParameterDataset,
    ) -> Vec<SParameterMetric> {
        let mut metrics = Vec::new();

        if let Some(min_s21) = curve_points.iter().filter_map(|p| p.s21_db).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) {
            metrics.push(SParameterMetric {
                id: "insertion_loss_max".to_string(),
                label: "Max Insertion Loss".to_string(),
                value: -min_s21,
                unit: "dB".to_string(),
                frequency_hz: curve_points.iter().find(|p| p.s21_db == Some(min_s21)).map(|p| p.frequency_hz),
                confidence: SParameterMetricConfidence::Medium,
                notes: vec!["From S21 magnitude".to_string()],
            });
        }

        if let Some(max_s21) = curve_points.iter().filter_map(|p| p.s21_db).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) {
            metrics.push(SParameterMetric {
                id: "s21_peak".to_string(),
                label: "S21 Peak".to_string(),
                value: max_s21,
                unit: "dB".to_string(),
                frequency_hz: curve_points.iter().find(|p| p.s21_db == Some(max_s21)).map(|p| p.frequency_hz),
                confidence: SParameterMetricConfidence::High,
                notes: vec!["Maximum S21 magnitude".to_string()],
            });
        }

        if let Some(max_rl) = curve_points.iter().filter_map(|p| p.return_loss_s11_db).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) {
            metrics.push(SParameterMetric {
                id: "return_loss_s11_max".to_string(),
                label: "Input Return Loss (max)".to_string(),
                value: max_rl,
                unit: "dB".to_string(),
                frequency_hz: curve_points.iter().find(|p| p.return_loss_s11_db == Some(max_rl)).map(|p| p.frequency_hz),
                confidence: SParameterMetricConfidence::Medium,
                notes: vec!["From S11 magnitude".to_string()],
            });
        }

        let min_vswr = curve_points.iter().filter_map(|p| p.vswr_s11).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        if let Some(vswr) = min_vswr {
            metrics.push(SParameterMetric {
                id: "vswr_s11_min".to_string(),
                label: "Minimum VSWR (input)".to_string(),
                value: vswr,
                unit: "ratio".to_string(),
                frequency_hz: curve_points.iter().find(|p| p.vswr_s11 == Some(vswr)).map(|p| p.frequency_hz),
                confidence: SParameterMetricConfidence::Medium,
                notes: vec!["From S11 magnitude".to_string()],
            });
        }

        let freq_min = curve_points.first().map(|p| p.frequency_hz);
        let freq_max = curve_points.last().map(|p| p.frequency_hz);
        if let (Some(fmin), Some(fmax)) = (freq_min, freq_max) {
            metrics.push(SParameterMetric {
                id: "frequency_range".to_string(),
                label: "Frequency Range".to_string(),
                value: fmax,
                unit: "Hz".to_string(),
                frequency_hz: None,
                confidence: SParameterMetricConfidence::High,
                notes: vec![format!("Start: {} Hz", fmin)],
            });
        }

        metrics.push(SParameterMetric {
            id: "reference_impedance".to_string(),
            label: "Reference Impedance".to_string(),
            value: dataset.reference_impedance_ohm,
            unit: "Ω".to_string(),
            frequency_hz: None,
            confidence: SParameterMetricConfidence::High,
            notes: vec!["From Touchstone option line".to_string()],
        });

        metrics
    }

    pub fn export_s_parameter_csv(
        &self,
        result: &SParameterAnalysisResult,
    ) -> Result<String, ApplicationError> {
        Ok(hotsas_core::build_s_parameter_csv(result))
    }

    pub fn get_last_result(&self) -> Option<SParameterAnalysisResult> {
        self.last_result.lock().ok()?.clone()
    }

    pub fn clear_last_result(&self) {
        if let Ok(mut guard) = self.last_result.lock() {
            *guard = None;
        }
    }
}
