use serde::{Deserialize, Serialize};

use crate::ComplexValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SParameterAnalysisSource {
    ImportedTouchstone,
    TwoPortFilterAnalysis,
    ManualDataset,
    SimulatedFoundation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterDataset {
    pub id: String,
    pub name: String,
    pub source: SParameterAnalysisSource,
    pub port_count: usize,
    pub reference_impedance_ohm: f64,
    pub frequency_unit: String,
    pub parameter_format: String,
    pub points: Vec<SParameterDataPoint>,
    pub warnings: Vec<SParameterDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterDataPoint {
    pub frequency_hz: f64,
    pub s11: Option<ComplexValue>,
    pub s21: Option<ComplexValue>,
    pub s12: Option<ComplexValue>,
    pub s22: Option<ComplexValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterCurvePoint {
    pub frequency_hz: f64,
    pub s11_db: Option<f64>,
    pub s21_db: Option<f64>,
    pub s12_db: Option<f64>,
    pub s22_db: Option<f64>,
    pub s11_phase_deg: Option<f64>,
    pub s21_phase_deg: Option<f64>,
    pub s12_phase_deg: Option<f64>,
    pub s22_phase_deg: Option<f64>,
    pub return_loss_s11_db: Option<f64>,
    pub return_loss_s22_db: Option<f64>,
    pub insertion_loss_s21_db: Option<f64>,
    pub vswr_s11: Option<f64>,
    pub vswr_s22: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterMetric {
    pub id: String,
    pub label: String,
    pub value: f64,
    pub unit: String,
    pub frequency_hz: Option<f64>,
    pub confidence: SParameterMetricConfidence,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SParameterMetricConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterDiagnostic {
    pub code: String,
    pub severity: SParameterSeverity,
    pub title: String,
    pub message: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SParameterSeverity {
    Info,
    Warning,
    Error,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterAnalysisResult {
    pub id: String,
    pub dataset: SParameterDataset,
    pub curve_points: Vec<SParameterCurvePoint>,
    pub metrics: Vec<SParameterMetric>,
    pub diagnostics: Vec<SParameterDiagnostic>,
    pub can_plot_s11: bool,
    pub can_plot_s21: bool,
    pub can_plot_s12: bool,
    pub can_plot_s22: bool,
    pub summary: String,
}

pub fn magnitude_db(c: &ComplexValue) -> Option<f64> {
    let mag = (c.re * c.re + c.im * c.im).sqrt();
    if mag <= 0.0 || !mag.is_finite() {
        return None;
    }
    Some(20.0 * mag.log10())
}

pub fn phase_deg(c: &ComplexValue) -> Option<f64> {
    let deg = c.im.atan2(c.re).to_degrees();
    if deg.is_finite() {
        Some(deg)
    } else {
        None
    }
}

pub fn return_loss_db(gamma_mag: f64) -> Option<f64> {
    if gamma_mag > 0.0 && gamma_mag.is_finite() {
        Some(-20.0 * gamma_mag.log10())
    } else {
        None
    }
}

pub fn vswr(gamma_mag: f64) -> Option<f64> {
    if gamma_mag >= 1.0 || gamma_mag < 0.0 || !gamma_mag.is_finite() {
        return None;
    }
    Some((1.0 + gamma_mag) / (1.0 - gamma_mag))
}

pub fn build_s_parameter_csv(result: &SParameterAnalysisResult) -> String {
    let mut lines = Vec::new();
    lines.push(
        "frequency_hz,s11_db,s21_db,s12_db,s22_db,s11_phase_deg,s21_phase_deg,s12_phase_deg,s22_phase_deg,return_loss_s11_db,return_loss_s22_db,insertion_loss_s21_db,vswr_s11,vswr_s22".to_string(),
    );
    for p in &result.curve_points {
        lines.push(format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            p.frequency_hz,
            fmt_opt(p.s11_db),
            fmt_opt(p.s21_db),
            fmt_opt(p.s12_db),
            fmt_opt(p.s22_db),
            fmt_opt(p.s11_phase_deg),
            fmt_opt(p.s21_phase_deg),
            fmt_opt(p.s12_phase_deg),
            fmt_opt(p.s22_phase_deg),
            fmt_opt(p.return_loss_s11_db),
            fmt_opt(p.return_loss_s22_db),
            fmt_opt(p.insertion_loss_s21_db),
            fmt_opt(p.vswr_s11),
            fmt_opt(p.vswr_s22),
        ));
    }
    lines.join("\n")
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(f) => format!("{:.6}", f),
        None => String::new(),
    }
}
