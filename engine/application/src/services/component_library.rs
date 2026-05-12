use crate::ApplicationError;
use hotsas_core::{
    CircuitProject, ComponentAssignment, ComponentDefinition, ComponentLibrary,
    ComponentLibraryQuery, ComponentLibrarySearchResult, FootprintDefinition,
};
use hotsas_ports::ComponentLibraryPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct ComponentLibraryService {
    library_port: Arc<dyn ComponentLibraryPort>,
}

impl ComponentLibraryService {
    pub fn new(library_port: Arc<dyn ComponentLibraryPort>) -> Self {
        Self { library_port }
    }

    pub fn load_builtin_library(&self) -> Result<ComponentLibrary, ApplicationError> {
        self.library_port
            .load_builtin_library()
            .map_err(ApplicationError::Port)
    }

    pub fn list_components(&self, library: &ComponentLibrary) -> Vec<ComponentDefinition> {
        library.components.clone()
    }

    pub fn search_components(
        &self,
        library: &ComponentLibrary,
        query: ComponentLibraryQuery,
    ) -> ComponentLibrarySearchResult {
        let search_lower = query.search.as_ref().map(|s| s.to_lowercase());
        let category_lower = query.category.as_ref().map(|s| s.to_lowercase());
        let manufacturer_lower = query.manufacturer.as_ref().map(|s| s.to_lowercase());
        let tag_set: std::collections::HashSet<String> =
            query.tags.iter().map(|t| t.to_lowercase()).collect();

        let filtered: Vec<ComponentDefinition> = library
            .components
            .iter()
            .filter(|c| {
                if let Some(ref search) = search_lower {
                    if !c.id.to_lowercase().contains(search)
                        && !c.name.to_lowercase().contains(search)
                        && !c.category.to_lowercase().contains(search)
                        && c.description
                            .as_ref()
                            .map_or(true, |d| !d.to_lowercase().contains(search))
                        && c.part_number
                            .as_ref()
                            .map_or(true, |p| !p.to_lowercase().contains(search))
                        && !c.tags.iter().any(|t| t.to_lowercase().contains(search))
                    {
                        return false;
                    }
                }
                if let Some(ref cat) = category_lower {
                    if c.category.to_lowercase() != *cat {
                        return false;
                    }
                }
                if !tag_set.is_empty() {
                    let c_tags: std::collections::HashSet<String> =
                        c.tags.iter().map(|t| t.to_lowercase()).collect();
                    if c_tags.is_disjoint(&tag_set) {
                        return false;
                    }
                }
                if let Some(ref mfr) = manufacturer_lower {
                    if c.manufacturer
                        .as_ref()
                        .map_or(true, |m| !m.to_lowercase().contains(mfr))
                    {
                        return false;
                    }
                }
                if let Some(has_symbol) = query.has_symbol {
                    let component_has = !c.symbol_ids.is_empty();
                    if component_has != has_symbol {
                        return false;
                    }
                }
                if let Some(has_footprint) = query.has_footprint {
                    let component_has = !c.footprint_ids.is_empty();
                    if component_has != has_footprint {
                        return false;
                    }
                }
                if let Some(has_sim) = query.has_simulation_model {
                    let component_has = !c.simulation_models.is_empty();
                    if component_has != has_sim {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        let mut categories: Vec<String> = library
            .components
            .iter()
            .map(|c| c.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        categories.sort();

        let mut tags: Vec<String> = library
            .components
            .iter()
            .flat_map(|c| c.tags.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        tags.sort();

        ComponentLibrarySearchResult {
            total_count: filtered.len(),
            components: filtered,
            categories,
            tags,
        }
    }

    pub fn get_component(
        &self,
        library: &ComponentLibrary,
        component_id: &str,
    ) -> Result<ComponentDefinition, ApplicationError> {
        library
            .components
            .iter()
            .find(|c| c.id == component_id)
            .cloned()
            .ok_or_else(|| {
                ApplicationError::InvalidInput(format!(
                    "component '{}' not found in library",
                    component_id
                ))
            })
    }

    pub fn find_definition(&self, definition_id: &str) -> Option<ComponentDefinition> {
        self.library_port
            .load_builtin_library()
            .ok()
            .and_then(|lib| {
                lib.components
                    .iter()
                    .find(|c| c.id == definition_id)
                    .cloned()
            })
            .or_else(|| {
                hotsas_core::built_in_component_library()
                    .components
                    .iter()
                    .find(|c| c.id == definition_id)
                    .cloned()
            })
    }

    pub fn get_symbol_for_component(
        &self,
        library: &ComponentLibrary,
        component_id: &str,
    ) -> Option<hotsas_core::SymbolDefinition> {
        let component = library.components.iter().find(|c| c.id == component_id)?;
        component
            .symbol_ids
            .first()
            .and_then(|id| hotsas_core::seed_symbol_for_kind(id))
            .or_else(|| {
                // Fallback: try matching by category if direct symbol id misses
                hotsas_core::seed_symbol_for_kind(&component.category)
            })
    }

    pub fn get_footprints_for_component(
        &self,
        library: &ComponentLibrary,
        component_id: &str,
    ) -> Vec<FootprintDefinition> {
        let Some(component) = library.components.iter().find(|c| c.id == component_id) else {
            return vec![];
        };
        library
            .footprints
            .iter()
            .filter(|f| component.footprint_ids.contains(&f.id))
            .cloned()
            .collect()
    }

    pub fn assign_component_to_instance(
        &self,
        project: &mut CircuitProject,
        assignment: ComponentAssignment,
    ) -> Result<(), ApplicationError> {
        let instance = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == assignment.instance_id)
            .ok_or_else(|| {
                ApplicationError::InvalidInput(format!(
                    "instance '{}' not found in project",
                    assignment.instance_id
                ))
            })?;

        instance.definition_id = assignment.component_definition_id;
        instance.selected_symbol_id = assignment.selected_symbol_id;
        instance.selected_footprint_id = assignment.selected_footprint_id;
        instance.selected_simulation_model_id = assignment.selected_simulation_model_id;

        Ok(())
    }
}
