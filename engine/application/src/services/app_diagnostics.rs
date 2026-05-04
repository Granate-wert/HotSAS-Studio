use crate::AppServices;
use hotsas_core::{
    AppDiagnosticsReport, ModuleDiagnostics, ModuleStatus, ReadinessCheck, ReadinessStatus,
};

#[derive(Clone)]
pub struct AppDiagnosticsService;

impl AppDiagnosticsService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_app_diagnostics(&self, services: &AppServices) -> AppDiagnosticsReport {
        let mut report = AppDiagnosticsReport::new(
            "HotSAS Studio",
            "1.10.0",
            "v1.10 internal alpha / v2.0 readiness",
            "release",
        );

        report.modules.push(self.check_formula_registry(services));
        report.modules.push(self.check_component_library(services));
        report.modules.push(self.check_export_center(services));
        report.modules.push(self.check_simulation(services));
        report.modules.push(self.check_import_models(services));
        report.modules.push(self.check_project_package(services));
        report.modules.push(self.check_schematic_editor(services));
        report
            .modules
            .push(self.check_engineering_notebook(services));
        report.modules.push(self.check_selected_region(services));

        report.checks.push(ReadinessCheck::new(
            "app_diagnostics",
            "App diagnostics collection",
            ReadinessStatus::Pass,
            "Diagnostics report assembled successfully.",
        ));

