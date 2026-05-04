use crate::AppServices;
use hotsas_core::{
    CircuitProject, ProductWorkflowStatus, ProjectSummary, WorkflowModuleStatus,
    WorkflowStatusKind, WorkflowStepStatus,
};

#[derive(Clone)]
pub struct ProductWorkflowService;

impl ProductWorkflowService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_product_workflow_status(
        &self,
        services: &AppServices,
        current_project: Option<&CircuitProject>,
    ) -> ProductWorkflowStatus {
        let mut status =
            ProductWorkflowStatus::new("HotSAS Studio", "2.0.0", "v2.0 Product Beta", "release");

        if let Some(project) = current_project {
            status = status.with_project(ProjectSummary::from_project(project));
        }

        // Workflow steps
        status = status
            .with_step(WorkflowStepStatus::new(
                "project",
                "Project",
                if current_project.is_some() {
                    WorkflowStatusKind::Ready
                } else {
                    WorkflowStatusKind::NotConfigured
                },
                "start",
                "Create or open a .circuit project.",
            ))
            .with_step(WorkflowStepStatus::new(
                "schematic",
                "Schematic",
                if current_project.is_some() {
                    WorkflowStatusKind::Ready
                } else {
                    WorkflowStatusKind::NotConfigured
                },
                "schematic",
                "View and edit the schematic diagram.",
            ))
            .with_step(WorkflowStepStatus::new(
                "formula_library",
                "Formula Library",
                WorkflowStatusKind::Ready,
                "formulas",
                "Calculate formulas from the registry.",
            ))
            .with_step(WorkflowStepStatus::new(
                "engineering_notebook",
                "Engineering Notebook",
                WorkflowStatusKind::Ready,
                "notebook",
                "Interactive notebook for assignments and formula calls.",
            ))
            .with_step(WorkflowStepStatus::new(
                "component_library",
                "Component Library",
                WorkflowStatusKind::Ready,
                "components",
                "Browse and assign library components.",
            ))
            .with_step(WorkflowStepStatus::new(
                "model_import",
                "Model Import",
                WorkflowStatusKind::Ready,
                "import",
                "Import SPICE and Touchstone models.",
            ))
            .with_step(WorkflowStepStatus::new(
                "simulation",
                "Simulation",
                WorkflowStatusKind::Ready,
                "simulation",
                "Run mock or ngspice simulations.",
            ))
            .with_step(WorkflowStepStatus::new(
                "selected_region",
                "Selected Region",
                WorkflowStatusKind::Ready,
                "schematic",
                "Preview and analyze selected schematic regions.",
            ))
            .with_step(WorkflowStepStatus::new(
                "export_center",
                "Export Center",
                WorkflowStatusKind::Ready,
                "export",
                "Export reports, netlists, BOM, and SVG.",
            ))
            .with_step(WorkflowStepStatus::new(
                "diagnostics",
                "Diagnostics",
                WorkflowStatusKind::Ready,
                "diagnostics",
                "Check module readiness and run self-checks.",
            ));

        // Module statuses
        status = self.add_module_statuses(status, services);

        // ngspice warning if unavailable
        match services.check_ngspice_availability() {
            Ok(a) => {
                if !a.available {
                    status.warnings.push(
                        "ngspice not available. Simulation will use mock engine.".to_string(),
                    );
                }
            }
            Err(e) => {
                status
                    .warnings
                    .push(format!("ngspice availability check failed: {e}"));
            }
        }

        status
    }

    pub fn run_product_beta_self_check(&self, services: &AppServices) -> ProductWorkflowStatus {
        let mut status = self.get_product_workflow_status(services, None);

        let project = services.create_rc_low_pass_demo_project();
        status.current_project = Some(ProjectSummary::from_project(&project));

        // Update project/schematic steps to Ready
        for step in &mut status.workflow_steps {
            if step.id == "project" || step.id == "schematic" {
                step.status = WorkflowStatusKind::Ready;
            }
        }

        // 1. Validate schematic
        let validation = services.validate_circuit(&project);
        if !validation.errors.is_empty() {
            status.blockers.push(format!(
                "Circuit validation errors: {}",
                validation.errors.len()
            ));
        }

        // 2. Calculate RC cutoff
        match services.calculate_rc_low_pass_cutoff(&project) {
            Ok(fc) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "formula_calculation",
                        "Formula Calculation",
                        WorkflowStatusKind::Ready,
                    )
                    .with_detail("rc_low_pass_cutoff", &format!("{:.2} Hz", fc.si_value())),
                );
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "formula_calculation",
                        "Formula Calculation",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("error", &e.to_string()),
                );
                status
                    .warnings
                    .push(format!("Formula calculation failed: {e}"));
            }
        }

        // 3. Generate SPICE netlist
        match services.generate_spice_netlist(&project) {
            Ok(netlist) => {
                let has_required = netlist.contains("R1") && netlist.contains("C1");
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "netlist_generation",
                        "Netlist Generation",
                        if has_required {
                            WorkflowStatusKind::Ready
                        } else {
                            WorkflowStatusKind::Limited
                        },
                    )
                    .with_detail("length", &netlist.len().to_string()),
                );
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "netlist_generation",
                        "Netlist Generation",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("error", &e.to_string()),
                );
                status
                    .warnings
                    .push(format!("Netlist generation failed: {e}"));
            }
        }

        // 4. Mock simulation
        match services.run_mock_ac_simulation(&project) {
            Ok(_) => {
                status = status.with_module(WorkflowModuleStatus::new(
                    "mock_simulation",
                    "Mock Simulation",
                    WorkflowStatusKind::Ready,
                ));
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "mock_simulation",
                        "Mock Simulation",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("error", &e.to_string()),
                );
                status.warnings.push(format!("Mock simulation failed: {e}"));
            }
        }

        // 5. ngspice availability
        match services.check_ngspice_availability() {
            Ok(a) => {
                if a.available {
                    status = status.with_module(
                        WorkflowModuleStatus::new(
                            "ngspice",
                            "ngspice Engine",
                            WorkflowStatusKind::Ready,
                        )
                        .with_detail("path", &a.executable_path.unwrap_or_default()),
                    );
                } else {
                    status = status.with_module(
                        WorkflowModuleStatus::new(
                            "ngspice",
                            "ngspice Engine",
                            WorkflowStatusKind::Unavailable,
                        )
                        .with_detail("reason", "ngspice not found in PATH"),
                    );
                    status.warnings.push(
                        "ngspice unavailable. Install ngspice for real simulation.".to_string(),
                    );
                }
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "ngspice",
                        "ngspice Engine",
                        WorkflowStatusKind::Unavailable,
                    )
                    .with_detail("error", &e.to_string()),
                );
                status.warnings.push(format!("ngspice check error: {e}"));
            }
        }

        // 6. Export capabilities
        let caps = services.export_center_service().list_capabilities();
        status = status.with_module(
            WorkflowModuleStatus::new(
                "export_center",
                "Export Center",
                if caps.len() >= 9 {
                    WorkflowStatusKind::Ready
                } else {
                    WorkflowStatusKind::Limited
                },
            )
            .with_detail("capabilities", &caps.len().to_string()),
        );

        // 7. Component library
        match services.component_library_service().load_builtin_library() {
            Ok(lib) => {
                let count = lib.components.len();
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "component_library",
                        "Component Library",
                        if count >= 12 {
                            WorkflowStatusKind::Ready
                        } else {
                            WorkflowStatusKind::Limited
                        },
                    )
                    .with_detail("components", &count.to_string()),
                );
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "component_library",
                        "Component Library",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("error", &e.to_string()),
                );
            }
        }

        // 8. SPICE parser smoke
        let spice_inline = ".model NPN NPN(IS=1e-15 BF=100)\n";
        match services
            .model_import_service()
            .import_spice_from_text(Some("smoke".to_string()), spice_inline.to_string())
        {
            Ok(report) => {
                let count = report.models.len() + report.subcircuits.len();
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "spice_parser",
                        "SPICE Parser",
                        WorkflowStatusKind::Ready,
                    )
                    .with_detail("detected_models", &count.to_string()),
                );
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "spice_parser",
                        "SPICE Parser",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("error", &e.to_string()),
                );
                status
                    .warnings
                    .push(format!("SPICE parser smoke failed: {e}"));
            }
        }

        // 9. Touchstone parser smoke
        let touchstone_inline = "# Hz S RI R 50\n1.0e9 0.9 0.1 0.1 0.9\n";
        match services
            .model_import_service()
            .import_touchstone_from_text(Some("smoke".to_string()), touchstone_inline.to_string())
        {
            Ok(report) => {
                let has_network = if report.network.is_some() {
                    "yes"
                } else {
                    "no"
                };
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "touchstone_parser",
                        "Touchstone Parser",
                        WorkflowStatusKind::Ready,
                    )
                    .with_detail("network_found", has_network),
                );
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "touchstone_parser",
                        "Touchstone Parser",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("error", &e.to_string()),
                );
                status
                    .warnings
                    .push(format!("Touchstone parser smoke failed: {e}"));
            }
        }

        if status.blockers.is_empty() {
            status
                .warnings
                .push("Product beta self-check completed with no blockers.".to_string());
        }

        status
    }

    pub fn create_integrated_demo_project(&self, services: &AppServices) -> CircuitProject {
        services.create_rc_low_pass_demo_project()
    }

    fn add_module_statuses(
        &self,
        mut status: ProductWorkflowStatus,
        services: &AppServices,
    ) -> ProductWorkflowStatus {
        // Formula registry
        status = status.with_module(
            WorkflowModuleStatus::new(
                "formula_registry",
                "Formula Registry",
                WorkflowStatusKind::Ready,
            )
            .with_detail("supported", "rc_low_pass_cutoff, ohms_law, voltage_divider"),
        );

        // Engineering notebook
        status = status.with_module(
            WorkflowModuleStatus::new(
                "engineering_notebook",
                "Engineering Notebook",
                WorkflowStatusKind::Ready,
            )
            .with_detail(
                "features",
                "assignment, formula call, preferred value lookup",
            ),
        );

        // Component library
        match services.component_library_service().load_builtin_library() {
            Ok(lib) => {
                let count = lib.components.len();
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "component_library",
                        "Component Library",
                        if count >= 12 {
                            WorkflowStatusKind::Ready
                        } else {
                            WorkflowStatusKind::Limited
                        },
                    )
                    .with_detail("components", &count.to_string()),
                );
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "component_library",
                        "Component Library",
                        WorkflowStatusKind::Unavailable,
                    )
                    .with_detail("error", &e.to_string()),
                );
            }
        }

        // Import models
        status = status.with_module(
            WorkflowModuleStatus::new("import_models", "Import Models", WorkflowStatusKind::Ready)
                .with_detail("spice", ".lib, .mod, .sp, .cir")
                .with_detail("touchstone", ".s1p, .s2p, .s3p, .s4p"),
        );

        // Export center
        let caps = services.export_center_service().list_capabilities();
        status = status.with_module(
            WorkflowModuleStatus::new(
                "export_center",
                "Export Center",
                if caps.len() >= 9 {
                    WorkflowStatusKind::Ready
                } else {
                    WorkflowStatusKind::Limited
                },
            )
            .with_detail("formats", &caps.len().to_string()),
        );

        // Simulation
        match services.check_ngspice_availability() {
            Ok(a) => {
                if a.available {
                    status = status.with_module(
                        WorkflowModuleStatus::new(
                            "simulation",
                            "Simulation Engine",
                            WorkflowStatusKind::Ready,
                        )
                        .with_detail("ngspice", "available")
                        .with_detail("mock", "available"),
                    );
                } else {
                    status = status.with_module(
                        WorkflowModuleStatus::new(
                            "simulation",
                            "Simulation Engine",
                            WorkflowStatusKind::Limited,
                        )
                        .with_detail("ngspice", "unavailable")
                        .with_detail("mock", "available"),
                    );
                }
            }
            Err(e) => {
                status = status.with_module(
                    WorkflowModuleStatus::new(
                        "simulation",
                        "Simulation Engine",
                        WorkflowStatusKind::Limited,
                    )
                    .with_detail("ngspice", &format!("check error: {e}"))
                    .with_detail("mock", "available"),
                );
            }
        }

        // Project package
        status = status.with_module(
            WorkflowModuleStatus::new(
                "project_package",
                "Project Package Storage",
                WorkflowStatusKind::Ready,
            )
            .with_detail("format", ".circuit"),
        );

        // Schematic editor
        status = status.with_module(
            WorkflowModuleStatus::new(
                "schematic_editor",
                "Schematic Editor",
                WorkflowStatusKind::Ready,
            )
            .with_detail("features", "template, canvas, validation"),
        );

        // Selected region
        status = status.with_module(
            WorkflowModuleStatus::new(
                "selected_region",
                "Selected Region Analysis",
                WorkflowStatusKind::Ready,
            )
            .with_detail("features", "preview, analyze, template match"),
        );

        // Diagnostics
        status = status.with_module(
            WorkflowModuleStatus::new("diagnostics", "Diagnostics", WorkflowStatusKind::Ready)
                .with_detail("features", "module status, readiness self-check"),
        );

        status
    }
}
