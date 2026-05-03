use hotsas_core::{
    EngineeringNotebook, NotebookBlock, NotebookBlockKind, NotebookEvaluationResult,
    NotebookEvaluationStatus, NotebookHistoryEntry,
};

#[test]
fn engineering_notebook_serializes_and_deserializes() {
    let notebook = EngineeringNotebook::new("nb-1", "Test Notebook");
    let json = serde_json::to_string(&notebook).expect("serialize");
    let restored: EngineeringNotebook = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(notebook.id, restored.id);
    assert_eq!(notebook.title, restored.title);
}

#[test]
fn notebook_block_stores_result() {
    let result = NotebookEvaluationResult::unsupported("test", "not supported");
    let block = NotebookBlock {
        id: "blk-1".to_string(),
        kind: NotebookBlockKind::Expression,
        input: "test".to_string(),
        result: Some(result),
        created_at: None,
        updated_at: None,
    };
    assert!(block.result.is_some());
    assert_eq!(block.input, "test");
}

#[test]
fn notebook_evaluation_result_stores_outputs() {
    use hotsas_core::{EngineeringUnit, ValueWithUnit};
    use std::collections::BTreeMap;

    let mut outputs = BTreeMap::new();
    outputs.insert(
        "fc".to_string(),
        ValueWithUnit::parse_with_default("159.15Hz", EngineeringUnit::Hertz).unwrap(),
    );
    let result = NotebookEvaluationResult {
        input: "rc_low_pass_cutoff(R=10k, C=100n)".to_string(),
        status: NotebookEvaluationStatus::Success,
        kind: NotebookBlockKind::FormulaCall,
        outputs,
        variables: BTreeMap::new(),
        message: None,
        warnings: vec![],
    };
    assert_eq!(result.outputs.len(), 1);
    assert!(result.outputs.contains_key("fc"));
}

#[test]
fn notebook_history_entry_stores_status() {
    let entry = NotebookHistoryEntry {
        id: "hist-0".to_string(),
        input: "R = 10k".to_string(),
        result_summary: "R=10k".to_string(),
        status: NotebookEvaluationStatus::Success,
    };
    assert_eq!(entry.status, NotebookEvaluationStatus::Success);
    assert_eq!(entry.status.as_str(), "success");
}
