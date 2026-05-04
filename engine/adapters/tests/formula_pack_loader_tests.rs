use hotsas_adapters::FormulaPackFileLoader;
use hotsas_core::EngineeringUnit;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn loads_filters_yaml_with_rc_low_pass_formula() {
    let loader = FormulaPackFileLoader;
    let pack = loader
        .load_pack_from_file(&formula_packs_dir().join("filters.yaml"))
        .unwrap();

    assert_eq!(pack.pack_id, "filters");
    assert_eq!(pack.title, "Filters");
    let formula = pack
        .formulas
        .iter()
        .find(|formula| formula.id == "rc_low_pass_cutoff")
        .unwrap();
    assert_eq!(formula.category, "filters/passive");
    assert_eq!(
        formula.linked_circuit_template_id.as_deref(),
        Some("rc_low_pass_template")
    );
    assert_eq!(formula.variables["R"].unit, EngineeringUnit::Ohm);
    assert_eq!(formula.variables["C"].unit, EngineeringUnit::Farad);
    assert_eq!(formula.outputs["fc"].unit, EngineeringUnit::Hertz);
    assert_eq!(
        formula
            .mapping
            .as_ref()
            .unwrap()
            .get("R")
            .map(String::as_str),
        Some("R1.resistance")
    );
}

#[test]
fn loads_all_builtin_formula_pack_files_from_directory_in_deterministic_order() {
    let loader = FormulaPackFileLoader;
    let packs = loader.load_pack_from_dir(&formula_packs_dir()).unwrap();
    let ids: Vec<_> = packs.iter().map(|pack| pack.pack_id.as_str()).collect();

    assert_eq!(
        ids,
        [
            "ac_impedance",
            "basic_electronics",
            "filters",
            "op_amp",
            "power_thermal",
            "smps",
            "transient",
            "utilities"
        ]
    );
    assert!(packs
        .iter()
        .any(|pack| pack.formulas.iter().any(|formula| formula.id == "ohms_law")));
    assert!(packs.iter().any(|pack| pack
        .formulas
        .iter()
        .any(|formula| formula.id == "voltage_divider")));
}

#[test]
fn loads_basic_electronics_yaml() {
    let loader = FormulaPackFileLoader;
    let pack = loader
        .load_pack_from_file(&formula_packs_dir().join("basic_electronics.yaml"))
        .unwrap();

    assert_eq!(pack.pack_id, "basic_electronics");
    assert_eq!(pack.title, "Basic Electronics");
    assert!(pack.formulas.iter().any(|f| f.id == "ohms_law"));
    assert!(pack.formulas.iter().any(|f| f.id == "voltage_divider"));
}

#[test]
fn loads_op_amp_yaml() {
    let loader = FormulaPackFileLoader;
    let pack = loader
        .load_pack_from_file(&formula_packs_dir().join("op_amp.yaml"))
        .unwrap();

    assert_eq!(pack.pack_id, "op_amp");
    assert_eq!(pack.title, "Operational Amplifiers");
    assert!(!pack.formulas.is_empty());
}

#[test]
fn loads_smps_yaml() {
    let loader = FormulaPackFileLoader;
    let pack = loader
        .load_pack_from_file(&formula_packs_dir().join("smps.yaml"))
        .unwrap();

    assert_eq!(pack.pack_id, "smps");
    assert_eq!(pack.title, "Switch-Mode Power Supplies");
    assert!(!pack.formulas.is_empty());
}

#[test]
fn loads_json_formula_pack() {
    let loader = FormulaPackFileLoader;
    let path = temp_path().join("pack.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        r#"{
  "packId": "json_pack",
  "title": "JSON Pack",
  "version": "0.1.0",
  "formulas": [{
    "id": "json_formula",
    "title": "JSON Formula",
    "category": "json/demo",
    "description": "Loaded from JSON.",
    "variables": { "X": { "unit": "", "description": "Input" } },
    "equations": [{ "id": "eq", "latex": "Y=X", "expression": "Y = X", "solve_for": ["Y"] }],
    "outputs": { "Y": { "unit": "", "description": "Output" } }
  }]
}"#,
    )
    .unwrap();

    let pack = loader.load_pack_from_file(&path).unwrap();

    assert_eq!(pack.pack_id, "json_pack");
    assert_eq!(pack.formulas[0].id, "json_formula");
}

#[test]
fn rejects_invalid_yaml_and_invalid_formula_packs() {
    let loader = FormulaPackFileLoader;
    let root = temp_path();
    fs::create_dir_all(&root).unwrap();

    let invalid_yaml = root.join("invalid.yaml");
    fs::write(&invalid_yaml, "packId: [").unwrap();
    assert!(loader.load_pack_from_file(&invalid_yaml).is_err());

    let missing_pack_id = root.join("missing_pack_id.yaml");
    fs::write(
        &missing_pack_id,
        r#"title: Missing Pack Id
version: 0.1.0
formulas:
  - id: formula
    title: Formula
    category: demo
    equations:
      - id: eq
        latex: Y=X
        expression: Y = X
        solve_for: [Y]
    variables:
      X:
        unit: ""
        description: Input
    outputs:
      Y:
        unit: ""
        description: Output
"#,
    )
    .unwrap();
    assert!(loader.load_pack_from_file(&missing_pack_id).is_err());

    let missing_outputs = root.join("missing_outputs.yaml");
    fs::write(
        &missing_outputs,
        r#"packId: invalid
title: Invalid
version: 0.1.0
formulas:
  - id: formula
    title: Formula
    category: demo
    equations:
      - id: eq
        latex: Y=X
        expression: Y = X
        solve_for: [Y]
    variables:
      X:
        unit: ""
        description: Input
"#,
    )
    .unwrap();
    assert!(loader.load_pack_from_file(&missing_outputs).is_err());
}

#[test]
fn rejects_formula_with_missing_id() {
    let loader = FormulaPackFileLoader;
    let root = temp_path();
    fs::create_dir_all(&root).unwrap();

    let missing_formula_id = root.join("missing_formula_id.yaml");
    fs::write(
        &missing_formula_id,
        r#"packId: test
version: 0.1.0
title: Test
formulas:
  - title: No Id
    category: demo
    equations:
      - id: eq
        latex: Y=X
        expression: Y = X
        solve_for: [Y]
    variables:
      X:
        unit: ""
        description: Input
    outputs:
      Y:
        unit: ""
        description: Output
"#,
    )
    .unwrap();
    assert!(loader.load_pack_from_file(&missing_formula_id).is_err());
}

#[test]
fn rejects_formula_with_no_equations() {
    let loader = FormulaPackFileLoader;
    let root = temp_path();
    fs::create_dir_all(&root).unwrap();

    let no_equations = root.join("no_equations.yaml");
    fs::write(
        &no_equations,
        r#"packId: test
version: 0.1.0
title: Test
formulas:
  - id: no_eq
    title: No Equations
    category: demo
    variables:
      X:
        unit: ""
        description: Input
    outputs:
      Y:
        unit: ""
        description: Output
"#,
    )
    .unwrap();
    assert!(loader.load_pack_from_file(&no_equations).is_err());
}

fn temp_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir()
        .join("hotsas-formula-pack-loader-tests")
        .join(timestamp.to_string())
}

fn formula_packs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../shared/formula_packs")
}
