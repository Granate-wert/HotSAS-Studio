use crate::ApplicationError;
use hotsas_core::{
    AttachImportedModelRequest, ComponentDefinition, ImportedModelDetails, ImportedModelSummary,
    SpiceImportReport, SpicePinMappingRequest, SpicePinMappingValidationReport,
    TouchstoneImportReport,
};
use hotsas_ports::{PortError, SpiceModelParserPort, TouchstoneParserPort};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModelImportService {
    spice_parser: Arc<dyn SpiceModelParserPort>,
    touchstone_parser: Arc<dyn TouchstoneParserPort>,
    imported_models: std::sync::Arc<Mutex<Vec<ImportedModelDetails>>>,
}

impl ModelImportService {
    pub fn new(
        spice_parser: Arc<dyn SpiceModelParserPort>,
        touchstone_parser: Arc<dyn TouchstoneParserPort>,
    ) -> Self {
        Self {
            spice_parser,
            touchstone_parser,
            imported_models: std::sync::Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn import_spice_from_text(
        &self,
        source_name: Option<String>,
        content: String,
    ) -> Result<SpiceImportReport, ApplicationError> {
        let report = self
            .spice_parser
            .parse_spice_models_from_str(source_name.clone(), &content)
            .map_err(map_port_error)?;

        let mut guard = self
            .imported_models
            .lock()
            .map_err(|_| ApplicationError::State("imported models lock poisoned".to_string()))?;

        for model in &report.models {
            guard.push(ImportedModelDetails {
                id: model.id.clone(),
                kind: hotsas_core::ImportedModelKind::SpiceModel,
                name: model.name.clone(),
                source: model.source.clone(),
                spice_model: Some(model.clone()),
                spice_subcircuit: None,
                touchstone_network: None,
            });
        }

        for subckt in &report.subcircuits {
            guard.push(ImportedModelDetails {
                id: subckt.id.clone(),
                kind: hotsas_core::ImportedModelKind::SpiceSubcircuit,
                name: subckt.name.clone(),
                source: subckt.source.clone(),
                spice_model: None,
                spice_subcircuit: Some(subckt.clone()),
                touchstone_network: None,
            });
        }

        Ok(report)
    }

    pub fn import_touchstone_from_text(
        &self,
        source_name: Option<String>,
        content: String,
    ) -> Result<TouchstoneImportReport, ApplicationError> {
        let report = self
            .touchstone_parser
            .parse_touchstone_from_str(source_name.clone(), &content)
            .map_err(map_port_error)?;

        if let Some(network) = &report.network {
            let mut guard = self.imported_models.lock().map_err(|_| {
                ApplicationError::State("imported models lock poisoned".to_string())
            })?;
            guard.push(ImportedModelDetails {
                id: network.id.clone(),
                kind: hotsas_core::ImportedModelKind::TouchstoneNetwork,
                name: network.name.clone(),
                source: network.source.clone(),
                spice_model: None,
                spice_subcircuit: None,
                touchstone_network: Some(network.clone()),
            });
        }

        Ok(report)
    }

    pub fn list_imported_models(&self) -> Result<Vec<ImportedModelSummary>, ApplicationError> {
        let guard = self
            .imported_models
            .lock()
            .map_err(|_| ApplicationError::State("imported models lock poisoned".to_string()))?;
        Ok(guard
            .iter()
            .map(|m| ImportedModelSummary {
                id: m.id.clone(),
                kind: m.kind.clone(),
                name: m.name.clone(),
                source_format: m.source.source_format.clone(),
            })
            .collect())
    }

    pub fn get_imported_model(
        &self,
        model_id: String,
    ) -> Result<ImportedModelDetails, ApplicationError> {
        let guard = self
            .imported_models
            .lock()
            .map_err(|_| ApplicationError::State("imported models lock poisoned".to_string()))?;
        guard
            .iter()
            .find(|m| m.id == model_id)
            .cloned()
            .ok_or_else(|| ApplicationError::NotFound(format!("model {model_id}")))
    }

    pub fn validate_spice_pin_mapping(
        &self,
        request: SpicePinMappingRequest,
    ) -> Result<SpicePinMappingValidationReport, ApplicationError> {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        let model = self.get_imported_model(request.model_id.clone())?;
        let pin_count = if let Some(subckt) = model.spice_subcircuit {
            subckt.pins.len()
        } else {
            0
        };

        if pin_count > 0 && request.mappings.len() != pin_count {
            warnings.push(format!(
                "mapping count {} does not match model pin count {}",
                request.mappings.len(),
                pin_count
            ));
        }

        let mut seen_model_pins = std::collections::HashSet::new();
        for entry in &request.mappings {
            if !seen_model_pins.insert(entry.model_pin.clone()) {
                errors.push(format!("duplicate model pin: {}", entry.model_pin));
            }
        }

        let valid = errors.is_empty();
        Ok(SpicePinMappingValidationReport {
            valid,
            warnings,
            errors,
        })
    }

    pub fn attach_imported_model_to_component(
        &self,
        request: AttachImportedModelRequest,
        component: &mut ComponentDefinition,
    ) -> Result<(), ApplicationError> {
        let model = self.get_imported_model(request.model_id.clone())?;

        let model_type = match model.kind {
            hotsas_core::ImportedModelKind::SpiceModel => "spice_model".to_string(),
            hotsas_core::ImportedModelKind::SpiceSubcircuit => "spice_subcircuit".to_string(),
            hotsas_core::ImportedModelKind::TouchstoneNetwork => "touchstone".to_string(),
            hotsas_core::ImportedModelKind::Unknown => "unknown".to_string(),
        };

        let mut pin_mapping = std::collections::BTreeMap::new();
        if let Some(mapping_req) = request.pin_mapping {
            let report = self.validate_spice_pin_mapping(mapping_req.clone())?;
            if !report.valid {
                return Err(ApplicationError::InvalidInput(format!(
                    "pin mapping invalid: {}",
                    report.errors.join("; ")
                )));
            }
            for entry in mapping_req.mappings {
                pin_mapping.insert(entry.model_pin, entry.component_pin);
            }
        }

        component
            .simulation_models
            .push(hotsas_core::SimulationModel {
                id: model.id,
                model_type,
                source_path: model.source.file_path,
                raw_model: None,
                raw_model_id: Some(request.model_id),
                pin_mapping,
            });

        Ok(())
    }
}

fn map_port_error(error: PortError) -> ApplicationError {
    match error {
        PortError::Storage(msg) => ApplicationError::Storage(msg),
        PortError::Formula(msg) => ApplicationError::InvalidInput(msg),
        PortError::Export(msg) => ApplicationError::Export(msg),
        PortError::Simulation(msg) => ApplicationError::Simulation(msg),
    }
}
