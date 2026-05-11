use hotsas_core::{
    build_s_parameter_csv, magnitude_db, phase_deg, return_loss_db, vswr, ComplexValue,
    SParameterAnalysisResult, SParameterAnalysisSource, SParameterCurvePoint, SParameterDataPoint,
    SParameterDataset, SParameterDiagnostic, SParameterMetric, SParameterMetricConfidence,
    SParameterSeverity,
};

fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "actual={actual}, expected={expected}, epsilon={epsilon}"
    );
}

#[test]
fn magnitude_db_zero_returns_none() {
    let c = ComplexValue { re: 0.0, im: 0.0 };
    assert_eq!(magnitude_db(&c), None);
}

#[test]
fn magnitude_db_one_returns_zero_db() {
    let c = ComplexValue { re: 1.0, im: 0.0 };
    assert_approx_eq(magnitude_db(&c).unwrap(), 0.0, 1e-9);
}

#[test]
fn magnitude_db_ten_returns_20_db() {
    let c = ComplexValue { re: 10.0, im: 0.0 };
    assert_approx_eq(magnitude_db(&c).unwrap(), 20.0, 1e-9);
}

#[test]
fn magnitude_db_sqrt2_returns_3_db() {
    let c = ComplexValue { re: 1.0, im: 1.0 };
    assert_approx_eq(magnitude_db(&c).unwrap(), 3.010299, 1e-5);
}

#[test]
fn phase_deg_zero_is_zero() {
    let c = ComplexValue { re: 1.0, im: 0.0 };
    assert_approx_eq(phase_deg(&c).unwrap(), 0.0, 1e-9);
}

#[test]
fn phase_deg_pure_imaginary_is_90() {
    let c = ComplexValue { re: 0.0, im: 1.0 };
    assert_approx_eq(phase_deg(&c).unwrap(), 90.0, 1e-9);
}

#[test]
fn phase_deg_negative_real_is_180() {
    let c = ComplexValue { re: -1.0, im: 0.0 };
    assert_approx_eq(phase_deg(&c).unwrap(), 180.0, 1e-9);
}

#[test]
fn return_loss_db_zero_returns_none() {
    assert_eq!(return_loss_db(0.0), None);
}

#[test]
fn return_loss_db_one_returns_zero() {
    assert_approx_eq(return_loss_db(1.0).unwrap(), 0.0, 1e-9);
}

#[test]
fn return_loss_db_point_one_returns_20_db() {
    assert_approx_eq(return_loss_db(0.1).unwrap(), 20.0, 1e-9);
}

#[test]
fn vswr_perfect_match() {
    assert_approx_eq(vswr(0.0).unwrap(), 1.0, 1e-9);
}

#[test]
fn vswr_open_circuit_returns_none() {
    assert_eq!(vswr(1.0), None);
}

#[test]
fn vswr_negative_returns_none() {
    assert_eq!(vswr(-0.1), None);
}

#[test]
fn vswr_half_returns_three() {
    assert_approx_eq(vswr(0.5).unwrap(), 3.0, 1e-9);
}

#[test]
fn build_csv_produces_header_and_rows() {
    let result = SParameterAnalysisResult {
        id: "test-1".to_string(),
        dataset: SParameterDataset {
            id: "ds-1".to_string(),
            name: "test".to_string(),
            source: SParameterAnalysisSource::ImportedTouchstone,
            port_count: 2,
            reference_impedance_ohm: 50.0,
            frequency_unit: "Hz".to_string(),
            parameter_format: "MA".to_string(),
            points: vec![
                SParameterDataPoint {
                    frequency_hz: 1e6,
                    s11: Some(ComplexValue { re: 0.5, im: 0.0 }),
                    s21: Some(ComplexValue { re: 0.9, im: 0.1 }),
                    s12: None,
                    s22: None,
                },
            ],
            warnings: vec![],
        },
        curve_points: vec![SParameterCurvePoint {
            frequency_hz: 1e6,
            s11_db: Some(-6.020600),
            s21_db: Some(-0.828034),
            s12_db: None,
            s22_db: None,
            s11_phase_deg: Some(0.0),
            s21_phase_deg: Some(6.340191),
            s12_phase_deg: None,
            s22_phase_deg: None,
            return_loss_s11_db: Some(6.020600),
            return_loss_s22_db: None,
            insertion_loss_s21_db: Some(0.828034),
            vswr_s11: Some(3.0),
            vswr_s22: None,
        }],
        metrics: vec![],
        diagnostics: vec![],
        can_plot_s11: true,
        can_plot_s21: true,
        can_plot_s12: false,
        can_plot_s22: false,
        summary: "test".to_string(),
    };

    let csv = build_s_parameter_csv(&result);
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2, "expected header + 1 data row");
    assert!(lines[0].contains("frequency_hz"), "header should contain frequency_hz");
    assert!(lines[0].contains("s11_db"), "header should contain s11_db");
    assert!(lines[1].contains("1000000"), "data row should contain frequency");
}

#[test]
fn build_csv_handles_none_values() {
    let result = SParameterAnalysisResult {
        id: "test-2".to_string(),
        dataset: SParameterDataset {
            id: "ds-2".to_string(),
            name: "test".to_string(),
            source: SParameterAnalysisSource::ImportedTouchstone,
            port_count: 1,
            reference_impedance_ohm: 50.0,
            frequency_unit: "Hz".to_string(),
            parameter_format: "MA".to_string(),
            points: vec![SParameterDataPoint {
                frequency_hz: 1e9,
                s11: Some(ComplexValue { re: 0.0, im: 0.0 }),
                s21: None,
                s12: None,
                s22: None,
            }],
            warnings: vec![],
        },
        curve_points: vec![SParameterCurvePoint {
            frequency_hz: 1e9,
            s11_db: None,
            s21_db: None,
            s12_db: None,
            s22_db: None,
            s11_phase_deg: None,
            s21_phase_deg: None,
            s12_phase_deg: None,
            s22_phase_deg: None,
            return_loss_s11_db: None,
            return_loss_s22_db: None,
            insertion_loss_s21_db: None,
            vswr_s11: None,
            vswr_s22: None,
        }],
        metrics: vec![],
        diagnostics: vec![],
        can_plot_s11: false,
        can_plot_s21: false,
        can_plot_s12: false,
        can_plot_s22: false,
        summary: "test".to_string(),
    };

    let csv = build_s_parameter_csv(&result);
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2);
    let cols: Vec<&str> = lines[1].split(',').collect();
    assert_eq!(cols.len(), 14);
    assert_eq!(cols[0], "1000000000");
    for i in 1..14 {
        assert_eq!(cols[i], "", "column {i} should be empty for None value");
    }
}
