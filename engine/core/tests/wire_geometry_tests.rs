use hotsas_core::{CircuitEndpoint, Point, Wire, WireGeometry, WireRoutingStyle};

#[test]
fn save_load_preserves_wire_route_points() {
    let wire = Wire {
        id: "wire-manual".to_string(),
        from: CircuitEndpoint {
            component_id: Some("R1".to_string()),
            pin_id: Some("2".to_string()),
            point: Point::new(140.0, 100.0),
        },
        to: CircuitEndpoint {
            component_id: Some("C1".to_string()),
            pin_id: Some("1".to_string()),
            point: Point::new(240.0, 160.0),
        },
        net_id: "net_out".to_string(),
        geometry: Some(WireGeometry {
            routing_style: WireRoutingStyle::Manual,
            points: vec![
                Point::new(160.0, 100.0),
                Point::new(160.0, 160.0),
                Point::new(240.0, 160.0),
            ],
        }),
    };

    let json = serde_json::to_string(&wire).expect("wire geometry should serialize");
    let restored: Wire = serde_json::from_str(&json).expect("wire geometry should deserialize");

    assert_eq!(restored.geometry, wire.geometry);
}
