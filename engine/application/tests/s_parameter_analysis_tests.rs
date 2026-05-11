use hotsas_application::SParameterAnalysisService;
use hotsas_core::{
    ComplexValue, ImportedModelSource, SParameterPoint, TouchstoneFrequencyUnit,
    TouchstoneNetworkData, TouchstoneParameterFormat,
};

fn sample_2port_network() -> TouchstoneNetworkData {
    TouchstoneNetworkData {
        id: "net-1".to_string(),
        name: "sample.s2p".to_string(),
        port_count: 2,
        frequency_unit: TouchstoneFrequencyUnit::MHz,
        parameter_format: TouchstoneParameterFormat::RI,
        reference_impedance_ohm: 50.0,
        points: vec![
            SParameterPoint {
                frequency_hz: 1e6,
                values: vec![
                    ComplexValue { re: 0.5, im: 0.0 },
                    ComplexValue { re: 0.9, im: 0.1 },
                    ComplexValue { re: 0.9, im: 0.1 },
                    ComplexValue { re: 0.4, im: 0.0 },
                ],
            },
            SParameterPoint {
                frequency_hz: 10e6,
                values: vec![
                    ComplexValue { re: 0.3, im: 0.1 },
                    ComplexValue { re: 0.8, im: 0.2 },
                    ComplexValue { re: 0.8, im: 0.2 },
                    ComplexValue { re: 0.3, im: 0.1 },
                ],
            },
            SParameterPoint {
                frequency_hz: 100e6,
                values: vec![
                    ComplexValue { re: 0.1, im: 0.0 },
                    ComplexValue { re: 0.5, im: 0.3 },
                    ComplexValue { re: 0.5, im: 0.3 },
                    ComplexValue { re: 0.1, im: 0.0 },
                ],
            },
        ],
        source: ImportedModelSource {
            file_name: Some("sample.s2p".to_string()),
            file_path: None,
            source_format: "touchstone".to_string(),
            content_hash: None,
        },
        warnings: vec![],
    }
}

fn sample_1port_network() -> TouchstoneNetworkData {
    TouchstoneNetworkData {
        id: "net-2".to_string(),
        name: "sample.s1p".to_string(),
        port_count: 1,
        frequency_unit: TouchstoneFrequencyUnit::GHz,
        parameter_format: TouchstoneParameterFormat::MA,
        reference_impedance_ohm: 50.0,
        points: vec![
            SParameterPoint {
                frequency_hz: 1e9,
                values: vec![ComplexValue { re: 0.2, im: 0.0 }],
            },
            SParameterPoint {
                frequency_hz: 2e9,
                values: vec![ComplexValue { re: 0.1, im: 0.05 }],
            },
        ],
        source: ImportedModelSource {
            file_name: Some("sample.s1p".to_string()),
            file_path: None,
            source_format: "touchstone".to_string(),
            content_hash: None,
        },
        warnings: vec![],
    }
}

#[test]
fn analyze_2port_network_produces_curve_points_and_metrics() {
    let svc = SParameterAnalysisService::new();
    let network = sample_2port_network();
    let result = svc
        .analyze_imported_touchstone_dataset(&network)
        .expect("should analyze 2-port network");

    assert_eq!(result.dataset.port_count, 2);
    assert_eq!(result.curve_points.len(), 3);
    assert!(result.can_plot_s11);
    assert!(result.can_plot_s21);
    assert!(result.can_plot_s12);
    assert!(result.can_plot_s22);
    assert!(!result.metrics.is_empty(), "should produce metrics");

    let cp = &result.curve_points[0];
    assert!(cp.s11_db.is_some());
    assert!(cp.s21_db.is_some());
    assert!(cp.s12_db.is_some());
    assert!(cp.s22_db.is_some());
    assert!(cp.s11_phase_deg.is_some());
    assert!(cp.return_loss_s11_db.is_some());
    assert!(cp.vswr_s11.is_some());
}

#[test]
fn analyze_1port_network_has_only_s11() {
    let svc = SParameterAnalysisService::new();
    let network = sample_1port_network();
    let result = svc
        .analyze_imported_touchstone_dataset(&network)
        .expect("should analyze 1-port network");

    assert_eq!(result.dataset.port_count, 1);
    assert!(result.can_plot_s11);
    assert!(!result.can_plot_s21);
    assert!(!result.can_plot_s12);
    assert!(!result.can_plot_s22);

    let cp = &result.curve_points[0];
    assert!(cp.s11_db.is_some());
    assert!(cp.s21_db.is_none());
}

#[test]
fn non_50_ohm_ref_produces_warning() {
    let svc = SParameterAnalysisService::new();
    let mut network = sample_2port_network();
    network.reference_impedance_ohm = 75.0;
    let result = svc
        .analyze_imported_touchstone_dataset(&network)
        .expect("should analyze");

    let has_warning = result
        .diagnostics
        .iter()
        .any(|d| d.code == "sparam_non_50_ohm_ref");
    assert!(has_warning, "should warn about non-50 ohm ref impedance");
}

#[test]
fn sparse_data_produces_warning() {
    let svc = SParameterAnalysisService::new();
    let mut network = sample_2port_network();
    network.points.truncate(1);
    let result = svc
        .analyze_imported_touchstone_dataset(&network)
        .expect("should analyze");

    let has_warning = result
        .diagnostics
        .iter()
        .any(|d| d.code == "sparam_sparse_data");
    assert!(has_warning, "should warn about sparse data");
}

#[test]
fn unsupported_port_count_returns_error() {
    let svc = SParameterAnalysisService::new();
    let mut network = sample_2port_network();
    network.port_count = 4;
    let result = svc.analyze_imported_touchstone_dataset(&network);
    assert!(result.is_err(), "4-port should be rejected");
}

#[test]
fn export_csv_roundtrip() {
    let svc = SParameterAnalysisService::new();
    let network = sample_2port_network();
    let result = svc
        .analyze_imported_touchstone_dataset(&network)
        .expect("should analyze");

    let csv = svc
        .export_s_parameter_csv(&result)
        .expect("should export CSV");
    assert!(csv.contains("frequency_hz"));
    assert!(csv.contains("s11_db"));
    assert!(csv.lines().count() >= 4); // header + 3 data rows
}

#[test]
fn last_result_storage() {
    let svc = SParameterAnalysisService::new();
    assert!(svc.get_last_result().is_none());

    let network = sample_1port_network();
    let _ = svc.analyze_imported_touchstone_dataset(&network);
    assert!(svc.get_last_result().is_some());

    svc.clear_last_result();
    assert!(svc.get_last_result().is_none());
}

#[test]
fn analyze_touchstone_report_with_warnings() {
    let svc = SParameterAnalysisService::new();
    let network = sample_2port_network();
    let report = hotsas_core::TouchstoneImportReport {
        status: hotsas_core::ModelImportStatus::Parsed,
        network: Some(network),
        warnings: vec!["Unusual header".to_string()],
        errors: vec![],
    };

    let result = svc
        .analyze_touchstone_report(report, Some("test.s2p".to_string()))
        .expect("should analyze report");

    let has_parser_warning = result
        .diagnostics
        .iter()
        .any(|d| d.code == "touchstone_parser_warning");
    assert!(has_parser_warning, "should surface parser warnings");
}
