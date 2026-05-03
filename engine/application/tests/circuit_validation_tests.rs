use hotsas_application::CircuitValidationService;
use hotsas_core::{CircuitModel, ConnectedPin, Net};

fn rc_low_pass_circuit() -> CircuitModel {
    hotsas_core::rc_low_pass_project().schematic
}

fn empty_circuit() -> CircuitModel {
    CircuitModel {
        id: "empty".to_string(),
        title: "Empty".to_string(),
        components: vec![],
        wires: vec![],
        nets: vec![],
        labels: vec![],
        probes: vec![],
        annotations: vec![],
    }
}

fn circuit_without_ground() -> CircuitModel {
    let mut circuit = rc_low_pass_circuit();
    circuit.nets.retain(|net| net.id != "gnd");
    circuit
}

fn circuit_with_duplicated_id() -> CircuitModel {
    let mut circuit = rc_low_pass_circuit();
    if let Some(first) = circuit.components.first().cloned() {
        circuit.components.push(first);
    }
    circuit
}

fn circuit_missing_resistance() -> CircuitModel {
    let mut circuit = rc_low_pass_circuit();
    if let Some(r1) = circuit
        .components
        .iter_mut()
        .find(|c| c.instance_id == "R1")
    {
        r1.overridden_parameters.remove("resistance");
    }
    circuit
}

fn circuit_with_floating_net() -> CircuitModel {
    let mut circuit = rc_low_pass_circuit();
    circuit.nets.push(Net {
        id: "floating".to_string(),
        name: "floating".to_string(),
        connected_pins: vec![ConnectedPin {
            pin_id: "1".to_string(),
            net_id: "floating".to_string(),
        }],
    });
    circuit
}

#[test]
fn valid_rc_low_pass_has_no_errors() {
    let service = CircuitValidationService::new();
    let report = service.validate(&rc_low_pass_circuit());
    assert!(
        report.errors.is_empty(),
        "expected no errors, got: {:?}",
        report.errors
    );
}

#[test]
fn missing_ground_returns_error() {
    let service = CircuitValidationService::new();
    let report = service.validate(&circuit_without_ground());
    assert!(
        report.errors.iter().any(|e| e.code == "missing_ground"),
        "expected missing_ground error"
    );
}

#[test]
fn empty_circuit_returns_error() {
    let service = CircuitValidationService::new();
    let report = service.validate(&empty_circuit());
    assert!(
        report.errors.iter().any(|e| e.code == "empty_circuit"),
        "expected empty_circuit error"
    );
}

#[test]
fn duplicated_component_id_returns_error() {
    let service = CircuitValidationService::new();
    let report = service.validate(&circuit_with_duplicated_id());
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.code == "duplicated_component_id"),
        "expected duplicated_component_id error"
    );
}

#[test]
fn missing_required_parameter_returns_error() {
    let service = CircuitValidationService::new();
    let report = service.validate(&circuit_missing_resistance());
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.code == "missing_required_parameter"),
        "expected missing_required_parameter error"
    );
}

#[test]
fn floating_net_returns_warning() {
    let service = CircuitValidationService::new();
    let report = service.validate(&circuit_with_floating_net());
    assert!(
        report.warnings.iter().any(|w| w.code == "floating_net"),
        "expected floating_net warning"
    );
}
