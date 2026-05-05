use hotsas_core::{
    built_in_component_library, built_in_footprints, schema_for_category, seed_symbol_for_kind,
    ComponentParameterKind, ComponentTolerance,
};

#[test]
fn built_in_library_has_at_least_23_components() {
    let lib = built_in_component_library();
    assert!(
        lib.components.len() >= 23,
        "expected at least 23 components, got {}",
        lib.components.len()
    );
}

#[test]
fn generic_resistor_exists() {
    let lib = built_in_component_library();
    assert!(lib.components.iter().any(|c| c.id == "generic_resistor"));
}

#[test]
fn generic_capacitor_exists() {
    let lib = built_in_component_library();
    assert!(lib.components.iter().any(|c| c.id == "generic_capacitor"));
}

#[test]
fn generic_op_amp_exists() {
    let lib = built_in_component_library();
    assert!(lib.components.iter().any(|c| c.id == "generic_op_amp"));
}

#[test]
fn every_component_has_id_name_category() {
    let lib = built_in_component_library();
    for c in &lib.components {
        assert!(!c.id.is_empty(), "component id must not be empty");
        assert!(!c.name.is_empty(), "component name must not be empty");
        assert!(
            !c.category.is_empty(),
            "component category must not be empty"
        );
    }
}

#[test]
fn every_component_has_at_least_one_tag() {
    let lib = built_in_component_library();
    for c in &lib.components {
        assert!(
            !c.tags.is_empty(),
            "component {} must have at least one tag",
            c.id
        );
    }
}

#[test]
fn resistor_has_resistance_parameter() {
    let lib = built_in_component_library();
    let resistor = lib
        .components
        .iter()
        .find(|c| c.id == "generic_resistor")
        .expect("generic_resistor must exist");
    assert!(resistor.parameters.contains_key("resistance"));
}

#[test]
fn capacitor_has_capacitance_parameter() {
    let lib = built_in_component_library();
    let capacitor = lib
        .components
        .iter()
        .find(|c| c.id == "generic_capacitor")
        .expect("generic_capacitor must exist");
    assert!(capacitor.parameters.contains_key("capacitance"));
}

#[test]
fn op_amp_has_symbol_id() {
    let lib = built_in_component_library();
    let op_amp = lib
        .components
        .iter()
        .find(|c| c.id == "generic_op_amp")
        .expect("generic_op_amp must exist");
    assert!(!op_amp.symbol_ids.is_empty());
}

#[test]
fn footprints_exist_for_common_packages() {
    let fps = built_in_footprints();
    assert!(!fps.is_empty());
    let ids: Vec<_> = fps.iter().map(|f| f.id.clone()).collect();
    assert!(ids.contains(&"axial_resistor_placeholder".to_string()));
    assert!(ids.contains(&"soic8_placeholder".to_string()));
}

#[test]
fn component_ids_are_unique() {
    let lib = built_in_component_library();
    let mut seen = std::collections::HashSet::new();
    for c in &lib.components {
        assert!(
            seen.insert(c.id.clone()),
            "duplicate component id: {}",
            c.id
        );
    }
}

#[test]
fn symbol_ids_referenced_by_components_exist_in_seed_symbols() {
    let lib = built_in_component_library();
    for c in &lib.components {
        for symbol_id in &c.symbol_ids {
            assert!(
                seed_symbol_for_kind(symbol_id).is_some(),
                "symbol id '{}' referenced by component '{}' does not exist in seed symbols",
                symbol_id,
                c.id
            );
        }
    }
}

#[test]
fn footprint_ids_referenced_by_components_exist_in_library() {
    let lib = built_in_component_library();
    let footprint_ids: std::collections::HashSet<_> =
        lib.footprints.iter().map(|f| f.id.clone()).collect();
    for c in &lib.components {
        for fp_id in &c.footprint_ids {
            assert!(
                footprint_ids.contains(fp_id),
                "footprint id '{}' referenced by component '{}' does not exist in library",
                fp_id,
                c.id
            );
        }
    }
}

