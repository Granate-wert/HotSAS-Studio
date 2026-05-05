use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CliStatus {
    Success,
    ValidationError,
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize)]
pub struct CliOutput<T> {
    pub status: CliStatus,
    pub command: String,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub fn print_output<T: Serialize>(output: &CliOutput<T>, json: bool) {
    if json {
        match serde_json::to_string_pretty(output) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Failed to serialize JSON output: {}", e),
        }
    } else {
        let status_label = match output.status {
            CliStatus::Success => "✓ SUCCESS",
            CliStatus::ValidationError => "✗ VALIDATION ERROR",
            CliStatus::Error => "✗ ERROR",
            CliStatus::Warning => "⚠ WARNING",
        };
        println!("[{}] {}", status_label, output.command);
        if !output.warnings.is_empty() {
            println!("  Warnings:");
            for w in &output.warnings {
                println!("    - {}", w);
            }
        }
        if !output.errors.is_empty() {
            println!("  Errors:");
            for e in &output.errors {
                println!("    - {}", e);
            }
        }
        if let Some(ref data) = output.data {
            if let Ok(json_str) = serde_json::to_string_pretty(data) {
                println!("  Data:\n{}", indent(&json_str, 4));
            }
        }
    }
}

fn indent(text: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
