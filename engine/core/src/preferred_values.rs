use crate::{
    preferred_value_tables::{E12_BASE, E192_BASE, E24_BASE, E3_BASE, E48_BASE, E6_BASE, E96_BASE},
    CoreError, ValueWithUnit,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferredValueSeries {
    E3,
    E6,
    E12,
    E24,
    E48,
    E96,
    E192,
}

impl PreferredValueSeries {
    pub fn count(self) -> usize {
        match self {
            Self::E3 => 3,
            Self::E6 => 6,
            Self::E12 => 12,
            Self::E24 => 24,
            Self::E48 => 48,
            Self::E96 => 96,
            Self::E192 => 192,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::E3 => "E3",
            Self::E6 => "E6",
            Self::E12 => "E12",
            Self::E24 => "E24",
            Self::E48 => "E48",
            Self::E96 => "E96",
            Self::E192 => "E192",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreferredValueResult {
    pub requested_value: ValueWithUnit,
    pub series: PreferredValueSeries,
    pub lower: Option<ValueWithUnit>,
    pub nearest: ValueWithUnit,
    pub higher: Option<ValueWithUnit>,
    pub error_percent: f64,
}

pub fn nearest_preferred_value(
    requested_value: ValueWithUnit,
    series: PreferredValueSeries,
) -> Result<PreferredValueResult, CoreError> {
    if requested_value.si_value() <= 0.0 || !requested_value.si_value().is_finite() {
        return Err(CoreError::ValueOutOfRange(
            "preferred value lookup requires a positive finite value".to_string(),
        ));
    }

    let values = generate_decade_values(
        series,
        requested_value.si_value() / 10.0,
        requested_value.si_value() * 10.0,
    )?;
    let lower_raw = values
        .iter()
        .copied()
        .filter(|value| *value <= requested_value.si_value())
        .max_by(|a, b| a.total_cmp(b));
    let higher_raw = values
        .iter()
        .copied()
        .filter(|value| *value >= requested_value.si_value())
        .min_by(|a, b| a.total_cmp(b));
    let nearest_raw = values
        .iter()
        .copied()
        .min_by(|a, b| {
            (a - requested_value.si_value())
                .abs()
                .total_cmp(&(b - requested_value.si_value()).abs())
        })
        .ok_or_else(|| CoreError::ValueOutOfRange("no preferred values generated".to_string()))?;

    let unit = requested_value.unit;
    Ok(PreferredValueResult {
        requested_value: requested_value.clone(),
        series,
        lower: lower_raw.map(|value| ValueWithUnit::new_si(value, unit)),
        nearest: ValueWithUnit::new_si(nearest_raw, unit),
        higher: higher_raw.map(|value| ValueWithUnit::new_si(value, unit)),
        error_percent: calculate_error_percent(requested_value.si_value(), nearest_raw),
    })
}

pub fn lower_preferred_value(
    requested_value: ValueWithUnit,
    series: PreferredValueSeries,
) -> Result<Option<ValueWithUnit>, CoreError> {
    Ok(nearest_preferred_value(requested_value, series)?.lower)
}

pub fn higher_preferred_value(
    requested_value: ValueWithUnit,
    series: PreferredValueSeries,
) -> Result<Option<ValueWithUnit>, CoreError> {
    Ok(nearest_preferred_value(requested_value, series)?.higher)
}

pub fn generate_decade_values(
    series: PreferredValueSeries,
    min: f64,
    max: f64,
) -> Result<Vec<f64>, CoreError> {
    if min <= 0.0 || max <= 0.0 || min > max || !min.is_finite() || !max.is_finite() {
        return Err(CoreError::ValueOutOfRange(
            "preferred value range must be positive and finite".to_string(),
        ));
    }

    let bases = base_values(series);
    let start_decade = min.log10().floor() as i32 - 1;
    let end_decade = max.log10().ceil() as i32 + 1;
    let mut values = Vec::new();

    for decade in start_decade..=end_decade {
        let multiplier = 10_f64.powi(decade);
        for base in bases {
            let value = base * multiplier;
            if value >= min && value <= max {
                values.push(value);
            }
        }
    }

    values.sort_by(|a, b| a.total_cmp(b));
    values.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    Ok(values)
}

pub fn calculate_error_percent(requested: f64, actual: f64) -> f64 {
    if requested == 0.0 {
        0.0
    } else {
        ((actual - requested) / requested) * 100.0
    }
}

fn base_values(series: PreferredValueSeries) -> &'static [f64] {
    match series {
        PreferredValueSeries::E3 => E3_BASE,
        PreferredValueSeries::E6 => E6_BASE,
        PreferredValueSeries::E12 => E12_BASE,
        PreferredValueSeries::E24 => E24_BASE,
        PreferredValueSeries::E48 => E48_BASE,
        PreferredValueSeries::E96 => E96_BASE,
        PreferredValueSeries::E192 => E192_BASE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EngineeringUnit;

    #[test]
    fn selects_nearest_e24_resistor_value() {
        let requested = ValueWithUnit::parse_with_default("15.93k", EngineeringUnit::Ohm).unwrap();
        let result = nearest_preferred_value(requested, PreferredValueSeries::E24).unwrap();

        assert_eq!(result.nearest.unit, EngineeringUnit::Ohm);
        assert_eq!(result.nearest.si_value(), 16_000.0);
        assert_eq!(result.lower.unwrap().si_value(), 15_000.0);
        assert_eq!(result.higher.unwrap().si_value(), 16_000.0);
        assert!(result.error_percent.abs() < 0.5);
    }

    #[test]
    fn generates_e24_values_across_requested_range() {
        let values = generate_decade_values(PreferredValueSeries::E24, 900.0, 2200.0).unwrap();

        assert!(values.contains(&1_000.0));
        assert!(values.contains(&2_200.0));
        assert!(!values.contains(&820.0));
    }
}
