//! Typed component parameter models for v2.4
//!
//! This module introduces schema-aware parameter definitions that complement
//! the existing flat `BTreeMap<String, ValueWithUnit>` storage in
//! `ComponentDefinition` and `ComponentInstance`. It provides validation,
//! grouping, tolerance, and metadata without breaking backward compatibility.

use crate::value::{EngineeringUnit, ValueWithUnit};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The origin of a parameter value in a component instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentParameterSource {
    /// Value comes from the library definition (default).
    Default,
    /// Value was overridden by the user for this instance.
    Override,
    /// Value was calculated or derived (e.g. ESR from frequency).
    Calculated,
    /// Value comes from a simulation model.
    SimulationModel,
}

impl std::fmt::Display for ComponentParameterSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentParameterSource::Default => write!(f, "default"),
            ComponentParameterSource::Override => write!(f, "override"),
            ComponentParameterSource::Calculated => write!(f, "calculated"),
            ComponentParameterSource::SimulationModel => write!(f, "simulation_model"),
        }
    }
}

/// Tolerance specification for a parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ComponentTolerance {
    /// Symmetric percentage tolerance, e.g. ±5%.
    SymmetricPercent { value: f64 },
    /// Asymmetric tolerance expressed as min/max ratio or absolute deviation.
    Asymmetric { minus: f64, plus: f64 },
    /// Absolute min/max bounds.
    MinMax { min: f64, max: f64 },
}

impl ComponentTolerance {
    /// Check if a raw value falls within the tolerance bounds.
    pub fn is_within(&self, nominal: f64, actual: f64) -> bool {
        match self {
            ComponentTolerance::SymmetricPercent { value } => {
                let delta = (actual - nominal).abs();
                let allowed = nominal.abs() * (value / 100.0);
                delta <= allowed
            }
            ComponentTolerance::Asymmetric { minus, plus } => {
                let low = nominal - minus;
                let high = nominal + plus;
                actual >= low && actual <= high
            }
            ComponentTolerance::MinMax { min, max } => actual >= *min && actual <= *max,
        }
    }

    /// Return the lower and upper bounds for a given nominal value.
    pub fn bounds(&self, nominal: f64) -> (f64, f64) {
        match self {
            ComponentTolerance::SymmetricPercent { value } => {
                let delta = nominal.abs() * (value / 100.0);
                (nominal - delta, nominal + delta)
            }
            ComponentTolerance::Asymmetric { minus, plus } => (nominal - minus, nominal + plus),
            ComponentTolerance::MinMax { min, max } => (*min, *max),
        }
    }
}

/// The kind of a component parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentParameterKind {
    /// Primary electrical parameter (e.g. resistance, capacitance).
    Primary,
    /// Secondary electrical parameter (e.g. ESR, leakage).
    Secondary,
    /// Thermal parameter (e.g. junction temperature, thermal resistance).
    Thermal,
    /// Mechanical / physical parameter (e.g. package, pin pitch).
    Mechanical,
    /// Simulation-related parameter.
    Simulation,
    /// Metadata / informational field.
    Metadata,
}

/// Definition of a single parameter in a component schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentParameterDefinition {
    /// Human-readable name (e.g. "Resistance").
    pub name: String,
    /// Machine key (e.g. "resistance").
    pub key: String,
    /// Description of what this parameter means.
    pub description: Option<String>,
    /// Expected unit.
    pub unit: EngineeringUnit,
    /// Parameter kind / grouping hint.
    pub kind: ComponentParameterKind,
    /// Whether this parameter is required.
    pub required: bool,
    /// Optional tolerance specification.
    pub tolerance: Option<ComponentTolerance>,
    /// Optional min/max bounds for the value itself (not tolerance).
    pub value_range: Option<(f64, f64)>,
    /// Default value if not specified.
    pub default_value: Option<ValueWithUnit>,
    /// Whether this parameter is editable per instance.
    pub editable: bool,
}

