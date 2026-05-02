use hotsas_core::rc_low_pass_formula;

#[test]
fn rc_low_pass_formula_has_expected_identity_and_contract() {
    let formula = rc_low_pass_formula();

    assert_eq!(formula.id, "rc_low_pass_cutoff");
    assert_eq!(
        formula.linked_circuit_template_id.as_deref(),
        Some("rc_low_pass_template")
    );
    assert!(formula.variables.contains_key("R"), "formula must define R");
    assert!(formula.variables.contains_key("C"), "formula must define C");
    assert!(formula.outputs.contains_key("fc"), "formula must output fc");
    assert!(
        formula
            .equations
            .iter()
            .any(|equation| equation.expression.contains("fc")
                && equation.expression.contains('R')
                && equation.expression.contains('C')),
        "formula expression must mention fc, R, and C"
    );
}
