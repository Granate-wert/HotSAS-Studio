pub mod commands;
pub mod errors;
pub mod output;

use hotsas_api::{ApiError, HotSasApi};
use hotsas_application::AppServices;
use std::sync::Arc;

pub fn initialize_cli(api: &HotSasApi) -> Result<(), ApiError> {
    use hotsas_adapters::FormulaPackFileLoader;
    let loader = FormulaPackFileLoader;
    match loader.load_builtin_packs() {
        Ok(packs) => {
            let _ = api.load_formula_packs(packs);
        }
        Err(_) => {
            // Builtin packs may not be available in all environments;
            // proceed without them.
        }
    }
    let _ = api.load_builtin_component_library();
    Ok(())
}

pub fn build_headless_api() -> HotSasApi {
    use hotsas_adapters::*;
    HotSasApi::new(AppServices::new(
        Arc::new(JsonProjectStorage),
        Arc::new(CircuitProjectPackageStorage::default()),
        Arc::new(SimpleFormulaEngine),
        Arc::new(SpiceNetlistExporter),
        Arc::new(MockSimulationEngine),
        Arc::new(NgspiceSimulationAdapter::new()),
        Arc::new(MarkdownReportExporter),
        Arc::new(JsonComponentLibraryStorage),
        Arc::new(BomCsvExporter),
        Arc::new(CsvSimulationDataExporter),
        Arc::new(ComponentLibraryJsonExporter),
        Arc::new(SvgSchematicExporter),
        Arc::new(SimpleSpiceModelParser::new()),
        Arc::new(SimpleTouchstoneParser::new()),
    ))
}
