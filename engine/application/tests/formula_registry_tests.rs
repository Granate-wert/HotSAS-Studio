use hotsas_application::{ApplicationError, FormulaRegistryService};
use hotsas_core::{ohms_law_formula, rc_low_pass_formula, voltage_divider_formula, FormulaPack};

#[test]
fn registry_lists_formulas_categories_and_pack_metadata() {
    let registry =
        FormulaRegistryService::new(vec![pack("filters", vec![rc_low_pass_formula()])]).unwrap();

    let formulas = registry.list_formulas();
    let categories = registry.list_categories();
    let metadata = registry.get_pack_metadata();

    assert_eq!(formulas.len(), 1);
    assert_eq!(formulas[0].id, "rc_low_pass_cutoff");
    assert_eq!(categories, ["filters/passive"]);
    assert_eq!(metadata.len(), 1);
    assert_eq!(metadata[0].pack_id, "filters");
    assert_eq!(metadata[0].formula_count, 1);
}

#[test]
fn registry_finds_formula_by_id_category_and_linked_template() {
    let registry =
        FormulaRegistryService::new(vec![pack("filters", vec![rc_low_pass_formula()])]).unwrap();

    let formula = registry.get_formula("rc_low_pass_cutoff").unwrap();
    let by_category = registry.list_by_category("filters/passive");
    let linked_template = registry
        .get_linked_template_id("rc_low_pass_cutoff")
        .unwrap();

    assert_eq!(formula.title, "RC Low-Pass Cutoff Frequency");
    assert_eq!(by_category.len(), 1);
    assert_eq!(linked_template.as_deref(), Some("rc_low_pass_template"));
}

#[test]
fn registry_rejects_duplicate_formula_ids_and_missing_formulas() {
    let result = FormulaRegistryService::new(vec![pack(
        "filters",
        vec![rc_low_pass_formula(), rc_low_pass_formula()],
    )]);

    assert!(matches!(
        result.unwrap_err(),
        ApplicationError::DuplicateFormulaId(id) if id == "rc_low_pass_cutoff"
    ));

    let registry =
        FormulaRegistryService::new(vec![pack("filters", vec![rc_low_pass_formula()])]).unwrap();
    assert!(matches!(
        registry.get_formula("missing").unwrap_err(),
        ApplicationError::FormulaNotFound(id) if id == "missing"
    ));
}

#[test]
fn registry_finds_ohms_law_and_voltage_divider() {
    let registry = FormulaRegistryService::new(vec![pack(
        "basic_electronics",
        vec![ohms_law_formula(), voltage_divider_formula()],
    )])
    .unwrap();

    let ohms = registry.get_formula("ohms_law").unwrap();
    let divider = registry.get_formula("voltage_divider").unwrap();

    assert_eq!(ohms.title, "Ohm's Law");
    assert_eq!(ohms.category, "basic_electronics/passive");
    assert!(ohms.variables.contains_key("I"));
    assert!(ohms.variables.contains_key("R"));
    assert!(ohms.outputs.contains_key("V"));

    assert_eq!(divider.title, "Voltage Divider");
    assert_eq!(divider.category, "basic_electronics/passive");
    assert!(divider.variables.contains_key("Vin"));
    assert!(divider.variables.contains_key("R1"));
    assert!(divider.variables.contains_key("R2"));
    assert!(divider.outputs.contains_key("Vout"));

    let by_category = registry.list_by_category("basic_electronics/passive");
    assert_eq!(by_category.len(), 2);
}

#[test]
fn registry_validates_linked_template_bindings() {
    let registry =
        FormulaRegistryService::new(vec![pack("filters", vec![rc_low_pass_formula()])]).unwrap();

    registry
        .validate_formula_bindings(&["rc_low_pass_template".to_string()])
        .unwrap();
    assert!(matches!(
        registry.validate_formula_bindings(&[]).unwrap_err(),
        ApplicationError::InvalidBinding(message) if message.contains("rc_low_pass_template")
    ));
}

fn pack(id: &str, formulas: Vec<hotsas_core::FormulaDefinition>) -> FormulaPack {
    FormulaPack {
        pack_id: id.to_string(),
        title: id.to_string(),
        version: "0.1.0".to_string(),
        formulas,
    }
}
