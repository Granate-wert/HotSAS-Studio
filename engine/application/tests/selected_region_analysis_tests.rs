use hotsas_application::SelectedRegionAnalysisService;
use hotsas_core::{
    CircuitModel, ComponentInstance, ConnectedPin, Net, Point, RegionAnalysisDirection,
    RegionAnalysisMode, SelectedRegionAnalysisRequest,
};

fn make_rc_circuit() -> CircuitModel {
    CircuitModel {
        id: "sch1".to_string(),
        title: "RC".to_string(),
        components: vec![
            ComponentInstance {
                instance_id: "R1".to_string(),
                definition_id: "generic_resistor".to_string(),
                selected_symbol_id: None,
                selected_footprint_id: None,
                selected_simulation_model_id: None,
                position: Point::new(0.0, 0.0),
                rotation_degrees: 0.0,
                connected_nets: vec![
                    ConnectedPin {
                        pin_id: "p1".to_string(),
                        net_id: "net_in".to_string(),
                    },
                    ConnectedPin {
                        pin_id: "p2".to_string(),
                        net_id: "net_out".to_string(),
                    },
                ],
                overridden_parameters: Default::default(),
                notes: None,
            },
            ComponentInstance {
                instance_id: "C1".to_string(),
                definition_id: "generic_capacitor".to_string(),
                selected_symbol_id: None,
                selected_footprint_id: None,
                selected_simulation_model_id: None,
                position: Point::new(10.0, 0.0),
                rotation_degrees: 0.0,
                connected_nets: vec![
                    ConnectedPin {
                        pin_id: "p1".to_string(),
                        net_id: "net_out".to_string(),
                    },
                    ConnectedPin {
                        pin_id: "p2".to_string(),
                        net_id: "gnd".to_string(),
                    },
                ],
                overridden_parameters: Default::default(),
                notes: None,
            },
        ],
        wires: vec![],
        nets: vec![
            Net {
                id: "net_in".to_string(),
                name: "net_in".to_string(),
                connected_pins: vec![ConnectedPin {
                    pin_id: "p1".to_string(),
                    net_id: "net_in".to_string(),
                }],
            },
            Net {
                id: "net_out".to_string(),
                name: "net_out".to_string(),
                connected_pins: vec![
                    ConnectedPin {
                        pin_id: "p2".to_string(),
                        net_id: "net_out".to_string(),
                    },
                    ConnectedPin {
                        pin_id: "p1".to_string(),
                        net_id: "net_out".to_string(),
                    },
                ],
            },
            Net {
                id: "gnd".to_string(),
                name: "gnd".to_string(),
                connected_pins: vec![ConnectedPin {
                    pin_id: "p2".to_string(),
                    net_id: "gnd".to_string(),
                }],
            },
        ],
        labels: vec![],
        probes: vec![],
        annotations: vec![],
    }
}

#[test]
fn preview_selected_region_returns_components_and_nets() {
    let service = SelectedRegionAnalysisService::new();
    let circuit = make_rc_circuit();
    let preview = service
        .preview_selected_region(&circuit, vec!["R1".to_string(), "C1".to_string()])
        .unwrap();
    assert_eq!(preview.selected_components.len(), 2);
    assert_eq!(preview.detected_internal_nets.len(), 3);
    assert_eq!(preview.detected_boundary_nets.len(), 0);
}

#[test]
fn analyze_selected_region_matches_rc_low_pass() {
    let service = SelectedRegionAnalysisService::new();
    let circuit = make_rc_circuit();
    let request = SelectedRegionAnalysisRequest {
        component_ids: vec!["R1".to_string(), "C1".to_string()],
        input_port: Some(hotsas_core::RegionPort {
            positive_net: "net_in".to_string(),
            negative_net: None,
            label: Some("Input".to_string()),
        }),
        output_port: Some(hotsas_core::RegionPort {
            positive_net: "net_out".to_string(),
            negative_net: None,
            label: Some("Output".to_string()),
        }),
        reference_node: Some("gnd".to_string()),
        analysis_direction: RegionAnalysisDirection::LeftToRight,
        analysis_mode: RegionAnalysisMode::AllAvailable,
    };
    let result = service.analyze_selected_region(&circuit, request).unwrap();
    let status_str = format!("{:?}", result.status);
    assert!(
        status_str.contains("Success") || status_str.contains("Partial"),
        "expected Success or Partial, got {:?}",
        result.status
    );
    assert!(!result.summary.is_empty());
}

#[test]
fn validate_empty_selection_returns_error() {
    let service = SelectedRegionAnalysisService::new();
    let circuit = make_rc_circuit();
    let request = SelectedRegionAnalysisRequest {
        component_ids: vec![],
        input_port: None,
        output_port: None,
        reference_node: None,
        analysis_direction: RegionAnalysisDirection::Custom,
        analysis_mode: RegionAnalysisMode::AllAvailable,
    };
    let issues = service.validate_selected_region(&circuit, &request);
    assert!(issues.iter().any(|i| i.code == "empty_selection"));
}

#[test]
fn preview_single_component_has_boundary_nets() {
    let service = SelectedRegionAnalysisService::new();
    let circuit = make_rc_circuit();
    let preview = service
        .preview_selected_region(&circuit, vec!["R1".to_string()])
        .unwrap();
    assert_eq!(preview.selected_components.len(), 1);
    assert!(preview.detected_boundary_nets.len() >= 1);
}

#[test]
fn analyze_unsupported_region_returns_partial() {
    let service = SelectedRegionAnalysisService::new();
    let circuit = make_rc_circuit();
    let request = SelectedRegionAnalysisRequest {
        component_ids: vec!["R1".to_string()],
        input_port: None,
        output_port: None,
        reference_node: None,
        analysis_direction: RegionAnalysisDirection::Custom,
        analysis_mode: RegionAnalysisMode::AllAvailable,
    };
    let result = service.analyze_selected_region(&circuit, request).unwrap();
    assert!(
        result.netlist_fragment.is_some() || !result.warnings.is_empty(),
        "expected netlist or warnings for unsupported region"
    );
}
