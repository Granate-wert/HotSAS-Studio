use hotsas_core::{
    ComponentPinMapping, ComponentPinRole, ModelParameterBinding, PersistedInstanceModelAssignment,
    PersistedModelAsset, PersistedModelAssetKind, PersistedModelAssetSource,
    PersistedModelAssetStatus, PersistedModelCatalog,
};

#[test]
fn model_catalog_roundtrip_preserves_spice_model() {
    let catalog = PersistedModelCatalog {
        assets: vec![PersistedModelAsset {
            id: "spice_1n4148".to_string(),
            name: "1N4148".to_string(),
            kind: PersistedModelAssetKind::SpiceModel,
            source: PersistedModelAssetSource::ImportedFile,
            source_file_name: Some("diodes.lib".to_string()),
            content_hash: Some("abc123".to_string()),
            package_asset_path: Some("models/spice/1n4148.json".to_string()),
            status: PersistedModelAssetStatus::Present,
            warnings: vec![],
            compatibility: Default::default(),
        }],
    };
    let json = serde_json::to_string(&catalog).unwrap();
    let restored: PersistedModelCatalog = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.assets.len(), 1);
    assert_eq!(restored.assets[0].id, "spice_1n4148");
    assert!(matches!(
        restored.assets[0].kind,
        PersistedModelAssetKind::SpiceModel
    ));
}

#[test]
fn model_catalog_roundtrip_preserves_subcircuit() {
    let catalog = PersistedModelCatalog {
        assets: vec![PersistedModelAsset {
            id: "subckt_opamp".to_string(),
            name: "TL072".to_string(),
            kind: PersistedModelAssetKind::SpiceSubcircuit,
            source: PersistedModelAssetSource::ImportedFile,
            source_file_name: Some("opamps.lib".to_string()),
            content_hash: None,
            package_asset_path: None,
            status: PersistedModelAssetStatus::Present,
            warnings: vec!["macro model".to_string()],
            compatibility: Default::default(),
        }],
    };
    let json = serde_json::to_string(&catalog).unwrap();
    let restored: PersistedModelCatalog = serde_json::from_str(&json).unwrap();
    assert!(matches!(
        restored.assets[0].kind,
        PersistedModelAssetKind::SpiceSubcircuit
    ));
}

#[test]
fn model_catalog_roundtrip_preserves_touchstone_dataset() {
    let catalog = PersistedModelCatalog {
        assets: vec![PersistedModelAsset {
            id: "s2p_filter".to_string(),
            name: "Filter.s2p".to_string(),
            kind: PersistedModelAssetKind::TouchstoneDataset,
            source: PersistedModelAssetSource::ImportedFile,
            source_file_name: Some("Filter.s2p".to_string()),
            content_hash: Some("def456".to_string()),
            package_asset_path: Some("models/touchstone/filter.s2p".to_string()),
            status: PersistedModelAssetStatus::Present,
            warnings: vec![],
            compatibility: Default::default(),
        }],
    };
    let json = serde_json::to_string(&catalog).unwrap();
    let restored: PersistedModelCatalog = serde_json::from_str(&json).unwrap();
    assert!(matches!(
        restored.assets[0].kind,
        PersistedModelAssetKind::TouchstoneDataset
    ));
}

#[test]
fn instance_assignment_roundtrip_preserves_model_ref() {
    let assignment = PersistedInstanceModelAssignment {
        instance_id: "R1".to_string(),
        component_definition_id: "resistor".to_string(),
        model_asset_id: "spice_1n4148".to_string(),
        pin_mappings: vec![],
        parameter_bindings: vec![],
    };
    let json = serde_json::to_string(&assignment).unwrap();
    let restored: PersistedInstanceModelAssignment = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.instance_id, "R1");
    assert_eq!(restored.model_asset_id, "spice_1n4148");
}

#[test]
fn pin_mapping_roundtrip_preserves_model_pin_order() {
    let assignment = PersistedInstanceModelAssignment {
        instance_id: "U1".to_string(),
        component_definition_id: "op_amp".to_string(),
        model_asset_id: "subckt_opamp".to_string(),
        pin_mappings: vec![
            ComponentPinMapping {
                component_pin_id: "in+".to_string(),
                model_pin_name: "IN_P".to_string(),
                model_pin_index: Some(0),
                role: Some(ComponentPinRole::Positive),
                required: true,
            },
            ComponentPinMapping {
                component_pin_id: "in-".to_string(),
                model_pin_name: "IN_N".to_string(),
                model_pin_index: Some(1),
                role: Some(ComponentPinRole::Negative),
                required: true,
            },
        ],
        parameter_bindings: vec![],
    };
    let json = serde_json::to_string(&assignment).unwrap();
    let restored: PersistedInstanceModelAssignment = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.pin_mappings.len(), 2);
    assert_eq!(restored.pin_mappings[0].model_pin_index, Some(0));
    assert_eq!(restored.pin_mappings[1].model_pin_name, "IN_N");
}

#[test]
fn parameter_binding_roundtrip_preserves_expression() {
    let assignment = PersistedInstanceModelAssignment {
        instance_id: "R1".to_string(),
        component_definition_id: "resistor".to_string(),
        model_asset_id: "builtin".to_string(),
        pin_mappings: vec![],
        parameter_bindings: vec![ModelParameterBinding {
            model_parameter_name: "resistance".to_string(),
            component_parameter_id: "resistance".to_string(),
            value_expression: Some("10k".to_string()),
            required: true,
        }],
    };
    let json = serde_json::to_string(&assignment).unwrap();
    let restored: PersistedInstanceModelAssignment = serde_json::from_str(&json).unwrap();
    assert_eq!(
        restored.parameter_bindings[0].value_expression,
        Some("10k".to_string())
    );
}
