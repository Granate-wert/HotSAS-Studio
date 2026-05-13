use hotsas_api::{
    AddComponentRequestDto, DeleteWireRequestDto, HotSasApi, PlaceComponentRequestDto,
};
use hotsas_application::AppServices;
use hotsas_core::{CircuitProject, ProjectPackageManifest};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, FormulaEnginePort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, StoragePort,
};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Default)]
struct FakeComponentLibraryStorage;

impl hotsas_ports::ComponentLibraryPort for FakeComponentLibraryStorage {
    fn load_builtin_library(
        &self,
    ) -> Result<hotsas_core::ComponentLibrary, hotsas_ports::PortError> {
        Ok(hotsas_core::built_in_component_library())
    }
    fn load_library_from_path(
        &self,
        _path: &std::path::Path,
    ) -> Result<hotsas_core::ComponentLibrary, hotsas_ports::PortError> {
        Err(hotsas_ports::PortError::Storage(
            "not implemented".to_string(),
        ))
    }
    fn save_library_to_path(
        &self,
        _path: &std::path::Path,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<(), hotsas_ports::PortError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(
        &self,
        _path: &Path,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<hotsas_core::CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &std::path::Path,
        _project: &CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, PortError> {
        Ok(ProjectPackageManifest::new(
            "test".to_string(),
            "Test".to_string(),
            "now".to_string(),
            "now".to_string(),
        ))
    }

    fn load_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }

    fn validate_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::ProjectPackageValidationReport, PortError> {
        Ok(hotsas_core::ProjectPackageValidationReport {
            valid: true,
            package_dir: "".to_string(),
            missing_files: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }

    fn save_model_catalog(
        &self,
        _package_dir: &std::path::Path,
        _catalog: &hotsas_core::PersistedModelCatalog,
    ) -> Result<(), PortError> {
        Ok(())
    }

    fn load_model_catalog(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::PersistedModelCatalog, PortError> {
        Ok(Default::default())
    }

    fn save_model_assignments(
        &self,
        _package_dir: &std::path::Path,
        _assignments: &[hotsas_core::PersistedInstanceModelAssignment],
    ) -> Result<(), PortError> {
        Ok(())
    }

    fn load_model_assignments(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<Vec<hotsas_core::PersistedInstanceModelAssignment>, PortError> {
        Ok(vec![])
    }
}

#[derive(Debug, Default)]
struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &hotsas_core::ValueWithUnit,
        _capacitance: &hotsas_core::ValueWithUnit,
    ) -> Result<hotsas_core::ValueWithUnit, PortError> {
        Err(PortError::Formula("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Ok("* SPICE netlist\nR1 1 2 10k\nC1 2 0 100n\nV1 1 0 DC 5\n".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn engine_name(&self) -> &str {
        "fake"
    }
    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
    fn export_html(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
    fn export_bom_json(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

fn fake_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
        Arc::new(FakeComponentLibraryStorage),
        Arc::new(FakeBomExporter),
        Arc::new(FakeSimulationDataExporter),
        Arc::new(FakeComponentLibraryExporter),
        Arc::new(FakeSchematicExporter),
        Arc::new(FakeSpiceParser),
        Arc::new(FakeTouchstoneParser),
    ))
}

#[test]
fn get_selected_component_r1_returns_parameters() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let selected = api.get_selected_component("R1".to_string()).unwrap();
    assert_eq!(selected.instance_id, "R1");
    assert_eq!(selected.component_kind, "generic_resistor");
    assert!(selected.parameters.iter().any(|p| p.name == "resistance"));
    assert!(selected.symbol.is_some());
}

#[test]
fn get_selected_component_missing_id_returns_error() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.get_selected_component("MISSING".to_string());
    assert!(result.is_err());
}

#[test]
fn update_component_parameter_r1_resistance_changes_project() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let updated = api
        .update_component_parameter(
            "R1".to_string(),
            "resistance".to_string(),
            "4.7k".to_string(),
            Some("Ohm".to_string()),
        )
        .unwrap();
    let r1 = updated
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    let resistance = r1
        .parameters
        .iter()
        .find(|p| p.name == "resistance")
        .unwrap();
    assert!(resistance.value.display.contains("4700"));
}

#[test]
fn update_component_parameter_invalid_value_returns_error() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.update_component_parameter(
        "R1".to_string(),
        "resistance".to_string(),
        "invalid".to_string(),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn validate_current_circuit_returns_report() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let report = api.validate_current_circuit().unwrap();
    assert!(report.valid);
    assert!(report.errors.is_empty());
}

struct FakeSpiceParser;

impl hotsas_ports::SpiceModelParserPort for FakeSpiceParser {
    fn parse_spice_models_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::SpiceImportReport, hotsas_ports::PortError> {
        Ok(hotsas_core::SpiceImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            source: hotsas_core::ImportedModelSource {
                file_name: None,
                file_path: None,
                source_format: "spice".to_string(),
                content_hash: None,
            },
            models: vec![],
            subcircuits: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }
}

struct FakeTouchstoneParser;

impl hotsas_ports::TouchstoneParserPort for FakeTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::TouchstoneImportReport, hotsas_ports::PortError> {
        Ok(hotsas_core::TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: None,
            warnings: vec![],
            errors: vec![],
        })
    }
}

#[test]
fn list_placeable_components_returns_real_library_items() {
    let api = fake_api();
    let components = api.list_placeable_components().unwrap();
    assert!(!components.is_empty());
    assert!(components
        .iter()
        .any(|c| c.definition_id == "generic_resistor"));
}

#[test]
fn place_component_adds_instance_at_position() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let before = api.get_current_project().unwrap();
    let count_before = before.schematic.components.len();