impl ComponentParameterDefinition {
    /// Validate a raw value against this parameter's constraints.
    pub fn validate(&self, value: &ValueWithUnit) -> Result<(), ParameterValidationError> {
        // Check unit compatibility.
        if value.unit != self.unit {
            return Err(ParameterValidationError::UnitMismatch {
                expected: self.unit.clone(),
                actual: value.unit.clone(),
            });
        }

        // Check value range.
        if let Some((min, max)) = self.value_range {
            let v = value.si_value();
            if v < min || v > max {
                return Err(ParameterValidationError::OutOfRange {
                    min,
                    max,
                    actual: v,
                });
            }
        }

        // Check tolerance if both nominal and tolerance exist.
        if let (Some(ref default), Some(ref tol)) = (self.default_value.as_ref(), &self.tolerance) {
            let nominal = default.si_value();
            let actual = value.si_value();
            if !tol.is_within(nominal, actual) {
                return Err(ParameterValidationError::ToleranceExceeded {
                    nominal,
                    actual,
                    tolerance: tol.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Errors that can occur during parameter validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ParameterValidationError {
    UnitMismatch {
        expected: EngineeringUnit,
        actual: EngineeringUnit,
    },
    OutOfRange {
        min: f64,
        max: f64,
        actual: f64,
    },
    ToleranceExceeded {
        nominal: f64,
        actual: f64,
        tolerance: ComponentTolerance,
    },
    MissingRequired {
        key: String,
    },
}

impl std::fmt::Display for ParameterValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterValidationError::UnitMismatch { expected, actual } => {
                write!(f, "Unit mismatch: expected {:?}, got {:?}", expected, actual)
            }
            ParameterValidationError::OutOfRange { min, max, actual } => {
                write!(f, "Value {} out of range [{}, {}]", actual, min, max)
            }
            ParameterValidationError::ToleranceExceeded {
                nominal,
                actual,
                tolerance,
            } => {
                write!(
                    f,
                    "Value {} exceeds tolerance around nominal {}: {:?}",
                    actual, nominal, tolerance
                )
            }
            ParameterValidationError::MissingRequired { key } => {
                write!(f, "Missing required parameter: {}", key)
            }
        }
    }
}

impl std::error::Error for ParameterValidationError {}

/// A group of related parameter definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentParameterGroup {
    /// Group name (e.g. "Electrical", "Thermal", "Mechanical").
    pub name: String,
    /// Group key (e.g. "electrical", "thermal").
    pub key: String,
    /// Parameters belonging to this group.
    pub parameters: Vec<ComponentParameterDefinition>,
}

impl ComponentParameterGroup {
    /// Find a parameter definition by key.
    pub fn get(&self, key: &str) -> Option<&ComponentParameterDefinition> {
        self.parameters.iter().find(|p| p.key == key)
    }
}

/// A typed parameter value in an instance context, including its source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentParameterValue {
    /// The actual value.
    pub value: ValueWithUnit,
    /// Where this value came from.
    pub source: ComponentParameterSource,
    /// Optional override reason or note.
    pub note: Option<String>,
}

/// Full parameter schema for a component category.
///
/// This is a companion to `ComponentDefinition`: the definition holds the
/// flat parameter values, while the schema describes what each key means,
/// its constraints, and grouping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentParameterSchema {
    /// Category this schema applies to.
    pub category: String,
    /// Parameter groups.
    pub groups: Vec<ComponentParameterGroup>,
}

impl ComponentParameterSchema {
    /// Look up a parameter definition across all groups.
    pub fn get_definition(&self, key: &str) -> Option<&ComponentParameterDefinition> {
        self.groups.iter().find_map(|g| g.get(key))
    }

    /// Validate a full parameter map against this schema.
    pub fn validate_map(
        &self,
        parameters: &BTreeMap<String, ValueWithUnit>,
    ) -> Vec<(String, ParameterValidationError)> {
        let mut errors = Vec::new();

        // Check each parameter in the map against its definition.
        for (key, value) in parameters {
            if let Some(def) = self.get_definition(key) {
                if let Err(e) = def.validate(value) {
                    errors.push((key.clone(), e));
                }
            }
        }

        // Check for missing required parameters.
        for group in &self.groups {
            for def in &group.parameters {
                if def.required && !parameters.contains_key(&def.key) {
                    errors.push((
                        def.key.clone(),
                        ParameterValidationError::MissingRequired {
                            key: def.key.clone(),
                        },
                    ));
                }
            }
        }

        errors
    }
}

/// Typed parameter bundle for a resistor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResistorParameters {
    pub resistance: ValueWithUnit,
    pub tolerance: Option<ComponentTolerance>,
    pub power_rating: Option<ValueWithUnit>,
    pub tempco: Option<ValueWithUnit>,
}

