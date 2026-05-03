use hotsas_application::ComponentLibraryService;
use hotsas_core::{ComponentAssignment, ComponentLibraryQuery};
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

fn fake_service() -> ComponentLibraryService {
    ComponentLibraryService::new(Arc::new(FakeComponentLibraryStorage))
}

fn load_lib() -> hotsas_core::ComponentLibrary {
    fake_service().load_builtin_library().unwrap()
}

#[test]
fn list_components_returns_all() {
    let lib = load_lib();
    let service = fake_service();
    let list = service.list_components(&lib);
    assert_eq!(list.len(), lib.components.len());
}

#[test]
fn search_by_resistor_returns_generic_resistor() {
    let lib = load_lib();
    let service = fake_service();
    let result = service.search_components(
        &lib,
        ComponentLibraryQuery {
            search: Some("resistor".to_string()),
            ..Default::default()
        },
    );
    assert!(result.components.iter().any(|c| c.id == "generic_resistor"));
}

#[test]
fn search_is_case_insensitive() {
    let lib = load_lib();
    let service = fake_service();
    let result_lower = service.search_components(
        &lib,
        ComponentLibraryQuery {
            search: Some("resistor".to_string()),
            ..Default::default()
        },
    );
    let result_upper = service.search_components(
        &lib,
        ComponentLibraryQuery {
            search: Some("RESISTOR".to_string()),
            ..Default::default()
        },
    );
    assert_eq!(result_lower.total_count, result_upper.total_count);
}

#[test]
fn filter_by_category_resistor_works() {
    let lib = load_lib();
    let service = fake_service();
    let result = service.search_components(
        &lib,
        ComponentLibraryQuery {
            category: Some("resistor".to_string()),
            ..Default::default()
        },
    );
    assert!(result.components.iter().all(|c| c.category == "resistor"));
}

#[test]
fn filter_has_footprint_works() {
    let lib = load_lib();
    let service = fake_service();
    let result = service.search_components(
        &lib,
        ComponentLibraryQuery {
            has_footprint: Some(true),
            ..Default::default()
        },
    );
    assert!(result
        .components
        .iter()
        .all(|c| !c.footprint_ids.is_empty()));
}

#[test]
fn get_component_generic_resistor_works() {
    let lib = load_lib();
    let service = fake_service();
    let component = service.get_component(&lib, "generic_resistor").unwrap();
    assert_eq!(component.id, "generic_resistor");
}

#[test]
fn get_component_missing_id_returns_error() {
    let lib = load_lib();
    let service = fake_service();
    let result = service.get_component(&lib, "missing_id");
    assert!(result.is_err());
}

#[test]
fn assign_component_to_instance_updates_definition_id() {
    let _lib = load_lib();
    let service = fake_service();
    let mut project = hotsas_core::rc_low_pass_project();
    let instance_id = project.schematic.components[0].instance_id.clone();
    service
        .assign_component_to_instance(
            &mut project,
            ComponentAssignment {
                instance_id: instance_id.clone(),
                component_definition_id: "generic_resistor".to_string(),
                selected_symbol_id: Some("resistor".to_string()),
                selected_footprint_id: None,
                selected_simulation_model_id: None,
            },
        )
        .unwrap();
    let instance = project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == instance_id)
        .unwrap();
    assert_eq!(instance.definition_id, "generic_resistor");
    assert_eq!(instance.selected_symbol_id, Some("resistor".to_string()));
}

#[test]
fn assign_preserves_existing_overridden_parameter_if_present() {
    let _lib = load_lib();
    let service = fake_service();
    let mut project = hotsas_core::rc_low_pass_project();
    let instance_id = project.schematic.components[0].instance_id.clone();
    project.schematic.components[0]
        .overridden_parameters
        .insert(
            "resistance".to_string(),
            hotsas_core::ValueWithUnit::parse_with_default(
                "4.7k",
                hotsas_core::EngineeringUnit::Ohm,
            )
            .unwrap(),
        );
    service
        .assign_component_to_instance(
            &mut project,
            ComponentAssignment {
                instance_id: instance_id.clone(),
                component_definition_id: "generic_resistor".to_string(),
                selected_symbol_id: None,
                selected_footprint_id: None,
                selected_simulation_model_id: None,
            },
        )
        .unwrap();
    let instance = project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == instance_id)
        .unwrap();
    assert!(instance.overridden_parameters.contains_key("resistance"));
}
