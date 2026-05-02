use hotsas_adapters::SimpleFormulaEngine;
use hotsas_core::{EngineeringUnit, ValueWithUnit};
use hotsas_ports::FormulaEnginePort;

fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "actual={actual}, expected={expected}, epsilon={epsilon}"
    );
}

#[test]
fn simple_formula_engine_calculates_rc_low_pass_cutoff() {
    let engine = SimpleFormulaEngine;
    let resistance = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
    let capacitance = ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap();

    let cutoff = engine
        .calculate_rc_low_pass_cutoff(&resistance, &capacitance)
        .unwrap();

    assert_eq!(cutoff.unit, EngineeringUnit::Hertz);
    assert_approx_eq(cutoff.si_value(), 159.154943, 0.000001);
}

#[test]
fn simple_formula_engine_rejects_zero_or_negative_values() {
    let engine = SimpleFormulaEngine;
    let valid_resistance = ValueWithUnit::new_si(10_000.0, EngineeringUnit::Ohm);
    let valid_capacitance = ValueWithUnit::new_si(100e-9, EngineeringUnit::Farad);

    for resistance in [
        ValueWithUnit::new_si(0.0, EngineeringUnit::Ohm),
        ValueWithUnit::new_si(-1.0, EngineeringUnit::Ohm),
    ] {
        assert!(
            engine
                .calculate_rc_low_pass_cutoff(&resistance, &valid_capacitance)
                .is_err(),
            "invalid resistance must return Err"
        );
    }

    for capacitance in [
        ValueWithUnit::new_si(0.0, EngineeringUnit::Farad),
        ValueWithUnit::new_si(-1.0, EngineeringUnit::Farad),
    ] {
        assert!(
            engine
                .calculate_rc_low_pass_cutoff(&valid_resistance, &capacitance)
                .is_err(),
            "invalid capacitance must return Err"
        );
    }
}

#[test]
fn simple_formula_engine_rejects_wrong_units() {
    let engine = SimpleFormulaEngine;
    let resistance = ValueWithUnit::new_si(10_000.0, EngineeringUnit::Volt);
    let capacitance = ValueWithUnit::new_si(100e-9, EngineeringUnit::Farad);

    assert!(engine
        .calculate_rc_low_pass_cutoff(&resistance, &capacitance)
        .is_err());
}
