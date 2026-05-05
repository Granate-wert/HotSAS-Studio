# HotSAS Studio v2.6 — Project Persistence / Save-Load UX Hardening

## Overview

Version 2.6 introduces project session persistence, dirty-state tracking, and save/load UX hardening. All disk writes are handled by the Rust backend; React is never the source of truth for persistence.

## Architecture

```text
Frontend (React)
  └── Tauri Commands
        └── API Facade (hotsas_api)
              └── AppServices (hotsas_application)
                    ├── ProjectSessionService
                    │   ├── session state (dirty, path, last_saved_at)
                    │   ├── recent projects list
                    │   └── settings persistence (JSON)
                    ├── SchematicEditingService
                    └── ProjectPackageService
```

## Core Models

### `ProjectSessionState`

```rust
pub struct ProjectSessionState {
    pub current_project_id: Option<String>,
    pub current_project_name: Option<String>,
    pub current_project_path: Option<String>,
    pub dirty: bool,
    pub last_saved_at: Option<String>,
    pub last_loaded_at: Option<String>,
    pub last_error: Option<String>,
}
```

### `RecentProjectEntry`

```rust
pub struct RecentProjectEntry {
    pub path: String,
    pub display_name: String,
    pub last_opened_at: String,
    pub exists: bool,
}
```

### `ProjectSaveResult` / `ProjectOpenResult`

Structured results with warnings array for non-fatal issues.

## Dirty Tracking Policy

The following actions mark the session as dirty:

- Add / move / delete schematic component
- Connect pins
- Rename net
- Update component parameter
- Assign component definition
- Attach SPICE model
- Apply notebook output to component

After a successful `save_current_project` or `save_project_as`, dirty is cleared.

## Storage Adapter

`LocalSettingsStorage` (`hotsas_adapters`) persists recent projects and session metadata as JSON to a local file (`hotsas_session.json` in the system temp directory). No SQLite, no cloud sync, no binary proprietary format.

## API / Tauri Commands

| Command                         | Description                                         |
| ------------------------------- | --------------------------------------------------- |
| `get_project_session_state`     | Returns current session state DTO                   |
| `save_current_project`          | Saves to existing path or returns error             |
| `save_project_as`               | Saves to provided path, updates session             |
| `open_project_package`          | Loads `.circuit` package with unsaved-changes guard |
| `list_recent_projects`          | Returns recent projects with `exists` flag          |
| `remove_recent_project`         | Removes entry by path                               |
| `clear_missing_recent_projects` | Removes entries whose paths no longer exist         |

## Unsaved Changes Guard

Opening or replacing a project while `dirty == true` requires explicit confirmation (`confirm_discard_unsaved: true`). If `false`, the backend returns a controlled error; the frontend shows a confirmation dialog.

## Frontend Components

- `ProjectToolbar` — New / Open / Save / Save As buttons, project name, dirty badge
- `RecentProjectsPanel` — Recent list with open / remove / clear-missing actions
- `UnsavedChangesBanner` — Alert banner when dirty, with Save / Save As buttons
- `ProjectPersistenceStatus` — Status indicator in workbench

## Testing

- Rust: `project_session_tests` (7 tests), `project_session_api_tests` (4 tests)
- Frontend: `ProjectToolbar.test.tsx`, `RecentProjectsPanel.test.tsx`, `UnsavedChangesBanner.test.tsx`

## Deferred

- Native file picker (per TZ, deferred to later version)
- Autosave daemon (per TZ, out of scope)
