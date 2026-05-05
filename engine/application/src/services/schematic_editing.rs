use hotsas_core::{
    AddComponentRequest, CircuitEndpoint, CircuitModel, CircuitProject, CircuitValidationReport,
    ComponentInstance, ConnectPinsRequest, ConnectedPin, DeleteComponentRequest,
    MoveComponentRequest, Net, Point, RenameNetRequest, SchematicEditResult, Wire,
};

#[derive(Clone)]
pub struct SchematicEditingService;

impl SchematicEditingService {
    pub fn new() -> Self {
        Self
    }

    pub fn add_component(
        &self,
        project: &mut CircuitProject,
        request: AddComponentRequest,
    ) -> Result<SchematicEditResult, String> {
        let instance_id = request.instance_id.clone().unwrap_or_else(|| {
            format!(
                "{}-{}",
                request.component_kind,
                project.schematic.components.len() + 1
            )
        });

        if project
            .schematic
            .components
            .iter()
            .any(|c| c.instance_id == instance_id)
        {
            return Err(format!("duplicate instance id: {instance_id}"));
        }

        let definition_id = request
            .component_definition_id
            .clone()
            .unwrap_or_else(|| request.component_kind.clone());

        let symbol_id = hotsas_core::seed_symbol_for_kind(&definition_id).map(|s| s.id.clone());

        let component = ComponentInstance {
            instance_id: instance_id.clone(),
            definition_id: definition_id.clone(),
            selected_symbol_id: symbol_id,
            selected_footprint_id: None,
            selected_simulation_model_id: None,
            position: request.position,
            rotation_degrees: request.rotation_deg,
            connected_nets: vec![],
            overridden_parameters: std::collections::BTreeMap::new(),
            notes: None,
        };

        project.schematic.components.push(component);
        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            message: format!("Added component {instance_id}"),
        })
    }

    pub fn move_component(
        &self,
        project: &mut CircuitProject,
        request: MoveComponentRequest,
    ) -> Result<SchematicEditResult, String> {
        let component = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == request.instance_id)
            .ok_or_else(|| format!("component '{}' not found", request.instance_id))?;

        component.position = request.position;
        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            message: format!("Moved component {}", request.instance_id),
        })
    }

    pub fn delete_component(
        &self,
        project: &mut CircuitProject,
        request: DeleteComponentRequest,
    ) -> Result<SchematicEditResult, String> {
        let idx = project
            .schematic
            .components
            .iter()
            .position(|c| c.instance_id == request.instance_id)
            .ok_or_else(|| format!("component '{}' not found", request.instance_id))?;

        // Remove wires connected to this component
        project.schematic.wires.retain(|w| {
            w.from.component_id.as_ref() != Some(&request.instance_id)
                && w.to.component_id.as_ref() != Some(&request.instance_id)
        });

        // Remove connected_pins from nets belonging to deleted component
        for net in &mut project.schematic.nets {
            net.connected_pins
                .retain(|cp| cp.component_id != request.instance_id);
        }

        project.schematic.components.remove(idx);
        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            message: format!("Deleted component {}", request.instance_id),
        })
    }

    fn component_has_pin(project: &CircuitProject, component_id: &str, pin_id: &str) -> bool {
        if let Some(comp) = project
            .schematic
            .components
            .iter()
            .find(|c| c.instance_id == component_id)
        {
            if let Some(symbol) = hotsas_core::seed_symbol_for_kind(&comp.definition_id) {
                return symbol.pins.iter().any(|p| p.id == pin_id);
            }
        }
        true
    }

    pub fn connect_pins(
        &self,
        project: &mut CircuitProject,
        request: ConnectPinsRequest,
    ) -> Result<SchematicEditResult, String> {
        // Validate components exist
        let from_exists = project
            .schematic
            .components
            .iter()
            .any(|c| c.instance_id == request.from_component_id);
        let to_exists = project
            .schematic
            .components
            .iter()
            .any(|c| c.instance_id == request.to_component_id);

        if !from_exists {
            return Err(format!(
                "component '{}' not found",
                request.from_component_id
            ));
        }
        if !to_exists {
            return Err(format!("component '{}' not found", request.to_component_id));
        }

        if !Self::component_has_pin(project, &request.from_component_id, &request.from_pin_id) {
            return Err(format!(
                "pin '{}' not found on component '{}'",
                request.from_pin_id, request.from_component_id
            ));
        }
        if !Self::component_has_pin(project, &request.to_component_id, &request.to_pin_id) {
            return Err(format!(
                "pin '{}' not found on component '{}'",
                request.to_pin_id, request.to_component_id
            ));
        }

        let net_name = request.net_name.clone().unwrap_or_else(|| {
            format!(
                "net_{}_{}",
                request.from_component_id, request.to_component_id
            )
        });

        // Find or create net
        let net_id =
            if let Some(existing) = project.schematic.nets.iter().find(|n| n.name == net_name) {
                existing.id.clone()
            } else {
                let new_id = format!("net-{}", project.schematic.nets.len() + 1);
                project.schematic.nets.push(Net {
                    id: new_id.clone(),
                    name: net_name.clone(),
                    connected_pins: vec![],
                });
                new_id
            };

        // Update component pin net associations
        let from_pin = ConnectedPin {
            component_id: request.from_component_id.clone(),
            pin_id: request.from_pin_id.clone(),
            net_id: net_id.clone(),
        };
        let to_pin = ConnectedPin {
            component_id: request.to_component_id.clone(),
            pin_id: request.to_pin_id.clone(),
            net_id: net_id.clone(),
        };

        if let Some(comp) = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == request.from_component_id)
        {
            comp.connected_nets.retain(|cn| {
                !(cn.component_id == request.from_component_id && cn.pin_id == request.from_pin_id)
            });
            comp.connected_nets.push(from_pin);
        }

        if let Some(comp) = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == request.to_component_id)
        {
            comp.connected_nets.retain(|cn| {
                !(cn.component_id == request.to_component_id && cn.pin_id == request.to_pin_id)
            });
            comp.connected_nets.push(to_pin);
        }

        // Update net connected_pins
        if let Some(net) = project.schematic.nets.iter_mut().find(|n| n.id == net_id) {
            net.connected_pins.retain(|cp| {
                !((cp.component_id == request.from_component_id
                    && cp.pin_id == request.from_pin_id)
                    || (cp.component_id == request.to_component_id
                        && cp.pin_id == request.to_pin_id))
            });
            net.connected_pins.push(ConnectedPin {
                component_id: request.from_component_id.clone(),
                pin_id: request.from_pin_id.clone(),
                net_id: net_id.clone(),
            });
            net.connected_pins.push(ConnectedPin {
                component_id: request.to_component_id.clone(),
                pin_id: request.to_pin_id.clone(),
                net_id: net_id.clone(),
            });
        }

        // Create wire for visual representation
        let wire_id = format!("wire-{}", project.schematic.wires.len() + 1);
        project.schematic.wires.push(Wire {
            id: wire_id,
            from: CircuitEndpoint {
                component_id: Some(request.from_component_id.clone()),
                pin_id: Some(request.from_pin_id.clone()),
                point: Point::new(0.0, 0.0),
            },
            to: CircuitEndpoint {
                component_id: Some(request.to_component_id.clone()),
                pin_id: Some(request.to_pin_id.clone()),
                point: Point::new(0.0, 0.0),
            },
            net_id: net_id.clone(),
        });

        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            message: format!("Connected pins via net {net_name}"),
        })
    }

    pub fn rename_net(
        &self,
        project: &mut CircuitProject,
        request: RenameNetRequest,
    ) -> Result<SchematicEditResult, String> {
        if request.new_name.trim().is_empty() {
            return Err("net name cannot be empty".to_string());
        }

        let net = project
            .schematic
            .nets
            .iter_mut()
            .find(|n| n.id == request.net_id)
            .ok_or_else(|| format!("net '{}' not found", request.net_id))?;

        net.name = request.new_name.clone();
        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            message: format!("Renamed net to {}", request.new_name),
        })
    }

    fn validate_after_edit(&self, schematic: &CircuitModel) -> CircuitValidationReport {
        let validator = crate::CircuitValidationService::new();
        validator.validate(schematic)
    }
}
