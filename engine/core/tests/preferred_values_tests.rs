use hotsas_core::{
    generate_decade_values, nearest_preferred_value, EngineeringUnit, PreferredValueSeries,
    ValueWithUnit,
};

fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "actual={actual}, expected={expected}, epsilon={epsilon}"
    );
}

fn value(input: &str) -> ValueWithUnit {
    ValueWithUnit::parse_with_default(input, EngineeringUnit::Ohm).unwrap()
}

#[test]
fn e_series_select_expected_exact_and_nearest_values() {
    let e24 = nearest_preferred_value(value("15.93k"), PreferredValueSeries::E24).unwrap();

    assert_approx_eq(e24.nearest.si_value(), 16_000.0, 1e-9);
    assert_approx_eq(e24.lower.unwrap().si_value(), 15_000.0, 1e-9);
    assert_approx_eq(e24.higher.unwrap().si_value(), 16_000.0, 1e-9);

    let exact_e24 = nearest_preferred_value(value("10k"), PreferredValueSeries::E24).unwrap();
    let exact_e12 = nearest_preferred_value(value("8.2k"), PreferredValueSeries::E12).unwrap();
    let exact_e6 = nearest_preferred_value(value("6.8k"), PreferredValueSeries::E6).unwrap();

    assert_approx_eq(exact_e24.nearest.si_value(), 10_000.0, 1e-9);
    assert_approx_eq(exact_e12.nearest.si_value(), 8_200.0, 1e-9);
    assert_approx_eq(exact_e6.nearest.si_value(), 6_800.0, 1e-9);
}

#[test]
fn e24_boundaries_return_ordered_finite_values() {
    for input in ["9.9k", "10k", "10.1k", "100n", "1u", "10u", "1M", "10M"] {
        let requested = ValueWithUnit::parse_with_default(input, EngineeringUnit::Ohm).unwrap();
        let result = nearest_preferred_value(requested.clone(), PreferredValueSeries::E24).unwrap();

        assert!(
            result.nearest.si_value() > 0.0,
            "nearest must be positive for {input}"
        );
        assert!(
            result.nearest.si_value().is_finite(),
            "nearest must be finite for {input}"
        );
        if let Some(lower) = result.lower {
            assert!(
                lower.si_value() <= requested.si_value(),
                "lower must not exceed requested for {input}"
            );
        }
        if let Some(higher) = result.higher {
            assert!(
                higher.si_value() >= requested.si_value(),
                "higher must not be below requested for {input}"
            );
        }
    }
}

#[test]
fn invalid_preferred_value_inputs_return_errors() {
    for si_value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let result = nearest_preferred_value(
            ValueWithUnit::new_si(si_value, EngineeringUnit::Ohm),
            PreferredValueSeries::E24,
        );

        assert!(
            result.is_err(),
            "expected preferred value lookup to reject {si_value}"
        );
    }
}

#[test]
fn generated_decade_values_are_sorted_unique_positive_and_finite() {
    let values = generate_decade_values(PreferredValueSeries::E24, 1e-9, 10_000_000.0).unwrap();

    assert!(!values.is_empty(), "generated values must not be empty");
    for window in values.windows(2) {
        assert!(window[0] < window[1], "values must be strictly ascending");
    }
    for generated in values {
        assert!(generated > 0.0, "generated value must be positive");
        assert!(generated.is_finite(), "generated value must be finite");
    }
}
