use crate::{
    CircuitEndpoint, CircuitLabel, CircuitModel, CircuitProject, CircuitTemplate,
    ComponentInstance, ConnectedPin, EngineeringUnit, FormulaDefinition, FormulaEquation,
    FormulaOutput, FormulaVariable, Net, Point, Probe, ProbeType, SimulationProfile,
    SimulationType, ValueWithUnit, Wire,
};
use std::collections::BTreeMap;

pub fn rc_low_pass_project() -> CircuitProject {
    let template = rc_low_pass_template();
    CircuitProject {
        id: "rc-low-pass-demo".to_string(),
        name: "RC Low-Pass Demo".to_string(),
        format_version: "1.0.0".to_string(),
        engine_version: "0.1.0".to_string(),
        project_type: "schematic-simulation".to_string(),
        created_at: "2026-05-01T00:00:00Z".to_string(),
        updated_at: "2026-05-01T00:00:00Z".to_string(),
        schematic: CircuitModel {
            id: "schematic-main".to_string(),
            title: "RC Low-Pass".to_string(),
            components: template.components,
            wires: template.wires,
            nets: vec![
                net("net_in", "Vin", vec![("V1", "p"), ("R1", "1")]),
                net("net_out", "Vout", vec![("R1", "2"), ("C1", "1")]),
                net("gnd", "GND", vec![("V1", "n"), ("C1", "2")]),
            ],
            labels: vec![
                CircuitLabel {
                    id: "label-vin".to_string(),
                    text: "Vin".to_string(),
                    position: Point::new(80.0, 160.0),
                    net_id: Some("net_in".to_string()),
                },
                CircuitLabel {
                    id: "label-vout".to_string(),
                    text: "Vout".to_string(),
                    position: Point::new(430.0, 160.0),
                    net_id: Some("net_out".to_string()),
                },
                CircuitLabel {
                    id: "label-gnd".to_string(),
                    text: "GND".to_string(),
                    position: Point::new(430.0, 320.0),
                    net_id: Some("gnd".to_string()),
                },
            ],
            probes: vec![Probe {
                id: "probe-vout".to_string(),
                probe_type: ProbeType::Voltage,
                target: "net_out".to_string(),
                reference_node: Some("gnd".to_string()),
            }],
            annotations: vec!["Demo vertical slice generated from rc_low_pass_template".to_string()],
        },
        simulation_profiles: vec![rc_low_pass_ac_profile()],
        linked_libraries: vec!["basic-electronics".to_string(), "filters".to_string()],
        reports: vec![],
    }
}

pub fn rc_low_pass_template() -> CircuitTemplate {
    let resistor = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
    let capacitor = ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap();
    let source_ac = ValueWithUnit::new_si(1.0, EngineeringUnit::Volt);
    let mut r_params = BTreeMap::new();
    r_params.insert("resistance".to_string(), resistor.clone());
    let mut c_params = BTreeMap::new();
    c_params.insert("capacitance".to_string(), capacitor.clone());
    let mut v_params = BTreeMap::new();
    v_params.insert("ac_magnitude".to_string(), source_ac);

    let components = vec![
        component(
            "V1",
            "voltage_source",
            80.0,
            210.0,
            vec![("p", "net_in"), ("n", "gnd")],
            v_params,
        ),
        component(
            "R1",
            "resistor",
            260.0,
            160.0,
            vec![("1", "net_in"), ("2", "net_out")],
            r_params,
        ),
        component(
            "C1",
            "capacitor",
            430.0,
            240.0,
            vec![("1", "net_out"), ("2", "gnd")],
            c_params,
        ),
    ];

    let wires = vec![
        wire(
            "wire-vin-r1",
            "V1",
            "p",
            80.0,
            160.0,
            "R1",
            "1",
            220.0,
            160.0,
            "net_in",
        ),
        wire(
            "wire-r1-c1",
            "R1",
            "2",
            300.0,
            160.0,
            "C1",
            "1",
            430.0,
            200.0,
            "net_out",
        ),
        wire(
            "wire-c1-gnd",
            "C1",
            "2",
            430.0,
            280.0,
            "V1",
            "n",
            80.0,
            320.0,
            "gnd",
        ),
    ];

    let mut default_parameters = BTreeMap::new();
    default_parameters.insert("R".to_string(), resistor);
    default_parameters.insert("C".to_string(), capacitor);

    let mut named_nodes = BTreeMap::new();
    named_nodes.insert("Vin".to_string(), "net_in".to_string());
    named_nodes.insert("Vout".to_string(), "net_out".to_string());
    named_nodes.insert("GND".to_string(), "gnd".to_string());

    CircuitTemplate {
        id: "rc_low_pass_template".to_string(),
        title: "RC Low-Pass".to_string(),
        components,
        wires,
        named_nodes,
        input_ports: vec!["Vin".to_string()],
        output_ports: vec!["Vout".to_string()],
        probes: vec![Probe {
            id: "probe-vout".to_string(),
            probe_type: ProbeType::Voltage,
            target: "net_out".to_string(),
            reference_node: Some("gnd".to_string()),
        }],
        default_parameters,
        compatible_formula_ids: vec!["rc_low_pass_cutoff".to_string()],
        simulation_profiles: vec![rc_low_pass_ac_profile()],
    }
}

