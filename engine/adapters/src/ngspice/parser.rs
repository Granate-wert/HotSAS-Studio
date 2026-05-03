use hotsas_core::GraphPoint;
use hotsas_ports::PortError;
use std::collections::BTreeMap;

pub struct NgspiceOutputParser;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedSeries {
    pub name: String,
    pub x_label: String,
    pub y_label: String,
    pub points: Vec<GraphPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedNgspiceOutput {
    pub series: Vec<ParsedSeries>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl NgspiceOutputParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_wrdata_file(&self, content: &str) -> Result<ParsedNgspiceOutput, PortError> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(PortError::Simulation(
                "ngspice output file is empty".to_string(),
            ));
        }

        let mut series_map: BTreeMap<usize, ParsedSeries> = BTreeMap::new();
        let mut warnings = vec![];
        let errors = vec![];

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('*') || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let x_val = match Self::parse_float(parts[0]) {
                Ok(v) => v,
                Err(e) => {
                    warnings.push(format!("could not parse x value '{}': {e}", parts[0]));
                    continue;
                }
            };

            for (col_idx, part) in parts.iter().skip(1).enumerate() {
                let y_val = match Self::parse_float(part) {
                    Ok(v) => v,
                    Err(e) => {
                        warnings.push(format!(
                            "could not parse y value '{}' in column {}: {e}",
                            part,
                            col_idx + 1
                        ));
                        continue;
                    }
                };

                let entry = series_map.entry(col_idx).or_insert_with(|| ParsedSeries {
                    name: format!("V{}", col_idx + 1),
                    x_label: "x".to_string(),
                    y_label: "y".to_string(),
                    points: vec![],
                });
                entry.points.push(GraphPoint { x: x_val, y: y_val });
            }
        }

        if series_map.is_empty() {
            return Err(PortError::Simulation(
                "no parseable data rows found in ngspice output".to_string(),
            ));
        }

        let series: Vec<ParsedSeries> = series_map.into_values().collect();
        Ok(ParsedNgspiceOutput {
            series,
            warnings,
            errors,
        })
    }

    pub fn parse_operating_point_stdout(
        &self,
        stdout: &str,
    ) -> Result<BTreeMap<String, f64>, PortError> {
        let mut measurements = BTreeMap::new();
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("v(") && trimmed.contains(')') {
                if let Some(close) = trimmed.find(')') {
                    let node = &trimmed[2..close];
                    if let Some(eq) = trimmed.find('=') {
                        let val_str = trimmed[eq + 1..].trim();
                        if let Ok(val) = Self::parse_float(val_str) {
                            measurements.insert(format!("v({node})"), val);
                        }
                    }
                }
            }
        }
        if measurements.is_empty() {
            return Err(PortError::Simulation(
                "no operating point values found in ngspice output".to_string(),
            ));
        }
        Ok(measurements)
    }

    fn parse_float(s: &str) -> Result<f64, String> {
        s.parse::<f64>()
            .map_err(|e| format!("invalid float '{s}': {e}"))
            .and_then(|v| {
                if v.is_finite() {
                    Ok(v)
                } else {
                    Err(format!("non-finite value: {s}"))
                }
            })
    }
}

impl Default for NgspiceOutputParser {
    fn default() -> Self {
        Self::new()
    }
}
