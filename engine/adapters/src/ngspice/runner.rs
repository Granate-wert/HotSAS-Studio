use hotsas_ports::PortError;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct NgspiceProcessRunner;

#[derive(Debug, Clone, PartialEq)]
pub struct NgspiceProcessResult {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub elapsed_ms: u64,
    pub timed_out: bool,
}

impl NgspiceProcessRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run(
        &self,
        executable: &Path,
        args: &[String],
        working_dir: &Path,
        timeout_ms: u64,
    ) -> Result<NgspiceProcessResult, PortError> {
        let start = Instant::now();
        let mut command = Command::new(executable);
        command
            .args(args)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command
            .spawn()
            .map_err(|e| PortError::Simulation(format!("failed to spawn ngspice: {e}")))?;

        let timeout = Duration::from_millis(timeout_ms);
        let result = loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let stdout = child
                        .stdout
                        .take()
                        .map(|mut o| {
                            let mut s = String::new();
                            std::io::Read::read_to_string(&mut o, &mut s)
                                .map(|_| s)
                                .unwrap_or_default()
                        })
                        .unwrap_or_default();
                    let stderr = child
                        .stderr
                        .take()
                        .map(|mut o| {
                            let mut s = String::new();
                            std::io::Read::read_to_string(&mut o, &mut s)
                                .map(|_| s)
                                .unwrap_or_default()
                        })
                        .unwrap_or_default();
                    break Ok(NgspiceProcessResult {
                        exit_code: status.code(),
                        stdout,
                        stderr,
                        elapsed_ms: start.elapsed().as_millis() as u64,
                        timed_out: false,
                    });
                }
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        let _ = child.kill();
                        break Ok(NgspiceProcessResult {
                            exit_code: None,
                            stdout: String::new(),
                            stderr: String::new(),
                            elapsed_ms: start.elapsed().as_millis() as u64,
                            timed_out: true,
                        });
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => {
                    break Err(PortError::Simulation(format!(
                        "failed to wait for ngspice: {e}"
                    )));
                }
            }
        };

        result
    }
}

impl Default for NgspiceProcessRunner {
    fn default() -> Self {
        Self::new()
    }
}
