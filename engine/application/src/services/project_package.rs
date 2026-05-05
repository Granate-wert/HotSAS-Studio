use crate::ApplicationError;
use hotsas_core::{CircuitProject, ProjectPackageManifest, ProjectPackageValidationReport};
use hotsas_ports::ProjectPackageStoragePort;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProjectPackageService {
    storage: Arc<dyn ProjectPackageStoragePort>,
}

impl ProjectPackageService {
    pub fn new(storage: Arc<dyn ProjectPackageStoragePort>) -> Self {
        Self { storage }
    }

    pub fn storage(&self) -> Arc<dyn ProjectPackageStoragePort> {
        self.storage.clone()
    }

    pub fn save_project_package(
        &self,
        package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, ApplicationError> {
        Ok(self.storage.save_project_package(package_dir, project)?)
    }

    pub fn load_project_package(
        &self,
        package_dir: &Path,
    ) -> Result<CircuitProject, ApplicationError> {
        Ok(self.storage.load_project_package(package_dir)?)
    }

    pub fn validate_project_package(
        &self,
        package_dir: &Path,
    ) -> Result<ProjectPackageValidationReport, ApplicationError> {
        Ok(self.storage.validate_project_package(package_dir)?)
    }
}
