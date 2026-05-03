use crate::{
    ComponentCategory, ComponentDefinition, ComponentLibrary, EngineeringUnit, FootprintDefinition,
    Pad, SimulationModel, Size2d, ValueWithUnit,
};
use std::collections::BTreeMap;

pub fn built_in_component_library() -> ComponentLibrary {
    let components = vec![
        generic_resistor(),
        generic_capacitor(),
        generic_inductor(),
        generic_diode(),
        generic_led(),
        generic_npn_bjt(),
        generic_pnp_bjt(),
        generic_n_mosfet(),
        generic_p_mosfet(),
        generic_op_amp(),
        generic_voltage_source(),
        ground_reference(),
    ];

    let symbols = vec![];
    // Symbols are stored separately in the symbol.rs seed system;
    // the library references them by id via symbol_ids in ComponentDefinition.
    // We keep the library-level symbols list empty for seed symbols
    // because they live in the core symbol module for schematic editor compatibility.

    let footprints = built_in_footprints();

    ComponentLibrary {
        id: "hotsas_builtin".to_string(),
        title: "HotSAS Built-in Library".to_string(),
        version: "1.5.0".to_string(),
        components,
        symbols,
        footprints,
        simulation_models: vec![],
        metadata: BTreeMap::new(),
    }
}

pub fn built_in_footprints() -> Vec<FootprintDefinition> {
    vec![
        placeholder_footprint("axial_resistor_placeholder", "Axial Resistor", "AXIAL-0.4"),
        placeholder_footprint(
            "radial_capacitor_placeholder",
            "Radial Capacitor",
            "RADIAL-5mm",
        ),
        placeholder_footprint("inductor_placeholder", "Inductor", "IND-AXIAL"),
        placeholder_footprint("do_41_diode_placeholder", "DO-41 Diode", "DO-41"),
        placeholder_footprint("led_5mm_placeholder", "5mm LED", "LED-5mm"),
        placeholder_footprint("to_92_placeholder", "TO-92", "TO-92"),
        placeholder_footprint("to_220_placeholder", "TO-220", "TO-220"),
        placeholder_footprint("soic8_placeholder", "SOIC-8", "SOIC-8"),
        placeholder_footprint("sot23_placeholder", "SOT-23", "SOT-23"),
        placeholder_footprint("ground_virtual_placeholder", "Virtual Ground", "VIRTUAL"),
    ]
}

fn placeholder_footprint(id: &str, name: &str, package_name: &str) -> FootprintDefinition {
    FootprintDefinition {
        id: id.to_string(),
        name: name.to_string(),
        package_name: package_name.to_string(),
        standard: None,
        pads: vec![Pad {
            pad_number: "1".to_string(),
            pad_type: "smd".to_string(),
            shape: "rect".to_string(),
            position: crate::Point::new(0.0, 0.0),
            size: Size2d {
                width: 1.0,
                height: 1.0,
            },
            drill: None,
            layers: vec!["top".to_string()],
        }],
        courtyard: vec![],
        silkscreen: vec![],
        assembly_outline: vec![],
        body_size: None,
        pitch: None,
        height: None,
        model_3d_path: None,
        metadata: BTreeMap::from([(
            "note".to_string(),
            "Placeholder footprint for v1.5 foundation".to_string(),
        )]),
    }
}

fn generic_resistor() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "resistance".to_string(),
        ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "power".to_string(),
        ValueWithUnit::parse_with_default("0.25", EngineeringUnit::Watt).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("5", EngineeringUnit::Percent).unwrap(),
    );
    ComponentDefinition {
        id: "generic_resistor".to_string(),
        name: "Generic Resistor".to_string(),
        category: ComponentCategory::Resistor.to_string(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic resistor for schematic calculations".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["resistor".to_string()],
        footprint_ids: vec!["axial_resistor_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "passive".to_string(),
            "resistor".to_string(),
            "generic".to_string(),
        ],
        metadata: BTreeMap::new(),
    }
}

fn generic_capacitor() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "capacitance".to_string(),
        ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "voltage".to_string(),
        ValueWithUnit::parse_with_default("50", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("10", EngineeringUnit::Percent).unwrap(),
    );
    ComponentDefinition {
        id: "generic_capacitor".to_string(),
        name: "Generic Capacitor".to_string(),
        category: CategoryString::capacitor(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic ceramic capacitor".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["capacitor".to_string()],
        footprint_ids: vec!["radial_capacitor_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "passive".to_string(),
            "capacitor".to_string(),
            "generic".to_string(),
        ],
        metadata: BTreeMap::new(),
    }
}

