use hotsas_core::{
    CircuitProject, CircuitQueryService, EngineeringUnit, FormulaDefinition, FormulaEquation,
    FormulaEvaluationResult, FormulaExpressionValidationResult, FormulaOutput, FormulaPack,
    FormulaVariable, GraphPoint, GraphSeries, ReportModel, SimulationProfile, SimulationResult,
    SimulationStatus, SimulationType, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::fs;
use std::path::{Path, PathBuf};

pub mod project_package_storage;
pub use project_package_storage::CircuitProjectPackageStorage;

#[derive(Debug, Default)]
pub struct FormulaPackFileLoader;

impl FormulaPackFileLoader {
    pub fn load_pack_from_file(&self, path: &Path) -> Result<FormulaPack, PortError> {
        let content = fs::read_to_string(path).map_err(|error| {
            PortError::Storage(format!(
                "could not read formula pack {}: {error}",
                path.display()
            ))
        })?;
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let raw: RawFormulaPack = match extension.as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&content).map_err(|error| {
                PortError::Storage(format!("could not parse YAML formula pack: {error}"))
            })?,
            "json" => serde_json::from_str(&content).map_err(|error| {
                PortError::Storage(format!("could not parse JSON formula pack: {error}"))
            })?,
            other => {
                return Err(PortError::Storage(format!(
                    "unsupported formula pack extension: {other}"
                )))
            }
        };
        let pack = raw.into_formula_pack()?;
        Self::validate_pack(&pack)?;
        Ok(pack)
    }

    pub fn load_pack_from_dir(&self, path: &Path) -> Result<Vec<FormulaPack>, PortError> {
        let mut files = fs::read_dir(path)
            .map_err(|error| {
                PortError::Storage(format!(
                    "could not read formula pack directory {}: {error}",
                    path.display()
                ))
            })?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .and_then(|extension| extension.to_str())
                    .map(|extension| matches!(extension, "yaml" | "yml" | "json"))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        files.sort();

        files
            .iter()
            .map(|path| self.load_pack_from_file(path))
            .collect()
    }

    pub fn load_builtin_packs(&self) -> Result<Vec<FormulaPack>, PortError> {
        for path in builtin_formula_pack_candidates() {
            if path.exists() {
                return self.load_pack_from_dir(&path);
            }
        }
        Ok(vec![FormulaPack {
            pack_id: "fallback_filters".to_string(),
            title: "Fallback Filters".to_string(),
            version: "0.1.0".to_string(),
            formulas: vec![hotsas_core::rc_low_pass_formula()],
        }])
    }

    pub fn validate_pack(pack: &FormulaPack) -> Result<(), PortError> {
        require_non_empty(&pack.pack_id, "packId")?;
        require_non_empty(&pack.title, "title")?;
        require_non_empty(&pack.version, "version")?;
        if pack.formulas.is_empty() {
            return Err(PortError::Formula(
                "formula pack must contain at least one formula".to_string(),
            ));
        }

        for formula in &pack.formulas {
            require_non_empty(&formula.id, "formula.id")?;
            require_non_empty(&formula.title, "formula.title")?;
            require_non_empty(&formula.category, "formula.category")?;
            if formula.equations.is_empty() {
                return Err(PortError::Formula(format!(
                    "formula {} must contain at least one equation",
                    formula.id
                )));
            }
            if formula.outputs.is_empty() {
                return Err(PortError::Formula(format!(
                    "formula {} must contain at least one output",
                    formula.id
                )));
            }
        }
        Ok(())
    }
}

