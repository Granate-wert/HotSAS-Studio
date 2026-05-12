# Post-v3.3 Audit Issue Register

| ID        | Severity | Area               | Status   | Found in                                             | Evidence                                                                                                                                                                       | Impact                                                                                                          | Fix / Decision                                                                                                                                                  | Tests                                                                                                | Commit               |
| --------- | -------- | ------------------ | -------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | -------------------- |
| AUDIT-001 | Medium   | UI/Layout          | Fixed    | `apps/desktop-tauri/src/styles.css`, screen wrappers | `body` forced `min-width: 1100px`; Simulation, Diagnostics, Import Models, and Advanced Reports did not consistently use the standard `screen-panel` / `screen-content` shell. | The required 1024px narrow-window check could show horizontal overflow or inconsistent padding/scroll behavior. | Removed the desktop-only body width, allowed the toolbar to wrap, added shared screen shell wrappers, and added a `screen-container` width rule.                | Targeted Vitest suite: 32 tests PASS; browser smoke at 1024x768 PASS.                                | Pending final commit |
| AUDIT-002 | Low      | UI                 | Fixed    | `apps/desktop-tauri/src/screens/navigation.tsx`      | Navigation label read `E Component Library`.                                                                                                                                   | Visible UI typo and mismatch with the documented Component Library workflow.                                    | Renamed the navigation item to `Component Library`.                                                                                                             | `src/screens/navigation.test.tsx` PASS.                                                              | Pending final commit |
| AUDIT-003 | Low      | Repository Hygiene | Fixed    | `.gitignore`, preflight `git status --short`         | Root-level chat exports, prompt files, local fix scripts, and context folders were untracked and visible to git status.                                                        | Higher risk of accidentally staging local context or temporary scripts during audit commits.                    | Added ignore rules for post-audit prompts, chat exports, root text dumps, local fix scripts, scratch reports, and context folders.                              | `git status --short` confirms unrelated context files are hidden while audit changes remain visible. | Pending final commit |
| AUDIT-004 | Low      | UI/Verification    | Deferred | Native Tauri manual smoke                            | The agent environment can run Vite/browser smoke but does not provide a native interactive Tauri window for full manual desktop QA.                                            | Native-only menu/window integration is not manually exercised in this audit pass.                               | Code-level UI audit, Vitest, Vite browser smoke, and `tauri:build` are used as substitute evidence. Manual native smoke remains a follow-up on a local desktop. | Browser smoke RUN; native GUI manual smoke NOT RUN.                                                  | Pending final commit |

## Severity Summary

| Severity | Count |
| -------- | ----: |
| Critical |     0 |
| High     |     0 |
| Medium   |     1 |
| Low      |     3 |
| Minor    |     0 |

## Status Summary

| Status           | Count |
| ---------------- | ----: |
| Fixed            |     3 |
| Deferred         |     1 |
| Not Reproducible |     0 |
| Out of Scope     |     0 |