impl ResistorParameters {
    /// Extract from a flat parameter map.
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Option<Self> {
        let resistance = map.get("resistance")?.clone();
        Some(ResistorParameters {
            resistance,
            tolerance: None, // tolerance is stored in ratings/metadata in current seeds
            power_rating: map.get("power").cloned(),
            tempco: map.get("tempco").cloned(),
        })
    }

    /// Convert back to a flat parameter map.
    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        map.insert("resistance".to_string(), self.resistance.clone());
        if let Some(p) = &self.power_rating {
            map.insert("power".to_string(), p.clone());
        }
        if let Some(t) = &self.tempco {
            map.insert("tempco".to_string(), t.clone());
        }
        map
    }
}

/// Typed parameter bundle for a capacitor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapacitorParameters {
    pub capacitance: ValueWithUnit,
    pub tolerance: Option<ComponentTolerance>,
    pub voltage_rating: Option<ValueWithUnit>,
    pub dielectric: Option<String>,
    pub esr: Option<ValueWithUnit>,
}

impl CapacitorParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Option<Self> {
        let capacitance = map.get("capacitance")?.clone();
        Some(CapacitorParameters {
            capacitance,
            tolerance: None,
            voltage_rating: map.get("voltage").cloned(),
            dielectric: None,
            esr: map.get("esr").cloned(),
        })
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        map.insert("capacitance".to_string(), self.capacitance.clone());
        if let Some(v) = &self.voltage_rating {
            map.insert("voltage".to_string(), v.clone());
        }
        if let Some(e) = &self.esr {
            map.insert("esr".to_string(), e.clone());
        }
        map
    }
}

/// Typed parameter bundle for an op-amp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpAmpParameters {
    pub gbw: Option<ValueWithUnit>,
    pub input_offset_voltage: Option<ValueWithUnit>,
    pub slew_rate: Option<ValueWithUnit>,
    pub input_bias_current: Option<ValueWithUnit>,
    pub supply_min: Option<ValueWithUnit>,
    pub supply_max: Option<ValueWithUnit>,
}

impl OpAmpParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Self {
        OpAmpParameters {
            gbw: map.get("gbw").cloned(),
            input_offset_voltage: map.get("input_offset_voltage").cloned(),
            slew_rate: map.get("slew_rate").cloned(),
            input_bias_current: map.get("input_bias_current").cloned(),
            supply_min: map.get("supply_min").cloned(),
            supply_max: map.get("supply_max").cloned(),
        }
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        if let Some(v) = &self.gbw {
            map.insert("gbw".to_string(), v.clone());
        }
        if let Some(v) = &self.input_offset_voltage {
            map.insert("input_offset_voltage".to_string(), v.clone());
        }
        if let Some(v) = &self.slew_rate {
            map.insert("slew_rate".to_string(), v.clone());
        }
        if let Some(v) = &self.input_bias_current {
            map.insert("input_bias_current".to_string(), v.clone());
        }
        if let Some(v) = &self.supply_min {
            map.insert("supply_min".to_string(), v.clone());
        }
        if let Some(v) = &self.supply_max {
            map.insert("supply_max".to_string(), v.clone());
        }
        map
    }
}

/// Typed parameter bundle for a MOSFET.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosfetParameters {
    pub vds_max: Option<ValueWithUnit>,
    pub id_max: Option<ValueWithUnit>,
    pub rds_on: Option<ValueWithUnit>,
    pub vgs_th: Option<ValueWithUnit>,
    pub qg: Option<ValueWithUnit>,
    pub ciss: Option<ValueWithUnit>,
    pub coss: Option<ValueWithUnit>,
}

impl MosfetParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Self {
        MosfetParameters {
            vds_max: map.get("vds_max").cloned(),
            id_max: map.get("id_max").cloned(),
            rds_on: map.get("rds_on").cloned(),
            vgs_th: map.get("vgs_th").cloned(),
            qg: map.get("qg").cloned(),
            ciss: map.get("ciss").cloned(),
            coss: map.get("coss").cloned(),
        }
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        if let Some(v) = &self.vds_max {
            map.insert("vds_max".to_string(), v.clone());
        }
        if let Some(v) = &self.id_max {
            map.insert("id_max".to_string(), v.clone());
        }
        if let Some(v) = &self.rds_on {
            map.insert("rds_on".to_string(), v.clone());
        }
        if let Some(v) = &self.vgs_th {
            map.insert("vgs_th".to_string(), v.clone());
        }
        if let Some(v) = &self.qg {
            map.insert("qg".to_string(), v.clone());
        }
        if let Some(v) = &self.ciss {
            map.insert("ciss".to_string(), v.clone());
        }
        if let Some(v) = &self.coss {
            map.insert("coss".to_string(), v.clone());
        }
        map
    }
}

