use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElectricalPinType {
    Passive,
    Input,
    Output,
    Power,
    Ground,
    Bidirectional,
    NotConnected,
}

impl std::fmt::Display for ElectricalPinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElectricalPinType::Passive => write!(f, "passive"),
            ElectricalPinType::Input => write!(f, "input"),
            ElectricalPinType::Output => write!(f, "output"),
            ElectricalPinType::Power => write!(f, "power"),
            ElectricalPinType::Ground => write!(f, "ground"),
            ElectricalPinType::Bidirectional => write!(f, "bidirectional"),
            ElectricalPinType::NotConnected => write!(f, "not_connected"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl std::fmt::Display for PinSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinSide::Left => write!(f, "left"),
            PinSide::Right => write!(f, "right"),
            PinSide::Top => write!(f, "top"),
            PinSide::Bottom => write!(f, "bottom"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PinPosition {
    pub x: f64,
    pub y: f64,
    pub side: PinSide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PinDefinition {
    pub id: String,
    pub name: String,
    pub number: String,
    pub electrical_type: ElectricalPinType,
    pub position: PinPosition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolDefinition {
    pub id: String,
    pub title: String,
    pub component_kind: String,
    pub pins: Vec<PinDefinition>,
    pub width: f64,
    pub height: f64,
}

impl SymbolDefinition {
    pub fn find_pin(&self, pin_id: &str) -> Option<&PinDefinition> {
        self.pins.iter().find(|p| p.id == pin_id)
    }
}

pub fn resistor_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "resistor".to_string(),
        title: "Resistor".to_string(),
        component_kind: "resistor".to_string(),
        pins: vec![
            PinDefinition {
                id: "1".to_string(),
                name: "1".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "2".to_string(),
                name: "2".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 40.0,
                    y: 0.0,
                    side: PinSide::Right,
                },
            },
        ],
        width: 80.0,
        height: 30.0,
    }
}

pub fn capacitor_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "capacitor".to_string(),
        title: "Capacitor".to_string(),
        component_kind: "capacitor".to_string(),
        pins: vec![
            PinDefinition {
                id: "1".to_string(),
                name: "1".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -20.0,
                    y: -30.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "2".to_string(),
                name: "2".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -20.0,
                    y: 30.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 40.0,
        height: 60.0,
    }
}

pub fn voltage_source_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "voltage_source".to_string(),
        title: "Voltage Source".to_string(),
        component_kind: "voltage_source".to_string(),
        pins: vec![
            PinDefinition {
                id: "p".to_string(),
                name: "p".to_string(),
                number: "p".to_string(),
                electrical_type: ElectricalPinType::Power,
                position: PinPosition {
                    x: 0.0,
                    y: -40.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "n".to_string(),
                name: "n".to_string(),
                number: "n".to_string(),
                electrical_type: ElectricalPinType::Ground,
                position: PinPosition {
                    x: 0.0,
                    y: 40.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 60.0,
        height: 80.0,
    }
}

pub fn ground_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "ground".to_string(),
        title: "Ground".to_string(),
        component_kind: "ground".to_string(),
        pins: vec![PinDefinition {
            id: "gnd".to_string(),
            name: "gnd".to_string(),
            number: "gnd".to_string(),
            electrical_type: ElectricalPinType::Ground,
            position: PinPosition {
                x: 0.0,
                y: -20.0,
                side: PinSide::Top,
            },
        }],
        width: 40.0,
        height: 40.0,
    }
}

/// Lookup a seed symbol by component kind.
pub fn seed_symbol_for_kind(kind: &str) -> Option<SymbolDefinition> {
    match kind {
        "resistor" => Some(resistor_symbol()),
        "capacitor" => Some(capacitor_symbol()),
        "voltage_source" => Some(voltage_source_symbol()),
        "ground" => Some(ground_symbol()),
        _ => None,
    }
}
