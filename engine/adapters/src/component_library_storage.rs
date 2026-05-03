use hotsas_core::{built_in_component_library, ComponentLibrary};
use hotsas_ports::{ComponentLibraryPort, PortError};
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct JsonComponentLibraryStorage;

impl ComponentLibraryPort for JsonComponentLibraryStorage {
    fn load_builtin_library(&self) -> Result<ComponentLibrary, PortError> {
        Ok(built_in_component_library())
    }

    fn load_library_from_path(&self, path: &Path) -> Result<ComponentLibrary, PortError> {
        let json = fs::read_to_string(path).map_err(|error| {
            PortError::Storage(format!(
                "could not read component library {}: {error}",
                path.display()
            ))
        })?;
        serde_json::from_str(&json).map_err(|error| {
            PortError::Storage(format!("could not parse component library JSON: {error}"))
        })
    }

    fn save_library_to_path(
        &self,
        path: &Path,
        library: &ComponentLibrary,
    ) -> Result<(), PortError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| PortError::Storage(error.to_string()))?;
        }
        let json = serde_json::to_string_pretty(library)
            .map_err(|error| PortError::Storage(error.to_string()))?;
        fs::write(path, json).map_err(|error| PortError::Storage(error.to_string()))
    }
}