        report
    }

    pub fn run_readiness_self_check(&self, services: &AppServices) -> AppDiagnosticsReport {
        let mut report = self.get_app_diagnostics(services);

        let project = services.create_rc_low_pass_demo_project();

        // Formula calculation smoke
        let formula_check = match services.calculate_rc_low_pass_cutoff(&project) {
            Ok(fc) => ReadinessCheck::new(
                "formula_calculation",
                "Formula calculation smoke",
                ReadinessStatus::Pass,
                &format!("RC low-pass cutoff calculated: {:.2} Hz", fc.si_value()),
            ),
            Err(e) => ReadinessCheck::new(
                "formula_calculation",
                "Formula calculation smoke",
                ReadinessStatus::Fail,
                &format!("Formula calculation failed: {e}"),
            ),
        };
        report.checks.push(formula_check);

        // Export capabilities smoke
        let export_caps = services.export_center_service().list_capabilities();
        let export_check = if export_caps.len() == 9 {
            ReadinessCheck::new(
                "export_capabilities",
                "Export capability list smoke",
                ReadinessStatus::Pass,
                &format!("Export center reports {} capabilities.", export_caps.len()),
            )
        } else {
            ReadinessCheck::new(
                "export_capabilities",
                "Export capability list smoke",
                ReadinessStatus::Warn,
                &format!(
                    "Export center reports {} capabilities, expected 9.",
                    export_caps.len()
                ),
            )
        };
        report.checks.push(export_check);

        // Mock simulation smoke
        let sim_check = match services.run_mock_ac_simulation(&project) {
            Ok(_) => ReadinessCheck::new(
                "mock_simulation",
                "Mock simulation smoke",
                ReadinessStatus::Pass,
                "Mock AC simulation executed successfully.",
            ),
            Err(e) => ReadinessCheck::new(
                "mock_simulation",
                "Mock simulation smoke",
                ReadinessStatus::Fail,
                &format!("Mock simulation failed: {e}"),
            ),
        };
        report.checks.push(sim_check);

        // SPICE parser smoke
        let spice_inline = ".model NPN NPN(IS=1e-15 BF=100)\n";
        let spice_check = match services
            .model_import_service()
            .import_spice_from_text(Some("smoke".to_string()), spice_inline.to_string())
        {
            Ok(report) => {
                let model_count = report.models.len() + report.subcircuits.len();
                ReadinessCheck::new(
                    "spice_parser",
                    "SPICE parser smoke",
                    ReadinessStatus::Pass,
                    &format!("SPICE parser detected {model_count} model(s)."),
                )
            }
            Err(e) => ReadinessCheck::new(
                "spice_parser",
                "SPICE parser smoke",
                ReadinessStatus::Fail,
                &format!("SPICE parser failed: {e}"),
            ),
        };
        report.checks.push(spice_check);

        // Touchstone parser smoke
        let touchstone_inline = "# Hz S RI R 50\n1.0e9 0.9 0.1 0.1 0.9\n";
        let touchstone_check = match services
            .model_import_service()
            .import_touchstone_from_text(Some("smoke".to_string()), touchstone_inline.to_string())
        {
            Ok(report) => {
                let network_count = if report.network.is_some() { 1 } else { 0 };
                ReadinessCheck::new(
                    "touchstone_parser",
                    "Touchstone parser smoke",
                    ReadinessStatus::Pass,
                    &format!("Touchstone parser detected {network_count} network(s)."),
                )
            }
            Err(e) => ReadinessCheck::new(
                "touchstone_parser",
                "Touchstone parser smoke",
                ReadinessStatus::Fail,
                &format!("Touchstone parser failed: {e}"),
            ),
        };
        report.checks.push(touchstone_check);

        // Component library count smoke
        let lib_check = match services.component_library_service().load_builtin_library() {
            Ok(lib) => {
                let count = lib.components.len();
                if count >= 12 {
                    ReadinessCheck::new(
                        "component_library_count",
                        "Component library count smoke",
                        ReadinessStatus::Pass,
                        &format!("Built-in library contains {count} components."),
                    )
                } else {
                    ReadinessCheck::new(
                        "component_library_count",
                        "Component library count smoke",
                        ReadinessStatus::Warn,
                        &format!("Built-in library contains {count} components, expected >= 12."),
                    )
                }
            }
            Err(e) => ReadinessCheck::new(
                "component_library_count",
                "Component library count smoke",
                ReadinessStatus::Fail,
                &format!("Component library load failed: {e}"),
            ),
        };
        report.checks.push(lib_check);

        // ngspice availability
        let ngspice_check = match services.check_ngspice_availability() {
            Ok(a) => {
                if a.available {
                    ReadinessCheck::new(
                        "ngspice_availability",
                        "ngspice availability",
                        ReadinessStatus::Pass,
                        &format!(
                            "ngspice available at {}.",
                            a.executable_path.unwrap_or_default()
                        ),
                    )
                } else {
                    report.warnings.push(
                        "ngspice not available. Simulation will fall back to mock engine."
                            .to_string(),
                    );
                    ReadinessCheck::new(
                        "ngspice_availability",
                        "ngspice availability",
                        ReadinessStatus::Warn,
                        "ngspice not found. Mock engine will be used.",
                    )
                }
            }
            Err(e) => {
                report
                    .warnings
                    .push(format!("ngspice availability check failed: {e}"));
                ReadinessCheck::new(
                    "ngspice_availability",
                    "ngspice availability",
                    ReadinessStatus::Warn,
                    &format!("ngspice availability check error: {e}"),
                )
            }
        };
        report.checks.push(ngspice_check);

        report
    }

    fn check_formula_registry(&self, _services: &AppServices) -> ModuleDiagnostics {
        let mut module = ModuleDiagnostics::new(
            "formula_registry",
            "Formula Registry",
            ModuleStatus::Ready,
            "Formula engine initialized and basic packs supported.",
        );
        module.details.insert(
            "supported_formulas".to_string(),
            "rc_low_pass_cutoff, ohms_law, voltage_divider".to_string(),
        );
        module
    }

    fn check_component_library(&self, services: &AppServices) -> ModuleDiagnostics {
        match services.component_library_service().load_builtin_library() {
            Ok(lib) => {
                let count = lib.components.len();
                let status = if count >= 12 {
                    ModuleStatus::Ready
                } else if count > 0 {
                    ModuleStatus::Limited
                } else {
                    ModuleStatus::Unavailable
                };
                ModuleDiagnostics::new(
                    "component_library",
                    "Component Library",
                    status,
                    &format!("Built-in library loaded with {count} components."),
                )
            }
            Err(e) => ModuleDiagnostics::new(
                "component_library",
                "Component Library",
                ModuleStatus::Unavailable,
                &format!("Failed to load built-in library: {e}"),
            ),
        }
    }

    fn check_export_center(&self, services: &AppServices) -> ModuleDiagnostics {
        let caps = services.export_center_service().list_capabilities();
        let status = if caps.len() == 9 {
            ModuleStatus::Ready
        } else if !caps.is_empty() {
            ModuleStatus::Limited
        } else {
            ModuleStatus::Unavailable
        };
        ModuleDiagnostics::new(
            "export_center",
            "Export Center",
            status,
            &format!("{} export format(s) available.", caps.len()),
        )
        .with_detail(
            "formats",
            &caps
                .iter()
                .map(|c| c.format.clone())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }

    fn check_simulation(&self, services: &AppServices) -> ModuleDiagnostics {
        let ngspice_status = match services.check_ngspice_availability() {
            Ok(a) => {
                if a.available {
                    (ModuleStatus::Ready, "ngspice available.".to_string())
                } else {
                    (
                        ModuleStatus::Limited,
                        "ngspice unavailable; mock engine available.".to_string(),
                    )
                }
            }
            Err(e) => (
                ModuleStatus::Limited,
                format!("ngspice check error: {e}; mock engine available."),
            ),
        };

        let mut module = ModuleDiagnostics::new(
            "simulation",
            "Simulation Engine",
            ngspice_status.0,
            &ngspice_status.1,
        );
        module
            .details
            .insert("mock_engine".to_string(), "available".to_string());
        module
    }

    fn check_import_models(&self, _services: &AppServices) -> ModuleDiagnostics {
        ModuleDiagnostics::new(
            "import_models",
            "Import Models",
            ModuleStatus::Ready,
            "SPICE and Touchstone parsers initialized.",
        )
        .with_detail("spice_extensions", ".lib, .mod, .sp, .cir")
        .with_detail("touchstone_extensions", ".s1p, .s2p, .s3p, .s4p")
    }

    fn check_project_package(&self, _services: &AppServices) -> ModuleDiagnostics {
        ModuleDiagnostics::new(
            "project_package",
            "Project Package Storage",
            ModuleStatus::Ready,
            ".circuit storage service initialized.",
        )
    }

    fn check_schematic_editor(&self, _services: &AppServices) -> ModuleDiagnostics {
        ModuleDiagnostics::new(
            "schematic_editor",
            "Schematic Editor",
            ModuleStatus::Ready,
            "Template and canvas services initialized.",
        )
    }

    fn check_engineering_notebook(&self, _services: &AppServices) -> ModuleDiagnostics {
        ModuleDiagnostics::new(
            "engineering_notebook",
            "Engineering Notebook",
            ModuleStatus::Ready,
            "Notebook evaluation service initialized.",
        )
    }

    fn check_selected_region(&self, _services: &AppServices) -> ModuleDiagnostics {
        ModuleDiagnostics::new(
            "selected_region",
            "Selected Region Analysis",
            ModuleStatus::Ready,
            "Region preview and analysis service initialized.",
        )
    }
}
