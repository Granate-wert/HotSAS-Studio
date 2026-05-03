use hotsas_core::{
    capacitor_symbol, ground_symbol, resistor_symbol, voltage_source_symbol, ElectricalPinType,
    PinSide,
};

#[test]
fn resistor_symbol_has_two_passive_pins() {
    let symbol = resistor_symbol();
    assert_eq!(symbol.pins.len(), 2);
    for pin in &symbol.pins {
        assert_eq!(pin.electrical_type, ElectricalPinType::Passive);
    }
    assert!(symbol.pins.iter().any(|p| p.position.side == PinSide::Left));
    assert!(symbol
        .pins
        .iter()
        .any(|p| p.position.side == PinSide::Right));
}

#[test]
fn capacitor_symbol_has_two_passive_pins() {
    let symbol = capacitor_symbol();
    assert_eq!(symbol.pins.len(), 2);
    for pin in &symbol.pins {
        assert_eq!(pin.electrical_type, ElectricalPinType::Passive);
    }
    assert!(symbol.pins.iter().any(|p| p.position.side == PinSide::Top));
    assert!(symbol
        .pins
        .iter()
        .any(|p| p.position.side == PinSide::Bottom));
}

#[test]
fn voltage_source_has_p_n_pins() {
    let symbol = voltage_source_symbol();
    assert_eq!(symbol.pins.len(), 2);
    assert!(symbol.pins.iter().any(|p| p.id == "p"));
    assert!(symbol.pins.iter().any(|p| p.id == "n"));
    let p_pin = symbol.pins.iter().find(|p| p.id == "p").unwrap();
    assert_eq!(p_pin.electrical_type, ElectricalPinType::Power);
    let n_pin = symbol.pins.iter().find(|p| p.id == "n").unwrap();
    assert_eq!(n_pin.electrical_type, ElectricalPinType::Ground);
}

#[test]
fn ground_has_gnd_pin() {
    let symbol = ground_symbol();
    assert_eq!(symbol.pins.len(), 1);
    assert_eq!(symbol.pins[0].id, "gnd");
    assert_eq!(symbol.pins[0].electrical_type, ElectricalPinType::Ground);
}

#[test]
fn pin_positions_are_finite() {
    for symbol in [
        resistor_symbol(),
        capacitor_symbol(),
        voltage_source_symbol(),
        ground_symbol(),
    ] {
        for pin in &symbol.pins {
            assert!(pin.position.x.is_finite());
            assert!(pin.position.y.is_finite());
        }
    }
}
