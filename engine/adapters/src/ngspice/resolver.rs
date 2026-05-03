use hotsas_core::NgspiceAvailability;
use hotsas_ports::PortError;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct NgspiceBinaryResolver;

impl NgspiceBinaryResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve(&self) -> Result<NgspiceAvailability, PortError> {
        if let Ok(env_path) = std::env::var("HOTSAS_NGSPICE_PATH") {
            let path = PathBuf::from(&env_path);
            if path.is_file() {
                return self.check_version(&path);
            }
            return Ok(NgspiceAvailability {
                available: false,
                executable_path: Some(env_path),
                version: None,
                message: Some("HOTSAS_NGSPICE_PATH points to a missing file".to_string()),
                warnings: vec![],
            });
        }

        let candidates = if cfg!(target_os = "windows") {
            vec!["ngspice.exe", "ngspice"]
        } else {
            vec!["ngspice"]
        };

        for candidate in candidates {
            if let Ok(path) = Self::find_in_path(candidate) {
                return self.check_version(&path);
            }
        }

        Ok(NgspiceAvailability {
            available: false,
            executable_path: None,
            version: None,
            message: Some(
                "ngspice not found in PATH. Set HOTSAS_NGSPICE_PATH or install ngspice."
                    .to_string(),
            ),
            warnings: vec![],
        })
    }

    fn find_in_path(name: &str) -> Result<PathBuf, PortError> {
        let output = Command::new(if cfg!(target_os = "windows") {
            "where"
        } else {
            "which"
        })
        .arg(name)
        .output()
        .map_err(|e| PortError::Simulation(format!("path lookup failed: {e}")))?;

        if !output.status.success() {
            return Err(PortError::Simulation(format!("{name} not found in PATH")));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout
            .lines()
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| PortError::Simulation(format!("{name} not found in PATH")))
    }

    fn check_version(&self, path: &Path) -> Result<NgspiceAvailability, PortError> {
        let path_str = path.to_string_lossy().to_string();
        let output = Command::new(path).arg("--version").output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let version = if stdout.is_empty() {
                    None
                } else {
                    Some(stdout)
                };
                let mut warnings = vec![];
                if !output.status.success() {
                    warnings.push("ngspice --version returned non-zero exit code".to_string());
                }
                Ok(NgspiceAvailability {
                    available: true,
                    executable_path: Some(path_str),
                    version,
                    message: Some("ngspice is available".to_string()),
                    warnings,
                })
            }
            Err(e) => Ok(NgspiceAvailability {
                available: false,
                executable_path: Some(path_str),
                version: None,
                message: Some(format!("failed to run ngspice --version: {e}")),
                warnings: vec![],
            }),
        }
    }
}

impl Default for NgspiceBinaryResolver {
    fn default() -> Self {
        Self::new()
    }
}
