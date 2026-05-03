use crate::CoreError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineeringPrefix {
    Pico,
    Nano,
    Micro,
    Milli,
    Kilo,
    Mega,
}

impl EngineeringPrefix {
    pub fn factor(self) -> f64 {
        match self {
            Self::Pico => 1e-12,
            Self::Nano => 1e-9,
            Self::Micro => 1e-6,
            Self::Milli => 1e-3,
            Self::Kilo => 1e3,
            Self::Mega => 1e6,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Pico => "p",
            Self::Nano => "n",
            Self::Micro => "u",
            Self::Milli => "m",
            Self::Kilo => "k",
            Self::Mega => "M",
        }
    }

    fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix.chars().next()? {
            'p' => Some(Self::Pico),
            'n' => Some(Self::Nano),
            'u' => Some(Self::Micro),
            'm' => Some(Self::Milli),
            'k' | 'K' => Some(Self::Kilo),
            'M' => Some(Self::Mega),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineeringUnit {
    Unitless,
    Ohm,
    Farad,
    Hertz,
    Volt,
    Ampere,
}

impl EngineeringUnit {
    pub fn parse(unit: &str) -> Result<Self, CoreError> {
        match unit.trim() {
            "" => Ok(Self::Unitless),
            "Ohm" | "ohm" | "R" => Ok(Self::Ohm),
            "F" | "farad" | "Farad" => Ok(Self::Farad),
            "Hz" | "hertz" | "Hertz" => Ok(Self::Hertz),
            "V" | "volt" | "Volt" => Ok(Self::Volt),
            "A" | "amp" | "Ampere" => Ok(Self::Ampere),
            other => Err(CoreError::InvalidUnit(other.to_string())),
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Unitless => "",
            Self::Ohm => "Ohm",
            Self::Farad => "F",
            Self::Hertz => "Hz",
            Self::Volt => "V",
            Self::Ampere => "A",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineeringValue {
    pub original: String,
    pub mantissa: f64,
    pub prefix: Option<EngineeringPrefix>,
    pub si_value: f64,
}

impl EngineeringValue {
    pub fn new_si(si_value: f64) -> Self {
        Self {
            original: format!("{si_value}"),
            mantissa: si_value,
            prefix: None,
            si_value,
        }
    }

    pub fn from_parts(
        mantissa: f64,
        prefix: Option<EngineeringPrefix>,
        original: impl Into<String>,
    ) -> Self {
        let factor = prefix.map_or(1.0, EngineeringPrefix::factor);
        Self {
            original: original.into(),
            mantissa,
            prefix,
            si_value: mantissa * factor,
        }
    }
}

impl FromStr for EngineeringValue {
    type Err = CoreError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(CoreError::InvalidEngineeringValue(input.to_string()));
        }

        let numeric_end = numeric_prefix_end(trimmed);
        if numeric_end == 0 {
            return Err(CoreError::InvalidEngineeringValue(input.to_string()));
        }

        let number_part = &trimmed[..numeric_end];
        let suffix = trimmed[numeric_end..].trim();
        let mantissa = number_part
            .parse::<f64>()
            .map_err(|_| CoreError::InvalidEngineeringValue(input.to_string()))?;
        let prefix = EngineeringPrefix::from_suffix(suffix);

        Ok(Self::from_parts(mantissa, prefix, trimmed))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueWithUnit {
    pub value: EngineeringValue,
    pub unit: EngineeringUnit,
}

impl ValueWithUnit {
    pub fn new_si(si_value: f64, unit: EngineeringUnit) -> Self {
        Self {
            value: EngineeringValue::new_si(si_value),
            unit,
        }
    }

    pub fn si_value(&self) -> f64 {
        self.value.si_value
    }

    pub fn original(&self) -> &str {
        &self.value.original
    }

    pub fn parse_with_default(
        input: &str,
        default_unit: EngineeringUnit,
    ) -> Result<Self, CoreError> {
        let trimmed = input.trim();
        let numeric_end = numeric_prefix_end(trimmed);
        if numeric_end == 0 {
            return Err(CoreError::InvalidEngineeringValue(input.to_string()));
        }

        let suffix = trimmed[numeric_end..].trim();
        let prefix = EngineeringPrefix::from_suffix(suffix);
        let unit_suffix = match prefix {
            Some(prefix) => suffix[prefix.symbol().len()..].trim(),
            None => suffix,
        };
        let unit = if unit_suffix.is_empty() {
            default_unit
        } else {
            EngineeringUnit::parse(unit_suffix)?
        };
        let value = EngineeringValue::from_str(trimmed)?;
        Ok(Self { value, unit })
    }
}

impl fmt::Display for ValueWithUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.unit == EngineeringUnit::Unitless {
            write!(f, "{}", self.value.si_value)
        } else {
            write!(f, "{} {}", self.value.si_value, self.unit.symbol())
        }
    }
}

fn numeric_prefix_end(input: &str) -> usize {
    let mut seen_digit = false;
    let mut seen_decimal = false;
    let mut seen_exponent = false;
    let mut previous_was_exponent = false;

    for (index, ch) in input.char_indices() {
        if ch.is_ascii_digit() {
            seen_digit = true;
            previous_was_exponent = false;
            continue;
        }

        if (ch == '+' || ch == '-') && (index == 0 || previous_was_exponent) {
            previous_was_exponent = false;
            continue;
        }

        if ch == '.' && !seen_decimal && !seen_exponent {
            seen_decimal = true;
            previous_was_exponent = false;
            continue;
        }

        if (ch == 'e' || ch == 'E') && seen_digit && !seen_exponent {
            seen_exponent = true;
            previous_was_exponent = true;
            continue;
        }

        return index;
    }

    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_required_v1_engineering_prefixes() {
        let resistor = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
        let capacitor = ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap();
        let micro = ValueWithUnit::parse_with_default("1u", EngineeringUnit::Farad).unwrap();
        let mega = ValueWithUnit::parse_with_default("1M", EngineeringUnit::Ohm).unwrap();

        assert_eq!(resistor.unit, EngineeringUnit::Ohm);
        assert_eq!(resistor.si_value(), 10_000.0);
        assert_close(capacitor.si_value(), 100e-9);
        assert_eq!(micro.si_value(), 1e-6);
        assert_eq!(mega.si_value(), 1e6);
    }

    #[test]
    fn parses_units_when_suffix_contains_unit() {
        let frequency =
            ValueWithUnit::parse_with_default("1MHz", EngineeringUnit::Unitless).unwrap();
        let capacitance =
            ValueWithUnit::parse_with_default("100nF", EngineeringUnit::Unitless).unwrap();

        assert_eq!(frequency.unit, EngineeringUnit::Hertz);
        assert_eq!(frequency.si_value(), 1e6);
        assert_eq!(capacitance.unit, EngineeringUnit::Farad);
        assert_close(capacitance.si_value(), 100e-9);
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-18,
            "expected {actual} to be within tolerance of {expected}"
        );
    }
}
