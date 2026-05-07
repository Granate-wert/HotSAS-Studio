use crate::EngineeringUnit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationGraphView {
    pub run_id: String,
    pub title: String,
    pub x_axis: SimulationAxis,
    pub y_axis: SimulationAxis,
    pub series: Vec<SimulationGraphSeries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationAxis {
    pub label: String,
    pub unit: Option<EngineeringUnit>,
    pub scale: SimulationAxisScale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationAxisScale {
    Linear,
    Log,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationGraphSeries {
    pub id: String,
    pub label: String,
    pub visible_by_default: bool,
    pub points_count: usize,
}
