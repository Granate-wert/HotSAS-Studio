use hotsas_adapters::{FormulaPackFileLoader, SimpleFormulaEngine};
use hotsas_core::{EngineeringUnit, FormulaDefinition, FormulaOutput, ValueWithUnit};
use hotsas_ports::FormulaEnginePort;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn generic_engine_evaluates_rc_low_pass_cutoff() {
    let engine = SimpleFormulaEngine;
    let formula = formula("rc_low_pass_cutoff");
    let result = engine
        .evaluate_formula(
            &formula,
            &BTreeMap::from([
                ("R".to_string(), value("10k", EngineeringUnit::Ohm)),
                ("C".to_string(), value("100n", EngineeringUnit::Farad)),
            ]),
        )
        .unwrap();

    assert_eq!(result.formula_id, "rc_low_pass_cutoff");
    assert_eq!(result.equation_id, "cutoff");
    assert_eq!(result.outputs["fc"].unit, EngineeringUnit::Hertz);
    assert_approx_eq(result.outputs["fc"].si_value(), 159.154943, 0.000001);
}

#[test]
fn generic_engine_evaluates_ohms_law() {
    let engine = SimpleFormulaEngine;
    let formula = formula("ohms_law");
    let result = engine
        .evaluate_formula(
            &formula,
            &BTreeMap::from([
                ("I".to_string(), value("2m", EngineeringUnit::Ampere)),
                ("R".to_string(), value("10k", EngineeringUnit::Ohm)),
            ]),
        )
        .unwrap();

    assert_eq!(result.outputs["V"].unit, EngineeringUnit::Volt);
    assert_approx_eq(result.outputs["V"].si_value(), 20.0, 0.000001);
}

#[test]
fn generic_engine_evaluates_voltage_divider() {
    let engine = SimpleFormulaEngine;
    let formula = formula("voltage_divider");
    let result = engine
        .evaluate_formula(
            &formula,
            &BTreeMap::from([
                ("Vin".to_string(), value("5", EngineeringUnit::Volt)),
                ("R1".to_string(), value("10k", EngineeringUnit::Ohm)),
                ("R2".to_string(), value("10k", EngineeringUnit::Ohm)),
            ]),
        )
        .unwrap();

    assert_eq!(result.outputs["Vout"].unit, EngineeringUnit::Volt);
    assert_approx_eq(result.outputs["Vout"].si_value(), 2.5, 0.000001);
}

#[test]
fn generic_engine_rejects_missing_variables_wrong_units_and_invalid_values() {
    let engine = SimpleFormulaEngine;
    let rc = formula("rc_low_pass_cutoff");
    let divider = formula("voltage_divider");

    assert!(engine
        .evaluate_formula(
            &rc,
            &BTreeMap::from([("R".to_string(), value("10k", EngineeringUnit::Ohm))]),
        )
        .is_err());
    assert!(engine
        .evaluate_formula(
            &rc,
            &BTreeMap::from([
                ("R".to_string(), value("10k", EngineeringUnit::Volt)),
                ("C".to_string(), value("100n", EngineeringUnit::Farad)),
            ]),
        )
        .is_err());
    assert!(engine
        .evaluate_formula(
            &rc,
            &BTreeMap::from([
                (
                    "R".to_string(),
                    ValueWithUnit::new_si(0.0, EngineeringUnit::Ohm)
                ),
                ("C".to_string(), value("100n", EngineeringUnit::Farad)),
            ]),
        )
        .is_err());
    assert!(engine
        .evaluate_formula(
            &divider,
            &BTreeMap::from([
                ("Vin".to_string(), value("5", EngineeringUnit::Volt)),
                (
                    "R1".to_string(),
                    ValueWithUnit::new_si(0.0, EngineeringUnit::Ohm)
                ),
                (
                    "R2".to_string(),
                    ValueWithUnit::new_si(0.0, EngineeringUnit::Ohm)
                ),
            ]),
        )
        .is_err());
}

#[test]
fn generic_engine_reports_supported_and_unsupported_expressions() {
    let engine = SimpleFormulaEngine;

    for expression in [
        "fc = 1 / (2*pi*R*C)",
        "V = I * R",
        "Vout = Vin * R2 / (R1 + R2)",
    ] {
        let validation = engine.validate_expression(expression).unwrap();
        assert!(validation.supported, "{expression} must be supported");
    }

    let validation = engine.validate_expression("X = @Y").unwrap();
    assert!(!validation.supported);
    assert!(validation.reason.unwrap().contains("unsupported"));
}

#[test]
fn generic_engine_rejects_unsupported_expression() {
    let engine = SimpleFormulaEngine;
    let mut formula = formula("ohms_law");
    formula.id = "custom".to_string();
    formula.equations[0].expression = "X = @Y".to_string();
    formula.outputs = BTreeMap::from([(
        "X".to_string(),
        FormulaOutput {
            unit: EngineeringUnit::Unitless,
            description: "Unsupported output".to_string(),
        },
    )]);

    let result = engine.evaluate_formula(
        &formula,
        &BTreeMap::from([(
            "Y".to_string(),
            ValueWithUnit::new_si(4.0, EngineeringUnit::Unitless),
        )]),
    );

    assert!(result.unwrap_err().to_string().contains("unexpected"));
}

fn formula(id: &str) -> FormulaDefinition {
    let loader = FormulaPackFileLoader;
    loader
        .load_pack_from_dir(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../shared/formula_packs"),
        )
        .unwrap()
        .into_iter()
        .flat_map(|pack| pack.formulas)
        .find(|formula| formula.id == id)
        .unwrap()
}

fn value(input: &str, unit: EngineeringUnit) -> ValueWithUnit {
    ValueWithUnit::parse_with_default(input, unit).unwrap()
}

fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "actual={actual}, expected={expected}, epsilon={epsilon}"
    );
}