pub fn rc_low_pass_formula() -> FormulaDefinition {
    let mut variables = BTreeMap::new();
    variables.insert(
        "R".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Ohm,
            description: "Resistance".to_string(),
            default: Some(ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap()),
        },
    );
    variables.insert(
        "C".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Farad,
            description: "Capacitance".to_string(),
            default: Some(
                ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap(),
            ),
        },
    );

    let mut outputs = BTreeMap::new();
    outputs.insert(
        "fc".to_string(),
        FormulaOutput {
            unit: EngineeringUnit::Hertz,
            description: "Cutoff frequency".to_string(),
        },
    );

    FormulaDefinition {
        id: "rc_low_pass_cutoff".to_string(),
        title: "RC Low-Pass Cutoff Frequency".to_string(),
        category: "filters/passive".to_string(),
        description: "Cutoff frequency of a first-order RC low-pass filter.".to_string(),
        equations: vec![FormulaEquation {
            id: "cutoff".to_string(),
            latex: "f_c = \\\\frac{1}{2\\\\pi R C}".to_string(),
            expression: "fc = 1 / (2*pi*R*C)".to_string(),
            solve_for: vec!["fc".to_string(), "R".to_string(), "C".to_string()],
        }],
        variables,
        outputs,
        assumptions: vec!["Ideal resistor and capacitor".to_string()],
        limitations: vec![
            "Parasitics and source/load impedance are ignored in v1 formula mode".to_string(),
        ],
        linked_circuit_template_id: Some("rc_low_pass_template".to_string()),
        mapping: Some(BTreeMap::from([
            ("R".to_string(), "R1.resistance".to_string()),
            ("C".to_string(), "C1.capacitance".to_string()),
            ("Vin".to_string(), "net_in".to_string()),
            ("Vout".to_string(), "net_out".to_string()),
        ])),
        default_simulation_profile: Some(rc_low_pass_ac_profile()),
        examples: vec!["R = 10k, C = 100n -> fc ~= 159.15 Hz".to_string()],
    }
}

pub fn ohms_law_formula() -> FormulaDefinition {
    let mut variables = BTreeMap::new();
    variables.insert(
        "I".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Ampere,
            description: "Current".to_string(),
            default: Some(
                ValueWithUnit::parse_with_default("10m", EngineeringUnit::Ampere).unwrap(),
            ),
        },
    );
    variables.insert(
        "R".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Ohm,
            description: "Resistance".to_string(),
            default: Some(ValueWithUnit::parse_with_default("1k", EngineeringUnit::Ohm).unwrap()),
        },
    );

    let mut outputs = BTreeMap::new();
    outputs.insert(
        "V".to_string(),
        FormulaOutput {
            unit: EngineeringUnit::Volt,
            description: "Voltage drop".to_string(),
        },
    );

    FormulaDefinition {
        id: "ohms_law".to_string(),
        title: "Ohm's Law".to_string(),
        category: "basic_electronics/passive".to_string(),
        description: "Voltage across a resistor given current and resistance.".to_string(),
        equations: vec![FormulaEquation {
            id: "voltage".to_string(),
            latex: "V = I R".to_string(),
            expression: "V = I * R".to_string(),
            solve_for: vec!["V".to_string(), "I".to_string(), "R".to_string()],
        }],
        variables,
        outputs,
        assumptions: vec!["Linear resistor".to_string()],
        limitations: vec!["Temperature effects ignored".to_string()],
        linked_circuit_template_id: None,
        mapping: None,
        default_simulation_profile: None,
        examples: vec!["I = 10mA, R = 1k -> V = 10V".to_string()],
    }
}