fn generic_inductor() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "inductance".to_string(),
        ValueWithUnit::parse_with_default("10u", EngineeringUnit::Henry).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "current".to_string(),
        ValueWithUnit::parse_with_default("1", EngineeringUnit::Ampere).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("10", EngineeringUnit::Percent).unwrap(),
    );
    ComponentDefinition {
        id: "generic_inductor".to_string(),
        name: "Generic Inductor".to_string(),
        category: CategoryString::inductor(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic inductor".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["inductor".to_string()],
        footprint_ids: vec!["inductor_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "passive".to_string(),
            "inductor".to_string(),
            "generic".to_string(),
        ],
        metadata: BTreeMap::new(),
    }
}

fn generic_diode() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "forward_voltage".to_string(),
        ValueWithUnit::parse_with_default("0.7", EngineeringUnit::Volt).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "reverse_voltage".to_string(),
        ValueWithUnit::parse_with_default("50", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "forward_current".to_string(),
        ValueWithUnit::parse_with_default("1", EngineeringUnit::Ampere).unwrap(),
    );
    ComponentDefinition {
        id: "generic_diode".to_string(),
        name: "Generic Diode".to_string(),
        category: CategoryString::diode(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic silicon diode".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["diode".to_string()],
        footprint_ids: vec!["do_41_diode_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "semiconductor".to_string(),
            "diode".to_string(),
            "generic".to_string(),
        ],
        metadata: BTreeMap::new(),
    }
}

