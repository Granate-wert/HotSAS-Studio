use crate::ApplicationError;
use hotsas_core::CircuitProject;
use hotsas_ports::NetlistExporterPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct NetlistGenerationService {
    netlist_exporter: Arc<dyn NetlistExporterPort>,
}

impl NetlistGenerationService {
    pub fn new(netlist_exporter: Arc<dyn NetlistExporterPort>) -> Self {
        Self { netlist_exporter }
    }

    pub fn generate_spice_netlist(
        &self,
        project: &CircuitProject,
    ) -> Result<String, ApplicationError> {
        Ok(self.netlist_exporter.export_spice_netlist(project)?)
    }
}