#[test]
fn smd_footprints_exist() {
    let fps = built_in_footprints();
    let ids: Vec<_> = fps.iter().map(|f| f.id.clone()).collect();
    assert!(ids.contains(&"smd_0603_placeholder".to_string()));
    assert!(ids.contains(&"smd_0805_placeholder".to_string()));
}

#[test]
fn real_like_resistor_10k_0603_exists() {
    let lib = built_in_component_library();
    let r = lib
        .components
        .iter()
        .find(|c| c.id == "resistor_10k_0603")
        .expect("resistor_10k_0603 must exist");
    assert_eq!(r.category, "resistor");
    assert!(r.parameters.contains_key("resistance"));
    assert!(r.metadata.contains_key("package"));
}

#[test]
fn real_like_capacitor_100n_0603_exists() {
    let lib = built_in_component_library();
    let c = lib
        .components
        .iter()
        .find(|c| c.id == "capacitor_100n_0603")
        .expect("capacitor_100n_0603 must exist");
    assert_eq!(c.category, "capacitor");
    assert!(c.parameters.contains_key("capacitance"));
    assert!(c.metadata.contains_key("dielectric"));
}

#[test]
fn ldo_ams1117_exists() {
    let lib = built_in_component_library();
    let ldo = lib
        .components
        .iter()
        .find(|c| c.id == "ldo_ams1117_3v3")
        .expect("ldo_ams1117_3v3 must exist");
    assert_eq!(ldo.category, "voltage_regulator");
    assert!(ldo.parameters.contains_key("output_voltage"));
}

#[test]
fn resistor_schema_has_primary_resistance() {
    let schema = schema_for_category("Resistor").expect("resistor schema must exist");
    let def = schema.get_definition("resistance").expect("resistance def must exist");
    assert_eq!(def.kind, ComponentParameterKind::Primary);
    assert!(def.required);
}

#[test]
fn capacitor_schema_validates_map() {
    let schema = schema_for_category("Capacitor").expect("capacitor schema must exist");
    let mut map = std::collections::BTreeMap::new();
    map.insert(
        "capacitance".to_string(),
        hotsas_core::ValueWithUnit::parse_with_default("100n", hotsas_core::EngineeringUnit::Farad)
            .unwrap(),
    );
    let errors = schema.validate_map(&map);
    assert!(errors.is_empty(), "expected no errors, got {:?}", errors);
}

#[test]
fn mosfet_irfz44n_exists_with_rds_on() {
    let lib = built_in_component_library();
    let m = lib
        .components
        .iter()
        .find(|c| c.id == "mosfet_irfz44n")
        .expect("mosfet_irfz44n must exist");
    assert!(m.parameters.contains_key("rds_on"));
    assert!(m.metadata.contains_key("SOA"));
}

#[test]
fn diode_1n4148_exists() {
    let lib = built_in_component_library();
    let d = lib
        .components
        .iter()
        .find(|c| c.id == "diode_1n4148")
        .expect("diode_1n4148 must exist");
    assert!(d.parameters.contains_key("forward_voltage"));
}

#[test]
fn bjt_2n2222_exists() {
    let lib = built_in_component_library();
    let b = lib
        .components
        .iter()
        .find(|c| c.id == "bjt_2n2222")
        .expect("bjt_2n2222 must exist");
    assert!(b.parameters.contains_key("vce_max"));
    assert!(b.metadata.contains_key("hfe_typ"));
}

#[test]
fn op_amp_lm358_has_simulation_model() {
    let lib = built_in_component_library();
    let op = lib
        .components
        .iter()
        .find(|c| c.id == "op_amp_lm358")
        .expect("op_amp_lm358 must exist");
    assert!(!op.simulation_models.is_empty());
}

#[test]
fn tolerance_bounds_work_correctly() {
    let tol = ComponentTolerance::SymmetricPercent { value: 1.0 };
    let (lo, hi) = tol.bounds(1_000.0);
    assert!((lo - 990.0).abs() < 0.01);
    assert!((hi - 1_010.0).abs() < 0.01);
}
