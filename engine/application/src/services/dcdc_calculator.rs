use crate::ApplicationError;
use hotsas_core::{
    DcdcCalculationResult, DcdcComputedValue, DcdcInput, DcdcOperatingMode, DcdcSimulationPlan,
    DcdcTemplateDefinition, DcdcTopology, DcdcWarning, DcdcWarningSeverity, EngineeringUnit,
    GraphPoint, GraphSeries, SimulationResult, SimulationStatus, ValueWithUnit,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct DcdcCalculatorService;

impl DcdcCalculatorService {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_dcdc(
        &self,
        input: DcdcInput,
    ) -> Result<DcdcCalculationResult, ApplicationError> {
        input.validate().map_err(ApplicationError::Core)?;
        match input.topology {
            DcdcTopology::Buck => self.calculate_buck(input),
            DcdcTopology::Boost => self.calculate_boost(input),
            DcdcTopology::InvertingBuckBoost => self.calculate_inverting_buck_boost(input),
            DcdcTopology::FourSwitchBuckBoost => self.calculate_four_switch_placeholder(input),
        }
    }

    fn calculate_buck(&self, input: DcdcInput) -> Result<DcdcCalculationResult, ApplicationError> {
        let vin = input.vin.si_value();
        let vout = input.vout.si_value();
        let iout = input.iout.si_value();
        let fs = input.switching_frequency.si_value();

        if vout >= vin {
            return Err(ApplicationError::InvalidInput(
                "Buck: Vout must be less than Vin".to_string(),
            ));
        }

        let duty = vout / vin;
        let efficiency = input.estimated_efficiency_percent.unwrap_or(90.0) / 100.0;
        let pin = vout * iout / efficiency;
        let iin = pin / vin;

        let mut values = Vec::new();
        let mut warnings = Vec::new();

        values.push(DcdcComputedValue {
            id: "duty_cycle".to_string(),
            label: "Duty Cycle".to_string(),
            value: ValueWithUnit::new_si(duty, EngineeringUnit::Unitless),
            formula: Some("D = Vout / Vin".to_string()),
            description: Some("Buck duty cycle".to_string()),
        });

        values.push(DcdcComputedValue {
            id: "input_current".to_string(),
            label: "Input Current".to_string(),
            value: ValueWithUnit::new_si(iin, EngineeringUnit::Ampere),
            formula: Some("Iin = Pout / (Vin * η)".to_string()),
            description: Some("Average input current".to_string()),
        });

        let (delta_il, _l_used) = if let Some(ref l_val) = input.inductor {
            let l = l_val.si_value();
            let di = (vin - vout) * duty / (l * fs);
            values.push(DcdcComputedValue {
                id: "inductor_ripple".to_string(),
                label: "Inductor Ripple Current".to_string(),
                value: ValueWithUnit::new_si(di, EngineeringUnit::Ampere),
                formula: Some("ΔIL = (Vin - Vout) * D / (L * fs)".to_string()),
                description: Some("Peak-to-peak inductor ripple".to_string()),
            });
            (di, l)
        } else {
            let ripple_percent = input.target_inductor_ripple_percent.unwrap_or(30.0);
            let di_target = iout * ripple_percent / 100.0;
            let l_min = (vin - vout) * duty / (di_target * fs);
            values.push(DcdcComputedValue {
                id: "min_inductance".to_string(),
                label: "Minimum Inductance".to_string(),
                value: ValueWithUnit::new_si(l_min, EngineeringUnit::Henry),
                formula: Some("Lmin = (Vin - Vout) * D / (ΔIL_target * fs)".to_string()),
                description: Some("Minimum inductance for target ripple".to_string()),
            });
            (di_target, l_min)
        };

        let i_switch_peak = iout + delta_il / 2.0;
        values.push(DcdcComputedValue {
            id: "switch_peak_current".to_string(),
            label: "Switch Peak Current".to_string(),
            value: ValueWithUnit::new_si(i_switch_peak, EngineeringUnit::Ampere),
            formula: Some("Isw_peak ≈ Iout + ΔIL / 2".to_string()),
            description: Some("Peak switch current".to_string()),
        });

        let i_boundary = delta_il / 2.0;
        values.push(DcdcComputedValue {
            id: "ccm_boundary".to_string(),
            label: "CCM/DCM Boundary".to_string(),
            value: ValueWithUnit::new_si(i_boundary, EngineeringUnit::Ampere),
            formula: Some("I_boundary ≈ ΔIL / 2".to_string()),
            description: Some("Current at CCM/DCM boundary".to_string()),
        });

        let mode = if iout > i_boundary {
            DcdcOperatingMode::Ccm
        } else if iout < i_boundary {
            DcdcOperatingMode::Dcm
        } else {
            DcdcOperatingMode::Boundary
        };

        if duty > 0.9 {
            warnings.push(DcdcWarning {
                code: "high_duty_cycle".to_string(),
                message: "Duty cycle is very high; consider topology or input voltage".to_string(),
                severity: DcdcWarningSeverity::Warning,
            });
        }
        if iout < i_boundary {
            warnings.push(DcdcWarning {
                code: "possible_dcm".to_string(),
                message: "Operating point may be in DCM".to_string(),
                severity: DcdcWarningSeverity::Info,
            });
        }

        if let Some(ref c_val) = input.output_capacitor {
            let c = c_val.si_value();
            let delta_v = delta_il / (8.0 * fs * c);
            values.push(DcdcComputedValue {
                id: "output_ripple".to_string(),
                label: "Output Voltage Ripple".to_string(),
                value: ValueWithUnit::new_si(delta_v, EngineeringUnit::Volt),
                formula: Some("ΔVout ≈ ΔIL / (8 * fs * Cout)".to_string()),
                description: Some("Estimated output voltage ripple".to_string()),
            });
        }

        let sim_plan = DcdcSimulationPlan {
            id: "buck-transient".to_string(),
            title: "Buck Transient".to_string(),
            profile_type: "transient".to_string(),
            recommended_stop_time: ValueWithUnit::new_si(5.0 / fs, EngineeringUnit::Second),
            recommended_time_step: Some(ValueWithUnit::new_si(
                1.0 / (fs * 100.0),
                EngineeringUnit::Second,
            )),
            signals: vec![
                "vout".to_string(),
                "il".to_string(),
                "switch_node".to_string(),
            ],
            notes: vec!["Ideal switch/control placeholder".to_string()],
        };

        Ok(DcdcCalculationResult {
            topology: DcdcTopology::Buck,
            operating_mode: mode,
            inputs: input,
            values,
            assumptions: vec![
                "Ideal switch and diode".to_string(),
                "Continuous inductor current (unless noted)".to_string(),
                "Negligible capacitor ESR".to_string(),
            ],
            limitations: vec![
                "No real control loop model".to_string(),
                "No parasitic resistances".to_string(),
                "No switching losses".to_string(),
            ],
            warnings,
            simulation_plan: Some(sim_plan),
            template_id: Some("buck_converter_template".to_string()),
        })
    }

    fn calculate_boost(&self, input: DcdcInput) -> Result<DcdcCalculationResult, ApplicationError> {
        let vin = input.vin.si_value();
        let vout = input.vout.si_value();
        let iout = input.iout.si_value();
        let fs = input.switching_frequency.si_value();

        if vout <= vin {
            return Err(ApplicationError::InvalidInput(
                "Boost: Vout must be greater than Vin".to_string(),
            ));
        }

        let duty = 1.0 - vin / vout;
        let efficiency = input.estimated_efficiency_percent.unwrap_or(90.0) / 100.0;
        let pout = vout * iout;
        let iin = pout / (vin * efficiency);

        let mut values = Vec::new();
        let mut warnings = Vec::new();

        values.push(DcdcComputedValue {
            id: "duty_cycle".to_string(),
            label: "Duty Cycle".to_string(),
            value: ValueWithUnit::new_si(duty, EngineeringUnit::Unitless),
            formula: Some("D = 1 - Vin / Vout".to_string()),
            description: Some("Boost duty cycle".to_string()),
        });

        values.push(DcdcComputedValue {
            id: "input_current".to_string(),
            label: "Input/Inductor Average Current".to_string(),
            value: ValueWithUnit::new_si(iin, EngineeringUnit::Ampere),
            formula: Some("Iin ≈ Pout / (Vin * η)".to_string()),
            description: Some("Average input/inductor current".to_string()),
        });

        let (delta_il, _l_used) = if let Some(ref l_val) = input.inductor {
            let l = l_val.si_value();
            let di = vin * duty / (l * fs);
            values.push(DcdcComputedValue {
                id: "inductor_ripple".to_string(),
                label: "Inductor Ripple Current".to_string(),
                value: ValueWithUnit::new_si(di, EngineeringUnit::Ampere),
                formula: Some("ΔIL = Vin * D / (L * fs)".to_string()),
                description: Some("Peak-to-peak inductor ripple".to_string()),
            });
            (di, l)
        } else {
            let ripple_percent = input.target_inductor_ripple_percent.unwrap_or(30.0);
            let di_target = iin * ripple_percent / 100.0;
            let l_min = vin * duty / (di_target * fs);
            values.push(DcdcComputedValue {
                id: "min_inductance".to_string(),
                label: "Minimum Inductance".to_string(),
                value: ValueWithUnit::new_si(l_min, EngineeringUnit::Henry),
                formula: Some("Lmin = Vin * D / (ΔIL_target * fs)".to_string()),
                description: Some("Minimum inductance for target ripple".to_string()),
            });
            (di_target, l_min)
        };

        let i_switch_peak = iin + delta_il / 2.0;
        values.push(DcdcComputedValue {
            id: "switch_peak_current".to_string(),
            label: "Switch Peak Current".to_string(),
            value: ValueWithUnit::new_si(i_switch_peak, EngineeringUnit::Ampere),
            formula: Some("Isw_peak ≈ Iin + ΔIL / 2".to_string()),
            description: Some("Peak switch current".to_string()),
        });

        values.push(DcdcComputedValue {
            id: "diode_avg_current".to_string(),
            label: "Diode Average Current".to_string(),
            value: ValueWithUnit::new_si(iout, EngineeringUnit::Ampere),
            formula: Some("I_diode_avg ≈ Iout".to_string()),
            description: Some("Average diode current".to_string()),
        });

        let i_boundary = delta_il / 2.0;
        let mode = if iin > i_boundary {
            DcdcOperatingMode::Ccm
        } else if iin < i_boundary {
            DcdcOperatingMode::Dcm
        } else {
            DcdcOperatingMode::Boundary
        };

        if duty > 0.85 {
            warnings.push(DcdcWarning {
                code: "high_duty_cycle".to_string(),
                message: "Boost duty cycle is very high".to_string(),
                severity: DcdcWarningSeverity::Warning,
            });
        }

        if let Some(ref c_val) = input.output_capacitor {
            let c = c_val.si_value();
            let delta_v = iout * duty / (fs * c);
            values.push(DcdcComputedValue {
                id: "output_ripple".to_string(),
                label: "Output Voltage Ripple".to_string(),
                value: ValueWithUnit::new_si(delta_v, EngineeringUnit::Volt),
                formula: Some("ΔVout ≈ Iout * D / (fs * Cout)".to_string()),
                description: Some("Estimated output voltage ripple".to_string()),
            });
        }

        Ok(DcdcCalculationResult {
            topology: DcdcTopology::Boost,
            operating_mode: mode,
            inputs: input,
            values,
            assumptions: vec![
                "Ideal switch and diode".to_string(),
                "Negligible capacitor ESR".to_string(),
            ],
            limitations: vec![
                "No real control loop model".to_string(),
                "No parasitic resistances".to_string(),
                "No switching losses".to_string(),
            ],
            warnings,
            simulation_plan: Some(DcdcSimulationPlan {
                id: "boost-transient".to_string(),
                title: "Boost Transient".to_string(),
                profile_type: "transient".to_string(),
                recommended_stop_time: ValueWithUnit::new_si(5.0 / fs, EngineeringUnit::Second),
                recommended_time_step: Some(ValueWithUnit::new_si(
                    1.0 / (fs * 100.0),
                    EngineeringUnit::Second,
                )),
                signals: vec![
                    "vout".to_string(),
                    "il".to_string(),
                    "switch_node".to_string(),
                ],
                notes: vec!["Ideal switch/control placeholder".to_string()],
            }),
            template_id: Some("boost_converter_template".to_string()),
        })
    }

    fn calculate_inverting_buck_boost(
        &self,
        input: DcdcInput,
    ) -> Result<DcdcCalculationResult, ApplicationError> {
        let vin = input.vin.si_value();
        let vout_raw = input.vout.si_value();
        let vout_mag = vout_raw.abs();
        let iout = input.iout.si_value();
        let fs = input.switching_frequency.si_value();

        if vout_raw > 0.0 {
            return Err(ApplicationError::InvalidInput(
                "Inverting Buck-Boost: Vout should be negative (or use magnitude mode)".to_string(),
            ));
        }

        let duty = vout_mag / (vout_mag + vin);
        let efficiency = input.estimated_efficiency_percent.unwrap_or(90.0) / 100.0;
        let pout = vout_mag * iout;
        let iin = pout / (vin * efficiency);

        let mut values = Vec::new();
        let mut warnings = Vec::new();

        values.push(DcdcComputedValue {
            id: "duty_cycle".to_string(),
            label: "Duty Cycle".to_string(),
            value: ValueWithUnit::new_si(duty, EngineeringUnit::Unitless),
            formula: Some("D = |Vout| / (|Vout| + Vin)".to_string()),
            description: Some("Inverting buck-boost duty cycle".to_string()),
        });

        values.push(DcdcComputedValue {
            id: "input_current".to_string(),
            label: "Input Current".to_string(),
            value: ValueWithUnit::new_si(iin, EngineeringUnit::Ampere),
            formula: Some("Iin ≈ Pout / (Vin * η)".to_string()),
            description: Some("Average input current".to_string()),
        });

        let delta_il = if let Some(ref l_val) = input.inductor {
            let l = l_val.si_value();
            let di = vin * duty / (l * fs);
            values.push(DcdcComputedValue {
                id: "inductor_ripple".to_string(),
                label: "Inductor Ripple Current".to_string(),
                value: ValueWithUnit::new_si(di, EngineeringUnit::Ampere),
                formula: Some("ΔIL = Vin * D / (L * fs)".to_string()),
                description: Some("Peak-to-peak inductor ripple".to_string()),
            });
            di
        } else {
            0.0
        };

        let i_switch_peak = iin + delta_il / 2.0;
        values.push(DcdcComputedValue {
            id: "switch_peak_current".to_string(),
            label: "Switch Peak Current".to_string(),
            value: ValueWithUnit::new_si(i_switch_peak, EngineeringUnit::Ampere),
            formula: Some("Isw_peak ≈ Iin + ΔIL / 2".to_string()),
            description: Some("Peak switch current".to_string()),
        });

        warnings.push(DcdcWarning {
            code: "inverting_bb_first_order".to_string(),
            message: "Inverting buck-boost model is a first-order ideal estimate in v2.2."
                .to_string(),
            severity: DcdcWarningSeverity::Info,
        });

        Ok(DcdcCalculationResult {
            topology: DcdcTopology::InvertingBuckBoost,
            operating_mode: DcdcOperatingMode::Unknown,
            inputs: input,
            values,
            assumptions: vec![
                "Ideal switch and diode".to_string(),
                "First-order ideal estimate".to_string(),
            ],
            limitations: vec![
                "No real control loop model".to_string(),
                "No parasitic resistances".to_string(),
                "No switching losses".to_string(),
                "Inverting topology is simplified".to_string(),
            ],
            warnings,
            simulation_plan: Some(DcdcSimulationPlan {
                id: "inverting-bb-transient".to_string(),
                title: "Inverting Buck-Boost Transient".to_string(),
                profile_type: "transient".to_string(),
                recommended_stop_time: ValueWithUnit::new_si(5.0 / fs, EngineeringUnit::Second),
                recommended_time_step: Some(ValueWithUnit::new_si(
                    1.0 / (fs * 100.0),
                    EngineeringUnit::Second,
                )),
                signals: vec![
                    "vout".to_string(),
                    "il".to_string(),
                    "switch_node".to_string(),
                ],
                notes: vec!["Ideal switch/control placeholder".to_string()],
            }),
            template_id: Some("inverting_buck_boost_template".to_string()),
        })
    }

    fn calculate_four_switch_placeholder(
        &self,
        input: DcdcInput,
    ) -> Result<DcdcCalculationResult, ApplicationError> {
        let vin = input.vin.si_value();
        let vout = input.vout.si_value();

        let mode_hint = if vin > vout * 1.1 {
            "buck_region"
        } else if vin * 1.1 < vout {
            "boost_region"
        } else {
            "pass_through_region"
        };

        let mut values = Vec::new();
        values.push(DcdcComputedValue {
            id: "mode_hint".to_string(),
            label: "Operating Region Hint".to_string(),
            value: ValueWithUnit::new_si(0.0, EngineeringUnit::Unitless),
            formula: None,
            description: Some(format!("Detected region: {mode_hint}").to_string()),
        });

        Ok(DcdcCalculationResult {
            topology: DcdcTopology::FourSwitchBuckBoost,
            operating_mode: DcdcOperatingMode::Unknown,
            inputs: input,
            values,
            assumptions: vec!["4-switch topology placeholder".to_string()],
            limitations: vec![
                "No real 4-switch control model in v2.2".to_string(),
                "Control strategy not modeled".to_string(),
            ],
            warnings: vec![
                DcdcWarning {
                    code: "four_switch_placeholder".to_string(),
                    message: "4-switch buck-boost is a placeholder in v2.2.".to_string(),
                    severity: DcdcWarningSeverity::Warning,
                },
                DcdcWarning {
                    code: "control_strategy_not_modeled".to_string(),
                    message: "Control strategy is not modeled.".to_string(),
                    severity: DcdcWarningSeverity::Warning,
                },
            ],
            simulation_plan: None,
            template_id: Some("four_switch_buck_boost_placeholder_template".to_string()),
        })
    }

    pub fn list_dcdc_templates(&self) -> Vec<DcdcTemplateDefinition> {
        vec![
            DcdcTemplateDefinition {
                id: "buck_converter_template".to_string(),
                title: "Buck Converter".to_string(),
                topology: DcdcTopology::Buck,
                description: "Step-down DC-DC converter with ideal switch/diode.".to_string(),
                supported_outputs: vec![
                    "duty_cycle".to_string(),
                    "inductor_ripple".to_string(),
                    "switch_peak_current".to_string(),
                    "output_ripple".to_string(),
                ],
                limitations: vec![
                    "No real control loop".to_string(),
                    "Ideal components".to_string(),
                ],
            },
            DcdcTemplateDefinition {
                id: "boost_converter_template".to_string(),
                title: "Boost Converter".to_string(),
                topology: DcdcTopology::Boost,
                description: "Step-up DC-DC converter with ideal switch/diode.".to_string(),
                supported_outputs: vec![
                    "duty_cycle".to_string(),
                    "inductor_ripple".to_string(),
                    "switch_peak_current".to_string(),
                    "output_ripple".to_string(),
                ],
                limitations: vec![
                    "No real control loop".to_string(),
                    "Ideal components".to_string(),
                ],
            },
            DcdcTemplateDefinition {
                id: "inverting_buck_boost_template".to_string(),
                title: "Inverting Buck-Boost".to_string(),
                topology: DcdcTopology::InvertingBuckBoost,
                description: "Inverting DC-DC converter (negative output).".to_string(),
                supported_outputs: vec![
                    "duty_cycle".to_string(),
                    "switch_peak_current".to_string(),
                ],
                limitations: vec![
                    "First-order estimate only".to_string(),
                    "No real control loop".to_string(),
                ],
            },
            DcdcTemplateDefinition {
                id: "four_switch_buck_boost_placeholder_template".to_string(),
                title: "4-Switch Buck-Boost (Placeholder)".to_string(),
                topology: DcdcTopology::FourSwitchBuckBoost,
                description: "4-switch buck-boost placeholder for future expansion.".to_string(),
                supported_outputs: vec!["mode_hint".to_string()],
                limitations: vec![
                    "No real control model".to_string(),
                    "No calculations performed".to_string(),
                ],
            },
        ]
    }

    pub fn generate_dcdc_netlist_preview(
        &self,
        topology: DcdcTopology,
        _input: &DcdcInput,
    ) -> Result<String, ApplicationError> {
        let header = "* HotSAS Studio DC-DC netlist preview\n* Limitations: ideal switch/control placeholder\n";
        let body = match topology {
            DcdcTopology::Buck => {
                "* Buck Converter\nV1 vin gnd DC 12\nS1 vin switch_node ideal_switch\nD1 switch_node vout ideal_diode\nL1 switch_node vout 47u\nC1 vout gnd 100u\nRload vout gnd 5\n"
            }
            DcdcTopology::Boost => {
                "* Boost Converter\nV1 vin gnd DC 5\nL1 vin switch_node 47u\nS1 switch_node gnd ideal_switch\nD1 switch_node vout ideal_diode\nC1 vout gnd 100u\nRload vout gnd 12\n"
            }
            DcdcTopology::InvertingBuckBoost => {
                "* Inverting Buck-Boost\nV1 vin gnd DC 12\nS1 vin switch_node ideal_switch\nD1 switch_node vout ideal_diode\nL1 switch_node vout 47u\nC1 vout gnd 100u\nRload vout gnd 5\n"
            }
            DcdcTopology::FourSwitchBuckBoost => {
                "* 4-Switch Buck-Boost (Placeholder)\n* No real control model in v2.2\nV1 vin gnd DC 12\nS1 vin switch_node ideal_switch_hs\nS2 switch_node gnd ideal_switch_ls\nS3 vout switch_node ideal_switch_hs2\nS4 switch_node gnd ideal_switch_ls2\nL1 switch_node mid 47u\nC1 vout gnd 100u\nRload vout gnd 5\n"
            }
        };
        Ok(format!("{header}\n{body}"))
    }

    pub fn create_dcdc_mock_transient_preview(
        &self,
        result: &DcdcCalculationResult,
    ) -> Result<hotsas_core::SimulationResult, ApplicationError> {
        let fs = result.inputs.switching_frequency.si_value();
        let t_stop = 5.0 / fs;
        let n_points = 200;
        let dt = t_stop / n_points as f64;

        let vout_target = result.inputs.vout.si_value().abs();
        let iout = result.inputs.iout.si_value();

        let mut time = Vec::with_capacity(n_points);
        let mut vout = Vec::with_capacity(n_points);
        let mut il = Vec::with_capacity(n_points);
        let mut switch_node = Vec::with_capacity(n_points);

        let tau = t_stop * 0.3;
        let ripple_v = result
            .find_value_si("output_ripple")
            .unwrap_or(vout_target * 0.01);
        let ripple_i = result
            .find_value_si("inductor_ripple")
            .unwrap_or(iout * 0.3);

        for i in 0..n_points {
            let t = i as f64 * dt;
            time.push(t);

            let settling = 1.0 - (-t / tau).exp();
            let v = vout_target * settling
                + ripple_v * (t * fs * 2.0 * std::f64::consts::PI).sin() * 0.5;
            vout.push(v);

            let i_avg = iout * settling;
            let i = i_avg + ripple_i * (t * fs * 2.0 * std::f64::consts::PI).sin() * 0.5;
            il.push(i);

            let sw = if (t * fs).fract() < result.find_value_si("duty_cycle").unwrap_or(0.5) {
                vout_target
            } else {
                0.0
            };
            switch_node.push(sw);
        }

        let mut vout_points = Vec::with_capacity(n_points);
        let mut il_points = Vec::with_capacity(n_points);
        let mut sw_points = Vec::with_capacity(n_points);
        for i in 0..n_points {
            vout_points.push(GraphPoint {
                x: time[i],
                y: vout[i],
            });
            il_points.push(GraphPoint {
                x: time[i],
                y: il[i],
            });
            sw_points.push(GraphPoint {
                x: time[i],
                y: switch_node[i],
            });
        }

        Ok(SimulationResult {
            id: "dcdc-mock-transient".to_string(),
            profile_id: "transient".to_string(),
            status: SimulationStatus::Completed,
            engine: "mock".to_string(),
            graph_series: vec![
                GraphSeries {
                    name: "Output Voltage".to_string(),
                    x_unit: EngineeringUnit::Second,
                    y_unit: EngineeringUnit::Volt,
                    points: vout_points,
                    metadata: BTreeMap::new(),
                },
                GraphSeries {
                    name: "Inductor Current".to_string(),
                    x_unit: hotsas_core::EngineeringUnit::Second,
                    y_unit: hotsas_core::EngineeringUnit::Ampere,
                    points: il_points,
                    metadata: BTreeMap::new(),
                },
                GraphSeries {
                    name: "Switch Node".to_string(),
                    x_unit: EngineeringUnit::Second,
                    y_unit: EngineeringUnit::Volt,
                    points: sw_points,
                    metadata: BTreeMap::new(),
                },
            ],
            measurements: BTreeMap::new(),
            warnings: vec!["Mock transient preview — ideal model".to_string()],
            errors: vec![],
            raw_data_path: None,
            metadata: BTreeMap::new(),
        })
    }
}
