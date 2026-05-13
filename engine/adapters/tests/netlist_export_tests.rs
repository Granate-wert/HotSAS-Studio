use hotsas_adapters::SpiceNetlistExporter;
use hotsas_core::{rc_low_pass_project, CircuitProject};
use hotsas_ports::NetlistExporterPort;

#[test]
fn spice_netlist_contains_required_rc_low_pass_fragments() {
    let exporter = SpiceNetlistExporter;
    let project = rc_low_pass_project();

    let netlist = exporter.export_spice_netlist(&project).unwrap();

    for fragment in ["V1", "R1", "C1", "net_in", "net_out", " 0 ", ".ac", ".end"] {
        assert!(
            netlist.contains(fragment),
            "netlist must contain fragment {fragment:?}"
        );
    }
    for forbidden in ["NaN", "inf", "Infinity", "undefined", "null"] {
        assert!(
            !netlist.contains(forbidden),
            "netlist must not contain {forbidden:?}"
        );
    }
}

#[test]
fn spice_netlist_errors_when_required_parameters_are_missing() {
    let exporter = SpiceNetlistExporter;

    let mut missing_resistance = rc_low_pass_project();
    parameter_map_mut(&mut missing_resistance, "R1").remove("resistance");
    assert!(exporter.export_spice_netlist(&missing_resistance).is_err());

    let mut missing_capacitance = rc_low_pass_project();
    parameter_map_mut(&mut missing_capacitance, "C1").remove("capacitance");
    assert!(exporter.export_spice_netlist(&missing_capacitance).is_err());
}

#[test]
fn spice_netlist_errors_when_required_components_are_missing() {
    let exporter = SpiceNetlistExporter;

    let mut missing_resistor = rc_low_pass_project();
    missing_resistor
        .schematic
        .components
        .retain(|component| component.instance_id != "R1");
    assert!(exporter.export_spice_netlist(&missing_resistor).is_err());

    let mut missing_capacitor = rc_low_pass_project();
    missing_capacitor
        .schematic
        .components
        .retain(|component| component.instance_id != "C1");
    assert!(exporter.export_spice_netlist(&missing_capacitor).is_err());
}

fn parameter_map_mut<'a>(
    project: &'a mut CircuitProject,
    component_id: &str,
) -> &'a mut std::collections::BTreeMap<String, hotsas_core::ValueWithUnit> {
    &mut project
        .schematic
        .components
        .iter_mut()
        .find(|component| component.instance_id == component_id)
        .unwrap_or_else(|| panic!("missing component {component_id}"))
        .overridden_parameters
}

#[test]
fn netlist_uses_updated_resistor_value() {
    let mut project = rc_low_pass_project();
    // Update R1 resistance via overridden_parameters
    let r1 = project
        .schematic
        .components
        .iter_mut()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    r1.overridden_parameters.insert(
        "resistance".to_string(),
        hotsas_core::ValueWithUnit::parse_with_default("4.7k", hotsas_core::EngineeringUnit::Ohm)
            .unwrap(),
    );

    let exporter = hotsas_adapters::UserCircuitSpiceNetlistExporter;
    let netlist = exporter.export_spice_netlist(&project).unwrap();
    assert!(
        netlist.contains("4700") || netlist.contains("4.7k"),
        "netlist should contain updated resistor value, got: {}",
        netlist
    );
}
