use hotsas_core::{
    CircuitModel, CircuitProject, ProjectPackageFiles, ProjectPackageManifest,
    ProjectPackageValidationReport, ReportIndex, ResultIndex,
};
use hotsas_ports::{PortError, ProjectPackageStoragePort};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct CircuitProjectPackageStorage;

impl CircuitProjectPackageStorage {
    fn ensure_circuit_extension(package_dir: &Path) -> Result<PathBuf, PortError> {
        if package_dir.extension().and_then(|ext| ext.to_str()) != Some("circuit") {
            return Err(PortError::Storage(
                "package directory must have .circuit extension".to_string(),
            ));
        }
        Ok(package_dir.to_path_buf())
    }

    fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), PortError> {
        let json = serde_json::to_string_pretty(value)
            .map_err(|e| PortError::Storage(format!("JSON serialization error: {e}")))?;
        fs::write(path, json).map_err(|e| PortError::Storage(format!("write error: {e}")))
    }

    fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, PortError> {
        let json =
            fs::read_to_string(path).map_err(|e| PortError::Storage(format!("read error: {e}")))?;
        serde_json::from_str(&json)
            .map_err(|e| PortError::Storage(format!("JSON deserialization error: {e}")))
    }
}

impl ProjectPackageStoragePort for CircuitProjectPackageStorage {
    fn save_project_package(
        &self,
        package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError> {
        let package_dir = Self::ensure_circuit_extension(package_dir)?;

        fs::create_dir_all(&package_dir)
            .map_err(|e| PortError::Storage(format!("create dir error: {e}")))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let datetime = chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());

        let files = ProjectPackageFiles::default();

        let manifest = ProjectPackageManifest::new(
            project.id.clone(),
            project.name.clone(),
            datetime.clone(),
            datetime,
        );

        // Create directories first
        for subdir in [
            "reports",
            "results",
            "models/spice",
            "models/touchstone",
            "symbols",
            "footprints",
            "exports",
        ] {
            fs::create_dir_all(package_dir.join(subdir))
                .map_err(|e| PortError::Storage(format!("create subdir error: {e}")))?;
        }

        // Write data files
        Self::write_json(&package_dir.join(&files.schematic), &project.schematic)?;
        Self::write_json(
            &package_dir.join(&files.simulation_profiles),
            &project.simulation_profiles,
        )?;
        Self::write_json(
            &package_dir.join(&files.reports_index),
            &ReportIndex::default(),
        )?;
        Self::write_json(
            &package_dir.join(&files.results_index),
            &ResultIndex::default(),
        )?;

        // components placeholder
        #[derive(serde::Serialize)]
        struct ComponentsFile {
            component_definitions: Vec<serde_json::Value>,
            component_instances: Vec<serde_json::Value>,
        }
        Self::write_json(
            &package_dir.join(&files.components),
            &ComponentsFile {
                component_definitions: vec![],
                component_instances: vec![],
            },
        )?;

        // formulas placeholder
        #[derive(serde::Serialize)]
        struct FormulasFile {
            formula_ids: Vec<String>,
            formula_results: Vec<serde_json::Value>,
        }
        Self::write_json(
            &package_dir.join(&files.formulas),
            &FormulasFile {
                formula_ids: vec![],
                formula_results: vec![],
            },
        )?;

        // Write manifest last
        Self::write_json(&package_dir.join("project.json"), &manifest)?;

        Ok(manifest)
    }

    fn load_project_package(&self, package_dir: &Path) -> Result<CircuitProject, PortError> {
        let package_dir = Self::ensure_circuit_extension(package_dir)?;

        let manifest: ProjectPackageManifest = Self::read_json(&package_dir.join("project.json"))?;

        let schematic: CircuitModel =
            Self::read_json(&package_dir.join(&manifest.files.schematic))?;

        let simulation_profiles: Vec<hotsas_core::SimulationProfile> =
            Self::read_json(&package_dir.join(&manifest.files.simulation_profiles))?;

        Ok(CircuitProject {
            id: manifest.project_id,
            name: manifest.project_name,
            format_version: manifest.format_version,
            engine_version: manifest.engine_version,
            project_type: "CircuitProject".to_string(),
            created_at: manifest.created_at,
            updated_at: manifest.updated_at,
            schematic,
            simulation_profiles,
            linked_libraries: vec![],
            reports: vec![],
        })
    }

    fn validate_project_package(
        &self,
        package_dir: &Path,
    ) -> Result<ProjectPackageValidationReport, PortError> {
        let package_dir = Self::ensure_circuit_extension(package_dir)?;

        let mut report = ProjectPackageValidationReport {
            valid: true,
            package_dir: package_dir.to_string_lossy().to_string(),
            missing_files: vec![],
            warnings: vec![],
            errors: vec![],
        };

        let manifest_path = package_dir.join("project.json");
        if !manifest_path.exists() {
            report.missing_files.push("project.json".to_string());
            report
                .errors
                .push("Missing project.json manifest".to_string());
        } else {
            match Self::read_json::<ProjectPackageManifest>(&manifest_path) {
                Ok(manifest) => {
                    let required_files = [
                        (&manifest.files.schematic as &str, "schematic"),
                        (&manifest.files.components, "components"),
                        (&manifest.files.formulas, "formulas"),
                        (&manifest.files.simulation_profiles, "simulation_profiles"),
                        (&manifest.files.reports_index, "reports_index"),
                        (&manifest.files.results_index, "results_index"),
                    ];
                    for (path, label) in &required_files {
                        let full = package_dir.join(path);
                        if !full.exists() {
                            report.missing_files.push(format!("{label} ({path})"));
                            report.errors.push(format!("Missing required file: {path}"));
                        }
                    }
                }
                Err(e) => {
                    report.errors.push(format!("Invalid project.json: {e}"));
                }
            }
        }

        if !report.missing_files.is_empty() || !report.errors.is_empty() {
            report.valid = false;
        }

        Ok(report)
    }
}