/// Typed parameter bundle for an inductor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InductorParameters {
    pub inductance: ValueWithUnit,
    pub current_rating: Option<ValueWithUnit>,
    pub dc_resistance: Option<ValueWithUnit>,
    pub shielded: Option<bool>,
}

impl InductorParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Option<Self> {
        let inductance = map.get("inductance")?.clone();
        Some(InductorParameters {
            inductance,
            current_rating: map.get("current").cloned(),
            dc_resistance: map.get("dc_resistance").cloned(),
            shielded: None,
        })
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        map.insert("inductance".to_string(), self.inductance.clone());
        if let Some(v) = &self.current_rating {
            map.insert("current".to_string(), v.clone());
        }
        if let Some(v) = &self.dc_resistance {
            map.insert("dc_resistance".to_string(), v.clone());
        }
        map
    }
}

/// Typed parameter bundle for a diode / LED.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiodeParameters {
    pub forward_voltage: Option<ValueWithUnit>,
    pub reverse_voltage: Option<ValueWithUnit>,
    pub forward_current: Option<ValueWithUnit>,
    pub reverse_recovery: Option<ValueWithUnit>,
}

impl DiodeParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Self {
        DiodeParameters {
            forward_voltage: map.get("forward_voltage").cloned(),
            reverse_voltage: map.get("reverse_voltage").cloned(),
            forward_current: map.get("forward_current").cloned(),
            reverse_recovery: map.get("reverse_recovery").cloned(),
        }
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        if let Some(v) = &self.forward_voltage {
            map.insert("forward_voltage".to_string(), v.clone());
        }
        if let Some(v) = &self.reverse_voltage {
            map.insert("reverse_voltage".to_string(), v.clone());
        }
        if let Some(v) = &self.forward_current {
            map.insert("forward_current".to_string(), v.clone());
        }
        if let Some(v) = &self.reverse_recovery {
            map.insert("reverse_recovery".to_string(), v.clone());
        }
        map
    }
}

/// Typed parameter bundle for a BJT.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BjtParameters {
    pub vce_max: Option<ValueWithUnit>,
    pub ic_max: Option<ValueWithUnit>,
    pub power: Option<ValueWithUnit>,
    pub hfe_typ: Option<f64>,
    pub hfe_min: Option<f64>,
}

impl BjtParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Self {
        BjtParameters {
            vce_max: map.get("vce_max").cloned(),
            ic_max: map.get("ic_max").cloned(),
            power: map.get("power").cloned(),
            hfe_typ: None,
            hfe_min: None,
        }
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        if let Some(v) = &self.vce_max {
            map.insert("vce_max".to_string(), v.clone());
        }
        if let Some(v) = &self.ic_max {
            map.insert("ic_max".to_string(), v.clone());
        }
        if let Some(v) = &self.power {
            map.insert("power".to_string(), v.clone());
        }
        map
    }
}

/// Typed parameter bundle for a voltage regulator (LDO / DC-DC).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegulatorParameters {
    pub output_voltage: Option<ValueWithUnit>,
    pub input_voltage_max: Option<ValueWithUnit>,
    pub dropout_voltage: Option<ValueWithUnit>,
    pub max_current: Option<ValueWithUnit>,
    pub psrr: Option<ValueWithUnit>,
    pub line_regulation: Option<ValueWithUnit>,
}

impl RegulatorParameters {
    pub fn from_map(map: &BTreeMap<String, ValueWithUnit>) -> Self {
        RegulatorParameters {
            output_voltage: map.get("output_voltage").cloned(),
            input_voltage_max: map.get("input_voltage_max").cloned(),
            dropout_voltage: map.get("dropout_voltage").cloned(),
            max_current: map.get("max_current").cloned(),
            psrr: map.get("psrr").cloned(),
            line_regulation: map.get("line_regulation").cloned(),
        }
    }

