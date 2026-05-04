use crate::{CoreError, EngineeringUnit, ValueWithUnit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DcdcTopology {
    Buck,
    Boost,
    InvertingBuckBoost,
    FourSwitchBuckBoost,
}

impl DcdcTopology {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Buck => "buck",
            Self::Boost => "boost",
            Self::InvertingBuckBoost => "inverting_buck_boost",
            Self::FourSwitchBuckBoost => "four_switch_buck_boost",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Buck => "Buck Converter",
            Self::Boost => "Boost Converter",
            Self::InvertingBuckBoost => "Inverting Buck-Boost",
            Self::FourSwitchBuckBoost => "4-Switch Buck-Boost",
        }
    }
}

impl std::fmt::Display for DcdcTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl std::str::FromStr for DcdcTopology {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "buck" => Ok(Self::Buck),
            "boost" => Ok(Self::Boost),
            "inverting_buck_boost" | "inverting" | "inverting_bb" => Ok(Self::InvertingBuckBoost),
            "four_switch_buck_boost" | "4switch" | "4_switch" => Ok(Self::FourSwitchBuckBoost),
            other => Err(CoreError::InvalidEngineeringValue(format!(
                "unknown DC-DC topology: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DcdcOperatingMode {
    Ccm,
    Dcm,
    Boundary,
    Unknown,
}

impl std::fmt::Display for DcdcOperatingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ccm => write!(f, "CCM"),
            Self::Dcm => write!(f, "DCM"),
            Self::Boundary => write!(f, "Boundary"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcdcInput {
    pub topology: DcdcTopology,
    pub vin: ValueWithUnit,
    pub vout: ValueWithUnit,
    pub iout: ValueWithUnit,
    pub switching_frequency: ValueWithUnit,
    pub inductor: Option<ValueWithUnit>,
    pub output_capacitor: Option<ValueWithUnit>,
    pub target_inductor_ripple_percent: Option<f64>,
    pub estimated_efficiency_percent: Option<f64>,
}

impl DcdcInput {
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.vin.si_value() <= 0.0 {
            return Err(CoreError::ValueOutOfRange(
                "Vin must be positive".to_string(),
            ));
        }
        if self.vout.si_value() == 0.0 {
            return Err(CoreError::ValueOutOfRange(
                "Vout must be non-zero".to_string(),
            ));
        }
        if self.iout.si_value() <= 0.0 {
            return Err(CoreError::ValueOutOfRange(
                "Iout must be positive".to_string(),
            ));
        }
        let fs = self.switching_frequency.si_value();
        if fs <= 0.0 {
            return Err(CoreError::ValueOutOfRange(
                "Switching frequency must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcdcComputedValue {
    pub id: String,
    pub label: String,
    pub value: ValueWithUnit,
    pub formula: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DcdcWarningSeverity {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for DcdcWarningSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcdcWarning {
    pub code: String,
    pub message: String,
    pub severity: DcdcWarningSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcdcSimulationPlan {
    pub id: String,
    pub title: String,
    pub profile_type: String,
    pub recommended_stop_time: ValueWithUnit,
    pub recommended_time_step: Option<ValueWithUnit>,
    pub signals: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcdcTemplateDefinition {
    pub id: String,
    pub title: String,
    pub topology: DcdcTopology,
    pub description: String,
    pub supported_outputs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcdcCalculationResult {
    pub topology: DcdcTopology,
    pub operating_mode: DcdcOperatingMode,
    pub inputs: DcdcInput,
    pub values: Vec<DcdcComputedValue>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub warnings: Vec<DcdcWarning>,
    pub simulation_plan: Option<DcdcSimulationPlan>,
    pub template_id: Option<String>,
}

impl DcdcCalculationResult {
    pub fn find_value(&self, id: &str) -> Option<&DcdcComputedValue> {
        self.values.iter().find(|v| v.id == id)
    }

    pub fn find_value_si(&self, id: &str) -> Option<f64> {
        self.find_value(id).map(|v| v.value.si_value())
    }
}
