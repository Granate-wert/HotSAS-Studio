use hotsas_application::{AppServices, ComponentModelMappingService, ModelImportService};
use hotsas_core::{
    ComponentModelAssignment, ComponentModelAssignmentStatus, PersistedInstanceModelAssignment,
    SimulationReadiness, SpiceModelReference, SpiceModelReferenceKind, SpiceModelSource,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ComponentLibraryPort, FormulaEnginePort,
    NetlistExporterPort, PortError, ProjectPackageStoragePort, ReportExporterPort,
    SchematicExporterPort, SimulationDataExporterPort, SimulationEnginePort, SpiceModelParserPort,
    StoragePort, TouchstoneParserPort,
};
use std::path::Path;
use std::sync::Arc;

fn fake_services() -> AppServices {
    AppServices::new(
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
    )
}

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
        _package_dir: &Path,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, PortError> {
        Ok(hotsas_core::ProjectPackageManifest::new(
            "test".to_string(),
            "Test".to_string(),
            "now".to_string(),
            "now".to_string(),
        ))
    }
    fn load_project_package(
        &self,
        _package_dir: &Path,
    ) -> Result<hotsas_core::CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
    fn validate_project_package(
        &self,
        _package_dir: &Path,
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
        _package_dir: &Path,
        _catalog: &hotsas_core::PersistedModelCatalog,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn load_model_catalog(
        &self,
        _package_dir: &Path,
    ) -> Result<hotsas_core::PersistedModelCatalog, PortError> {
        Ok(Default::default())
    }
    fn save_model_assignments(
        &self,
        _package_dir: &Path,
        _assignments: &[hotsas_core::PersistedInstanceModelAssignment],
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn load_model_assignments(
        &self,
        _package_dir: &Path,
    ) -> Result<Vec<hotsas_core::PersistedInstanceModelAssignment>, PortError> {
        Ok(vec![])
    }
}

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

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

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

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Ok("".to_string())
    }
    fn export_html(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

struct FakeComponentLibraryStorage;

impl ComponentLibraryPort for FakeComponentLibraryStorage {
    fn load_builtin_library(&self) -> Result<hotsas_core::ComponentLibrary, PortError> {
        Ok(hotsas_core::built_in_component_library())
    }
    fn load_library_from_path(
        &self,
        _path: &Path,
    ) -> Result<hotsas_core::ComponentLibrary, PortError> {
        Ok(hotsas_core::built_in_component_library())
    }
    fn save_library_to_path(
        &self,
        _path: &Path,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<(), PortError> {
        Ok(())
    }
}

struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
    fn export_bom_json(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

struct FakeSpiceParser;

impl SpiceModelParserPort for FakeSpiceParser {
    fn parse_spice_models_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::SpiceImportReport, PortError> {
        Ok(hotsas_core::SpiceImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            source: hotsas_core::ImportedModelSource {
                file_name: None,
                file_path: Some("".to_string()),
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

impl TouchstoneParserPort for FakeTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::TouchstoneImportReport, PortError> {
        Ok(hotsas_core::TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: None,
            warnings: vec![],
            errors: vec![],
        })
    }
}

#[test]
fn model_import_service_builds_catalog_from_imported_models() {
    let parser = Arc::new(FakeSpiceParser);
    let touchstone = Arc::new(FakeTouchstoneParser);
    let model_import = ModelImportService::new(parser, touchstone);

    // Initially empty
    let catalog = model_import.build_persisted_model_catalog().unwrap();
    assert_eq!(catalog.assets.len(), 0);
}

#[test]
fn component_model_mapping_builds_persisted_assignment() {
    let mapping = ComponentModelMappingService::new();
    let assignment = ComponentModelAssignment {
        component_definition_id: "resistor".to_string(),
        component_instance_id: Some("R1".to_string()),
        model_ref: Some(SpiceModelReference {
            id: "builtin_resistor_primitive".to_string(),
            display_name: "Resistor".to_string(),
            model_kind: SpiceModelReferenceKind::PrimitiveModel,
            source: SpiceModelSource::Builtin,
            status: ComponentModelAssignmentStatus::AssignedBuiltin,
            limitations: vec![],
            warnings: vec![],
        }),
        pin_mappings: vec![],
        parameter_bindings: vec![],
        status: ComponentModelAssignmentStatus::AssignedBuiltin,
        readiness: SimulationReadiness::ready(),
        diagnostics: vec![],
    };

    let persisted = mapping.build_persisted_instance_assignment(&assignment);
    assert!(persisted.is_some());
    let p = persisted.unwrap();
    assert_eq!(p.instance_id, "R1");
    assert_eq!(p.model_asset_id, "builtin_resistor_primitive");
}

#[test]
fn component_model_mapping_applies_persisted_assignments() {
    let mapping = ComponentModelMappingService::new();
    let mut project = hotsas_core::rc_low_pass_project();
    let persisted = vec![PersistedInstanceModelAssignment {
        instance_id: project.schematic.components[0].instance_id.clone(),
        component_definition_id: project.schematic.components[0].definition_id.clone(),
        model_asset_id: "custom_model".to_string(),
        pin_mappings: vec![],
        parameter_bindings: vec![],
    }];

    mapping.apply_persisted_assignments(&mut project, &persisted);
    assert_eq!(
        project.schematic.components[0].selected_simulation_model_id,
        Some("custom_model".to_string())
    );
}