fn builtin_formula_pack_candidates() -> Vec<PathBuf> {
    vec![
        PathBuf::from("shared/formula_packs"),
        PathBuf::from("../shared/formula_packs"),
        PathBuf::from("../../shared/formula_packs"),
        PathBuf::from("../../../shared/formula_packs"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../shared/formula_packs"),
    ]
}

fn require_non_empty(value: &str, field: &str) -> Result<(), PortError> {
    if value.trim().is_empty() {
        return Err(PortError::Formula(format!("{field} must not be empty")));
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct RawFormulaPack {
    #[serde(rename = "packId")]
    pack_id: String,
    title: String,
    version: String,
    formulas: Vec<RawFormulaDefinition>,
}

impl RawFormulaPack {
    fn into_formula_pack(self) -> Result<FormulaPack, PortError> {
        Ok(FormulaPack {
            pack_id: self.pack_id,
            title: self.title,
            version: self.version,
            formulas: self
                .formulas
                .into_iter()
                .map(RawFormulaDefinition::into_formula_definition)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(serde::Deserialize)]
struct RawFormulaDefinition {
    id: String,
    title: String,
    category: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    variables: BTreeMap<String, RawFormulaVariable>,
    equations: Vec<FormulaEquation>,
    outputs: BTreeMap<String, RawFormulaOutput>,
    #[serde(default)]
    assumptions: Vec<String>,
    #[serde(default)]
    limitations: Vec<String>,
    #[serde(rename = "linkedCircuitTemplateId")]
    linked_circuit_template_id: Option<String>,
    #[serde(default)]
    mapping: Option<BTreeMap<String, String>>,
    #[serde(rename = "defaultSimulation")]
    default_simulation: Option<RawDefaultSimulation>,
    #[serde(default)]
    examples: Vec<String>,
}

impl RawFormulaDefinition {
    fn into_formula_definition(self) -> Result<FormulaDefinition, PortError> {
        Ok(FormulaDefinition {
            id: self.id,
            title: self.title,
            category: self.category,
            description: self.description,
            equations: self.equations,
            variables: self
                .variables
                .into_iter()
                .map(|(name, variable)| Ok((name, variable.into_formula_variable()?)))
                .collect::<Result<BTreeMap<_, _>, PortError>>()?,
            outputs: self
                .outputs
                .into_iter()
                .map(|(name, output)| Ok((name, output.into_formula_output()?)))
                .collect::<Result<BTreeMap<_, _>, PortError>>()?,
            assumptions: self.assumptions,
            limitations: self.limitations,
            linked_circuit_template_id: self.linked_circuit_template_id,
            mapping: self.mapping,
            default_simulation_profile: self
                .default_simulation
                .map(RawDefaultSimulation::into_simulation_profile)
                .transpose()?,
            examples: self.examples,
        })
    }
}

#[derive(serde::Deserialize)]
struct RawFormulaVariable {
    unit: String,
    description: String,
    #[serde(default)]
    default: Option<String>,
}

impl RawFormulaVariable {
    fn into_formula_variable(self) -> Result<FormulaVariable, PortError> {
        let unit = EngineeringUnit::parse(&self.unit)
            .map_err(|error| PortError::Formula(error.to_string()))?;
        let default = self
            .default
            .map(|value| ValueWithUnit::parse_with_default(&value, unit))
            .transpose()
            .map_err(|error| PortError::Formula(error.to_string()))?;
        Ok(FormulaVariable {
            unit,
            description: self.description,
            default,
        })
    }
}

#[derive(serde::Deserialize)]
struct RawFormulaOutput {
    unit: String,
    description: String,
}

impl RawFormulaOutput {
    fn into_formula_output(self) -> Result<FormulaOutput, PortError> {
        Ok(FormulaOutput {
            unit: EngineeringUnit::parse(&self.unit)
                .map_err(|error| PortError::Formula(error.to_string()))?,
            description: self.description,
        })
    }
}

#[derive(serde::Deserialize)]
struct RawDefaultSimulation {
    #[serde(rename = "type")]
    simulation_type: String,
    start: Option<String>,
    stop: Option<String>,
    #[serde(rename = "pointsPerDecade")]
    points_per_decade: Option<u32>,
}

impl RawDefaultSimulation {
    fn into_simulation_profile(self) -> Result<SimulationProfile, PortError> {
        let simulation_type = match self.simulation_type.as_str() {
            "ac_sweep" => SimulationType::AcSweep,
            other => {
                return Err(PortError::Formula(format!(
                    "unsupported default simulation type: {other}"
                )))
            }
        };
        let mut parameters = BTreeMap::new();
        if let Some(start) = self.start {
            parameters.insert(
                "start".to_string(),
                ValueWithUnit::parse_with_default(&start, EngineeringUnit::Hertz)
                    .map_err(|error| PortError::Formula(error.to_string()))?,
            );
        }
        if let Some(stop) = self.stop {
            parameters.insert(
                "stop".to_string(),
                ValueWithUnit::parse_with_default(&stop, EngineeringUnit::Hertz)
                    .map_err(|error| PortError::Formula(error.to_string()))?,
            );
        }
        if let Some(points) = self.points_per_decade {
            parameters.insert(
                "points_per_decade".to_string(),
                ValueWithUnit::new_si(points as f64, EngineeringUnit::Unitless),
            );
        }

        Ok(SimulationProfile {
            id: "default-ac-sweep".to_string(),
            simulation_type,
            parameters,
            requested_outputs: vec!["gain_db".to_string(), "phase_deg".to_string()],
        })
    }
}

#[derive(Debug, Default)]
pub struct JsonProjectStorage;

impl StoragePort for JsonProjectStorage {
    fn save_project(&self, path: &Path, project: &CircuitProject) -> Result<(), PortError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| PortError::Storage(error.to_string()))?;
        }
        let json = serde_json::to_string_pretty(project)
            .map_err(|error| PortError::Storage(error.to_string()))?;
        fs::write(path, json).map_err(|error| PortError::Storage(error.to_string()))
    }

    fn load_project(&self, path: &Path) -> Result<CircuitProject, PortError> {
        let json =
            fs::read_to_string(path).map_err(|error| PortError::Storage(error.to_string()))?;
        serde_json::from_str(&json).map_err(|error| PortError::Storage(error.to_string()))
    }
}

#[derive(Debug, Default)]
pub struct SimpleFormulaEngine;

impl FormulaEnginePort for SimpleFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        resistance: &ValueWithUnit,
        capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        if resistance.unit != EngineeringUnit::Ohm {
            return Err(PortError::Formula("R must be expressed in Ohm".to_string()));
        }
        if capacitance.unit != EngineeringUnit::Farad {
            return Err(PortError::Formula("C must be expressed in F".to_string()));
        }
        if resistance.si_value() <= 0.0 || capacitance.si_value() <= 0.0 {
            return Err(PortError::Formula("R and C must be positive".to_string()));
        }

        let cutoff = 1.0 / (2.0 * PI * resistance.si_value() * capacitance.si_value());
        Ok(ValueWithUnit::new_si(cutoff, EngineeringUnit::Hertz))
    }

    fn evaluate_formula(
        &self,
        formula: &FormulaDefinition,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<FormulaEvaluationResult, PortError> {
        let equation = formula
            .equations
            .first()
            .ok_or_else(|| PortError::Formula("formula has no equations".to_string()))?;
        let outputs = match formula.id.as_str() {
            "rc_low_pass_cutoff" => self.evaluate_rc_low_pass(variables)?,
            "ohms_law" => self.evaluate_ohms_law(variables)?,
            "voltage_divider" => self.evaluate_voltage_divider(variables)?,
            _ => self.evaluate_expression(&equation.expression, variables, &formula.outputs)?,
        };

        Ok(FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: equation.id.clone(),
            expression: equation.expression.clone(),
            inputs: variables.clone(),
            outputs,
            warnings: vec![],
        })
    }

    fn evaluate_expression(
        &self,
        expression: &str,
        variables: &BTreeMap<String, ValueWithUnit>,
        _expected_outputs: &BTreeMap<String, FormulaOutput>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        match normalize_expression(expression).as_str() {
            "fc=1/(2*pi*R*C)" => self.evaluate_rc_low_pass(variables),
            "V=I*R" => self.evaluate_ohms_law(variables),
            "Vout=Vin*R2/(R1+R2)" => self.evaluate_voltage_divider(variables),
            _ => Err(PortError::Formula(format!(
                "unsupported expression: {expression}"
            ))),
        }
    }

    fn validate_expression(
        &self,
        expression: &str,
    ) -> Result<FormulaExpressionValidationResult, PortError> {
        let supported = matches!(
            normalize_expression(expression).as_str(),
            "fc=1/(2*pi*R*C)" | "V=I*R" | "Vout=Vin*R2/(R1+R2)"
        );
        Ok(FormulaExpressionValidationResult {
            expression: expression.to_string(),
            supported,
            reason: if supported {
                None
            } else {
                Some(format!("unsupported expression: {expression}"))
            },
        })
    }
}

impl SimpleFormulaEngine {
    fn evaluate_rc_low_pass(
        &self,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        let resistance = require_variable(variables, "R", EngineeringUnit::Ohm)?;
        let capacitance = require_variable(variables, "C", EngineeringUnit::Farad)?;
        if resistance.si_value() <= 0.0 || capacitance.si_value() <= 0.0 {
            return Err(PortError::Formula("R and C must be positive".to_string()));
        }
        Ok(BTreeMap::from([(
            "fc".to_string(),
            ValueWithUnit::new_si(
                1.0 / (2.0 * PI * resistance.si_value() * capacitance.si_value()),
                EngineeringUnit::Hertz,
            ),
        )]))
    }

    fn evaluate_ohms_law(
        &self,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        let current = require_variable(variables, "I", EngineeringUnit::Ampere)?;
        let resistance = require_variable(variables, "R", EngineeringUnit::Ohm)?;
        Ok(BTreeMap::from([(
            "V".to_string(),
            ValueWithUnit::new_si(
                current.si_value() * resistance.si_value(),
                EngineeringUnit::Volt,
            ),
        )]))
    }

    fn evaluate_voltage_divider(
        &self,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        let input_voltage = require_variable(variables, "Vin", EngineeringUnit::Volt)?;
        let upper = require_variable(variables, "R1", EngineeringUnit::Ohm)?;
        let lower = require_variable(variables, "R2", EngineeringUnit::Ohm)?;
        if upper.si_value() <= 0.0 || lower.si_value() <= 0.0 {
            return Err(PortError::Formula("R1 and R2 must be positive".to_string()));
        }
        Ok(BTreeMap::from([(
            "Vout".to_string(),
            ValueWithUnit::new_si(
                input_voltage.si_value() * lower.si_value() / (upper.si_value() + lower.si_value()),
                EngineeringUnit::Volt,
            ),
        )]))
    }
}

fn require_variable<'a>(
    variables: &'a BTreeMap<String, ValueWithUnit>,
    name: &str,
    unit: EngineeringUnit,
) -> Result<&'a ValueWithUnit, PortError> {
    let value = variables
        .get(name)
        .ok_or_else(|| PortError::Formula(format!("missing variable {name}")))?;
    if value.unit != unit {
        return Err(PortError::Formula(format!(
            "{name} must be expressed in {}",
            unit.symbol()
        )));
    }
    Ok(value)
}