    let result = api.place_schematic_component(PlaceComponentRequestDto {
        component_definition_id: "generic_capacitor".to_string(),
        x: 350.0,
        y: 450.0,
        rotation_deg: 0.0,
    });
    assert!(result.is_ok());
    let after = api.get_current_project().unwrap();
    assert_eq!(after.schematic.components.len(), count_before + 1);
    let placed = after
        .schematic
        .components
        .iter()
        .find(|c| c.x == 350.0 && c.y == 450.0);
    assert!(placed.is_some());
}

#[test]
fn place_component_marks_project_dirty() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    // Save clears dirty
    api.save_project_as("D:\\test\\project.circuit".to_string())
        .unwrap();
    let state_saved = api.get_project_session_state().unwrap();
    assert!(!state_saved.dirty);

    api.place_schematic_component(PlaceComponentRequestDto {
        component_definition_id: "generic_capacitor".to_string(),
        x: 100.0,
        y: 100.0,
        rotation_deg: 0.0,
    })
    .unwrap();

    let state_after = api.get_project_session_state().unwrap();
    assert!(state_after.dirty);
}

#[test]
fn delete_wire_removes_connection_and_updates_net() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let before = api.get_current_project().unwrap();
    let wire_id = before.schematic.wires[0].id.clone();

    let result = api.delete_schematic_wire(DeleteWireRequestDto {
        wire_id: wire_id.clone(),
    });
    assert!(result.is_ok());
    let after = api.get_current_project().unwrap();
    assert!(!after.schematic.wires.iter().any(|w| w.id == wire_id));
}

#[test]
fn undo_after_add_component_removes_component() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let before = api.get_current_project().unwrap();
    let count_before = before.schematic.components.len();

    api.place_schematic_component(PlaceComponentRequestDto {
        component_definition_id: "generic_capacitor".to_string(),
        x: 100.0,
        y: 100.0,
        rotation_deg: 0.0,
    })
    .unwrap();

    let after_place = api.get_current_project().unwrap();
    assert_eq!(after_place.schematic.components.len(), count_before + 1);

    let undone = api.undo_schematic_edit().unwrap();
    assert_eq!(undone.schematic.components.len(), count_before);
}

