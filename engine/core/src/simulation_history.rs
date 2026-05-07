use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationRunHistory {
    pub project_id: String,
    pub runs: Vec<SimulationRunHistoryEntry>,
    pub max_entries: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationRunHistoryEntry {
    pub run_id: String,
    pub profile_id: String,
    pub profile_name: String,
    pub analysis_type: String,
    pub engine_used: String,
    pub status: String,
    pub created_at: String,
    pub warnings_count: usize,
    pub errors_count: usize,
    pub series_count: usize,
    pub measurements_count: usize,
}
