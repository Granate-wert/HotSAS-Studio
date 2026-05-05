use crate::{
    ComponentCategory, ComponentDefinition, ComponentLibrary, EngineeringUnit, FootprintDefinition,
    Pad, SimulationModel, Size2d, ValueWithUnit,
};
use std::collections::BTreeMap;

pub fn built_in_component_library() -> ComponentLibrary {
    let components = vec![
        generic_resistor(),
        resistor_10k_0603(),
        resistor_1k_axial(),
        resistor_100r_0805(),
        generic_capacitor(),
        capacitor_100n_0603(),
        capacitor_10u_0805(),
        capacitor_100u_electrolytic(),
        generic_inductor(),
        inductor_47u(),
        generic_diode(),
        diode_1n4148(),
        diode_schottky(),
        generic_led(),
        generic_npn_bjt(),
        bjt_2n2222(),
        generic_pnp_bjt(),
        bjt_2n2907(),
        generic_n_mosfet(),
        mosfet_irfz44n(),
        generic_p_mosfet(),
        generic_op_amp(),
        op_amp_lm358(),
        op_amp_rail_rail_placeholder(),
        generic_voltage_source(),
        ldo_ams1117(),
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
        placeholder_footprint("smd_0603_placeholder", "SMD 0603", "0603"),
        placeholder_footprint("smd_0805_placeholder", "SMD 0805", "0805"),
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
            raw_model_id: None,
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

fn resistor_10k_0603() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "resistance".to_string(),
        ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "power".to_string(),
        ValueWithUnit::parse_with_default("0.1", EngineeringUnit::Watt).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("1", EngineeringUnit::Percent).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("tempco".to_string(), "100 ppm/°C".to_string());
    metadata.insert("package".to_string(), "0603".to_string());
    ComponentDefinition {
        id: "resistor_10k_0603".to_string(),
        name: "Resistor 10k 1% 0603".to_string(),
        category: ComponentCategory::Resistor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("RC0603FR-0710KL".to_string()),
        description: Some("Thick film SMD resistor 10kΩ 1% 0.1W 0603".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["resistor".to_string()],
        footprint_ids: vec!["smd_0603_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "resistor".to_string(), "smd".to_string(), "0603".to_string()],
        metadata,
    }
}

fn resistor_1k_axial() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "resistance".to_string(),
        ValueWithUnit::parse_with_default("1k", EngineeringUnit::Ohm).unwrap(),
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
    let mut metadata = BTreeMap::new();
    metadata.insert("tempco".to_string(), "200 ppm/°C".to_string());
    metadata.insert("package".to_string(), "AXIAL-0.4".to_string());
    ComponentDefinition {
        id: "resistor_1k_axial".to_string(),
        name: "Resistor 1k 5% Axial".to_string(),
        category: ComponentCategory::Resistor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("CFR-25JB-1K".to_string()),
        description: Some("Carbon film axial resistor 1kΩ 5% 0.25W".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["resistor".to_string()],
        footprint_ids: vec!["axial_resistor_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "resistor".to_string(), "axial".to_string()],
        metadata,
    }
}

fn resistor_100r_0805() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "resistance".to_string(),
        ValueWithUnit::parse_with_default("100", EngineeringUnit::Ohm).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "power".to_string(),
        ValueWithUnit::parse_with_default("0.125", EngineeringUnit::Watt).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("1", EngineeringUnit::Percent).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("tempco".to_string(), "100 ppm/°C".to_string());
    metadata.insert("package".to_string(), "0805".to_string());
    ComponentDefinition {
        id: "resistor_100r_0805".to_string(),
        name: "Resistor 100R 1% 0805".to_string(),
        category: ComponentCategory::Resistor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("RC0805FR-07100RL".to_string()),
        description: Some("Thick film SMD resistor 100Ω 1% 0.125W 0805".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["resistor".to_string()],
        footprint_ids: vec!["smd_0805_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "resistor".to_string(), "smd".to_string(), "0805".to_string()],
        metadata,
    }
}

fn capacitor_100n_0603() -> ComponentDefinition {
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
    let mut metadata = BTreeMap::new();
    metadata.insert("dielectric".to_string(), "X7R".to_string());
    metadata.insert("esr".to_string(), "50 mΩ".to_string());
    metadata.insert("package".to_string(), "0603".to_string());
    ComponentDefinition {
        id: "capacitor_100n_0603".to_string(),
        name: "Capacitor 100nF 50V X7R 0603".to_string(),
        category: ComponentCategory::Capacitor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("GRM188R71H104KA93D".to_string()),
        description: Some("MLCC 100nF 50V X7R ±10% 0603".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["capacitor".to_string()],
        footprint_ids: vec!["smd_0603_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "capacitor".to_string(), "ceramic".to_string(), "0603".to_string()],
        metadata,
    }
}

fn capacitor_10u_0805() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "capacitance".to_string(),
        ValueWithUnit::parse_with_default("10u", EngineeringUnit::Farad).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "voltage".to_string(),
        ValueWithUnit::parse_with_default("25", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("20", EngineeringUnit::Percent).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("dielectric".to_string(), "X5R".to_string());
    metadata.insert("esr".to_string(), "30 mΩ".to_string());
    metadata.insert("package".to_string(), "0805".to_string());
    ComponentDefinition {
        id: "capacitor_10u_0805".to_string(),
        name: "Capacitor 10uF 25V X5R 0805".to_string(),
        category: ComponentCategory::Capacitor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("GRM21BR61E106KA73L".to_string()),
        description: Some("MLCC 10µF 25V X5R ±20% 0805".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["capacitor".to_string()],
        footprint_ids: vec!["smd_0805_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "capacitor".to_string(), "ceramic".to_string(), "0805".to_string()],
        metadata,
    }
}

fn capacitor_100u_electrolytic() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "capacitance".to_string(),
        ValueWithUnit::parse_with_default("100u", EngineeringUnit::Farad).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "voltage".to_string(),
        ValueWithUnit::parse_with_default("25", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("20", EngineeringUnit::Percent).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("dielectric".to_string(), "Aluminum Electrolytic".to_string());
    metadata.insert("esr".to_string(), "0.5 Ω".to_string());
    metadata.insert("package".to_string(), "RADIAL-6.3x11".to_string());
    ComponentDefinition {
        id: "capacitor_100u_electrolytic".to_string(),
        name: "Capacitor 100uF 25V Electrolytic".to_string(),
        category: ComponentCategory::Capacitor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("UHE1E101MPD".to_string()),
        description: Some("Aluminum electrolytic 100µF 25V ±20% radial".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["capacitor".to_string()],
        footprint_ids: vec!["radial_capacitor_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "capacitor".to_string(), "electrolytic".to_string()],
        metadata,
    }
}

fn inductor_47u() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "inductance".to_string(),
        ValueWithUnit::parse_with_default("47u", EngineeringUnit::Henry).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "current".to_string(),
        ValueWithUnit::parse_with_default("0.5", EngineeringUnit::Ampere).unwrap(),
    );
    ratings.insert(
        "tolerance".to_string(),
        ValueWithUnit::parse_with_default("20", EngineeringUnit::Percent).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("package".to_string(), "SMD 5.2x5.0".to_string());
    metadata.insert("shielded".to_string(), "no".to_string());
    ComponentDefinition {
        id: "inductor_47u".to_string(),
        name: "Inductor 47uH 0.5A".to_string(),
        category: ComponentCategory::Inductor.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("SRN6045-470M".to_string()),
        description: Some("Power inductor 47µH 0.5A 20% SMD".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["inductor".to_string()],
        footprint_ids: vec!["inductor_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["passive".to_string(), "inductor".to_string(), "smd".to_string()],
        metadata,
    }
}

fn diode_1n4148() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "forward_voltage".to_string(),
        ValueWithUnit::parse_with_default("0.715", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "reverse_recovery".to_string(),
        ValueWithUnit::parse_with_default("4n", EngineeringUnit::Second).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "reverse_voltage".to_string(),
        ValueWithUnit::parse_with_default("100", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "forward_current".to_string(),
        ValueWithUnit::parse_with_default("0.3", EngineeringUnit::Ampere).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("package".to_string(), "DO-35".to_string());
    metadata.insert("type".to_string(), "Small signal switching diode".to_string());
    ComponentDefinition {
        id: "diode_1n4148".to_string(),
        name: "1N4148 Small Signal Diode".to_string(),
        category: ComponentCategory::Diode.to_string(),
        manufacturer: Some("ON Semiconductor".to_string()),
        part_number: Some("1N4148".to_string()),
        description: Some("High-speed switching diode 100V 300mA DO-35".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["diode".to_string()],
        footprint_ids: vec!["do_41_diode_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["semiconductor".to_string(), "diode".to_string(), "signal".to_string()],
        metadata,
    }
}

fn diode_schottky() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "forward_voltage".to_string(),
        ValueWithUnit::parse_with_default("0.38", EngineeringUnit::Volt).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "reverse_voltage".to_string(),
        ValueWithUnit::parse_with_default("40", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "forward_current".to_string(),
        ValueWithUnit::parse_with_default("1", EngineeringUnit::Ampere).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("package".to_string(), "SOD-123".to_string());
    metadata.insert("type".to_string(), "Schottky barrier".to_string());
    ComponentDefinition {
        id: "diode_schottky_ss14".to_string(),
        name: "SS14 Schottky Diode".to_string(),
        category: ComponentCategory::Diode.to_string(),
        manufacturer: Some("Generic".to_string()),
        part_number: Some("SS14".to_string()),
        description: Some("Schottky barrier rectifier 40V 1A SOD-123".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["diode".to_string()],
        footprint_ids: vec!["do_41_diode_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["semiconductor".to_string(), "diode".to_string(), "schottky".to_string()],
        metadata,
    }
}

fn bjt_2n2222() -> ComponentDefinition {
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
    metadata.insert("hfe_typ".to_string(), "200".to_string());
    metadata.insert("hfe_min".to_string(), "100".to_string());
    metadata.insert("package".to_string(), "TO-92".to_string());
    metadata.insert("type".to_string(), "NPN switching transistor".to_string());
    ComponentDefinition {
        id: "bjt_2n2222".to_string(),
        name: "2N2222 NPN Transistor".to_string(),
        category: ComponentCategory::Bjt.to_string(),
        manufacturer: Some("ON Semiconductor".to_string()),
        part_number: Some("2N2222A".to_string()),
        description: Some("NPN BJT 40V 600mA 0.5W TO-92 hFE=100–300".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["bjt_npn".to_string()],
        footprint_ids: vec!["to_92_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["semiconductor".to_string(), "transistor".to_string(), "bjt".to_string(), "npn".to_string()],
        metadata,
    }
}

fn bjt_2n2907() -> ComponentDefinition {
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
        ValueWithUnit::parse_with_default("0.4", EngineeringUnit::Watt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("hfe_typ".to_string(), "200".to_string());
    metadata.insert("hfe_min".to_string(), "100".to_string());
    metadata.insert("package".to_string(), "TO-92".to_string());
    metadata.insert("type".to_string(), "PNP switching transistor".to_string());
    ComponentDefinition {
        id: "bjt_2n2907".to_string(),
        name: "2N2907 PNP Transistor".to_string(),
        category: ComponentCategory::Bjt.to_string(),
        manufacturer: Some("ON Semiconductor".to_string()),
        part_number: Some("2N2907A".to_string()),
        description: Some("PNP BJT 40V 600mA 0.4W TO-92 hFE=100–300".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["bjt_pnp".to_string()],
        footprint_ids: vec!["to_92_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["semiconductor".to_string(), "transistor".to_string(), "bjt".to_string(), "pnp".to_string()],
        metadata,
    }
}

fn mosfet_irfz44n() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "vds_max".to_string(),
        ValueWithUnit::parse_with_default("55", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "id_max".to_string(),
        ValueWithUnit::parse_with_default("49", EngineeringUnit::Ampere).unwrap(),
    );
    parameters.insert(
        "rds_on".to_string(),
        ValueWithUnit::parse_with_default("17.5m", EngineeringUnit::Ohm).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("package".to_string(), "TO-220".to_string());
    metadata.insert("type".to_string(), "N-channel power MOSFET".to_string());
    metadata.insert("Qg".to_string(), "63 nC".to_string());
    metadata.insert("Ciss".to_string(), "1470 pF".to_string());
    metadata.insert("Coss".to_string(), "360 pF".to_string());
    metadata.insert("SOA".to_string(), "See datasheet".to_string());
    ComponentDefinition {
        id: "mosfet_irfz44n".to_string(),
        name: "IRFZ44N Power N-MOSFET".to_string(),
        category: ComponentCategory::Mosfet.to_string(),
        manufacturer: Some("Infineon".to_string()),
        part_number: Some("IRFZ44NPBF".to_string()),
        description: Some("N-channel power MOSFET 55V 49A 17.5mΩ TO-220".to_string()),
        parameters,
        ratings: BTreeMap::new(),
        symbol_ids: vec!["mosfet_n".to_string()],
        footprint_ids: vec!["to_220_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["semiconductor".to_string(), "transistor".to_string(), "mosfet".to_string(), "n-channel".to_string(), "power".to_string()],
        metadata,
    }
}

fn op_amp_lm358() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "gbw".to_string(),
        ValueWithUnit::parse_with_default("1M", EngineeringUnit::Hertz).unwrap(),
    );
    parameters.insert(
        "input_offset_voltage".to_string(),
        ValueWithUnit::parse_with_default("2m", EngineeringUnit::Volt).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "supply_min".to_string(),
        ValueWithUnit::parse_with_default("3", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "supply_max".to_string(),
        ValueWithUnit::parse_with_default("32", EngineeringUnit::Volt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("slew_rate".to_string(), "0.3 V/us".to_string());
    metadata.insert("input_bias_current".to_string(), "45 nA".to_string());
    metadata.insert("package".to_string(), "SOIC-8".to_string());
    metadata.insert("type".to_string(), "Dual general-purpose op-amp".to_string());
    metadata.insert("channels".to_string(), "2".to_string());
    ComponentDefinition {
        id: "op_amp_lm358".to_string(),
        name: "LM358 Op-Amp".to_string(),
        category: ComponentCategory::OpAmp.to_string(),
        manufacturer: Some("Texas Instruments".to_string()),
        part_number: Some("LM358DR".to_string()),
        description: Some("Dual op-amp 1MHz GBW 0.3V/us 3–32V SOIC-8".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["op_amp".to_string()],
        footprint_ids: vec!["soic8_placeholder".to_string()],
        simulation_models: vec![SimulationModel {
            id: "lm358_spice_model".to_string(),
            model_type: "spice".to_string(),
            source_path: None,
            raw_model: None,
            raw_model_id: None,
            pin_mapping: BTreeMap::new(),
        }],
        datasheets: vec![],
        tags: vec!["active".to_string(), "opamp".to_string(), "dual".to_string()],
        metadata,
    }
}

fn op_amp_rail_rail_placeholder() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "gbw".to_string(),
        ValueWithUnit::parse_with_default("5M", EngineeringUnit::Hertz).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "supply_min".to_string(),
        ValueWithUnit::parse_with_default("2.7", EngineeringUnit::Volt).unwrap(),
    );
    ratings.insert(
        "supply_max".to_string(),
        ValueWithUnit::parse_with_default("5.5", EngineeringUnit::Volt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("slew_rate".to_string(), "5 V/us".to_string());
    metadata.insert("package".to_string(), "SOT-23-5".to_string());
    metadata.insert("type".to_string(), "Rail-to-rail op-amp".to_string());
    metadata.insert("placeholder".to_string(), "true".to_string());
    ComponentDefinition {
        id: "op_amp_rail_rail".to_string(),
        name: "Rail-to-Rail Op-Amp (placeholder)".to_string(),
        category: ComponentCategory::OpAmp.to_string(),
        manufacturer: None,
        part_number: None,
        description: Some("Rail-to-rail op-amp placeholder — replace with real part".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["op_amp".to_string()],
        footprint_ids: vec!["sot23_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["active".to_string(), "opamp".to_string(), "placeholder".to_string()],
        metadata,
    }
}

fn ldo_ams1117() -> ComponentDefinition {
    let mut parameters = BTreeMap::new();
    parameters.insert(
        "output_voltage".to_string(),
        ValueWithUnit::parse_with_default("3.3", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "dropout_voltage".to_string(),
        ValueWithUnit::parse_with_default("1.15", EngineeringUnit::Volt).unwrap(),
    );
    parameters.insert(
        "max_current".to_string(),
        ValueWithUnit::parse_with_default("1", EngineeringUnit::Ampere).unwrap(),
    );
    let mut ratings = BTreeMap::new();
    ratings.insert(
        "input_voltage_max".to_string(),
        ValueWithUnit::parse_with_default("15", EngineeringUnit::Volt).unwrap(),
    );
    let mut metadata = BTreeMap::new();
    metadata.insert("package".to_string(), "SOT-223".to_string());
    metadata.insert("type".to_string(), "Fixed LDO voltage regulator".to_string());
    metadata.insert("psrr".to_string(), "70 dB".to_string());
    metadata.insert("line_reg".to_string(), "0.2%".to_string());
    ComponentDefinition {
        id: "ldo_ams1117_3v3".to_string(),
        name: "AMS1117-3.3 LDO Regulator".to_string(),
        category: ComponentCategory::VoltageRegulator.to_string(),
        manufacturer: Some("Advanced Monolithic Systems".to_string()),
        part_number: Some("AMS1117-3.3".to_string()),
        description: Some("LDO 3.3V 1A 1.15V dropout SOT-223".to_string()),
        parameters,
        ratings,
        symbol_ids: vec!["voltage_source".to_string()],
        footprint_ids: vec!["to_220_placeholder".to_string()],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec!["active".to_string(), "regulator".to_string(), "ldo".to_string(), "power".to_string()],
        metadata,
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
