use hotsas_core::{CircuitModel, CircuitValidationReport};
use std::collections::HashSet;

#[derive(Clone)]
pub struct CircuitValidationService;

impl CircuitValidationService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, circuit: &CircuitModel) -> CircuitValidationReport {
        let mut report = CircuitValidationReport::new();

        self.check_empty_circuit(circuit, &mut report);
        self.check_duplicated_component_ids(circuit, &mut report);
        self.check_missing_ground(circuit, &mut report);
        self.check_missing_required_parameters(circuit, &mut report);
        self.check_floating_nets(circuit, &mut report);
        self.check_unknown_references(circuit, &mut report);

        report
    }

    fn check_empty_circuit(&self, circuit: &CircuitModel, report: &mut CircuitValidationReport) {
        if circuit.components.is_empty() {
            report.add_error(
                "empty_circuit",
                "Circuit contains no components.",
                None,
                None,
            );
        }
    }

    fn check_duplicated_component_ids(
        &self,
        circuit: &CircuitModel,
        report: &mut CircuitValidationReport,
    ) {
        let mut seen = HashSet::new();
        for component in &circuit.components {
            if !seen.insert(component.instance_id.clone()) {
                report.add_error(
                    "duplicated_component_id",
                    &format!(
                        "Component instance id '{}' is duplicated.",
                        component.instance_id
                    ),
                    Some(component.instance_id.clone()),
                    None,
                );
            }
        }
    }

    fn check_missing_ground(&self, circuit: &CircuitModel, report: &mut CircuitValidationReport) {
        let has_ground = circuit.nets.iter().any(|net| {
            let id = net.id.to_lowercase();
            let name = net.name.to_lowercase();
            id.contains("gnd")
                || id.contains("ground")
                || name.contains("gnd")
                || name.contains("ground")
        });
        if !has_ground {
            report.add_error(
                "missing_ground",
                "Circuit has no ground/reference net.",
                None,
                None,
            );
        }
    }

    fn check_missing_required_parameters(
        &self,
        circuit: &CircuitModel,
        report: &mut CircuitValidationReport,
    ) {
        for component in &circuit.components {
            let params = &component.overridden_parameters;
            match component.definition_id.as_str() {
                "resistor" => {
                    if !params.contains_key("resistance") {
                        report.add_error(
                            "missing_required_parameter",
                            &format!(
                                "{} is missing required parameter 'resistance'.",
                                component.instance_id
                            ),
                            Some(component.instance_id.clone()),
                            None,
                        );
                    }
                }
                "capacitor" => {
                    if !params.contains_key("capacitance") {
                        report.add_error(
                            "missing_required_parameter",
                            &format!(
                                "{} is missing required parameter 'capacitance'.",
                                component.instance_id
                            ),
                            Some(component.instance_id.clone()),
                            None,
                        );
                    }
                }
                "voltage_source" => {
                    if !params.contains_key("amplitude") && !params.contains_key("ac_magnitude") {
                        report.add_error(
                            "missing_required_parameter",
                            &format!(
                                "{} is missing required parameter 'amplitude' or 'ac_magnitude'.",
                                component.instance_id
                            ),
                            Some(component.instance_id.clone()),
                            None,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn check_floating_nets(&self, circuit: &CircuitModel, report: &mut CircuitValidationReport) {
        for net in &circuit.nets {
            let pin_count = net.connected_pins.len();
            if pin_count < 2 {
                // Allow probe/test nets if explicitly labeled
                let is_test_net = net.name.to_lowercase().contains("probe")
                    || net.name.to_lowercase().contains("test");
                if !is_test_net {
                    report.add_warning(
                        "floating_net",
                        &format!(
                            "Net '{}' has only {} connected pin(s).",
                            net.name, pin_count
                        ),
                        None,
                        Some(net.id.clone()),
                    );
                }
            }
        }
    }

    fn check_unknown_references(
        &self,
        circuit: &CircuitModel,
        report: &mut CircuitValidationReport,
    ) {
        let component_ids: HashSet<_> = circuit
            .components
            .iter()
            .map(|c| c.instance_id.clone())
            .collect();
        let net_ids: HashSet<_> = circuit.nets.iter().map(|n| n.id.clone()).collect();

        for wire in &circuit.wires {
            if let Some(ref cid) = wire.from.component_id {
                if !component_ids.contains(cid) {
                    report.add_error(
                        "unknown_component_net",
                        &format!("Wire references unknown component '{}'.", cid),
                        Some(cid.clone()),
                        Some(wire.net_id.clone()),
                    );
                }
            }
            if let Some(ref cid) = wire.to.component_id {
                if !component_ids.contains(cid) {
                    report.add_error(
                        "unknown_component_net",
                        &format!("Wire references unknown component '{}'.", cid),
                        Some(cid.clone()),
                        Some(wire.net_id.clone()),
                    );
                }
            }
            if !net_ids.contains(&wire.net_id) {
                report.add_error(
                    "unknown_component_net",
                    &format!("Wire references unknown net '{}'.", wire.net_id),
                    None,
                    Some(wire.net_id.clone()),
                );
            }
        }
    }
}

impl Default for CircuitValidationService {
    fn default() -> Self {
        Self::new()
    }
}
