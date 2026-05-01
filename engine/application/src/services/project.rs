use crate::ApplicationError;
use hotsas_core::CircuitProject;
use hotsas_ports::StoragePort;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProjectService {
    storage: Arc<dyn StoragePort>,
}

impl ProjectService {
    pub fn new(storage: Arc<dyn StoragePort>) -> Self {
        Self { storage }
    }

    pub fn save_project(
        &self,
        path: &Path,
        project: &CircuitProject,
    ) -> Result<(), ApplicationError> {
        Ok(self.storage.save_project(path, project)?)
    }

    pub fn load_project(&self, path: &Path) -> Result<CircuitProject, ApplicationError> {
        Ok(self.storage.load_project(path)?)
    }
}
