use crate::ApplicationError;
use hotsas_core::{
    SimulationAxis, SimulationAxisScale, SimulationGraphSeries, SimulationGraphView,
    UserCircuitSimulationRun,
};

#[derive(Clone)]
pub struct SimulationGraphService;

impl SimulationGraphService {
    pub fn new() -> Self {
        Self
    }

    pub fn build_graph_view(
        &self,
        run: &UserCircuitSimulationRun,
    ) -> Result<SimulationGraphView, ApplicationError> {
        let result = run
            .result
            .as_ref()
            .ok_or_else(|| ApplicationError::MissingProjectState("no result in run".to_string()))?;

        let x_axis = if let Some(first_series) = result.series.first() {
            SimulationAxis {
                label: first_series
                    .x_unit
                    .as_ref()
                    .map(|u| u.symbol().to_string())
                    .unwrap_or_else(|| "X".to_string()),
                unit: first_series.x_unit.clone(),
                scale: SimulationAxisScale::Log,
            }
        } else {
            SimulationAxis {
                label: "X".to_string(),
                unit: None,
                scale: SimulationAxisScale::Linear,
            }
        };

        let y_axis = SimulationAxis {
            label: "Value".to_string(),
            unit: None,
            scale: SimulationAxisScale::Linear,
        };

        let series: Vec<SimulationGraphSeries> = result
            .series
            .iter()
            .map(|s| SimulationGraphSeries {
                id: s.id.clone(),
                label: s.label.clone(),
                visible_by_default: true,
                points_count: s.points.len(),
            })
            .collect();

        Ok(SimulationGraphView {
            run_id: run.id.clone(),
            title: format!("{} — {}", run.profile.name, run.engine_used),
            x_axis,
            y_axis,
            series,
        })
    }

    pub fn export_run_series_csv(
        &self,
        run: &UserCircuitSimulationRun,
    ) -> Result<String, ApplicationError> {
        let result = run
            .result
            .as_ref()
            .ok_or_else(|| ApplicationError::MissingProjectState("no result in run".to_string()))?;

        let mut lines = vec!["series_id,series_label,x,y".to_string()];
        for series in &result.series {
            for point in &series.points {
                lines.push(format!(
                    "{},{},{},{}",
                    series.id, series.label, point.x, point.y
                ));
            }
        }
        Ok(lines.join("\n"))
    }

    pub fn export_run_series_json(
        &self,
        run: &UserCircuitSimulationRun,
    ) -> Result<String, ApplicationError> {
        let result = run
            .result
            .as_ref()
            .ok_or_else(|| ApplicationError::MissingProjectState("no result in run".to_string()))?;

        let json = serde_json::json!({
            "run_id": run.id,
            "profile": run.profile.name,
            "engine": run.engine_used,
            "series": result.series.iter().map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "label": s.label,
                    "points": s.points.iter().map(|p| {
                        serde_json::json!({"x": p.x, "y": p.y})
                    }).collect::<Vec<_>>(),
                })
            }).collect::<Vec<_>>(),
        });

        serde_json::to_string_pretty(&json)
            .map_err(|e| ApplicationError::Export(format!("JSON serialization failed: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{
        EngineeringUnit, SimulationPoint, SimulationSeries, UserCircuitSimulationProfile,
        UserCircuitSimulationResult, UserCircuitSimulationRun, UserCircuitSimulationStatus,
        ValueWithUnit,
    };

    fn make_run_with_series() -> UserCircuitSimulationRun {
        UserCircuitSimulationRun {
            id: "run-1".to_string(),
            project_id: "proj".to_string(),
            profile: UserCircuitSimulationProfile {
                id: "p".to_string(),
                name: "AC Sweep".to_string(),
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
            warnings: vec![],
            errors: vec![],
            result: Some(UserCircuitSimulationResult {
                summary: vec![],
                series: vec![SimulationSeries {
                    id: "s1".to_string(),
                    label: "V(out)".to_string(),
                    x_unit: Some(EngineeringUnit::Hertz),
                    y_unit: Some(EngineeringUnit::Volt),
                    points: vec![
                        SimulationPoint { x: 1.0, y: 2.0 },
                        SimulationPoint { x: 10.0, y: 3.0 },
                    ],
                }],
                raw_output_excerpt: None,
                netlist_hash: None,
            }),
            created_at: "now".to_string(),
        }
    }

    fn make_run_without_result() -> UserCircuitSimulationRun {
        UserCircuitSimulationRun {
            id: "run-2".to_string(),
            project_id: "proj".to_string(),
            profile: UserCircuitSimulationProfile {
                id: "p".to_string(),
                name: "OP".to_string(),
                analysis_type: hotsas_core::UserCircuitAnalysisType::OperatingPoint,
                engine: hotsas_core::UserCircuitSimulationEngine::Mock,
                probes: vec![],
                ac: None,
                transient: None,
                op: None,
            },
            generated_netlist: "".to_string(),
            status: UserCircuitSimulationStatus::Failed,
            engine_used: "mock".to_string(),
            warnings: vec![],
            errors: vec![],
            result: None,
            created_at: "now".to_string(),
        }
    }

    #[test]
    fn build_graph_view_ok() {
        let svc = SimulationGraphService::new();
        let run = make_run_with_series();
        let view = svc.build_graph_view(&run).unwrap();
        assert_eq!(view.run_id, "run-1");
        assert_eq!(view.title, "AC Sweep — mock");
        assert_eq!(view.series.len(), 1);
        assert_eq!(view.series[0].points_count, 2);
        assert_eq!(view.x_axis.label, "Hz");
    }

    #[test]
    fn build_graph_view_no_result_fails() {
        let svc = SimulationGraphService::new();
        let run = make_run_without_result();
        assert!(svc.build_graph_view(&run).is_err());
    }

    #[test]
    fn export_csv_ok() {
        let svc = SimulationGraphService::new();
        let run = make_run_with_series();
        let csv = svc.export_run_series_csv(&run).unwrap();
        assert!(csv.starts_with("series_id,series_label,x,y"));
        assert!(csv.contains("s1,V(out),1,2"));
        assert!(csv.contains("s1,V(out),10,3"));
    }

    #[test]
    fn export_csv_no_result_fails() {
        let svc = SimulationGraphService::new();
        let run = make_run_without_result();
        assert!(svc.export_run_series_csv(&run).is_err());
    }

    #[test]
    fn export_json_ok() {
        let svc = SimulationGraphService::new();
        let run = make_run_with_series();
        let json = svc.export_run_series_json(&run).unwrap();
        assert!(json.contains("\"run_id\": \"run-1\""));
        assert!(json.contains("\"profile\": \"AC Sweep\""));
        assert!(json.contains("\"id\": \"s1\""));
    }

    #[test]
    fn export_json_no_result_fails() {
        let svc = SimulationGraphService::new();
        let run = make_run_without_result();
        assert!(svc.export_run_series_json(&run).is_err());
    }
}
