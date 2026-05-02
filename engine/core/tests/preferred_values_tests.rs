use hotsas_core::{
    calculate_error_percent, generate_decade_values, nearest_preferred_value, EngineeringUnit,
    PreferredValueSeries, ValueWithUnit,
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

// --- v1.1.5 exact table tests ---

#[test]
fn exact_table_lengths() {
    use hotsas_core::preferred_value_tables::*;

    assert_eq!(E3_BASE.len(), 3);
    assert_eq!(E6_BASE.len(), 6);
    assert_eq!(E12_BASE.len(), 12);
    assert_eq!(E24_BASE.len(), 24);
    assert_eq!(E48_BASE.len(), 48);
    assert_eq!(E96_BASE.len(), 96);
    assert_eq!(E192_BASE.len(), 192);
}

#[test]
fn exact_table_quality() {
    use hotsas_core::preferred_value_tables::*;

    for (name, table) in [
        ("E3", E3_BASE),
        ("E6", E6_BASE),
        ("E12", E12_BASE),
        ("E24", E24_BASE),
        ("E48", E48_BASE),
        ("E96", E96_BASE),
        ("E192", E192_BASE),
    ] {
        assert!(
            table.first().copied().unwrap_or(0.0) - 1.0 < 1e-12,
            "{name} first value must be 1.0"
        );
        assert!(
            table.last().copied().unwrap_or(10.0) < 10.0,
            "{name} last value must be < 10.0"
        );
        for window in table.windows(2) {
            assert!(window[0] < window[1], "{name} must be strictly ascending");
        }
        for &value in table {
            assert!(value > 0.0, "{name} value must be positive");
            assert!(value.is_finite(), "{name} value must be finite");
            assert!(!value.is_nan(), "{name} value must not be NaN");
        }
    }
}

#[test]
fn e48_contains_known_values() {
    use hotsas_core::preferred_value_tables::E48_BASE;

    let known = [1.00, 1.05, 1.21, 1.62, 3.16, 4.87, 9.53];
    for value in known {
        assert!(
            E48_BASE.iter().any(|&v| (v - value).abs() < 1e-9),
            "E48 must contain {value}"
        );
    }
}

#[test]
fn e96_contains_known_values() {
    use hotsas_core::preferred_value_tables::E96_BASE;

    let known = [1.00, 1.02, 1.05, 1.58, 1.62, 4.99, 9.76];
    for value in known {
        assert!(
            E96_BASE.iter().any(|&v| (v - value).abs() < 1e-9),
            "E96 must contain {value}"
        );
    }
}

#[test]
fn e192_contains_known_values() {
    use hotsas_core::preferred_value_tables::E192_BASE;

    let known = [1.00, 1.01, 1.02, 1.05, 1.10, 1.54, 3.16, 4.99, 9.76, 9.88];
    for value in known {
        assert!(
            E192_BASE.iter().any(|&v| (v - value).abs() < 1e-9),
            "E192 must contain {value}"
        );
    }
}

#[test]
fn e96_decade_generation_contains_expected_values() {
    let values = generate_decade_values(PreferredValueSeries::E96, 10.0, 100.0).unwrap();

    for expected in [10.0, 10.2, 10.5, 15.8, 16.2, 49.9, 97.6] {
        assert!(
            values.iter().any(|&v| (v - expected).abs() < 1e-9),
            "E96 decade 10-100 must contain {expected}"
        );
    }
}

#[test]
fn nearest_lower_higher_behavior() {
    // E24 nearest 15.93k -> 16k
    let e24 = nearest_preferred_value(value("15.93k"), PreferredValueSeries::E24).unwrap();
    assert_approx_eq(e24.nearest.si_value(), 16_000.0, 1e-9);

    // E96 nearest 15.93k -> 15.8k (error 130 vs 270)
    let e96 = nearest_preferred_value(value("15.93k"), PreferredValueSeries::E96).unwrap();
    assert_approx_eq(e96.nearest.si_value(), 15_800.0, 1e-9);
    assert_approx_eq(e96.lower.unwrap().si_value(), 15_800.0, 1e-9);
    assert_approx_eq(e96.higher.unwrap().si_value(), 16_200.0, 1e-9);

    // Exact match: nearest/lower/higher all return the same value
    let exact_e24 = nearest_preferred_value(value("10k"), PreferredValueSeries::E24).unwrap();
    assert_approx_eq(exact_e24.nearest.si_value(), 10_000.0, 1e-9);
    assert_approx_eq(exact_e24.lower.unwrap().si_value(), 10_000.0, 1e-9);
    assert_approx_eq(exact_e24.higher.unwrap().si_value(), 10_000.0, 1e-9);

    let exact_e96 = nearest_preferred_value(value("15.8k"), PreferredValueSeries::E96).unwrap();
    assert_approx_eq(exact_e96.nearest.si_value(), 15_800.0, 1e-9);
    assert_approx_eq(exact_e96.lower.unwrap().si_value(), 15_800.0, 1e-9);
    assert_approx_eq(exact_e96.higher.unwrap().si_value(), 15_800.0, 1e-9);
}

#[test]
fn calculate_error_percent_returns_expected_value() {
    let error = calculate_error_percent(15_930.0, 16_000.0);
    assert_approx_eq(error, 0.4394, 1e-4);
}