pub fn voltage_divider_formula() -> FormulaDefinition {
    let mut variables = BTreeMap::new();
    variables.insert(
        "Vin".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Volt,
            description: "Input voltage".to_string(),
            default: Some(ValueWithUnit::parse_with_default("5", EngineeringUnit::Volt).unwrap()),
        },
    );
    variables.insert(
        "R1".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Ohm,
            description: "Top resistor".to_string(),
            default: Some(ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap()),
        },
    );
    variables.insert(
        "R2".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Ohm,
            description: "Bottom resistor".to_string(),
            default: Some(ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap()),
        },
    );

    let mut outputs = BTreeMap::new();
    outputs.insert(
        "Vout".to_string(),
        FormulaOutput {
            unit: EngineeringUnit::Volt,
            description: "Output voltage".to_string(),
        },
    );

    FormulaDefinition {
        id: "voltage_divider".to_string(),
        title: "Voltage Divider".to_string(),
        category: "basic_electronics/passive".to_string(),
        description: "Output voltage of a resistive voltage divider.".to_string(),
        equations: vec![FormulaEquation {
            id: "vout".to_string(),
            latex: "V_{out} = V_{in} \\frac{R_2}{R_1 + R_2}".to_string(),
            expression: "Vout = Vin * R2 / (R1 + R2)".to_string(),
            solve_for: vec!["Vout".to_string()],
        }],
        variables,
        outputs,
        assumptions: vec!["No load current".to_string()],
        limitations: vec!["Loading effects ignored".to_string()],
        linked_circuit_template_id: None,
        mapping: None,
        default_simulation_profile: None,
        examples: vec!["Vin = 5V, R1 = 10k, R2 = 10k -> Vout = 2.5V".to_string()],
    }
}

pub fn rc_low_pass_ac_profile() -> SimulationProfile {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "start".to_string(),
        ValueWithUnit::new_si(10.0, EngineeringUnit::Hertz),
    );
    parameters.insert(
        "stop".to_string(),
        ValueWithUnit::new_si(1_000_000.0, EngineeringUnit::Hertz),
    );
    parameters.insert(
        "points_per_decade".to_string(),
        ValueWithUnit::new_si(100.0, EngineeringUnit::Unitless),
    );

    SimulationProfile {
        id: "ac-sweep".to_string(),
        simulation_type: SimulationType::AcSweep,
        parameters,
        requested_outputs: vec!["gain_db".to_string(), "phase_deg".to_string()],
    }
}

fn component(
    instance_id: &str,
    definition_id: &str,
    x: f64,
    y: f64,
    pins: Vec<(&str, &str)>,
    overridden_parameters: BTreeMap<String, ValueWithUnit>,
) -> ComponentInstance {
    ComponentInstance {
        instance_id: instance_id.to_string(),
        definition_id: definition_id.to_string(),
        selected_symbol_id: Some(format!("{definition_id}_symbol")),
        selected_footprint_id: Some(format!("{definition_id}_placeholder_footprint")),
        selected_simulation_model_id: None,
        position: Point::new(x, y),
        rotation_degrees: 0.0,
        connected_nets: pins
            .into_iter()
            .map(|(pin_id, net_id)| ConnectedPin {
                pin_id: pin_id.to_string(),
                net_id: net_id.to_string(),
            })
            .collect(),
        overridden_parameters,
        notes: None,
    }
}

fn wire(
    id: &str,
    from_component: &str,
    from_pin: &str,
    from_x: f64,
    from_y: f64,
    to_component: &str,
    to_pin: &str,
    to_x: f64,
    to_y: f64,
    net_id: &str,
) -> Wire {
    Wire {
        id: id.to_string(),
        from: CircuitEndpoint {
            component_id: Some(from_component.to_string()),
            pin_id: Some(from_pin.to_string()),
            point: Point::new(from_x, from_y),
        },
        to: CircuitEndpoint {
            component_id: Some(to_component.to_string()),
            pin_id: Some(to_pin.to_string()),
            point: Point::new(to_x, to_y),
        },
        net_id: net_id.to_string(),
    }
}

fn net(id: &str, name: &str, pins: Vec<(&str, &str)>) -> Net {
    Net {
        id: id.to_string(),
        name: name.to_string(),
        connected_pins: pins
            .into_iter()
            .map(|(component_id, pin_id)| ConnectedPin {
                pin_id: format!("{component_id}.{pin_id}"),
                net_id: id.to_string(),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_template_is_bound_to_cutoff_formula() {
        let template = rc_low_pass_template();
        let formula = rc_low_pass_formula();

        assert_eq!(
            formula.linked_circuit_template_id.as_deref(),
            Some("rc_low_pass_template")
        );
        assert!(template
            .compatible_formula_ids
            .contains(&"rc_low_pass_cutoff".to_string()));
        assert_eq!(template.named_nodes.get("Vin").unwrap(), "net_in");
        assert_eq!(template.named_nodes.get("Vout").unwrap(), "net_out");
    }
}
