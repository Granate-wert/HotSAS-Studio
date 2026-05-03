use hotsas_core::SimulationAnalysisKind;
use hotsas_ports::PortError;

pub struct NgspiceControlBlockBuilder;

impl NgspiceControlBlockBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_control_block(
        &self,
        kind: &SimulationAnalysisKind,
        netlist: &str,
        output_variables: &[String],
    ) -> Result<String, PortError> {
        let mut result = netlist.to_string();

        // Remove any existing .end if present so we can append control block
        if result.trim_end().ends_with(".end") {
            result = result
                .trim_end()
                .trim_end_matches(".end")
                .trim_end()
                .to_string();
        }

        match kind {
            SimulationAnalysisKind::OperatingPoint => {
                result.push_str("\n.control\nop\nprint all\n.endc\n.end");
            }
            SimulationAnalysisKind::AcSweep => {
                let outputs = if output_variables.is_empty() {
                    "v(net_out)"
                } else {
                    &output_variables.join(" ")
                };
                result.push_str(&format!(
                    "\n.control\nset filetype=ascii\nrun\nwrdata ac_output.csv frequency {outputs}\n.endc\n.end"
                ));
            }
            SimulationAnalysisKind::Transient => {
                let outputs = if output_variables.is_empty() {
                    "time v(net_in) v(net_out)"
                } else {
                    &format!("time {}", output_variables.join(" "))
                };
                result.push_str(&format!(
                    "\n.control\nset filetype=ascii\nrun\nwrdata tran_output.csv {outputs}\n.endc\n.end"
                ));
            }
            SimulationAnalysisKind::DcSweep => {
                return Err(PortError::Simulation(
                    "DC sweep not yet supported in v1.8".to_string(),
                ));
            }
        }

        Ok(result)
    }
}

impl Default for NgspiceControlBlockBuilder {
    fn default() -> Self {
        Self::new()
    }
}
