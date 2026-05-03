use hotsas_core::{
    ComplexValue, ImportedModelSource, ModelImportStatus, SParameterPoint, TouchstoneFrequencyUnit,
    TouchstoneImportReport, TouchstoneNetworkData, TouchstoneParameterFormat,
};
use hotsas_ports::{PortError, TouchstoneParserPort};

pub struct SimpleTouchstoneParser;

impl SimpleTouchstoneParser {
    pub fn new() -> Self {
        Self
    }

    fn port_count_from_name(name: &str) -> Option<usize> {
        let lower = name.to_lowercase();
        if lower.ends_with(".s1p") {
            Some(1)
        } else if lower.ends_with(".s2p") {
            Some(2)
        } else {
            None
        }
    }

    fn parse_option_line(
        line: &str,
    ) -> Option<(TouchstoneFrequencyUnit, TouchstoneParameterFormat, f64)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        let mut freq_unit = TouchstoneFrequencyUnit::GHz;
        let mut param_format = TouchstoneParameterFormat::MA;
        let mut ref_impedance = 50.0;

        for (i, part) in parts.iter().enumerate() {
            let upper = part.to_uppercase();
            match upper.as_str() {
                "HZ" => freq_unit = TouchstoneFrequencyUnit::Hz,
                "KHZ" => freq_unit = TouchstoneFrequencyUnit::KHz,
                "MHZ" => freq_unit = TouchstoneFrequencyUnit::MHz,
                "GHZ" => freq_unit = TouchstoneFrequencyUnit::GHz,
                "S" => {}                   // parameter type S, ignored
                "Y" | "Z" | "H" | "G" => {} // unsupported but present
                "RI" => param_format = TouchstoneParameterFormat::RI,
                "MA" => param_format = TouchstoneParameterFormat::MA,
                "DB" => param_format = TouchstoneParameterFormat::DB,
                "R" => {
                    if i + 1 < parts.len() {
                        if let Ok(val) = parts[i + 1].parse::<f64>() {
                            ref_impedance = val;
                        }
                    }
                }
                _ => {}
            }
        }
        Some((freq_unit, param_format, ref_impedance))
    }

    fn convert_to_complex(format: &TouchstoneParameterFormat, a: f64, b: f64) -> ComplexValue {
        match format {
            TouchstoneParameterFormat::RI => ComplexValue { re: a, im: b },
            TouchstoneParameterFormat::MA => {
                let rad = b.to_radians();
                ComplexValue {
                    re: a * rad.cos(),
                    im: a * rad.sin(),
                }
            }
            TouchstoneParameterFormat::DB => {
                let mag = 10f64.powf(a / 20.0);
                let rad = b.to_radians();
                ComplexValue {
                    re: mag * rad.cos(),
                    im: mag * rad.sin(),
                }
            }
        }
    }

    fn freq_to_hz(value: f64, unit: &TouchstoneFrequencyUnit) -> f64 {
        match unit {
            TouchstoneFrequencyUnit::Hz => value,
            TouchstoneFrequencyUnit::KHz => value * 1e3,
            TouchstoneFrequencyUnit::MHz => value * 1e6,
            TouchstoneFrequencyUnit::GHz => value * 1e9,
        }
    }
}

impl Default for SimpleTouchstoneParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchstoneParserPort for SimpleTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        source_name: Option<String>,
        content: &str,
    ) -> Result<TouchstoneImportReport, PortError> {
        let port_count = source_name
            .as_deref()
            .and_then(|n| Self::port_count_from_name(n))
            .unwrap_or(2);

        let mut warnings: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        let mut points: Vec<SParameterPoint> = Vec::new();

        let mut freq_unit = TouchstoneFrequencyUnit::GHz;
        let mut param_format = TouchstoneParameterFormat::MA;
        let mut ref_impedance = 50.0;
        let mut option_parsed = false;

        for (line_idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with('!') {
                continue;
            }
            if trimmed.starts_with('#') {
                if let Some((fu, pf, ri)) = Self::parse_option_line(trimmed) {
                    freq_unit = fu;
                    param_format = pf;
                    ref_impedance = ri;
                    option_parsed = true;
                } else {
                    warnings.push(format!("malformed option line at {line_idx}"));
                }
                continue;
            }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.len() < 2 {
                continue;
            }

            let freq_raw: f64 = match tokens[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    warnings.push(format!("invalid frequency value at line {line_idx}"));
                    continue;
                }
            };
            let frequency_hz = Self::freq_to_hz(freq_raw, &freq_unit);

            let expected_pairs = port_count * port_count;
            let expected_tokens = 1 + expected_pairs * 2;
            if tokens.len() != expected_tokens {
                warnings.push(format!(
                    "wrong column count at line {line_idx}: expected {expected_tokens}, got {}",
                    tokens.len()
                ));
                continue;
            }

            let mut values: Vec<ComplexValue> = Vec::with_capacity(expected_pairs);
            for i in 0..expected_pairs {
                let a: f64 = match tokens[1 + i * 2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        warnings.push(format!(
                            "invalid numeric at line {line_idx} column {}",
                            1 + i * 2
                        ));
                        continue;
                    }
                };
                let b: f64 = match tokens[2 + i * 2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        warnings.push(format!(
                            "invalid numeric at line {line_idx} column {}",
                            2 + i * 2
                        ));
                        continue;
                    }
                };
                values.push(Self::convert_to_complex(&param_format, a, b));
            }

            if values.len() == expected_pairs {
                points.push(SParameterPoint {
                    frequency_hz,
                    values,
                });
            }
        }

        if !option_parsed {
            warnings.push("missing option line; using defaults GHz / S / MA / R 50".to_string());
        }

        if points.is_empty() {
            errors.push("no valid data points found".to_string());
        }

        let status = if !errors.is_empty() {
            ModelImportStatus::Failed
        } else if !warnings.is_empty() {
            ModelImportStatus::ParsedWithWarnings
        } else {
            ModelImportStatus::Parsed
        };

        let network = if points.is_empty() {
            None
        } else {
            Some(TouchstoneNetworkData {
                id: "touchstone-0".to_string(),
                name: source_name
                    .clone()
                    .unwrap_or_else(|| "imported".to_string()),
                port_count,
                frequency_unit: freq_unit,
                parameter_format: param_format,
                reference_impedance_ohm: ref_impedance,
                points,
                source: ImportedModelSource {
                    file_name: source_name,
                    file_path: None,
                    source_format: "touchstone".to_string(),
                    content_hash: None,
                },
                warnings: vec![],
            })
        };

        Ok(TouchstoneImportReport {
            status,
            network,
            warnings,
            errors,
        })
    }
}
