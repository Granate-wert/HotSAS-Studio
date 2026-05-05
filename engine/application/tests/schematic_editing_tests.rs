use hotsas_application::SchematicEditingService;
use hotsas_core::{
    AddComponentRequest, CircuitProject, ConnectPinsRequest, DeleteComponentRequest,
    MoveComponentRequest, Point, RenameNetRequest,
};

fn empty_project() -> CircuitProject {
    CircuitProject {
        id: "test".to_string(),
        name: "Test".to_string(),
        format_version: "1".to_string(),
        engine_version: "0.1.4".to_string(),
        project_type: "schematic".to_string(),
        created_at: "2024-01-01".to_string(),
        updated_at: "2024-01-01".to_string(),
        schematic: hotsas_core::CircuitModel {
            id: "sch1".to_string(),
            title: "Test".to_string(),
            components: vec![],
            wires: vec![],
            nets: vec![],
            labels: vec![],
            probes: vec![],
            annotations: vec![],
        },
        simulation_profiles: vec![],
        linked_libraries: vec![],
        reports: vec![],
    }
}

#[test]
fn add_resistor_component_succeeds() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.components.len(), 1);
    assert_eq!(edit.project.schematic.components[0].instance_id, "R1");
    assert_eq!(edit.message, "Added component R1");
}

#[test]
fn duplicate_instance_id_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("duplicate"));
}

#[test]
fn move_component_updates_position() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.move_component(
        &mut project,
        MoveComponentRequest {
            instance_id: "R1".to_string(),
            position: Point::new(300.0, 400.0),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.components[0].position.x, 300.0);
    assert_eq!(edit.project.schematic.components[0].position.y, 400.0);
}

#[test]
fn move_unknown_component_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.move_component(
        &mut project,
        MoveComponentRequest {
            instance_id: "R99".to_string(),
            position: Point::new(300.0, 400.0),
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn delete_component_removes_instance() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R1".to_string(),
        },
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().project.schematic.components.len(), 0);
}

#[test]
fn delete_unknown_component_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R99".to_string(),
        },
    );
    assert!(result.is_err());
}

#[test]
fn connect_pins_creates_net_and_wire() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "p1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "p1".to_string(),
            net_name: Some("net_test".to_string()),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.nets.len(), 1);
    assert_eq!(edit.project.schematic.wires.len(), 1);
    assert_eq!(edit.project.schematic.nets[0].name, "net_test");
}

#[test]
fn connect_unknown_component_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "p1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "p1".to_string(),
            net_name: None,
        },
    );
    assert!(result.is_err());
}

#[test]
fn rename_net_succeeds() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    project.schematic.nets.push(hotsas_core::Net {
        id: "n1".to_string(),
        name: "old_name".to_string(),
        connected_pins: vec![],
    });

    let result = svc.rename_net(
        &mut project,
        RenameNetRequest {
            net_id: "n1".to_string(),
            new_name: "new_name".to_string(),
        },
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().project.schematic.nets[0].name, "new_name");
}

#[test]
fn rename_net_empty_name_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    project.schematic.nets.push(hotsas_core::Net {
        id: "n1".to_string(),
        name: "old_name".to_string(),
        connected_pins: vec![],
    });

    let result = svc.rename_net(
        &mut project,
        RenameNetRequest {
            net_id: "n1".to_string(),
            new_name: "   ".to_string(),
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("empty"));
}