fn normalize_expression(expression: &str) -> String {
    expression
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect()
}

#[derive(Debug, Default)]
pub struct SpiceNetlistExporter;

impl NetlistExporterPort for SpiceNetlistExporter {
    fn export_spice_netlist(&self, project: &CircuitProject) -> Result<String, PortError> {
        let resistance = require_component_parameter(project, "R1", "resistance")?;
        let capacitance = require_component_parameter(project, "C1", "capacitance")?;

        Ok(format!(
            "* HotSAS Studio - RC Low-Pass Demo\n* Source of truth: CircuitModel\nV1 net_in 0 AC 1\nR1 net_in net_out {}\nC1 net_out 0 {}\n.ac dec 100 10 1e6\n.end",
            format_si(resistance.si_value()),
            format_si(capacitance.si_value())
        ))
    }
}

#[derive(Debug, Default)]
pub struct MockSimulationEngine;

impl SimulationEnginePort for MockSimulationEngine {
    fn run_ac_sweep(
        &self,
        project: &CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        let resistance = require_component_parameter(project, "R1", "resistance")?;
        let capacitance = require_component_parameter(project, "C1", "capacitance")?;
        let cutoff = 1.0 / (2.0 * PI * resistance.si_value() * capacitance.si_value());
        let start = profile
            .parameters
            .get("start")
            .map(ValueWithUnit::si_value)
            .unwrap_or(10.0);
        let stop = profile
            .parameters
            .get("stop")
            .map(ValueWithUnit::si_value)
            .unwrap_or(1_000_000.0);
        let mut gain_points = Vec::new();
        let mut phase_points = Vec::new();

        for index in 0..80 {
            let t = index as f64 / 79.0;
            let frequency = 10_f64.powf(start.log10() + t * (stop.log10() - start.log10()));
            let normalized = frequency / cutoff;
            let gain = 1.0 / (1.0 + normalized.powi(2)).sqrt();
            let gain_db = 20.0 * gain.log10();
            let phase_deg = -normalized.atan().to_degrees();
            gain_points.push(GraphPoint {
                x: frequency,
                y: gain_db,
            });
            phase_points.push(GraphPoint {
                x: frequency,
                y: phase_deg,
            });
        }

        let mut measurements = BTreeMap::new();
        measurements.insert(
            "fc".to_string(),
            ValueWithUnit::new_si(cutoff, EngineeringUnit::Hertz),
        );

        Ok(SimulationResult {
            id: "mock-ac-rc-low-pass".to_string(),
            profile_id: profile.id.clone(),
            status: SimulationStatus::Completed,
            graph_series: vec![
                GraphSeries {
                    name: "Gain".to_string(),
                    x_unit: EngineeringUnit::Hertz,
                    y_unit: EngineeringUnit::Unitless,
                    points: gain_points,
                    metadata: BTreeMap::from([("quantity".to_string(), "dB".to_string())]),
                },
                GraphSeries {
                    name: "Phase".to_string(),
                    x_unit: EngineeringUnit::Hertz,
                    y_unit: EngineeringUnit::Unitless,
                    points: phase_points,
                    metadata: BTreeMap::from([("quantity".to_string(), "degrees".to_string())]),
                },
            ],
            measurements,
            warnings: vec![
                "Mock simulation result; ngspice integration is a later adapter.".to_string(),
            ],
            errors: vec![],
            raw_data_path: None,
        })
    }
}

