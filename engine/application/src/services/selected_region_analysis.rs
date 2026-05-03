use crate::ApplicationError;
use hotsas_core::{
    normalize_component_ids, CircuitModel, EquivalentCircuitSummary, MatchedRegionTemplate,
    RegionAnalysisDirection, RegionAnalysisMode, RegionComponentSummary, RegionGraphSpec,
    RegionNetSummary, RegionNetlistFragment, RegionTransferFunction, SelectedCircuitRegion,
    SelectedRegionAnalysisRequest, SelectedRegionAnalysisResult, SelectedRegionAnalysisStatus,
    SelectedRegionIssue, SelectedRegionIssueSeverity, SelectedRegionPreview,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct SelectedRegionAnalysisService;

impl SelectedRegionAnalysisService {
    pub fn new() -> Self {
        Self
    }

    pub fn preview_selected_region(
        &self,
        circuit: &CircuitModel,
        component_ids: Vec<String>,
    ) -> Result<SelectedRegionPreview, ApplicationError> {
        let ids = normalize_component_ids(&component_ids);
        let _subcircuit = self.build_subcircuit_view(circuit, &ids)?;

        let selected_components: Vec<RegionComponentSummary> = circuit
            .components
            .iter()
            .filter(|c| ids.contains(&c.instance_id))
            .map(|c| RegionComponentSummary {
                instance_id: c.instance_id.clone(),
                definition_id: Some(c.definition_id.clone()),
                component_kind: c.definition_id.clone(),
                display_label: c.instance_id.clone(),
                connected_nets: c.connected_nets.iter().map(|p| p.net_id.clone()).collect(),
            })
            .collect();

        let (internal_nets, boundary_nets) = self.summarize_nets(circuit, &ids)?;

        let mut warnings = vec![];
        let mut errors = vec![];

        if ids.is_empty() {
            errors.push(SelectedRegionIssue {
                code: "empty_selection".to_string(),
                severity: SelectedRegionIssueSeverity::Error,
                message: "Select at least one component before running region analysis."
                    .to_string(),
                component_id: None,
                net_id: None,
            });
        }

        if boundary_nets.is_empty() && !ids.is_empty() {
            warnings.push(SelectedRegionIssue {
                code: "no_boundary_nets".to_string(),
                severity: SelectedRegionIssueSeverity::Warning,
                message:
                    "Selected region has no detected boundary nets; only structural analysis is available."
                        .to_string(),
                component_id: None,
                net_id: None,
            });
        }

        let suggested_input_nets: Vec<String> = boundary_nets
            .iter()
            .filter(|n| !n.is_ground)
            .map(|n| n.net_id.clone())
            .collect();
        let suggested_output_nets: Vec<String> = suggested_input_nets.clone();
        let suggested_reference_nodes: Vec<String> = boundary_nets
            .iter()
            .filter(|n| n.is_ground)
            .map(|n| n.net_id.clone())
            .collect();

        let region = SelectedCircuitRegion {
            id: format!("region-{}", ids.join("-")),
            title: "Selected Region".to_string(),
            component_ids: ids.clone(),
            internal_nets: internal_nets.iter().map(|n| n.net_id.clone()).collect(),
            boundary_nets: boundary_nets.iter().map(|n| n.net_id.clone()).collect(),
            input_port: None,
            output_port: None,
            reference_node: suggested_reference_nodes.first().cloned(),
            analysis_direction: RegionAnalysisDirection::LeftToRight,
            analysis_mode: RegionAnalysisMode::AllAvailable,
            metadata: BTreeMap::new(),
        };

        Ok(SelectedRegionPreview {
            region,
            selected_components,
            detected_internal_nets: internal_nets,
            detected_boundary_nets: boundary_nets,
            suggested_input_nets,
            suggested_output_nets,
            suggested_reference_nodes,
            warnings,
            errors,
        })
    }

    pub fn analyze_selected_region(
        &self,
        circuit: &CircuitModel,
        request: SelectedRegionAnalysisRequest,
    ) -> Result<SelectedRegionAnalysisResult, ApplicationError> {
        let ids = normalize_component_ids(&request.component_ids);
        let issues = self.validate_selected_region(circuit, &request);
        let has_errors = issues
            .iter()
            .any(|i| i.severity == SelectedRegionIssueSeverity::Error);

        let subcircuit = self.build_subcircuit_view(circuit, &ids)?;

        let region = SelectedCircuitRegion {
            id: format!("region-{}", ids.join("-")),
            title: "Selected Region".to_string(),
            component_ids: ids.clone(),
            internal_nets: subcircuit.internal_nets.clone(),
            boundary_nets: subcircuit.boundary_nets.clone(),
            input_port: request.input_port.clone(),
            output_port: request.output_port.clone(),
            reference_node: request.reference_node.clone(),
            analysis_direction: request.analysis_direction,
            analysis_mode: request.analysis_mode,
            metadata: BTreeMap::new(),
        };

        if has_errors {
            return Ok(SelectedRegionAnalysisResult {
                region,
                status: SelectedRegionAnalysisStatus::Error,
                summary: "Region analysis cannot proceed due to validation errors.".to_string(),
                matched_template: None,
                equivalent_circuit: None,
                transfer_function: None,
                measurements: vec![],
                graph_specs: vec![],
                netlist_fragment: None,
                warnings: issues
                    .iter()
                    .filter(|i| i.severity == SelectedRegionIssueSeverity::Warning)
                    .cloned()
                    .collect(),
                errors: issues
                    .iter()
                    .filter(|i| i.severity == SelectedRegionIssueSeverity::Error)
                    .cloned()
                    .collect(),
                report_section_markdown: None,
            });
        }

        let matched_template = self.match_known_region_template(circuit, &ids, &request);
        let netlist_fragment = if ids.is_empty() {
            None
        } else {
            Some(self.generate_region_netlist_fragment(circuit, &ids, &request)?)
        };

        let (status, summary, transfer_function, equivalent_circuit, measurements, graph_specs) =
            if let Some(ref template) = matched_template {
                (
                    SelectedRegionAnalysisStatus::Success,
                    format!(
                        "Matched template: {}. {}",
                        template.title, template.explanation
                    ),
                    self.transfer_function_for_template(template),
                    Some(EquivalentCircuitSummary {
                        title: template.title.clone(),
                        description: template.explanation.clone(),
                        assumptions: vec!["Ideal components assumed".to_string()],
                        limitations: vec![
                            "Parasitics ignored".to_string(),
                            "Symbolic analysis is limited to supported templates in v1.6."
                                .to_string(),
                        ],
                    }),
                    vec![],
                    self.graph_specs_for_template(template),
                )
            } else if ids.is_empty() {
                (
                    SelectedRegionAnalysisStatus::Error,
                    "No components selected.".to_string(),
                    None,
                    None,
                    vec![],
                    vec![],
                )
            } else {
                (
                    SelectedRegionAnalysisStatus::Partial,
                    "Structural analysis completed. No supported v1.6 template matched.".to_string(),
                    None,
                    None,
                    vec![],
                    vec![
                        RegionGraphSpec {
                            id: "gain".to_string(),
                            title: "Gain".to_string(),
                            x_unit: None,
                            y_unit: None,
                            description: "Gain vs frequency".to_string(),
                            available: false,
                            unavailable_reason: Some("Graph data is not generated in v1.6. This version exposes available graph specs for future simulation integration.".to_string()),
                        },
                        RegionGraphSpec {
                            id: "phase".to_string(),
                            title: "Phase".to_string(),
                            x_unit: None,
                            y_unit: None,
                            description: "Phase vs frequency".to_string(),
                            available: false,
                            unavailable_reason: Some("Graph data is not generated in v1.6.".to_string()),
                        },
                    ],
                )
            };

        let mut warnings: Vec<SelectedRegionIssue> = issues
            .iter()
            .filter(|i| i.severity == SelectedRegionIssueSeverity::Warning)
            .cloned()
            .collect();
        if matched_template.is_none() && !ids.is_empty() {
            warnings.push(SelectedRegionIssue {
                code: "unsupported_region_complexity".to_string(),
                severity: SelectedRegionIssueSeverity::Warning,
                message: "This selected region is not recognized as a supported v1.6 template."
                    .to_string(),
                component_id: None,
                net_id: None,
            });
        }

        let report_md =
            self.build_report_markdown(&region, &matched_template, &netlist_fragment, &warnings);

        Ok(SelectedRegionAnalysisResult {
            region,
            status,
            summary,
            matched_template,
            equivalent_circuit,
            transfer_function,
            measurements,
            graph_specs,
            netlist_fragment,
            warnings,
            errors: vec![],
            report_section_markdown: Some(report_md),
        })
    }

    pub fn validate_selected_region(
        &self,
        circuit: &CircuitModel,
        request: &SelectedRegionAnalysisRequest,
    ) -> Vec<SelectedRegionIssue> {
        let mut issues = vec![];
        let ids = normalize_component_ids(&request.component_ids);

        if ids.is_empty() {
            issues.push(SelectedRegionIssue {
                code: "empty_selection".to_string(),
                severity: SelectedRegionIssueSeverity::Error,
                message: "Select at least one component before running region analysis."
                    .to_string(),
                component_id: None,
                net_id: None,
            });
        }

        for id in &ids {
            if !circuit.components.iter().any(|c| c.instance_id == *id) {
                issues.push(SelectedRegionIssue {
                    code: "unknown_component".to_string(),
                    severity: SelectedRegionIssueSeverity::Error,
                    message: format!(
                        "Selected component '{}' does not exist in current circuit.",
                        id
                    ),
                    component_id: Some(id.clone()),
                    net_id: None,
                });
            }
        }

        if request.input_port.is_none() {
            issues.push(SelectedRegionIssue {
                code: "missing_input_port".to_string(),
                severity: SelectedRegionIssueSeverity::Error,
                message: "Input port is not configured.".to_string(),
                component_id: None,
                net_id: None,
            });
        }

        if request.output_port.is_none() {
            issues.push(SelectedRegionIssue {
                code: "missing_output_port".to_string(),
                severity: SelectedRegionIssueSeverity::Error,
                message: "Output port is not configured.".to_string(),
                component_id: None,
                net_id: None,
            });
        }

        if request.reference_node.is_none() {
            issues.push(SelectedRegionIssue {
                code: "missing_reference_node".to_string(),
                severity: SelectedRegionIssueSeverity::Warning,
                message:
                    "Reference node is not explicitly set; default GND will be used if available."
                        .to_string(),
                component_id: None,
                net_id: None,
            });
        }

        issues
    }

    pub fn build_subcircuit_view(
        &self,
        circuit: &CircuitModel,
        component_ids: &[String],
    ) -> Result<SubcircuitView, ApplicationError> {
        let selected_set: BTreeSet<String> = component_ids.iter().cloned().collect();
        let mut net_to_components: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for comp in &circuit.components {
            for pin in &comp.connected_nets {
                net_to_components
                    .entry(pin.net_id.clone())
                    .or_default()
                    .push(comp.instance_id.clone());
            }
        }

        let mut internal_nets = vec![];
        let mut boundary_nets = vec![];
        let mut external_nets = vec![];

        for (net_id, comps) in net_to_components {
            let unique: BTreeSet<String> = comps.into_iter().collect();
            let selected_count = unique.intersection(&selected_set).count();
            let total = unique.len();
            if selected_count == total && !selected_set.is_empty() {
                internal_nets.push(net_id);
            } else if selected_count > 0 {
                boundary_nets.push(net_id);
            } else {
                external_nets.push(net_id);
            }
        }

        Ok(SubcircuitView {
            component_ids: component_ids.to_vec(),
            internal_nets,
            boundary_nets,
            external_nets,
        })
    }

    pub fn detect_boundary_nets(
        &self,
        circuit: &CircuitModel,
        component_ids: &[String],
    ) -> Result<Vec<RegionNetSummary>, ApplicationError> {
        self.summarize_nets(circuit, component_ids)
            .map(|(_, boundary)| boundary)
    }

    fn summarize_nets(
        &self,
        circuit: &CircuitModel,
        component_ids: &[String],
    ) -> Result<(Vec<RegionNetSummary>, Vec<RegionNetSummary>), ApplicationError> {
        let selected_set: BTreeSet<String> = component_ids.iter().cloned().collect();
        let mut net_to_components: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for comp in &circuit.components {
            for pin in &comp.connected_nets {
                net_to_components
                    .entry(pin.net_id.clone())
                    .or_default()
                    .push(comp.instance_id.clone());
            }
        }

        let mut internal = vec![];
        let mut boundary = vec![];

        for net in &circuit.nets {
            let comps = net_to_components.get(&net.id).cloned().unwrap_or_default();
            let unique: BTreeSet<String> = comps.into_iter().collect();
            let selected_comps: Vec<String> = unique.intersection(&selected_set).cloned().collect();
            let external_comps: Vec<String> = unique.difference(&selected_set).cloned().collect();
            let is_ground = net.name.to_lowercase() == "gnd" || net.id.to_lowercase() == "gnd";

            let summary = RegionNetSummary {
                net_id: net.id.clone(),
                net_name: net.name.clone(),
                connected_selected_components: selected_comps,
                connected_external_components: external_comps,
                is_ground,
                role_hint: None,
            };

            let selected_count = unique.intersection(&selected_set).count();
            let total = unique.len();
            if selected_count == total && !selected_set.is_empty() {
                internal.push(summary);
            } else if selected_count > 0 {
                boundary.push(summary);
            }
        }

        Ok((internal, boundary))
    }

    pub fn match_known_region_template(
        &self,
        circuit: &CircuitModel,
        component_ids: &[String],
        _request: &SelectedRegionAnalysisRequest,
    ) -> Option<MatchedRegionTemplate> {
        let selected: Vec<&hotsas_core::ComponentInstance> = circuit
            .components
            .iter()
            .filter(|c| component_ids.contains(&c.instance_id))
            .collect();

        if selected.len() == 2 {
            let has_resistor = selected.iter().any(|c| {
                c.definition_id == "resistor"
                    || c.definition_id == "generic_resistor"
                    || c.instance_id.starts_with('R')
            });
            let has_capacitor = selected.iter().any(|c| {
                c.definition_id == "capacitor"
                    || c.definition_id == "generic_capacitor"
                    || c.instance_id.starts_with('C')
            });

            if has_resistor && has_capacitor {
                let r_nets: BTreeSet<String> = selected
                    .iter()
                    .find(|c| {
                        c.definition_id == "resistor"
                            || c.definition_id == "generic_resistor"
                            || c.instance_id.starts_with('R')
                    })
                    .map(|c| c.connected_nets.iter().map(|p| p.net_id.clone()).collect())
                    .unwrap_or_default();
                let c_nets: BTreeSet<String> = selected
                    .iter()
                    .find(|c| {
                        c.definition_id == "capacitor"
                            || c.definition_id == "generic_capacitor"
                            || c.instance_id.starts_with('C')
                    })
                    .map(|c| c.connected_nets.iter().map(|p| p.net_id.clone()).collect())
                    .unwrap_or_default();

                let common: Vec<String> = r_nets.intersection(&c_nets).cloned().collect();
                if !common.is_empty() {
                    return Some(MatchedRegionTemplate {
                        template_id: "rc_low_pass_template".to_string(),
                        title: "RC Low-Pass Filter".to_string(),
                        confidence: 0.92,
                        formula_ids: vec!["rc_low_pass_cutoff".to_string()],
                        explanation: "Two-passive topology with shared node between resistor and capacitor, consistent with a first-order RC low-pass filter.".to_string(),
                    });
                }
            }

            let has_r1 = selected.iter().any(|c| c.instance_id == "R1");
            let has_r2 = selected.iter().any(|c| c.instance_id == "R2");
            if has_r1 && has_r2 {
                let r1_nets: BTreeSet<String> = selected
                    .iter()
                    .find(|c| c.instance_id == "R1")
                    .map(|c| c.connected_nets.iter().map(|p| p.net_id.clone()).collect())
                    .unwrap_or_default();
                let r2_nets: BTreeSet<String> = selected
                    .iter()
                    .find(|c| c.instance_id == "R2")
                    .map(|c| c.connected_nets.iter().map(|p| p.net_id.clone()).collect())
                    .unwrap_or_default();
                let common: Vec<String> = r1_nets.intersection(&r2_nets).cloned().collect();
                if !common.is_empty() {
                    return Some(MatchedRegionTemplate {
                        template_id: "voltage_divider".to_string(),
                        title: "Voltage Divider".to_string(),
                        confidence: 0.9,
                        formula_ids: vec!["voltage_divider".to_string()],
                        explanation: "Two resistors in series with a shared intermediate node, consistent with a resistive voltage divider.".to_string(),
                    });
                }
            }
        }

        None
    }

    pub fn generate_region_netlist_fragment(
        &self,
        circuit: &CircuitModel,
        component_ids: &[String],
        request: &SelectedRegionAnalysisRequest,
    ) -> Result<RegionNetlistFragment, ApplicationError> {
        let mut lines = vec![];
        lines.push(format!(
            "* Selected region fragment: {} component(s)",
            component_ids.len()
        ));

        for comp in &circuit.components {
            if !component_ids.contains(&comp.instance_id) {
                continue;
            }
            let nets: Vec<String> = comp
                .connected_nets
                .iter()
                .map(|p| p.net_id.clone())
                .collect();
            let value = comp
                .overridden_parameters
                .iter()
                .next()
                .map(|(_, v)| v.original())
                .unwrap_or("?");
            let line = if nets.len() >= 2 {
                format!("{} {} {}", comp.instance_id, nets.join(" "), value)
            } else {
                format!("{} {}", comp.instance_id, nets.join(" "))
            };
            lines.push(line);
        }

        if let Some(ref input) = request.input_port {
            lines.push(format!(
                "* Input: {} (+) {}",
                input.positive_net,
                input.negative_net.as_deref().unwrap_or("GND")
            ));
        }
        if let Some(ref output) = request.output_port {
            lines.push(format!(
                "* Output: {} (+) {}",
                output.positive_net,
                output.negative_net.as_deref().unwrap_or("GND")
            ));
        }
        if let Some(ref ref_node) = request.reference_node {
            lines.push(format!("* Reference: {}", ref_node));
        }

        let boundary_nets = self.detect_boundary_nets(circuit, component_ids)?;
        if !boundary_nets.is_empty() {
            lines.push(format!(
                "* Boundary nets: {}",
                boundary_nets
                    .iter()
                    .map(|n| n.net_id.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        Ok(RegionNetlistFragment {
            title: "Selected Region SPICE Fragment".to_string(),
            format: "spice".to_string(),
            content: lines.join("\n"),
            warnings: vec!["This is a structural fragment only. Real ngspice integration is planned for a later version.".to_string()],
        })
    }

    fn transfer_function_for_template(
        &self,
        template: &MatchedRegionTemplate,
    ) -> Option<RegionTransferFunction> {
        match template.template_id.as_str() {
            "rc_low_pass_template" => Some(RegionTransferFunction {
                expression: "H(s) = 1 / (1 + s*R*C)".to_string(),
                latex: Some("H(s) = \\frac{1}{1 + sRC}".to_string()),
                output_name: "Vout/Vin".to_string(),
                unit: None,
                availability_note: Some(
                    "Transfer function is available for matched templates only in v1.6."
                        .to_string(),
                ),
            }),
            "voltage_divider" => Some(RegionTransferFunction {
                expression: "Vout = Vin * R2 / (R1 + R2)".to_string(),
                latex: Some("V_{out} = V_{in} \\frac{R_2}{R_1 + R_2}".to_string()),
                output_name: "Vout".to_string(),
                unit: Some(hotsas_core::EngineeringUnit::Volt),
                availability_note: Some(
                    "Transfer function is available for matched templates only in v1.6."
                        .to_string(),
                ),
            }),
            _ => None,
        }
    }

    fn graph_specs_for_template(&self, template: &MatchedRegionTemplate) -> Vec<RegionGraphSpec> {
        let available = template.template_id == "rc_low_pass_template";
        vec![
            RegionGraphSpec {
                id: "gain".to_string(),
                title: "Gain".to_string(),
                x_unit: Some(hotsas_core::EngineeringUnit::Hertz),
                y_unit: None,
                description: "Magnitude response".to_string(),
                available,
                unavailable_reason: if available {
                    None
                } else {
                    Some("Graph data is not generated in v1.6.".to_string())
                },
            },
            RegionGraphSpec {
                id: "phase".to_string(),
                title: "Phase".to_string(),
                x_unit: Some(hotsas_core::EngineeringUnit::Hertz),
                y_unit: Some(hotsas_core::EngineeringUnit::Unitless),
                description: "Phase response".to_string(),
                available,
                unavailable_reason: if available {
                    None
                } else {
                    Some("Graph data is not generated in v1.6.".to_string())
                },
            },
        ]
    }

    fn build_report_markdown(
        &self,
        region: &SelectedCircuitRegion,
        matched_template: &Option<MatchedRegionTemplate>,
        netlist_fragment: &Option<RegionNetlistFragment>,
        warnings: &[SelectedRegionIssue],
    ) -> String {
        let mut lines = vec![];
        lines.push("## Selected Region Analysis".to_string());
        lines.push(String::new());
        lines.push("Components:".to_string());
        for id in &region.component_ids {
            lines.push(format!("- {}", id));
        }
        lines.push(String::new());
        lines.push("Boundary nets:".to_string());
        for net in &region.boundary_nets {
            lines.push(format!("- {}", net));
        }
        if let Some(ref template) = matched_template {
            lines.push(String::new());
            lines.push(format!("Matched template: {}", template.title));
        }
        if !warnings.is_empty() {
            lines.push(String::new());
            lines.push("Warnings:".to_string());
            for w in warnings {
                lines.push(format!("- {}", w.message));
            }
        }
        if let Some(ref fragment) = netlist_fragment {
            lines.push(String::new());
            lines.push("SPICE fragment:".to_string());
            lines.push("```spice".to_string());
            lines.push(fragment.content.clone());
            lines.push("```".to_string());
        }
        lines.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubcircuitView {
    pub component_ids: Vec<String>,
    pub internal_nets: Vec<String>,
    pub boundary_nets: Vec<String>,
    pub external_nets: Vec<String>,
}