#[test]
fn redo_after_undo_restores_component() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let before = api.get_current_project().unwrap();
    let count_before = before.schematic.components.len();

    api.place_schematic_component(PlaceComponentRequestDto {
        component_definition_id: "generic_capacitor".to_string(),
        x: 100.0,
        y: 100.0,
        rotation_deg: 0.0,
    })
    .unwrap();

    api.undo_schematic_edit().unwrap();
    let redone = api.redo_schematic_edit().unwrap();
    assert_eq!(redone.schematic.components.len(), count_before + 1);
}

#[test]
fn undo_after_connect_wire_removes_wire() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let before = api.get_current_project().unwrap();
    let wire_count_before = before.schematic.wires.len();

    api.connect_schematic_pins(hotsas_api::ConnectPinsRequestDto {
        from_component_id: "R1".to_string(),
        from_pin_id: "2".to_string(),
        to_component_id: "C1".to_string(),
        to_pin_id: "1".to_string(),
        net_name: None,
    })
    .unwrap();

    let after_connect = api.get_current_project().unwrap();
    assert_eq!(after_connect.schematic.wires.len(), wire_count_before + 1);

    let undone = api.undo_schematic_edit().unwrap();
    assert_eq!(undone.schematic.wires.len(), wire_count_before);
}

#[test]
fn netlist_preview_uses_backend_netlist_service() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let preview = api.generate_current_schematic_netlist_preview().unwrap();
    assert!(!preview.netlist.is_empty());
    assert!(
        preview.netlist.contains("R1")
            || preview.netlist.contains("C1")
            || preview.netlist.contains("V1")
    );
}

#[test]
fn get_schematic_selection_details_includes_definition_defaults() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let details = api
        .get_schematic_selection_details(hotsas_api::SchematicSelectionRequestDto {
            kind: "component".to_string(),
            id: "R1".to_string(),
        })
        .unwrap();
    assert_eq!(details.kind, "component");
    let resistance_field = details
        .editable_fields
        .iter()
        .find(|f| f.field_id == "resistance")
        .expect("resistance field should be present");
    assert_eq!(resistance_field.label, "Resistance");
    assert!(resistance_field.editable);
    assert_eq!(resistance_field.unit.as_deref(), Some("Ohm"));
}

#[test]
fn update_schematic_quick_parameter_updates_resistor_value() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api
        .update_schematic_quick_parameter(hotsas_api::UpdateQuickParameterRequestDto {
            component_id: "R1".to_string(),
            parameter_id: "resistance".to_string(),
            value: "4.7k".to_string(),
        })
        .unwrap();
    let r1 = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    let resistance = r1
        .parameters
        .iter()
        .find(|p| p.name == "resistance")
        .unwrap();
    assert_eq!(resistance.value.unit, "Ohm");
    assert!(resistance.value.si_value - 4700.0 < 0.1);
}

// v3.6-pre-fix2: newly placed palette components must have editable parameters

#[test]
fn add_schematic_component_capacitor_uses_generic_capacitor_definition_or_defaults() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .add_schematic_component(AddComponentRequestDto {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: None,
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let project = result.project;
    let cap = project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_capacitor")
        .expect("component should resolve to generic_capacitor definition");
    assert!(
        cap.parameters.iter().any(|p| p.name == "capacitance"),
        "newly placed capacitor should have capacitance parameter from definition defaults"
    );
}

#[test]
fn place_schematic_component_generic_capacitor_gets_default_parameters() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_capacitor".to_string(),
            x: 200.0,
            y: 200.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let project = result.project;
    let cap = project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_capacitor")
        .expect("component should have generic_capacitor definition_id");
    assert!(
        cap.parameters.iter().any(|p| p.name == "capacitance"),
        "newly placed generic_capacitor should have capacitance parameter from definition defaults"
    );
}

