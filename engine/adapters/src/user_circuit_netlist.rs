use hotsas_core::{CircuitProject, ComponentInstance, ValueWithUnit};
use hotsas_ports::{NetlistExporterPort, PortError};
use std::collections::BTreeMap;

/// Generic SPICE netlist exporter for user-built circuits.
/// Supports resistor, capacitor, inductor, voltage source, ground, diode.
#[derive(Debug, Default)]
pub struct UserCircuitSpiceNetlistExporter;

impl NetlistExporterPort for UserCircuitSpiceNetlistExporter {
    fn export_spice_netlist(&self, project: &CircuitProject) -> Result<String, PortError> {
        let mut lines = vec![];
        lines.push(format!("* HotSAS Studio - User Circuit Netlist"));
        lines.push(format!("* Project: {}", project.name));
        lines.push(String::new());

        // Build net name mapping: ground nets → "0"
        let net_renames = build_ground_net_renames(project);

        for component in &project.schematic.components {
            let element = spice_element_for_component(component, &net_renames)?;
            if element.is_empty() {
                continue;
            }
            lines.push(element);
        }

        lines.push(String::new());
        lines.push(".end".to_string());

        Ok(lines.join("\n"))
    }
}

fn build_ground_net_renames(project: &CircuitProject) -> BTreeMap<String, String> {
    let mut renames = BTreeMap::new();
    for component in &project.schematic.components {
        if is_ground(component) {
            for cn in &component.connected_nets {
                renames.insert(cn.net_id.clone(), "0".to_string());
            }
        }
    }
    renames
}

fn is_ground(component: &ComponentInstance) -> bool {
    component.definition_id.contains("ground")
}

fn designator_for_component(component: &ComponentInstance) -> Result<String, PortError> {
    let letter = spice_letter(&component.definition_id)?;
    if component.instance_id.starts_with(letter) {
        Ok(component.instance_id.clone())
    } else {
        Ok(format!("{}{}", letter, sanitize(&component.instance_id)))
    }
}

fn spice_element_for_component(
    component: &ComponentInstance,
    net_renames: &BTreeMap<String, String>,
) -> Result<String, PortError> {
    let letter = spice_letter(&component.definition_id)?;
    if letter == 'G' {
        // Ground reference is represented by node 0; no SPICE element needed.
        return Ok(String::new());
    }

    // Determine node ordering using symbol pin order if available.
    let ordered_nets = ordered_connected_nets(component, net_renames)?;
    let value = component_value(component)?;
    let designator = designator_for_component(component)?;

    match letter {
        'R' | 'C' | 'L' => {
            if ordered_nets.len() < 2 {
                return Ok(format!(
                    "* {} {} has fewer than 2 connections",
                    designator, component.definition_id
                ));
            }
            Ok(format!("{} {} {} {}", designator, ordered_nets[0], ordered_nets[1], value))
        }
        'V' => {
            if ordered_nets.len() < 2 {
                return Ok(format!(
                    "* {} {} has fewer than 2 connections",
                    designator, component.definition_id
                ));
            }
            // Use DC source by default; workflow service may append AC analysis.
            Ok(format!(
                "{} {} {} DC {}",
                designator, ordered_nets[0], ordered_nets[1], value
            ))
        }
        'D' => {
            if ordered_nets.len() < 2 {
                return Ok(format!(
                    "* {} {} has fewer than 2 connections",
                    designator, component.definition_id
                ));
            }
            // Generic diode model placeholder
            Ok(format!(
                "{} {} {} D_GENERIC",
                designator, ordered_nets[0], ordered_nets[1]
            ))
        }
        _ => Ok(format!(
            "* Unsupported component: {} ({})",
            component.definition_id, designator
        )),
    }
}

fn ordered_connected_nets(
    component: &ComponentInstance,
    net_renames: &BTreeMap<String, String>,
) -> Result<Vec<String>, PortError> {
    // Get symbol to determine pin order
    let symbol = hotsas_core::seed_symbol_for_kind(&component.definition_id);

    let mut pairs: Vec<(String, String)> = component
        .connected_nets
        .iter()
        .map(|cn| {
            let net = net_renames
                .get(&cn.net_id)
                .cloned()
                .unwrap_or_else(|| cn.net_id.replace(|c: char| !c.is_alphanumeric() && c != '_', ""));
            (cn.pin_id.clone(), net)
        })
        .collect();

    if let Some(sym) = symbol {
        // Sort by the order of pins in the symbol definition
        let pin_order: BTreeMap<String, usize> = sym
            .pins
            .iter()
            .enumerate()
            .map(|(idx, pin)| (pin.id.clone(), idx))
            .collect();
        pairs.sort_by_key(|(pin_id, _)| pin_order.get(pin_id).copied().unwrap_or(usize::MAX));
    }

    Ok(pairs.into_iter().map(|(_, net)| net).collect())
}

fn spice_letter(definition_id: &str) -> Result<char, PortError> {
    match definition_id {
        id if id.contains("resistor") => Ok('R'),
        id if id.contains("capacitor") => Ok('C'),
        id if id.contains("inductor") => Ok('L'),
        id if id.contains("voltage_source") || id.contains("vsource") => Ok('V'),
        id if id.contains("ground") => Ok('G'),
        id if id.contains("diode") => Ok('D'),
        _ => Err(PortError::Export(format!(
            "unknown component type for SPICE: {}",
            definition_id
        ))),
    }
}

