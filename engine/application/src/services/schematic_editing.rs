use hotsas_core::{
    AddComponentRequest, CircuitEndpoint, CircuitModel, CircuitProject, CircuitValidationReport,
    ComponentInstance, ConnectPinsRequest, ConnectedPin, DeleteComponentRequest, DeleteWireRequest,
    MoveComponentRequest, Net, Point, RenameNetRequest, SchematicEditResult,
    UpdateQuickParameterRequest, ValueWithUnit, Wire,
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

        let requested_id = request
            .component_definition_id
            .clone()
            .unwrap_or_else(|| request.component_kind.clone());

        // Resolve definition against the built-in library with fallback logic
        let library = hotsas_core::built_in_component_library();
        let definition = library
            .components
            .iter()
            .find(|d| d.id == requested_id)
            .or_else(|| {
                library
                    .components
                    .iter()
                    .find(|d| d.id == format!("generic_{}", requested_id))
            });

        let definition_id = definition
            .map(|d| d.id.clone())
            .unwrap_or_else(|| requested_id.clone());

        // Copy default parameters from the library definition so the component
        // is immediately editable in the UI.
        let mut overridden_parameters = std::collections::BTreeMap::new();
        if let Some(def) = definition {
            for (name, value) in &def.parameters {
                overridden_parameters.insert(name.clone(), value.clone());
            }
        }

        // Resolve symbol: prefer the library definition's symbol_ids, then fall
        // back to the legacy seed_symbol_for_kind lookup.
        let symbol_id = definition
            .and_then(|d| d.symbol_ids.first().cloned())
            .or_else(|| hotsas_core::seed_symbol_for_kind(&definition_id).map(|s| s.id.clone()));

        let component = ComponentInstance {
            instance_id: instance_id.clone(),
            definition_id: definition_id.clone(),
            selected_symbol_id: symbol_id,
            selected_footprint_id: None,
            selected_simulation_model_id: None,
            position: request.position,
            rotation_degrees: request.rotation_deg,
            connected_nets: vec![],
            overridden_parameters,
            notes: None,
        };

        project.schematic.components.push(component);
        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            validation_errors: validation.errors,
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
            validation_errors: validation.errors,
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
            validation_errors: validation.errors,
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
            geometry: None,
        });

        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            validation_errors: validation.errors,
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
            validation_errors: validation.errors,
            message: format!("Renamed net to {}", request.new_name),
        })
    }

    pub fn delete_wire(
        &self,
        project: &mut CircuitProject,
        request: DeleteWireRequest,
    ) -> Result<SchematicEditResult, String> {
        let wire = project
            .schematic
            .wires
            .iter()
            .find(|w| w.id == request.wire_id)
            .cloned()
            .ok_or_else(|| format!("wire '{}' not found", request.wire_id))?;

        // Remove wire
        project.schematic.wires.retain(|w| w.id != request.wire_id);

        // Remove connected_pins from net that belong to this wire's endpoints
        if let Some(net) = project
            .schematic
            .nets
            .iter_mut()
            .find(|n| n.id == wire.net_id)
        {
            net.connected_pins.retain(|cp| {
                !((Some(&cp.component_id) == wire.from.component_id.as_ref()
                    && Some(&cp.pin_id) == wire.from.pin_id.as_ref())
                    || (Some(&cp.component_id) == wire.to.component_id.as_ref()
                        && Some(&cp.pin_id) == wire.to.pin_id.as_ref()))
            });
        }

        // Remove connected_nets from components that belong to this wire
        if let Some(from_id) = &wire.from.component_id {
            if let Some(comp) = project
                .schematic
                .components
                .iter_mut()
                .find(|c| c.instance_id == *from_id)
            {
                if let Some(pin_id) = &wire.from.pin_id {
                    comp.connected_nets
                        .retain(|cn| !(cn.component_id == *from_id && cn.pin_id == *pin_id));
                }
            }
        }
        if let Some(to_id) = &wire.to.component_id {
            if let Some(comp) = project
                .schematic
                .components
                .iter_mut()
                .find(|c| c.instance_id == *to_id)
            {
                if let Some(pin_id) = &wire.to.pin_id {
                    comp.connected_nets
                        .retain(|cn| !(cn.component_id == *to_id && cn.pin_id == *pin_id));
                }
            }
        }

        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            validation_errors: validation.errors,
            message: format!("Deleted wire {}", request.wire_id),
        })
    }

    pub fn update_component_quick_parameter(
        &self,
        project: &mut CircuitProject,
        request: UpdateQuickParameterRequest,
    ) -> Result<SchematicEditResult, String> {
        let component = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == request.component_id)
            .ok_or_else(|| format!("component '{}' not found", request.component_id))?;

        let unit = parameter_unit(&request.parameter_id);
        let value = ValueWithUnit::parse_with_default(&request.value, unit)
            .map_err(|e| format!("invalid value: {e}"))?;

        component
            .overridden_parameters
            .insert(request.parameter_id.clone(), value);
        project.updated_at = format!("{:?}", std::time::SystemTime::now());

        let validation = self.validate_after_edit(&project.schematic);
        Ok(SchematicEditResult {
            project: project.clone(),
            validation_warnings: validation.warnings,
            validation_errors: validation.errors,
            message: format!(
                "Updated {}.{} = {}",
                request.component_id, request.parameter_id, request.value
            ),
        })
    }

    fn validate_after_edit(&self, schematic: &CircuitModel) -> CircuitValidationReport {
        let validator = crate::CircuitValidationService::new();
        validator.validate(schematic)
    }
}

/// Infer the engineering unit for a parameter key.
fn parameter_unit(parameter_id: &str) -> hotsas_core::EngineeringUnit {
    match parameter_id {
        "resistance" => hotsas_core::EngineeringUnit::Ohm,
        "capacitance" => hotsas_core::EngineeringUnit::Farad,
        "inductance" => hotsas_core::EngineeringUnit::Henry,
        "voltage" | "ac_magnitude" | "dc_voltage" => hotsas_core::EngineeringUnit::Volt,
        "current" => hotsas_core::EngineeringUnit::Ampere,
        "frequency" => hotsas_core::EngineeringUnit::Hertz,
        _ => hotsas_core::EngineeringUnit::Unitless,
    }
}
