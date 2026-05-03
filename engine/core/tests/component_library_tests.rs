use hotsas_core::{built_in_component_library, built_in_footprints, seed_symbol_for_kind};

#[test]
fn built_in_library_has_at_least_12_components() {
    let lib = built_in_component_library();
    assert!(
        lib.components.len() >= 12,
        "expected at least 12 components, got {}",
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
