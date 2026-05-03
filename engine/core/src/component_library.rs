use crate::{ComponentDefinition, FootprintDefinition, SimulationModel, SymbolDefinition};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentLibrary {
    pub id: String,
    pub title: String,
    pub version: String,
    pub components: Vec<ComponentDefinition>,
    pub symbols: Vec<SymbolDefinition>,
    pub footprints: Vec<FootprintDefinition>,
    pub simulation_models: Vec<SimulationModel>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentCategory {
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Led,
    OpAmp,
    Bjt,
    Mosfet,
    VoltageRegulator,
    Connector,
    Source,
    Ground,
    Generic,
}

impl std::fmt::Display for ComponentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resistor => write!(f, "resistor"),
            Self::Capacitor => write!(f, "capacitor"),
            Self::Inductor => write!(f, "inductor"),
            Self::Diode => write!(f, "diode"),
            Self::Led => write!(f, "led"),
            Self::OpAmp => write!(f, "opamp"),
            Self::Bjt => write!(f, "bjt"),
            Self::Mosfet => write!(f, "mosfet"),
            Self::VoltageRegulator => write!(f, "voltage_regulator"),
            Self::Connector => write!(f, "connector"),
            Self::Source => write!(f, "source"),
            Self::Ground => write!(f, "ground"),
            Self::Generic => write!(f, "generic"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentLibraryQuery {
    pub search: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub manufacturer: Option<String>,
    pub has_symbol: Option<bool>,
    pub has_footprint: Option<bool>,
    pub has_simulation_model: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentLibrarySearchResult {
    pub components: Vec<ComponentDefinition>,
    pub total_count: usize,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentAssignment {
    pub instance_id: String,
    pub component_definition_id: String,
    pub selected_symbol_id: Option<String>,
    pub selected_footprint_id: Option<String>,
    pub selected_simulation_model_id: Option<String>,
}
