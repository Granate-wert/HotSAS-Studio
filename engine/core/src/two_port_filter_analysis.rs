use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAnalysisScope {
    WholeCircuit,
    SelectedRegion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitAnalysisPort {
    pub label: String,
    pub positive_net_id: String,
    pub negative_net_id: Option<String>,
    pub reference_node_id: Option<String>,
    pub nominal_impedance_ohm: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrequencySweepScale {
    Linear,
    Logarithmic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequencySweepSettings {
    pub start_hz: f64,
    pub stop_hz: f64,
    pub points: usize,
    pub points_per_decade: Option<usize>,
    pub scale: FrequencySweepScale,
}

impl FrequencySweepSettings {
    pub fn validate(&self) -> Vec<FilterAnalysisDiagnostic> {
        let mut diagnostics = Vec::new();
        if self.start_hz <= 0.0 {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_invalid_frequency_sweep".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Invalid frequency sweep".to_string(),
                message: "start_hz must be greater than 0".to_string(),
                suggested_fix: Some("Set start_hz to a positive value".to_string()),
                related_component_id: None,
                related_net_id: None,
                related_model_id: None,
            });
        }
        if self.stop_hz <= self.start_hz {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_invalid_frequency_sweep".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Invalid frequency sweep".to_string(),
                message: "stop_hz must be greater than start_hz".to_string(),
                suggested_fix: Some("Increase stop_hz above start_hz".to_string()),
                related_component_id: None,
                related_net_id: None,
                related_model_id: None,
            });
        }
        if self.points < 2 {
            diagnostics.push(FilterAnalysisDiagnostic {
                code: "filter_invalid_frequency_sweep".to_string(),
                severity: FilterAnalysisSeverity::Blocking,
                title: "Invalid frequency sweep".to_string(),
                message: "points must be at least 2".to_string(),
                suggested_fix: Some("Set points to 2 or more".to_string()),
                related_component_id: None,
                related_net_id: None,
                related_model_id: None,
            });
        }
        if matches!(self.scale, FrequencySweepScale::Logarithmic) {
            if self.points_per_decade.map(|p| p < 1).unwrap_or(true) {
                diagnostics.push(FilterAnalysisDiagnostic {
                    code: "filter_invalid_frequency_sweep".to_string(),
                    severity: FilterAnalysisSeverity::Warning,
                    title: "Logarithmic sweep may be coarse".to_string(),
                    message: "points_per_decade is missing or too low for logarithmic sweep"
                        .to_string(),
                    suggested_fix: Some("Set points_per_decade to at least 10".to_string()),
                    related_component_id: None,
                    related_net_id: None,
                    related_model_id: None,
                });
            }
        }
        diagnostics
    }

    pub fn frequencies(&self) -> Vec<f64> {
        match self.scale {
            FrequencySweepScale::Linear => {
                let step =
                    (self.stop_hz - self.start_hz) / (self.points.saturating_sub(1).max(1) as f64);
                (0..self.points)
                    .map(|i| self.start_hz + step * i as f64)
                    .collect()
            }
            FrequencySweepScale::Logarithmic => {
                let log_start = self.start_hz.log10();
                let log_stop = self.stop_hz.log10();
                let count = self.points.saturating_sub(1).max(1) as f64;
                (0..self.points)
                    .map(|i| {
                        let t = i as f64 / count;
                        let log_f = log_start + t * (log_stop - log_start);
                        10f64.powf(log_f)
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAnalysisMethod {
    Auto,
    TemplateAnalytic,
    Mock,
    Ngspice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectedFilterKind {
    LowPass,
    HighPass,
    BandPass,
    BandStop,
    AllPass,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterNetworkAnalysisRequest {
    pub project_id: String,
    pub scope: FilterAnalysisScope,
    pub selected_component_ids: Vec<String>,
    pub input_port: CircuitAnalysisPort,
    pub output_port: CircuitAnalysisPort,
    pub sweep: FrequencySweepSettings,
    pub method: FilterAnalysisMethod,
    pub source_amplitude_v: Option<f64>,
    pub requested_metrics: Vec<FilterMetricKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterSweepPoint {
    pub frequency_hz: f64,
    pub vin_magnitude: Option<f64>,
    pub vout_magnitude: Option<f64>,
    pub transfer_magnitude: Option<f64>,
    pub gain_db: Option<f64>,
    pub attenuation_db: Option<f64>,
    pub phase_deg: Option<f64>,
    pub zin_magnitude_ohm: Option<f64>,
    pub zin_phase_deg: Option<f64>,
    pub zout_magnitude_ohm: Option<f64>,
    pub zout_phase_deg: Option<f64>,
}

impl FilterSweepPoint {
    pub fn compute_derived(&mut self) {
        if let (Some(vin), Some(vout)) = (self.vin_magnitude, self.vout_magnitude) {
            if vin > 0.0 {
                self.transfer_magnitude = Some(vout / vin);
            }
        }
        if let Some(tm) = self.transfer_magnitude {
            if tm > 0.0 {
                let db = 20.0 * tm.log10();
                self.gain_db = Some(db);
                self.attenuation_db = Some(-db);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterMetricKind {
    CutoffFrequency,
    LowerCutoffFrequency,
    UpperCutoffFrequency,
    Bandwidth,
    PeakGain,
    PassbandRipple,
    StopbandAttenuation,
    AttenuationAtFrequency,
    InputImpedance,
    OutputImpedance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterMetricConfidence {
    Exact,
    Estimated,
    Approximate,
    NotAvailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterMetricValue {
    pub kind: FilterMetricKind,
    pub label: String,
    pub value: Option<f64>,
    pub unit: String,
    pub frequency_hz: Option<f64>,
    pub confidence: FilterMetricConfidence,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAnalysisSeverity {
    Info,
    Warning,
    Error,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterAnalysisDiagnostic {
    pub code: String,
    pub severity: FilterAnalysisSeverity,
    pub title: String,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub related_component_id: Option<String>,
    pub related_net_id: Option<String>,
    pub related_model_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterNetworkAnalysisResult {
    pub analysis_id: String,
    pub project_id: String,
    pub request: FilterNetworkAnalysisRequest,
    pub method_used: FilterAnalysisMethod,
    pub detected_filter_kind: DetectedFilterKind,
    pub can_trust_as_engineering_estimate: bool,
    pub points: Vec<FilterSweepPoint>,
    pub metrics: Vec<FilterMetricValue>,
    pub diagnostics: Vec<FilterAnalysisDiagnostic>,
    pub generated_netlist_preview: Option<String>,
    pub created_at: String,
}

pub fn estimate_cutoff_from_gain_db(
    points: &[FilterSweepPoint],
    filter_kind: DetectedFilterKind,
) -> Vec<FilterMetricValue> {
    let mut metrics = Vec::new();
    match filter_kind {
        DetectedFilterKind::LowPass | DetectedFilterKind::HighPass => {
            let ref_gain = points
                .iter()
                .filter_map(|p| p.gain_db)
                .fold(f64::NEG_INFINITY, f64::max);
            if ref_gain.is_finite() {
                let target = ref_gain - 3.0;
                let mut cutoff: Option<f64> = None;
                for window in points.windows(2) {
                    let a = &window[0];
                    let b = &window[1];
                    if let (Some(ga), Some(gb)) = (a.gain_db, b.gain_db) {
                        if (ga >= target && gb <= target) || (ga <= target && gb >= target) {
                            if (gb - ga).abs() > 1e-12 {
                                let t = (target - ga) / (gb - ga);
                                cutoff =
                                    Some(a.frequency_hz + t * (b.frequency_hz - a.frequency_hz));
                            } else {
                                cutoff = Some(a.frequency_hz);
                            }
                            break;
                        }
                    }
                }
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::CutoffFrequency,
                    label: "Cutoff frequency".to_string(),
                    value: cutoff,
                    unit: "Hz".to_string(),
                    frequency_hz: cutoff,
                    confidence: if cutoff.is_some() {
                        FilterMetricConfidence::Estimated
                    } else {
                        FilterMetricConfidence::NotAvailable
                    },
                    note: if cutoff.is_none() {
                        Some("No -3 dB crossing detected".to_string())
                    } else {
                        None
                    },
                });
                if cutoff.is_none() {
                    metrics.push(FilterMetricValue {
                        kind: FilterMetricKind::Bandwidth,
                        label: "Bandwidth".to_string(),
                        value: None,
                        unit: "Hz".to_string(),
                        frequency_hz: None,
                        confidence: FilterMetricConfidence::NotAvailable,
                        note: Some("Cutoff not detected".to_string()),
                    });
                }
            } else {
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::CutoffFrequency,
                    label: "Cutoff frequency".to_string(),
                    value: None,
                    unit: "Hz".to_string(),
                    frequency_hz: None,
                    confidence: FilterMetricConfidence::NotAvailable,
                    note: Some("No gain data".to_string()),
                });
            }
        }
        DetectedFilterKind::BandPass | DetectedFilterKind::BandStop => {
            let ref_gain = points
                .iter()
                .filter_map(|p| p.gain_db)
                .fold(f64::NEG_INFINITY, f64::max);
            if ref_gain.is_finite() {
                let target_low = ref_gain - 3.0;
                let mut lower: Option<f64> = None;
                let mut upper: Option<f64> = None;
                for window in points.windows(2) {
                    let a = &window[0];
                    let b = &window[1];
                    if let (Some(ga), Some(gb)) = (a.gain_db, b.gain_db) {
                        if lower.is_none()
                            && ((ga >= target_low && gb <= target_low)
                                || (ga <= target_low && gb >= target_low))
                        {
                            if (gb - ga).abs() > 1e-12 {
                                let t = (target_low - ga) / (gb - ga);
                                lower =
                                    Some(a.frequency_hz + t * (b.frequency_hz - a.frequency_hz));
                            } else {
                                lower = Some(a.frequency_hz);
                            }
                        }
                        if upper.is_none()
                            && lower.is_some()
                            && ((ga <= target_low && gb >= target_low)
                                || (ga >= target_low && gb <= target_low))
                        {
                            if (gb - ga).abs() > 1e-12 {
                                let t = (target_low - ga) / (gb - ga);
                                upper =
                                    Some(a.frequency_hz + t * (b.frequency_hz - a.frequency_hz));
                            } else {
                                upper = Some(a.frequency_hz);
                            }
                            break;
                        }
                    }
                }
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::LowerCutoffFrequency,
                    label: "Lower cutoff".to_string(),
                    value: lower,
                    unit: "Hz".to_string(),
                    frequency_hz: lower,
                    confidence: if lower.is_some() {
                        FilterMetricConfidence::Estimated
                    } else {
                        FilterMetricConfidence::NotAvailable
                    },
                    note: if lower.is_none() {
                        Some("No lower -3 dB crossing detected".to_string())
                    } else {
                        None
                    },
                });
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::UpperCutoffFrequency,
                    label: "Upper cutoff".to_string(),
                    value: upper,
                    unit: "Hz".to_string(),
                    frequency_hz: upper,
                    confidence: if upper.is_some() {
                        FilterMetricConfidence::Estimated
                    } else {
                        FilterMetricConfidence::NotAvailable
                    },
                    note: if upper.is_none() {
                        Some("No upper -3 dB crossing detected".to_string())
                    } else {
                        None
                    },
                });
                let bw = match (lower, upper) {
                    (Some(l), Some(u)) => Some(u - l),
                    _ => None,
                };
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::Bandwidth,
                    label: "Bandwidth".to_string(),
                    value: bw,
                    unit: "Hz".to_string(),
                    frequency_hz: None,
                    confidence: if bw.is_some() {
                        FilterMetricConfidence::Estimated
                    } else {
                        FilterMetricConfidence::NotAvailable
                    },
                    note: if bw.is_none() {
                        Some("Both cutoffs required for bandwidth".to_string())
                    } else {
                        None
                    },
                });
            } else {
                metrics.push(FilterMetricValue {
                    kind: FilterMetricKind::Bandwidth,
                    label: "Bandwidth".to_string(),
                    value: None,
                    unit: "Hz".to_string(),
                    frequency_hz: None,
                    confidence: FilterMetricConfidence::NotAvailable,
                    note: Some("No gain data".to_string()),
                });
            }
        }
        _ => {}
    }
    metrics
}

pub fn estimate_peak_gain(points: &[FilterSweepPoint]) -> Option<FilterMetricValue> {
    let max_db = points
        .iter()
        .filter_map(|p| p.gain_db)
        .fold(f64::NEG_INFINITY, f64::max);
    if max_db.is_finite() {
        Some(FilterMetricValue {
            kind: FilterMetricKind::PeakGain,
            label: "Peak gain".to_string(),
            value: Some(max_db),
            unit: "dB".to_string(),
            frequency_hz: None,
            confidence: FilterMetricConfidence::Estimated,
            note: None,
        })
    } else {
        None
    }
}

pub fn detect_filter_kind_from_points(points: &[FilterSweepPoint]) -> DetectedFilterKind {
    if points.len() < 3 {
        return DetectedFilterKind::Unknown;
    }
    let gains: Vec<f64> = points.iter().filter_map(|p| p.gain_db).collect();
    if gains.len() < 3 {
        return DetectedFilterKind::Unknown;
    }
    let first = gains.first().copied().unwrap_or(0.0);
    let last = gains.last().copied().unwrap_or(0.0);
    let max_gain = gains.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_gain = gains.iter().copied().fold(f64::INFINITY, f64::min);
    let mid_idx = gains.len() / 2;
    let mid = gains[mid_idx];

    if max_gain < -40.0 {
        return DetectedFilterKind::Unknown;
    }

    let passband_variation = max_gain - min_gain;
    if passband_variation < 1.0 {
        return DetectedFilterKind::AllPass;
    }

    if first > max_gain - 3.0 && last < max_gain - 10.0 {
        return DetectedFilterKind::LowPass;
    }
    if last > max_gain - 3.0 && first < max_gain - 10.0 {
        return DetectedFilterKind::HighPass;
    }
    if mid > first + 3.0 && mid > last + 3.0 {
        return DetectedFilterKind::BandPass;
    }
    if mid < first - 3.0 && mid < last - 3.0 {
        return DetectedFilterKind::BandStop;
    }
    DetectedFilterKind::Unknown
}

pub fn build_filter_analysis_csv(result: &FilterNetworkAnalysisResult) -> String {
    let mut lines = vec![
        "frequency_hz,vin_magnitude,vout_magnitude,transfer_magnitude,gain_db,attenuation_db,phase_deg,zin_magnitude_ohm,zin_phase_deg,zout_magnitude_ohm,zout_phase_deg".to_string(),
    ];
    for p in &result.points {
        lines.push(format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            p.frequency_hz,
            opt_f64(p.vin_magnitude),
            opt_f64(p.vout_magnitude),
            opt_f64(p.transfer_magnitude),
            opt_f64(p.gain_db),
            opt_f64(p.attenuation_db),
            opt_f64(p.phase_deg),
            opt_f64(p.zin_magnitude_ohm),
            opt_f64(p.zin_phase_deg),
            opt_f64(p.zout_magnitude_ohm),
            opt_f64(p.zout_phase_deg),
        ));
    }
    lines.join("\n")
}

fn opt_f64(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{:.12}", x),
        None => String::new(),
    }
}