#[derive(Debug, Default)]
pub struct MarkdownReportExporter;

impl ReportExporterPort for MarkdownReportExporter {
    fn export_markdown(&self, report: &ReportModel) -> Result<String, PortError> {
        let mut markdown = format!("# {}\n\n", report.title);
        for section in &report.sections {
            markdown.push_str(&format!(
                "## {}\n\n{}\n\n",
                section.title, section.body_markdown
            ));
        }
        if !report.included_bom.is_empty() {
            markdown.push_str(
                "## BOM\n\n| Designator | Quantity | Value | Description |\n|---|---:|---|---|\n",
            );
            for line in &report.included_bom {
                let value = line
                    .value
                    .as_ref()
                    .map(|value| format!("{:.6} {}", value.si_value(), value.unit.symbol()))
                    .unwrap_or_else(|| "-".to_string());
                markdown.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    line.designator, line.quantity, value, line.description
                ));
            }
            markdown.push('\n');
        }
        Ok(markdown)
    }

    fn export_html(&self, report: &ReportModel) -> Result<String, PortError> {
        let markdown = self.export_markdown(report)?;
        let escaped = markdown
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        Ok(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body><pre>{}</pre></body></html>",
            report.title, escaped
        ))
    }
}

#[derive(Debug, Default)]
pub struct PdfReportExporterPlaceholder;

