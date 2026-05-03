use hotsas_adapters::{
    CircuitProjectPackageStorage, JsonComponentLibraryStorage, JsonProjectStorage,
    MarkdownReportExporter, MockSimulationEngine, SimpleFormulaEngine, SpiceNetlistExporter,
};
use hotsas_application::AppServices;
use hotsas_core::EngineeringUnit;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn backend_vertical_slice_runs_end_to_end_with_current_adapters() {
    let services = AppServices::new(
        Arc::new(JsonProjectStorage),
        Arc::new(CircuitProjectPackageStorage::default()),
        Arc::new(SimpleFormulaEngine),
        Arc::new(SpiceNetlistExporter),
        Arc::new(MockSimulationEngine),
        Arc::new(MarkdownReportExporter),
        Arc::new(JsonComponentLibraryStorage),
    );

    let project = services.create_rc_low_pass_demo_project();
    let cutoff = services.calculate_rc_low_pass_cutoff(&project).unwrap();
    let nearest = services.nearest_e24_for_resistor(&project).unwrap();
    let netlist = services.generate_spice_netlist(&project).unwrap();
    let simulation = services.run_mock_ac_simulation(&project).unwrap();
    let report = services.build_report_model(&project, &cutoff, &nearest, &netlist, &simulation);
    let markdown = services.export_markdown_report(&report).unwrap();
    let html = services.export_html_report(&report).unwrap();
    let save_path = temp_path().join("project.json");

    services.save_project(&save_path, &project).unwrap();
    let loaded = services.load_project(&save_path).unwrap();

    assert!(!project.id.is_empty(), "project id must not be empty");
    assert!(!project.name.is_empty(), "project name must not be empty");
    assert!(cutoff.si_value() > 0.0, "cutoff must be positive");
    assert_eq!(cutoff.unit, EngineeringUnit::Hertz);
    assert!(
        nearest.nearest.si_value() > 0.0,
        "nearest preferred value must be positive"
    );
    for fragment in ["V1", "R1", "C1", ".ac"] {
        assert!(
            netlist.contains(fragment),
            "netlist must contain fragment {fragment:?}"
        );
    }
    assert!(
        !simulation.graph_series.is_empty(),
        "simulation graph series must not be empty"
    );
    assert!(!markdown.is_empty(), "markdown report must not be empty");
    assert!(!html.is_empty(), "html report must not be empty");
    assert!(save_path.exists(), "saved project file must exist");
    assert_eq!(loaded.id, project.id);
}

fn temp_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir()
        .join("hotsas-vertical-slice")
        .join(timestamp.to_string())
}