fn component_value(component: &ComponentInstance) -> Result<String, PortError> {
    let param_name = match spice_letter(&component.definition_id)? {
        'R' => "resistance",
        'C' => "capacitance",
        'L' => "inductance",
        'V' => "voltage",
        'D' => return Ok("0.7".to_string()),
        _ => return Ok("".to_string()),
    };

    if let Some(value) = component.overridden_parameters.get(param_name) {
        Ok(format_value(value))
    } else if let Some(value) = default_parameter_value(component, param_name) {
        Ok(format_value(&value))
    } else {
        Ok("".to_string())
    }
}

fn default_parameter_value(
    component: &ComponentInstance,
    param_name: &str,
) -> Option<ValueWithUnit> {
    // Look up built-in library defaults for generic components
    let library = hotsas_core::built_in_component_library();
    let def = library.components.iter().find(|c| c.id == component.definition_id)?;
    def.parameters.get(param_name).cloned()
}

fn format_value(value: &ValueWithUnit) -> String {
    let v = value.si_value();
    if v.abs() >= 1e4 || v.abs() < 1e-3 {
        format!("{v:.6e}")
    } else {
        let s = format!("{v:.6}");
        s.trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn sanitize(id: &str) -> String {
    id.replace(|c: char| !c.is_alphanumeric(), "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{CircuitModel, CircuitProject, ComponentInstance, ConnectedPin, Net, Point};
    use std::collections::BTreeMap;

    fn make_project() -> CircuitProject {
        CircuitProject {
            id: "test".to_string(),
            name: "Test".to_string(),
            format_version: "1".to_string(),
            engine_version: "1".to_string(),
            project_type: "circuit".to_string(),
            created_at: "now".to_string(),
            updated_at: "now".to_string(),
            schematic: CircuitModel {
                id: "sch".to_string(),
                title: "Test".to_string(),
                components: vec![],
                wires: vec![],
                nets: vec![],
                labels: vec![],
                probes: vec![],
                annotations: vec![],
            },
            simulation_profiles: vec![],
            linked_libraries: vec![],
            reports: vec![],
        }
    }

    fn add_component(project: &mut CircuitProject, instance_id: &str, definition_id: &str) {
        project.schematic.components.push(ComponentInstance {
            instance_id: instance_id.to_string(),
            definition_id: definition_id.to_string(),
            selected_symbol_id: None,
            selected_footprint_id: None,
            selected_simulation_model_id: None,
            position: Point::new(0.0, 0.0),
            rotation_degrees: 0.0,
            connected_nets: vec![],
            overridden_parameters: BTreeMap::new(),
            notes: None,
        });
    }

    fn connect_pin(project: &mut CircuitProject, comp_id: &str, pin_id: &str, net_id: &str) {
        if let Some(comp) = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == comp_id)
        {
            comp.connected_nets.push(ConnectedPin {
                component_id: comp_id.to_string(),
                pin_id: pin_id.to_string(),
                net_id: net_id.to_string(),
            });
        }
        if !project.schematic.nets.iter().any(|n| n.id == net_id) {
            project.schematic.nets.push(Net {
                id: net_id.to_string(),
                name: net_id.to_string(),
                connected_pins: vec![],
            });
        }
    }

    #[test]
    fn exports_simple_rc_netlist() {
        let mut project = make_project();
        add_component(&mut project, "R1", "generic_resistor");
        add_component(&mut project, "C1", "generic_capacitor");
        add_component(&mut project, "V1", "generic_voltage_source");
        add_component(&mut project, "GND1", "ground_reference");

        connect_pin(&mut project, "R1", "1", "net_in");
        connect_pin(&mut project, "R1", "2", "net_out");
        connect_pin(&mut project, "C1", "1", "net_out");
        connect_pin(&mut project, "C1", "2", "net_gnd");
        connect_pin(&mut project, "V1", "p", "net_in");
        connect_pin(&mut project, "V1", "n", "net_gnd");
        connect_pin(&mut project, "GND1", "gnd", "net_gnd");

        // Override values
        project.schematic.components[0]
            .overridden_parameters
            .insert(
                "resistance".to_string(),
                ValueWithUnit::parse_with_default("10k", hotsas_core::EngineeringUnit::Ohm).unwrap(),
            );
        project.schematic.components[1]
            .overridden_parameters
            .insert(
                "capacitance".to_string(),
                ValueWithUnit::parse_with_default("100n", hotsas_core::EngineeringUnit::Farad)
                    .unwrap(),
            );
        project.schematic.components[2]
            .overridden_parameters
            .insert(
                "voltage".to_string(),
                ValueWithUnit::parse_with_default("5", hotsas_core::EngineeringUnit::Volt).unwrap(),
            );

        let exporter = UserCircuitSpiceNetlistExporter;
        let netlist = exporter.export_spice_netlist(&project).unwrap();

        assert!(netlist.contains("R1 net_in net_out 1.000000e4"));
        assert!(netlist.contains("C1 net_out 0 1.000000e-7"));
        assert!(netlist.contains("V1 net_in 0 DC 5"));
        // Ground net should be renamed to 0
        assert!(!netlist.contains("net_gnd"));
    }
}
