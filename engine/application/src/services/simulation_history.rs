use crate::ApplicationError;
use hotsas_core::{SimulationRunHistory, SimulationRunHistoryEntry, UserCircuitSimulationRun};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SimulationHistoryService {
    history: Arc<Mutex<BTreeMap<String, SimulationRunHistory>>>,
    max_entries: usize,
}

impl SimulationHistoryService {
    pub fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(BTreeMap::new())),
            max_entries: 50,
        }
    }

    pub fn add_run(&self, run: &UserCircuitSimulationRun) -> Result<(), ApplicationError> {
        let entry = SimulationRunHistoryEntry {
            run_id: run.id.clone(),
            profile_id: run.profile.id.clone(),
            profile_name: run.profile.name.clone(),
            analysis_type: format!("{:?}", run.profile.analysis_type),
            engine_used: run.engine_used.clone(),
            status: format!("{:?}", run.status),
            created_at: run.created_at.clone(),
            warnings_count: run.warnings.len(),
            errors_count: run.errors.len(),
            series_count: run.result.as_ref().map(|r| r.series.len()).unwrap_or(0),
            measurements_count: run.result.as_ref().map(|r| r.summary.len()).unwrap_or(0),
        };

        let mut guard = self
            .history
            .lock()
            .map_err(|_| ApplicationError::State("simulation history lock poisoned".to_string()))?;

        let history = guard
            .entry(run.project_id.clone())
            .or_insert_with(|| SimulationRunHistory {
                project_id: run.project_id.clone(),
                runs: vec![],
                max_entries: self.max_entries,
            });

        history.runs.push(entry);
        while history.runs.len() > self.max_entries {
            history.runs.remove(0);
        }

        Ok(())
    }

    pub fn list_runs(
        &self,
        project_id: &str,
    ) -> Result<Vec<SimulationRunHistoryEntry>, ApplicationError> {
        let guard = self
            .history
            .lock()
            .map_err(|_| ApplicationError::State("simulation history lock poisoned".to_string()))?;

        Ok(guard
            .get(project_id)
            .map(|h| h.runs.clone())
            .unwrap_or_default())
    }

    pub fn get_run(
        &self,
        project_id: &str,
        run_id: &str,
    ) -> Result<Option<SimulationRunHistoryEntry>, ApplicationError> {
        let guard = self
            .history
            .lock()
            .map_err(|_| ApplicationError::State("simulation history lock poisoned".to_string()))?;

        Ok(guard
            .get(project_id)
            .and_then(|h| h.runs.iter().find(|r| r.run_id == run_id).cloned()))
    }

    pub fn delete_run(&self, project_id: &str, run_id: &str) -> Result<(), ApplicationError> {
        let mut guard = self
            .history
            .lock()
            .map_err(|_| ApplicationError::State("simulation history lock poisoned".to_string()))?;

        if let Some(history) = guard.get_mut(project_id) {
            history.runs.retain(|r| r.run_id != run_id);
        }

        Ok(())
    }

    pub fn clear_runs(&self, project_id: &str) -> Result<(), ApplicationError> {
        let mut guard = self
            .history
            .lock()
            .map_err(|_| ApplicationError::State("simulation history lock poisoned".to_string()))?;

        if let Some(history) = guard.get_mut(project_id) {
            history.runs.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{
        EngineeringUnit, EngineeringValue, SimulationMeasurement, SimulationPoint,
        SimulationSeries, UserCircuitSimulationProfile, UserCircuitSimulationResult,
        UserCircuitSimulationRun, UserCircuitSimulationStatus, ValueWithUnit,
    };

    fn make_run(
        id: &str,
        project_id: &str,
        warnings: usize,
        errors: usize,
        series: usize,
        measurements: usize,
    ) -> UserCircuitSimulationRun {
        UserCircuitSimulationRun {
            id: id.to_string(),
            project_id: project_id.to_string(),
            profile: UserCircuitSimulationProfile {
                id: "p".to_string(),
                name: "Test".to_string(),
                analysis_type: hotsas_core::UserCircuitAnalysisType::AcSweep,
                engine: hotsas_core::UserCircuitSimulationEngine::Mock,
                probes: vec![],
                ac: None,
                transient: None,
                op: None,
            },
            generated_netlist: "".to_string(),
            status: UserCircuitSimulationStatus::Succeeded,
            engine_used: "mock".to_string(),
            warnings: (0..warnings)
                .map(|i| hotsas_core::SimulationWorkflowWarning {
                    code: format!("W{i}"),
                    message: "warn".to_string(),
                })
                .collect(),
            errors: (0..errors)
                .map(|i| hotsas_core::SimulationWorkflowError {
                    code: format!("E{i}"),
                    message: "err".to_string(),
                })
                .collect(),
            result: Some(UserCircuitSimulationResult {
                summary: (0..measurements)
                    .map(|i| SimulationMeasurement {
                        name: format!("M{i}"),
                        value: ValueWithUnit::new_si(1.0, EngineeringUnit::Volt),
                        unit_symbol: "V".to_string(),
                    })
                    .collect(),
                series: (0..series)
                    .map(|i| SimulationSeries {
                        id: format!("S{i}"),
                        label: format!("Series {i}"),
                        x_unit: None,
                        y_unit: None,
                        points: vec![SimulationPoint { x: 0.0, y: 0.0 }],
                    })
                    .collect(),
                raw_output_excerpt: None,
                netlist_hash: None,
            }),
            created_at: "now".to_string(),
        }
    }

    #[test]
    fn add_and_list_runs() {
        let svc = SimulationHistoryService::new();
        let run = make_run("r1", "proj-a", 1, 0, 2, 3);
        svc.add_run(&run).unwrap();
        let runs = svc.list_runs("proj-a").unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].run_id, "r1");
        assert_eq!(runs[0].warnings_count, 1);
        assert_eq!(runs[0].series_count, 2);
        assert_eq!(runs[0].measurements_count, 3);
    }

    #[test]
    fn list_runs_isolated_per_project() {
        let svc = SimulationHistoryService::new();
        svc.add_run(&make_run("r1", "proj-a", 0, 0, 0, 0)).unwrap();
        svc.add_run(&make_run("r2", "proj-b", 0, 0, 0, 0)).unwrap();
        assert_eq!(svc.list_runs("proj-a").unwrap().len(), 1);
        assert_eq!(svc.list_runs("proj-b").unwrap().len(), 1);
    }

    #[test]
    fn get_run_found() {
        let svc = SimulationHistoryService::new();
        svc.add_run(&make_run("r1", "proj-a", 0, 0, 0, 0)).unwrap();
        let run = svc.get_run("proj-a", "r1").unwrap();
        assert!(run.is_some());
    }

    #[test]
    fn get_run_not_found() {
        let svc = SimulationHistoryService::new();
        svc.add_run(&make_run("r1", "proj-a", 0, 0, 0, 0)).unwrap();
        let run = svc.get_run("proj-a", "r2").unwrap();
        assert!(run.is_none());
    }

    #[test]
    fn delete_run_removes_entry() {
        let svc = SimulationHistoryService::new();
        svc.add_run(&make_run("r1", "proj-a", 0, 0, 0, 0)).unwrap();
        svc.add_run(&make_run("r2", "proj-a", 0, 0, 0, 0)).unwrap();
        svc.delete_run("proj-a", "r1").unwrap();
        assert_eq!(svc.list_runs("proj-a").unwrap().len(), 1);
        assert!(svc.get_run("proj-a", "r1").unwrap().is_none());
    }

    #[test]
    fn clear_runs_empties_project() {
        let svc = SimulationHistoryService::new();
        svc.add_run(&make_run("r1", "proj-a", 0, 0, 0, 0)).unwrap();
        svc.clear_runs("proj-a").unwrap();
        assert_eq!(svc.list_runs("proj-a").unwrap().len(), 0);
    }

    #[test]
    fn max_entries_enforced() {
        let svc = SimulationHistoryService::new();
        for i in 0..55 {
            svc.add_run(&make_run(&format!("r{i}"), "proj-a", 0, 0, 0, 0))
                .unwrap();
        }
        let runs = svc.list_runs("proj-a").unwrap();
        assert_eq!(runs.len(), 50);
    }
}