fn generic_led() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "forward_voltage".to_string(),
        ValueWithUnit::parse_with_default("2.0", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "forward_current".to_string(),
        ValueWithUnit::parse_with_default("20m", EngineeringUnit::Ampere).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "max_current".to_string(),
        ValueWithUnit::parse_with_default("30m", EngineeringUnit::Ampere).unwrap(),
    );
    ComponentDefinition {
        id: "generic_led".to_string(),
        name: "Generic LED".to_string(),
        category: CategoryString::led(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic red LED".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["led".to_string()],
        footprint_ids: vec!["led_5mm_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["opto".to_string(), "led".to_string(), "generic".to_string()],
        metadata: BTreeMap::new(),
    }
}

fn generic_npn_bjt() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "vce_max".to_string(),
        ValueWithUnit::parse_with_default("40", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "ic_max".to_string(),
        ValueWithUnit::parse_with_default("0.6", EngineeringUnit::Ampere).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "power".to_string(),
        ValueWithUnit::parse_with_default("0.5", EngineeringUnit::Watt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("hfe_typ".to_string(), "100".to_string());
    metadata.insert(
        "package_hint".to_string(),
        "TO-92/SOT-23 placeholder".to_string(),
    );
    ComponentDefinition {
        id: "generic_npn_bjt".to_string(),
        name: "Generic NPN BJT".to_string(),
        category: CategoryString::bjt(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic NPN bipolar junction transistor".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["bjt_npn".to_string()],
        footprint_ids: vec![
            "to_92_placeholder".to_string(),
            "sot23_placeholder".to_string(),
        ],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "semiconductor".to_string(),
            "transistor".to_string(),
            "bjt".to_string(),
            "npn".to_string(),
        ],
        metadata,
    }
}

fn generic_pnp_bjt() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "vce_max".to_string(),
        ValueWithUnit::parse_with_default("40", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "ic_max".to_string(),
        ValueWithUnit::parse_with_default("0.6", EngineeringUnit::Ampere).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "power".to_string(),
        ValueWithUnit::parse_with_default("0.5", EngineeringUnit::Watt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("hfe_typ".to_string(), "100".to_string());
    metadata.insert(
        "package_hint".to_string(),
        "TO-92/SOT-23 placeholder".to_string(),
    );
    ComponentDefinition {
        id: "generic_pnp_bjt".to_string(),
        name: "Generic PNP BJT".to_string(),
        category: CategoryString::bjt(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic PNP bipolar junction transistor".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["bjt_pnp".to_string()],
        footprint_ids: vec![
            "to_92_placeholder".to_string(),
            "sot23_placeholder".to_string(),
        ],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "semiconductor".to_string(),
            "transistor".to_string(),
            "bjt".to_string(),
            "pnp".to_string(),
        ],
        metadata,
    }
}

fn generic_n_mosfet() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "vds_max".to_string(),
        ValueWithUnit::parse_with_default("60", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "id_max".to_string(),
        ValueWithUnit::parse_with_default("10", EngineeringUnit::Ampere).unwrap(),
    );
    parameters.insert(
        "rds_on".to_string(),
        ValueWithUnit::parse_with_default("50m", EngineeringUnit::Ohm).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "package_hint".to_string(),
        "TO-220/SO-8 placeholder".to_string(),
    );
    ComponentDefinition {
        id: "generic_n_mosfet".to_string(),
        name: "Generic N-MOSFET".to_string(),
        category: CategoryString::mosfet(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic N-channel MOSFET".to_string()),
        parameters,
        ratings: BTreeMap::new(),
        symbol_ids: vec!["mosfet_n".to_string()],
        footprint_ids: vec![
            "to_220_placeholder".to_string(),
            "soic8_placeholder".to_string(),
        ],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "semiconductor".to_string(),
            "transistor".to_string(),
            "mosfet".to_string(),
            "n-channel".to_string(),
        ],
        metadata,
    }
}

fn generic_p_mosfet() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "vds_max".to_string(),
        ValueWithUnit::parse_with_default("60", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "id_max".to_string(),
        ValueWithUnit::parse_with_default("10", EngineeringUnit::Ampere).unwrap(),
    );
    parameters.insert(
        "rds_on".to_string(),
        ValueWithUnit::parse_with_default("80m", EngineeringUnit::Ohm).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "package_hint".to_string(),
        "TO-220/SO-8 placeholder".to_string(),
    );
    ComponentDefinition {
        id: "generic_p_mosfet".to_string(),
        name: "Generic P-MOSFET".to_string(),
        category: CategoryString::mosfet(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic P-channel MOSFET".to_string()),
        parameters,
        ratings: BTreeMap::new(),
        symbol_ids: vec!["mosfet_p".to_string()],
        footprint_ids: vec![
            "to_220_placeholder".to_string(),
            "soic8_placeholder".to_string(),
        ],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "semiconductor".to_string(),
            "transistor".to_string(),
            "mosfet".to_string(),
            "p-channel".to_string(),
        ],
        metadata,
    }
}

fn generic_op_amp() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "gbw".to_string(),
        ValueWithUnit::parse_with_default("1M", EngineeringUnit::Hertz).unwrap(),
    );
    parameters.insert(
        "input_offset_voltage".to_string(),
        ValueWithUnit::parse_with_default("1m", EngineeringUnit::Volt).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "supply_min".to_string(),
        ValueWithUnit::parse_with_default("3", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "supply_max".to_string(),
        ValueWithUnit::parse_with_default("36", EngineeringUnit::Volt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("slew_rate".to_string(), "1 V/us".to_string());
    ComponentDefinition {
        id: "generic_op_amp".to_string(),
        name: "Generic Op-Amp".to_string(),
        category: CategoryString::opamp(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic operational amplifier".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["op_amp".to_string()],
        footprint_ids: vec!["soic8_placeholder".to_string()],
        simulation_models: vec![SimulationModel {
            id: "generic_op_amp_model".to_string(),
            model_type: "spice".to_string(),
            source_path: None,
            raw_model: None,
            pin_mapping: BTreeMap::new(),
        }],
        datasheets: vec![],
        tags: vec![
            "active".to_string(),
            "opamp".to_string(),
            "generic".to_string(),
        ],
        metadata,
    }
}

fn generic_voltage_source() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "voltage".to_string(),
        ValueWithUnit::parse_with_default("5", EngineeringUnit::Volt).unwrap(),
    );
    ComponentDefinition {
        id: "generic_voltage_source".to_string(),
        name: "Generic Voltage Source".to_string(),
        category: CategoryString::source(),
        manufacturer: None,
        part_number: None,
        description: Some("Generic DC voltage source".to_string()),
        parameters,
        ratings: BTreeMap::new(),
        symbol_ids: vec!["voltage_source".to_string()],
        footprint_ids: vec![],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![
            "source".to_string(),
            "voltage".to_string(),
            "generic".to_string(),
        ],
        metadata: BTreeMap::new(),
    }
}

fn ground_reference() -> ComponentDefinition {
    ComponentDefinition {
        id: "ground_reference".to_string(),
        name: "Ground Reference".to_string(),
        category: CategoryString::ground(),
        manufacturer: None,
        part_number: None,
        description: Some("Circuit ground reference".to_string()),
        parameters: BTreeMap::new(),
        ratings: BTreeMap::new(),
        symbol_ids: vec!["ground".to_string()],
        footprint_ids: vec!["ground_virtual_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["reference".to_string(), "ground".to_string()],
        metadata: BTreeMap::new(),
    }
}

struct CategoryString;

impl CategoryString {
    fn capacitor() -> String {
        ComponentCategory::Capacitor.to_string()
    }
    fn inductor() -> String {
        ComponentCategory::Inductor.to_string()
    }
    fn diode() -> String {
        ComponentCategory::Diode.to_string()
    }
    fn led() -> String {
        ComponentCategory::Led.to_string()
    }
    fn bjt() -> String {
        ComponentCategory::Bjt.to_string()
    }
    fn mosfet() -> String {
        ComponentCategory::Mosfet.to_string()
    }
    fn opamp() -> String {
        ComponentCategory::OpAmp.to_string()
    }
    fn source() -> String {
        ComponentCategory::Source.to_string()
    }
    fn ground() -> String {
        ComponentCategory::Ground.to_string()
    }
}
