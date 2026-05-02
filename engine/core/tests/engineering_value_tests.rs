use hotsas_core::{EngineeringUnit, ValueWithUnit};

fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "actual={actual}, expected={expected}, epsilon={epsilon}"
    );
}

#[test]
fn parses_common_engineering_values_with_default_units() {
    let cases = [
        ("10k", EngineeringUnit::Ohm, 10_000.0, EngineeringUnit::Ohm),
        (
            "100n",
            EngineeringUnit::Farad,
            100e-9,
            EngineeringUnit::Farad,
        ),
        ("1u", EngineeringUnit::Farad, 1e-6, EngineeringUnit::Farad),
        ("1M", EngineeringUnit::Ohm, 1e6, EngineeringUnit::Ohm),
        ("1.5k", EngineeringUnit::Ohm, 1_500.0, EngineeringUnit::Ohm),
        ("0", EngineeringUnit::Ohm, 0.0, EngineeringUnit::Ohm),
    ];

    for (input, default_unit, expected_value, expected_unit) in cases {
        let parsed = ValueWithUnit::parse_with_default(input, default_unit).unwrap();

        assert_approx_eq(parsed.si_value(), expected_value, 1e-18);
        assert_eq!(parsed.unit, expected_unit, "unit mismatch for {input}");
    }
}

#[test]
fn parses_supported_unit_suffixes() {
    let cases = [
        (
            "1MHz",
            EngineeringUnit::Unitless,
            1e6,
            EngineeringUnit::Hertz,
        ),
        (
            "100nF",
            EngineeringUnit::Unitless,
            100e-9,
            EngineeringUnit::Farad,
        ),
        (
            "10kOhm",
            EngineeringUnit::Unitless,
            10_000.0,
            EngineeringUnit::Ohm,
        ),
    ];

    for (input, default_unit, expected_value, expected_unit) in cases {
        let parsed = ValueWithUnit::parse_with_default(input, default_unit).unwrap();

        assert_approx_eq(parsed.si_value(), expected_value, 1e-18);
        assert_eq!(parsed.unit, expected_unit, "unit mismatch for {input}");
    }
}

#[test]
fn invalid_engineering_values_return_errors_without_panicking() {
    for input in [
        "", "abc", "10x", "k10", "1..5k", "NaN", "inf", "+inf", "-inf",
    ] {
        let parsed = ValueWithUnit::parse_with_default(input, EngineeringUnit::Ohm);

        assert!(parsed.is_err(), "expected {input:?} to return an error");
    }
}
