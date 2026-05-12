use hotsas_application::{AppServices, ProjectPackageService, SchematicEditingService};
use hotsas_core::{
    rc_low_pass_project, AddComponentRequest, CircuitProject, ConnectPinsRequest,
    ProjectPackageManifest, ProjectPackageValidationReport, ReportModel, SimulationProfile,
    SimulationResult, UpdateQuickParameterRequest, ValueWithUnit,
};
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

fn temp_package_dir() -> std::path::PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp = std::env::temp_dir().join(format!("hotsas_test_{timestamp}.circuit"));
    let _ = std::fs::remove_dir_all(&temp);
    temp
}

#[derive(Debug, Default)]
struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(&self, _path: &Path, _project: &CircuitProject) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
}

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Default)]
struct FakeProjectPackageStorage {
    projects: Mutex<HashMap<std::path::PathBuf, CircuitProject>>,
}

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        package_dir: &std::path::Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError> {
        let mut map = self.projects.lock().unwrap();
        map.insert(package_dir.to_path_buf(), project.clone());
        Ok(ProjectPackageManifest::new(
            project.id.clone(),
            project.name.clone(),
            "now".to_string(),
            "now".to_string(),
        ))
    }

    fn load_project_package(
        &self,
        package_dir: &std::path::Path,
    ) -> Result<CircuitProject, PortError> {
        let map = self.projects.lock().unwrap();
        map.get(package_dir)
            .cloned()
            .ok_or_else(|| PortError::Storage("not found".to_string()))
    }

    fn validate_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<ProjectPackageValidationReport, PortError> {
        Ok(ProjectPackageValidationReport {
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
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        Err(PortError::Formula("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
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
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
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

#[test]
fn project_package_service_save_load_roundtrip() {
    let dir = temp_package_dir();
    let storage =
        Arc::new(FakeProjectPackageStorage::default()) as Arc<dyn ProjectPackageStoragePort>;
    let service = ProjectPackageService::new(storage);
    let project = rc_low_pass_project();

    let manifest = service.save_project_package(&dir, &project).unwrap();
    assert_eq!(manifest.project_id, project.id);

    let loaded = service.load_project_package(&dir).unwrap();
    assert_eq!(loaded.id, project.id);
    assert_eq!(loaded.name, project.name);
}

#[test]
fn app_services_exposes_project_package_service() {
    let services = fake_services();
    let dir = temp_package_dir();
    let project = rc_low_pass_project();

    let manifest = services.save_project_package(&dir, &project).unwrap();
    assert!(!manifest.project_id.is_empty());
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
fn save_load_roundtrip_preserves_interactive_edits() {
    let dir = temp_package_dir();
    let storage =
        Arc::new(FakeProjectPackageStorage::default()) as Arc<dyn ProjectPackageStoragePort>;
    let service = ProjectPackageService::new(storage);
    let mut project = rc_low_pass_project();
    let edit_svc = SchematicEditingService::new();

    // Interactive edits: place component
    edit_svc
        .add_component(
            &mut project,
            AddComponentRequest {
                component_kind: "capacitor".to_string(),
                component_definition_id: None,
                instance_id: Some("C2".to_string()),
                position: hotsas_core::Point::new(400.0, 400.0),
                rotation_deg: 0.0,
            },
        )
        .unwrap();

    // Connect pins
    edit_svc
        .connect_pins(
            &mut project,
            ConnectPinsRequest {
                from_component_id: "C1".to_string(),
                from_pin_id: "2".to_string(),
                to_component_id: "C2".to_string(),
                to_pin_id: "1".to_string(),
                net_name: Some("net_c1_c2".to_string()),
            },
        )
        .unwrap();

    // Update parameter
    edit_svc
        .update_component_quick_parameter(
            &mut project,
            UpdateQuickParameterRequest {
                component_id: "R1".to_string(),
                parameter_id: "resistance".to_string(),
                value: "4.7k".to_string(),
            },
        )
        .unwrap();

    let manifest = service.save_project_package(&dir, &project).unwrap();
    assert_eq!(manifest.project_id, project.id);

    let loaded = service.load_project_package(&dir).unwrap();
    assert_eq!(
        loaded.schematic.components.len(),
        project.schematic.components.len()
    );
    let loaded_r1 = loaded
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    let resistance = loaded_r1.overridden_parameters.get("resistance").unwrap();
    assert_eq!(resistance.original(), "4.7k");
    let loaded_c2 = loaded
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "C2");
    assert!(loaded_c2.is_some());
    assert_eq!(loaded_c2.unwrap().position.x, 400.0);
    let net = loaded.schematic.nets.iter().find(|n| n.name == "net_c1_c2");
    assert!(net.is_some());
    assert!(loaded
        .schematic
        .wires
        .iter()
        .any(|w| w.net_id == net.as_ref().unwrap().id));
}