fn require_component_parameter(
    project: &CircuitProject,
    component_id: &str,
    parameter: &str,
) -> Result<ValueWithUnit, PortError> {
    CircuitQueryService::require_component_parameter(project, component_id, parameter)
        .map_err(|error| PortError::Export(error.to_string()))
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{rc_low_pass_project, EngineeringUnit};

    #[test]
    fn calculates_rc_low_pass_formula() {
        let engine = SimpleFormulaEngine;
        let r = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
        let c = ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap();

        let result = engine.calculate_rc_low_pass_cutoff(&r, &c).unwrap();

        assert_eq!(result.unit, EngineeringUnit::Hertz);
        assert!((result.si_value() - 159.154943).abs() < 0.001);
    }

    #[test]
    fn exports_rc_low_pass_spice_netlist() {
        let exporter = SpiceNetlistExporter;
        let project = rc_low_pass_project();

        let netlist = exporter.export_spice_netlist(&project).unwrap();

        assert!(netlist.contains("V1 net_in 0 AC 1"));
        assert!(netlist.contains("R1 net_in net_out"));
        assert!(netlist.contains("C1 net_out 0"));
        assert!(netlist.contains(".ac dec 100 10 1e6"));
    }

    #[test]
    fn exports_markdown_report() {
        let exporter = MarkdownReportExporter;
        let report = ReportModel {
            id: "report".to_string(),
            title: "RC Report".to_string(),
            sections: vec![hotsas_core::ReportSection {
                title: "Formula".to_string(),
                body_markdown: "fc calculation".to_string(),
            }],
            included_schematic_images: vec![],
            included_formulas: vec![],
            included_simulation_results: vec![],
            included_bom: vec![],
            export_settings: BTreeMap::new(),
        };

        let markdown = exporter.export_markdown(&report).unwrap();

        assert!(markdown.starts_with("# RC Report"));
        assert!(markdown.contains("## Formula"));
    }
}
