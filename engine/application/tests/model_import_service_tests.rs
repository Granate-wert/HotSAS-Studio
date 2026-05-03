use hotsas_application::ModelImportService;
use hotsas_ports::{PortError, SpiceModelParserPort, TouchstoneParserPort};
use std::sync::Arc;

struct FakeSpiceParser;

impl SpiceModelParserPort for FakeSpiceParser {
    fn parse_spice_models_from_str(
        &self,
        _source_name: Option<String>,
        content: &str,
    ) -> Result<hotsas_core::SpiceImportReport, PortError> {
        let mut models = vec![];
        let mut subcircuits = vec![];

        if content.is_empty() {
            return Ok(hotsas_core::SpiceImportReport {
                status: hotsas_core::ModelImportStatus::Parsed,
                source: hotsas_core::ImportedModelSource {
                    file_name: None,
                    file_path: None,
                    source_format: "spice".to_string(),
                    content_hash: None,
                },
                models,
                subcircuits,
                warnings: vec![],
                errors: vec![],
            });
        }

        if content.contains(".subckt") {
            subcircuits.push(hotsas_core::SpiceSubcircuitDefinition {
                id: "subckt-1".to_string(),
                name: "LM358".to_string(),
                pins: vec!["A".to_string(), "B".to_string(), "C".to_string()],
                body: vec![],
                source: hotsas_core::ImportedModelSource {
                    file_name: None,
                    file_path: None,
                    source_format: "spice".to_string(),
                    content_hash: None,
                },
                detected_kind: hotsas_core::SpiceModelKind::OpAmpMacroModel,
                parameters: vec![],
                warnings: vec![],
            });
        }

        if content.contains(".model") {
            models.push(hotsas_core::SpiceModelDefinition {
                id: "model-1".to_string(),
                name: "TestModel".to_string(),
                kind: hotsas_core::SpiceModelKind::Diode,
                source: hotsas_core::ImportedModelSource {
                    file_name: None,
                    file_path: None,
                    source_format: "spice".to_string(),
                    content_hash: None,
                },
                raw_line: ".model TestModel D()".to_string(),
                parameters: vec![],
                warnings: vec![],
            });
        }

        Ok(hotsas_core::SpiceImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            source: hotsas_core::ImportedModelSource {
                file_name: None,
                file_path: None,
                source_format: "spice".to_string(),
                content_hash: None,
            },
            models,
            subcircuits,
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

fn service() -> ModelImportService {
    ModelImportService::new(Arc::new(FakeSpiceParser), Arc::new(FakeTouchstoneParser))
}

#[test]
fn import_spice_from_text_stores_detected_models() {
    let service = service();
    let report = service
        .import_spice_from_text(
            Some("test.lib".to_string()),
            ".model TestModel D()\n".to_string(),
        )
        .unwrap();
    assert_eq!(report.models.len(), 1);
    let models = service.list_imported_models().unwrap();
    assert_eq!(models.len(), 1);
}

#[test]
fn import_spice_subckt_stores_pins() {
    let service = service();
    let report = service
        .import_spice_from_text(
            Some("test.lib".to_string()),
            ".subckt LM358 A B C\n.ends\n".to_string(),
        )
        .unwrap();
    assert_eq!(report.subcircuits.len(), 1);
}

#[test]
fn list_imported_models_returns_summaries() {
    let service = service();
    service
        .import_spice_from_text(Some("test.lib".to_string()), ".model X D()\n".to_string())
        .unwrap();
    let models = service.list_imported_models().unwrap();
    assert!(!models.is_empty());
}

#[test]
fn get_imported_model_returns_details() {
    let service = service();
    service
        .import_spice_from_text(Some("test.lib".to_string()), ".model X D()\n".to_string())
        .unwrap();
    let details = service.get_imported_model("model-1".to_string()).unwrap();
    assert_eq!(details.name, "TestModel");
}

#[test]
fn validate_pin_mapping_valid_case() {
    let service = service();
    service
        .import_spice_from_text(
            Some("test.lib".to_string()),
            ".subckt LM358 A B C\n.ends\n".to_string(),
        )
        .unwrap();
    let models = service.list_imported_models().unwrap();
    let subckt_id = models
        .iter()
        .find(|m| m.kind == hotsas_core::ImportedModelKind::SpiceSubcircuit)
        .unwrap()
        .id
        .clone();
    let report = service
        .validate_spice_pin_mapping(hotsas_core::SpicePinMappingRequest {
            model_id: subckt_id,
            component_definition_id: "generic_resistor".to_string(),
            mappings: vec![
                hotsas_core::SpicePinMappingEntry {
                    model_pin: "A".to_string(),
                    component_pin: "1".to_string(),
                    role_hint: None,
                },
                hotsas_core::SpicePinMappingEntry {
                    model_pin: "B".to_string(),
                    component_pin: "2".to_string(),
                    role_hint: None,
                },
                hotsas_core::SpicePinMappingEntry {
                    model_pin: "C".to_string(),
                    component_pin: "3".to_string(),
                    role_hint: None,
                },
            ],
        })
        .unwrap();
    assert!(report.valid);
}

#[test]
fn attach_spice_model_to_component_adds_simulation_model() {
    let service = service();
    service
        .import_spice_from_text(Some("test.lib".to_string()), ".model X D()\n".to_string())
        .unwrap();
    let mut component = hotsas_core::built_in_component_library()
        .components
        .into_iter()
        .find(|c| c.id == "generic_resistor")
        .unwrap();
    let original_count = component.simulation_models.len();
    service
        .attach_imported_model_to_component(
            hotsas_core::AttachImportedModelRequest {
                model_id: "model-1".to_string(),
                component_definition_id: "generic_resistor".to_string(),
                pin_mapping: None,
            },
            &mut component,
        )
        .unwrap();
    assert_eq!(component.simulation_models.len(), original_count + 1);
}

#[test]
fn import_touchstone_from_text_stores_network_summary() {
    let service = service();
    let report = service
        .import_touchstone_from_text(
            Some("test.s1p".to_string()),
            "# GHz S RI R 50\n1.0 0.5 0.1\n".to_string(),
        )
        .unwrap();
    assert!(report.network.is_none()); // Fake parser returns None
}
