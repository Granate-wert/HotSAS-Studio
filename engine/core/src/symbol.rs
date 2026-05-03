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

pub fn inductor_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "inductor".to_string(),
        title: "Inductor".to_string(),
        component_kind: "inductor".to_string(),
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

pub fn diode_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "diode".to_string(),
        title: "Diode".to_string(),
        component_kind: "diode".to_string(),
        pins: vec![
            PinDefinition {
                id: "anode".to_string(),
                name: "A".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "cathode".to_string(),
                name: "K".to_string(),
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

pub fn led_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "led".to_string(),
        title: "LED".to_string(),
        component_kind: "led".to_string(),
        pins: vec![
            PinDefinition {
                id: "anode".to_string(),
                name: "A".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "cathode".to_string(),
                name: "K".to_string(),
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

pub fn bjt_npn_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "bjt_npn".to_string(),
        title: "NPN BJT".to_string(),
        component_kind: "bjt_npn".to_string(),
        pins: vec![
            PinDefinition {
                id: "collector".to_string(),
                name: "C".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: -40.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "base".to_string(),
                name: "B".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "emitter".to_string(),
                name: "E".to_string(),
                number: "3".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: 40.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 80.0,
        height: 80.0,
    }
}

pub fn bjt_pnp_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "bjt_pnp".to_string(),
        title: "PNP BJT".to_string(),
        component_kind: "bjt_pnp".to_string(),
        pins: vec![
            PinDefinition {
                id: "collector".to_string(),
                name: "C".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: -40.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "base".to_string(),
                name: "B".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "emitter".to_string(),
                name: "E".to_string(),
                number: "3".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: 40.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 80.0,
        height: 80.0,
    }
}

pub fn mosfet_n_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "mosfet_n".to_string(),
        title: "N-MOSFET".to_string(),
        component_kind: "mosfet_n".to_string(),
        pins: vec![
            PinDefinition {
                id: "drain".to_string(),
                name: "D".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: -40.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "gate".to_string(),
                name: "G".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "source".to_string(),
                name: "S".to_string(),
                number: "3".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: 40.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 80.0,
        height: 80.0,
    }
}

pub fn mosfet_p_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "mosfet_p".to_string(),
        title: "P-MOSFET".to_string(),
        component_kind: "mosfet_p".to_string(),
        pins: vec![
            PinDefinition {
                id: "drain".to_string(),
                name: "D".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: -40.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "gate".to_string(),
                name: "G".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: -40.0,
                    y: 0.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "source".to_string(),
                name: "S".to_string(),
                number: "3".to_string(),
                electrical_type: ElectricalPinType::Passive,
                position: PinPosition {
                    x: 30.0,
                    y: 40.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 80.0,
        height: 80.0,
    }
}

pub fn op_amp_symbol() -> SymbolDefinition {
    SymbolDefinition {
        id: "op_amp".to_string(),
        title: "Op-Amp".to_string(),
        component_kind: "op_amp".to_string(),
        pins: vec![
            PinDefinition {
                id: "inverting".to_string(),
                name: "-".to_string(),
                number: "2".to_string(),
                electrical_type: ElectricalPinType::Input,
                position: PinPosition {
                    x: -40.0,
                    y: -10.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "non_inverting".to_string(),
                name: "+".to_string(),
                number: "3".to_string(),
                electrical_type: ElectricalPinType::Input,
                position: PinPosition {
                    x: -40.0,
                    y: 10.0,
                    side: PinSide::Left,
                },
            },
            PinDefinition {
                id: "output".to_string(),
                name: "OUT".to_string(),
                number: "1".to_string(),
                electrical_type: ElectricalPinType::Output,
                position: PinPosition {
                    x: 40.0,
                    y: 0.0,
                    side: PinSide::Right,
                },
            },
            PinDefinition {
                id: "vcc".to_string(),
                name: "VCC".to_string(),
                number: "4".to_string(),
                electrical_type: ElectricalPinType::Power,
                position: PinPosition {
                    x: 0.0,
                    y: -40.0,
                    side: PinSide::Top,
                },
            },
            PinDefinition {
                id: "vee".to_string(),
                name: "VEE".to_string(),
                number: "5".to_string(),
                electrical_type: ElectricalPinType::Power,
                position: PinPosition {
                    x: 0.0,
                    y: 40.0,
                    side: PinSide::Bottom,
                },
            },
        ],
        width: 80.0,
        height: 80.0,
    }
}

/// Lookup a seed symbol by component kind.
pub fn seed_symbol_for_kind(kind: &str) -> Option<SymbolDefinition> {
    match kind {
        "resistor" => Some(resistor_symbol()),
        "capacitor" => Some(capacitor_symbol()),
        "inductor" => Some(inductor_symbol()),
        "voltage_source" => Some(voltage_source_symbol()),
        "ground" => Some(ground_symbol()),
        "diode" => Some(diode_symbol()),
        "led" => Some(led_symbol()),
        "bjt_npn" => Some(bjt_npn_symbol()),
        "bjt_pnp" => Some(bjt_pnp_symbol()),
        "mosfet_n" => Some(mosfet_n_symbol()),
        "mosfet_p" => Some(mosfet_p_symbol()),
        "op_amp" => Some(op_amp_symbol()),
        _ => None,
    }
}
