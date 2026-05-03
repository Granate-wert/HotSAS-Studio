use hotsas_core::{
    ImportedModelSource, ModelImportStatus, SpiceImportReport, SpiceModelDefinition,
    SpiceModelKind, SpiceModelParameter, SpiceSubcircuitDefinition,
};
use hotsas_ports::{PortError, SpiceModelParserPort};
use std::collections::HashMap;

pub struct SimpleSpiceModelParser;

impl SimpleSpiceModelParser {
    pub fn new() -> Self {
        Self
    }

    fn kind_from_type(type_str: &str) -> SpiceModelKind {
        match type_str.to_uppercase().as_str() {
            "D" => SpiceModelKind::Diode,
            "NPN" => SpiceModelKind::BjtNpn,
            "PNP" => SpiceModelKind::BjtPnp,
            "NMOS" => SpiceModelKind::MosfetN,
            "PMOS" => SpiceModelKind::MosfetP,
            "NJF" => SpiceModelKind::JfetN,
            "PJF" => SpiceModelKind::JfetP,
            "R" => SpiceModelKind::Resistor,
            "C" => SpiceModelKind::Capacitor,
            "L" => SpiceModelKind::Inductor,
            _ => SpiceModelKind::Unknown,
        }
    }

    fn heuristic_subckt_kind(name: &str) -> SpiceModelKind {
        let lower = name.to_lowercase();
        if lower.contains("opamp")
            || lower.contains("op_amp")
            || lower.contains("lm358")
            || lower.contains("tl072")
            || lower.contains("ua741")
        {
            SpiceModelKind::OpAmpMacroModel
        } else {
            SpiceModelKind::IcMacroModel
        }
    }

    fn parse_parameters(param_str: &str) -> Vec<SpiceModelParameter> {
        let mut params = Vec::new();
        let mut tokens = param_str.split_whitespace().peekable();

        while let Some(token) = tokens.next() {
            if let Some((key, value)) = token.split_once('=') {
                params.push(SpiceModelParameter {
                    name: key.to_string(),
                    value: value.to_string(),
                    unit_hint: None,
                });
            } else if let Some(next_token) = tokens.peek() {
                if next_token.starts_with('=') {
                    let value = if next_token.len() > 1 {
                        next_token[1..].to_string()
                    } else {
                        tokens.next();
                        tokens.next().unwrap_or("").to_string()
                    };
                    params.push(SpiceModelParameter {
                        name: token.to_string(),
                        value,
                        unit_hint: None,
                    });
                }
            }
        }
        params
    }
}

impl Default for SimpleSpiceModelParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SpiceModelParserPort for SimpleSpiceModelParser {
    fn parse_spice_models_from_str(
        &self,
        source_name: Option<String>,
        content: &str,
    ) -> Result<SpiceImportReport, PortError> {
        let source = ImportedModelSource {
            file_name: source_name.clone(),
            file_path: None,
            source_format: "spice".to_string(),
            content_hash: None,
        };

        let mut models: Vec<SpiceModelDefinition> = Vec::new();
        let mut subcircuits: Vec<SpiceSubcircuitDefinition> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        // Preprocess: handle line continuations and comments
        let lines: Vec<&str> = content.lines().collect();
        let mut logical_lines: Vec<String> = Vec::new();
        let mut idx = 0;
        while idx < lines.len() {
            let line = lines[idx].trim();
            if line.is_empty() {
                idx += 1;
                continue;
            }
            // Skip comment lines entirely
            if line.starts_with('*') || line.starts_with(';') {
                idx += 1;
                continue;
            }
            // Inline comments after content
            let line = line.splitn(2, ';').next().unwrap_or(line);
            let line = line.splitn(2, "$ ").next().unwrap_or(line);

            let mut current = line.to_string();
            idx += 1;
            // Handle continuation lines starting with '+'
            while idx < lines.len() {
                let next = lines[idx].trim();
                if next.starts_with('+') {
                    current.push(' ');
                    current.push_str(next[1..].trim());
                    idx += 1;
                } else {
                    break;
                }
            }
            logical_lines.push(current);
        }

        let mut in_subckt: Option<(String, Vec<String>, Vec<String>, usize)> = None;

        for (line_idx, line) in logical_lines.iter().enumerate() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            let directive = tokens[0].to_lowercase();

            match directive.as_str() {
                ".model" => {
                    if tokens.len() >= 3 {
                        let name = tokens[1].to_string();
                        let kind = Self::kind_from_type(tokens[2]);
                        if kind == SpiceModelKind::Unknown {
                            warnings.push(format!(
                                "unknown model type '{}' for model '{}'",
                                tokens[2], name
                            ));
                        }
                        let rest = line
                            .find(tokens[2])
                            .map(|pos| &line[pos + tokens[2].len()..])
                            .unwrap_or("");
                        let parameters = Self::parse_parameters(rest);
                        models.push(SpiceModelDefinition {
                            id: format!("spice-model-{line_idx}"),
                            name: name.clone(),
                            kind,
                            source: source.clone(),
                            raw_line: line.clone(),
                            parameters,
                            warnings: vec![],
                        });
                    } else {
                        warnings.push(format!("malformed .model line at logical line {line_idx}"));
                    }
                }
                ".subckt" => {
                    if tokens.len() >= 2 {
                        let name = tokens[1].to_string();
                        let pins = tokens[2..].iter().map(|s| s.to_string()).collect();
                        in_subckt = Some((name, pins, Vec::new(), line_idx));
                    } else {
                        warnings.push(format!("malformed .subckt line at logical line {line_idx}"));
                    }
                }
                ".ends" => {
                    if let Some((name, pins, body, start_idx)) = in_subckt.take() {
                        let detected_kind = Self::heuristic_subckt_kind(&name);
                        subcircuits.push(SpiceSubcircuitDefinition {
                            id: format!("spice-subckt-{start_idx}"),
                            name: name.clone(),
                            pins,
                            body,
                            source: source.clone(),
                            detected_kind,
                            parameters: vec![],
                            warnings: vec![],
                        });
                    } else {
                        warnings.push(format!(".ends without matching .subckt at line {line_idx}"));
                    }
                }
                ".include" | ".lib" | ".param" | ".func" | ".control" | ".measure" | ".option"
                | ".options" | ".ac" | ".dc" | ".tran" | ".op" | ".end" => {
                    warnings.push(format!("unsupported directive '{}' skipped", directive));
                }
                _ => {
                    // Body line inside subckt
                    if in_subckt.is_some() {
                        if let Some((_, _, ref mut body, _)) = in_subckt {
                            body.push(line.clone());
                        }
                    }
                }
            }
        }

        // Unclosed subckt
        if let Some((name, pins, body, start_idx)) = in_subckt {
            warnings.push(format!(
                "unclosed .subckt '{}' starting at logical line {start_idx}; treating as complete",
                name
            ));
            let detected_kind = Self::heuristic_subckt_kind(&name);
            subcircuits.push(SpiceSubcircuitDefinition {
                id: format!("spice-subckt-{start_idx}"),
                name,
                pins,
                body,
                source: source.clone(),
                detected_kind,
                parameters: vec![],
                warnings: vec![],
            });
        }

        let status = if !errors.is_empty() {
            ModelImportStatus::Failed
        } else if !warnings.is_empty() {
            ModelImportStatus::ParsedWithWarnings
        } else {
            ModelImportStatus::Parsed
        };

        Ok(SpiceImportReport {
            status,
            source,
            models,
            subcircuits,
            warnings,
            errors,
        })
    }
}
