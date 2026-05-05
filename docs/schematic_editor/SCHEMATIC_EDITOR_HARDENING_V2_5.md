# HotSAS Studio v2.5 — Schematic Editor Hardening

## What changed

After v2.5, the Schematic Editor is no longer just a viewer/demo canvas. It supports basic editing operations through backend commands.

### Supported edit commands

| Command          | Backend method                              | Frontend UI                    |
| ---------------- | ------------------------------------------- | ------------------------------ |
| Add component    | `SchematicEditingService::add_component`    | Component Palette buttons      |
| Move component   | `SchematicEditingService::move_component`   | React Flow drag stop → backend |
| Delete component | `SchematicEditingService::delete_component` | Toolbar Delete button          |
| Connect pins     | `SchematicEditingService::connect_pins`     | Connection Panel form          |
| Rename net       | `SchematicEditingService::rename_net`       | Net Label Editor form          |

### Architecture rule

```text
React Flow = view adapter only
Source of truth = Rust CircuitModel / ProjectDto
```

All edits go through Tauri commands → API facade → SchematicEditingService → CircuitModel mutation. The frontend refreshes by receiving an updated `ProjectDto`.

## Limitations

- No interactive wire drag tool. Connection is created through a controlled form (select from/to component + pin + optional net name).
- No autorouting.
- No PCB editor.
- No full KiCad/Altium-like editor.
- Component position is backend-driven; React Flow drag only sends the final position on drag stop.

## Validation

After every edit, `SchematicEditingService` runs `CircuitValidationService::validate()` and returns warnings. These are displayed in the Validation tab of the side panel.

## Files

| Layer               | File                                                                              | Purpose                                                  |
| ------------------- | --------------------------------------------------------------------------------- | -------------------------------------------------------- |
| Core models         | `engine/core/src/schematic_editing.rs`                                            | Request/result structs                                   |
| Application service | `engine/application/src/services/schematic_editing.rs`                            | Edit logic + validation                                  |
| API DTOs            | `engine/api/src/dto.rs`                                                           | `AddComponentRequestDto`, `SchematicEditResultDto`, etc. |
| API facade          | `engine/api/src/facade.rs`                                                        | Facade methods for 6 commands                            |
| Tauri commands      | `apps/desktop-tauri/src-tauri/src/lib.rs`                                         | Command registration                                     |
| Frontend types      | `apps/desktop-tauri/src/types/index.ts`                                           | TypeScript DTOs                                          |
| Frontend API        | `apps/desktop-tauri/src/api/index.ts`                                             | API wrappers                                             |
| Frontend store      | `apps/desktop-tauri/src/store/index.ts`                                           | Zustand state for editing                                |
| Component palette   | `apps/desktop-tauri/src/components/schematic-editor/ComponentPalette.tsx`         | 8 component buttons                                      |
| Toolbar             | `apps/desktop-tauri/src/components/schematic-editor/SchematicToolbar.tsx`         | Delete/Connect/Rename actions                            |
| Connection panel    | `apps/desktop-tauri/src/components/schematic-editor/ConnectionPanel.tsx`          | Pin-to-pin connection form                               |
| Net editor          | `apps/desktop-tauri/src/components/schematic-editor/NetLabelEditor.tsx`           | Net rename form                                          |
| Status panel        | `apps/desktop-tauri/src/components/schematic-editor/SchematicEditStatusPanel.tsx` | Validation warnings display                              |
| Canvas              | `apps/desktop-tauri/src/components/SchematicCanvas.tsx`                           | React Flow with onNodeDragStop                           |
| Screen              | `apps/desktop-tauri/src/screens/SchematicScreen.tsx`                              | Integration of all editor components                     |

## Known limitations

```text
- Connection tool is form-based, not drag-to-wire.
- Wire geometry is simplified (no bend points).
- No multi-select or bulk delete.
- No undo/redo stack.
```