#[test]
fn get_selected_component_for_newly_placed_resistor_includes_resistance() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_resistor".to_string(),
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let placed = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_resistor")
        .unwrap();

    let selected = api
        .get_selected_component(placed.instance_id.clone())
        .unwrap();
    assert_eq!(selected.component_kind, "generic_resistor");
    let resistance = selected
        .parameters
        .iter()
        .find(|p| p.name == "resistance")
        .expect("Properties panel should show resistance for newly placed resistor");
    assert_eq!(resistance.unit.as_deref(), Some("Ohm"));
}

#[test]
fn get_selection_details_for_newly_placed_capacitor_includes_capacitance_field() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_capacitor".to_string(),
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let placed = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_capacitor")
        .unwrap();

    let details = api
        .get_schematic_selection_details(hotsas_api::SchematicSelectionRequestDto {
            kind: "component".to_string(),
            id: placed.instance_id.clone(),
        })
        .unwrap();

    let field = details
        .editable_fields
        .iter()
        .find(|f| f.field_id == "capacitance")
        .expect("Selection tab should show capacitance for newly placed capacitor");
    assert_eq!(field.label, "Capacitance");
    assert!(field.editable);
    assert_eq!(field.unit.as_deref(), Some("F"));
}

#[test]
fn get_selection_details_for_newly_placed_resistor_includes_resistance_field() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_resistor".to_string(),
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let placed = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_resistor")
        .unwrap();

    let details = api
        .get_schematic_selection_details(hotsas_api::SchematicSelectionRequestDto {
            kind: "component".to_string(),
            id: placed.instance_id.clone(),
        })
        .unwrap();

    let field = details
        .editable_fields
        .iter()
        .find(|f| f.field_id == "resistance")
        .expect("Selection tab should show resistance for newly placed resistor");
    assert_eq!(field.label, "Resistance");
    assert!(field.editable);
    assert_eq!(field.unit.as_deref(), Some("Ohm"));
}

#[test]
fn get_selection_details_for_newly_placed_voltage_source_includes_voltage_field() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_voltage_source".to_string(),
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let placed = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_voltage_source")
        .unwrap();

    let details = api
        .get_schematic_selection_details(hotsas_api::SchematicSelectionRequestDto {
            kind: "component".to_string(),
            id: placed.instance_id.clone(),
        })
        .unwrap();

    let field = details
        .editable_fields
        .iter()
        .find(|f| f.field_id == "voltage")
        .expect("Selection tab should show voltage for newly placed voltage source");
    assert_eq!(field.label, "Voltage");
    assert!(field.editable);
    assert_eq!(field.unit.as_deref(), Some("V"));
}

#[test]
fn update_newly_placed_capacitor_parameter_updates_override() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_capacitor".to_string(),
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let placed = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_capacitor")
        .unwrap();

    let updated = api
        .update_component_parameter(
            placed.instance_id.clone(),
            "capacitance".to_string(),
            "10u".to_string(),
            None,
        )
        .unwrap();

    let cap = updated
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == placed.instance_id)
        .unwrap();
    let param = cap
        .parameters
        .iter()
        .find(|p| p.name == "capacitance")
        .unwrap();
    assert_eq!(param.value.unit, "F");
    assert!(param.value.si_value - 1e-5 < 1e-10);
}

#[test]
fn save_load_preserves_updated_value_for_newly_placed_component() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let result = api
        .place_schematic_component(PlaceComponentRequestDto {
            component_definition_id: "generic_capacitor".to_string(),
            x: 100.0,
            y: 100.0,
            rotation_deg: 0.0,
        })
        .unwrap();

    let placed = result
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.definition_id == "generic_capacitor")
        .unwrap();

    // Update the value
    api.update_component_parameter(
        placed.instance_id.clone(),
        "capacitance".to_string(),
        "22u".to_string(),
        None,
    )
    .unwrap();

    // Save project
    api.save_project_as("D:\\test\\project.circuit".to_string())
        .unwrap();

    // Verify current project still has the updated value
    let project = api.get_current_project().unwrap();
    let cap = project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == placed.instance_id)
        .unwrap();
    let param = cap
        .parameters
        .iter()
        .find(|p| p.name == "capacitance")
        .unwrap();
    assert_eq!(param.value.original, "22u");
}