    pub fn to_map(&self) -> BTreeMap<String, ValueWithUnit> {
        let mut map = BTreeMap::new();
        if let Some(v) = &self.output_voltage {
            map.insert("output_voltage".to_string(), v.clone());
        }
        if let Some(v) = &self.input_voltage_max {
            map.insert("input_voltage_max".to_string(), v.clone());
        }
        if let Some(v) = &self.dropout_voltage {
            map.insert("dropout_voltage".to_string(), v.clone());
        }
        if let Some(v) = &self.max_current {
            map.insert("max_current".to_string(), v.clone());
        }
        if let Some(v) = &self.psrr {
            map.insert("psrr".to_string(), v.clone());
        }
        if let Some(v) = &self.line_regulation {
            map.insert("line_regulation".to_string(), v.clone());
        }
        map
    }
}

// ---------------------------------------------------------------------------
// Parameter schema helpers — build schemas for known categories
// ---------------------------------------------------------------------------

/// Build the parameter schema for resistors.
pub fn resistor_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "Resistor".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Resistance".to_string(),
                        key: "resistance".to_string(),
                        description: Some("Nominal resistance value".to_string()),
                        unit: EngineeringUnit::Ohm,
                        kind: ComponentParameterKind::Primary,
                        required: true,
                        tolerance: None,
                        value_range: Some((0.0, 1e12)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Power Rating".to_string(),
                        key: "power".to_string(),
                        description: Some("Maximum continuous power dissipation".to_string()),
                        unit: EngineeringUnit::Watt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
            ComponentParameterGroup {
                name: "Thermal".to_string(),
                key: "thermal".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Temperature Coefficient".to_string(),
                        key: "tempco".to_string(),
                        description: Some("Resistance change per degree Celsius".to_string()),
                        unit: EngineeringUnit::PpmPerCelsius,
                        kind: ComponentParameterKind::Thermal,
                        required: false,
                        tolerance: None,
                        value_range: Some((-1e6, 1e6)),
                        default_value: None,
                        editable: false,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for capacitors.
pub fn capacitor_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "Capacitor".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Capacitance".to_string(),
                        key: "capacitance".to_string(),
                        description: Some("Nominal capacitance value".to_string()),
                        unit: EngineeringUnit::Farad,
                        kind: ComponentParameterKind::Primary,
                        required: true,
                        tolerance: None,
                        value_range: Some((0.0, 1e6)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Voltage Rating".to_string(),
                        key: "voltage".to_string(),
                        description: Some("Maximum DC working voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "ESR".to_string(),
                        key: "esr".to_string(),
                        description: Some("Equivalent series resistance".to_string()),
                        unit: EngineeringUnit::Ohm,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e6)),
                        default_value: None,
                        editable: false,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for op-amps.
pub fn op_amp_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "OpAmp".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Gain-Bandwidth Product".to_string(),
                        key: "gbw".to_string(),
                        description: Some("Unity-gain bandwidth".to_string()),
                        unit: EngineeringUnit::Hertz,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e12)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Input Offset Voltage".to_string(),
                        key: "input_offset_voltage".to_string(),
                        description: Some("DC input offset voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1.0)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Slew Rate".to_string(),
                        key: "slew_rate".to_string(),
                        description: Some("Maximum rate of output voltage change".to_string()),
                        unit: EngineeringUnit::VoltPerMicrosecond,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e6)),
                        default_value: None,
                        editable: false,
                    },
                    ComponentParameterDefinition {
                        name: "Input Bias Current".to_string(),
                        key: "input_bias_current".to_string(),
                        description: Some("DC input bias current".to_string()),
                        unit: EngineeringUnit::Ampere,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1.0)),
                        default_value: None,
                        editable: false,
                    },
                ],
            },
            ComponentParameterGroup {
                name: "Supply".to_string(),
                key: "supply".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Min Supply Voltage".to_string(),
                        key: "supply_min".to_string(),
                        description: Some("Minimum supply voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: false,
                    },
                    ComponentParameterDefinition {
                        name: "Max Supply Voltage".to_string(),
                        key: "supply_max".to_string(),
                        description: Some("Maximum supply voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: false,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for MOSFETs.
pub fn mosfet_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "Mosfet".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "VDS Max".to_string(),
                        key: "vds_max".to_string(),
                        description: Some("Maximum drain-source voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "ID Max".to_string(),
                        key: "id_max".to_string(),
                        description: Some("Maximum continuous drain current".to_string()),
                        unit: EngineeringUnit::Ampere,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "RDS(on)".to_string(),
                        key: "rds_on".to_string(),
                        description: Some("Drain-source on-resistance".to_string()),
                        unit: EngineeringUnit::Ohm,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e6)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for inductors.
pub fn inductor_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "Inductor".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Inductance".to_string(),
                        key: "inductance".to_string(),
                        description: Some("Nominal inductance value".to_string()),
                        unit: EngineeringUnit::Henry,
                        kind: ComponentParameterKind::Primary,
                        required: true,
                        tolerance: None,
                        value_range: Some((0.0, 1e6)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Current Rating".to_string(),
                        key: "current".to_string(),
                        description: Some("Maximum DC current before saturation".to_string()),
                        unit: EngineeringUnit::Ampere,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for diodes.
pub fn diode_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "Diode".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Forward Voltage".to_string(),
                        key: "forward_voltage".to_string(),
                        description: Some("Typical forward voltage drop".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Reverse Voltage".to_string(),
                        key: "reverse_voltage".to_string(),
                        description: Some("Maximum repetitive reverse voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Forward Current".to_string(),
                        key: "forward_current".to_string(),
                        description: Some("Maximum average forward current".to_string()),
                        unit: EngineeringUnit::Ampere,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for BJTs.
pub fn bjt_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "Bjt".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Electrical".to_string(),
                key: "electrical".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "VCE Max".to_string(),
                        key: "vce_max".to_string(),
                        description: Some("Maximum collector-emitter voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "IC Max".to_string(),
                        key: "ic_max".to_string(),
                        description: Some("Maximum collector current".to_string()),
                        unit: EngineeringUnit::Ampere,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Power Dissipation".to_string(),
                        key: "power".to_string(),
                        description: Some("Maximum power dissipation".to_string()),
                        unit: EngineeringUnit::Watt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
        ],
    }
}

/// Build the parameter schema for voltage regulators.
pub fn regulator_schema() -> ComponentParameterSchema {
    ComponentParameterSchema {
        category: "VoltageRegulator".to_string(),
        groups: vec![
            ComponentParameterGroup {
                name: "Output".to_string(),
                key: "output".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Output Voltage".to_string(),
                        key: "output_voltage".to_string(),
                        description: Some("Regulated output voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Max Current".to_string(),
                        key: "max_current".to_string(),
                        description: Some("Maximum output current".to_string()),
                        unit: EngineeringUnit::Ampere,
                        kind: ComponentParameterKind::Primary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
            ComponentParameterGroup {
                name: "Supply".to_string(),
                key: "supply".to_string(),
                parameters: vec![
                    ComponentParameterDefinition {
                        name: "Input Voltage Max".to_string(),
                        key: "input_voltage_max".to_string(),
                        description: Some("Maximum input voltage".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e4)),
                        default_value: None,
                        editable: true,
                    },
                    ComponentParameterDefinition {
                        name: "Dropout Voltage".to_string(),
                        key: "dropout_voltage".to_string(),
                        description: Some("Minimum headroom for regulation".to_string()),
                        unit: EngineeringUnit::Volt,
                        kind: ComponentParameterKind::Secondary,
                        required: false,
                        tolerance: None,
                        value_range: Some((0.0, 1e3)),
                        default_value: None,
                        editable: true,
                    },
                ],
            },
        ],
    }
}

/// Get the parameter schema for a given component category string.
pub fn schema_for_category(category: &str) -> Option<ComponentParameterSchema> {
    match category {
        "Resistor" => Some(resistor_schema()),
        "Capacitor" => Some(capacitor_schema()),
        "Inductor" => Some(inductor_schema()),
        "Diode" => Some(diode_schema()),
        "LED" => Some(diode_schema()),
        "Bjt" => Some(bjt_schema()),
        "Mosfet" => Some(mosfet_schema()),
        "OpAmp" => Some(op_amp_schema()),
        "VoltageRegulator" => Some(regulator_schema()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tolerance_symmetric_percent() {
        let tol = ComponentTolerance::SymmetricPercent { value: 5.0 };
        assert!(tol.is_within(100.0, 102.0));
        assert!(tol.is_within(100.0, 95.0));
        assert!(!tol.is_within(100.0, 110.0));

        let (lo, hi) = tol.bounds(100.0);
        assert!((lo - 95.0).abs() < 1e-9);
        assert!((hi - 105.0).abs() < 1e-9);
    }

    #[test]
    fn test_tolerance_asymmetric() {
        let tol = ComponentTolerance::Asymmetric {
            minus: 1.0,
            plus: 2.0,
        };
        assert!(tol.is_within(10.0, 11.0));
        assert!(!tol.is_within(10.0, 13.0));
    }

    #[test]
    fn test_tolerance_min_max() {
        let tol = ComponentTolerance::MinMax { min: -10.0, max: 10.0 };
        assert!(tol.is_within(0.0, 5.0));
        assert!(!tol.is_within(0.0, 15.0));
    }

    #[test]
    fn test_parameter_definition_validate_ok() {
        let def = ComponentParameterDefinition {
            name: "Resistance".to_string(),
            key: "resistance".to_string(),
            description: None,
            unit: EngineeringUnit::Ohm,
            kind: ComponentParameterKind::Primary,
            required: true,
            tolerance: None,
            value_range: Some((0.0, 1e6)),
            default_value: None,
            editable: true,
        };
        let val = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
        assert!(def.validate(&val).is_ok());
    }

    #[test]
    fn test_parameter_definition_validate_unit_mismatch() {
        let def = ComponentParameterDefinition {
            name: "Resistance".to_string(),
            key: "resistance".to_string(),
            description: None,
            unit: EngineeringUnit::Ohm,
            kind: ComponentParameterKind::Primary,
            required: true,
            tolerance: None,
            value_range: None,
            default_value: None,
            editable: true,
        };
        let val = ValueWithUnit::parse_with_default("10", EngineeringUnit::Volt).unwrap();
        let err = def.validate(&val).unwrap_err();
        assert!(matches!(err, ParameterValidationError::UnitMismatch { .. }));
    }

    #[test]
    fn test_parameter_definition_validate_out_of_range() {
        let def = ComponentParameterDefinition {
            name: "Resistance".to_string(),
            key: "resistance".to_string(),
            description: None,
            unit: EngineeringUnit::Ohm,
            kind: ComponentParameterKind::Primary,
            required: true,
            tolerance: None,
            value_range: Some((0.0, 100.0)),
            default_value: None,
            editable: true,
        };
        let val = ValueWithUnit::parse_with_default("200", EngineeringUnit::Ohm).unwrap();
        let err = def.validate(&val).unwrap_err();
        assert!(matches!(err, ParameterValidationError::OutOfRange { .. }));
    }

    #[test]
    fn test_schema_validate_map() {
        let schema = resistor_schema();
        let mut map = BTreeMap::new();
        map.insert(
            "resistance".to_string(),
            ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap(),
        );
        let errors = schema.validate_map(&map);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_schema_validate_map_missing_required() {
        let schema = resistor_schema();
        let map = BTreeMap::new();
        let errors = schema.validate_map(&map);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0].1,
            ParameterValidationError::MissingRequired { .. }
        ));
    }

    #[test]
    fn test_resistor_parameters_roundtrip() {
        let r = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
        let p = ValueWithUnit::parse_with_default("0.1", EngineeringUnit::Watt).unwrap();
        let rp = ResistorParameters {
            resistance: r.clone(),
            tolerance: None,
            power_rating: Some(p.clone()),
            tempco: None,
        };
        let map = rp.to_map();
        assert_eq!(map.get("resistance"), Some(&r));
        assert_eq!(map.get("power"), Some(&p));

        let rp2 = ResistorParameters::from_map(&map).unwrap();
        assert_eq!(rp2.resistance, r);
        assert_eq!(rp2.power_rating, Some(p));
    }

    #[test]
    fn test_parameter_source_display() {
        assert_eq!(ComponentParameterSource::Override.to_string(), "override");
    }

    #[test]
    fn test_schema_for_category() {
        assert!(schema_for_category("Resistor").is_some());
        assert!(schema_for_category("Capacitor").is_some());
        assert!(schema_for_category("Unknown").is_none());
    }
}
