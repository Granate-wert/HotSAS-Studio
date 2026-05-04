# Internal Alpha Build Guide

## What is an internal alpha build?

An internal alpha build is a locally compiled Windows `.exe` of HotSAS Studio, produced for **internal verification only**. It is **not** a public release, not a GitHub Release, and not a v2.0 beta.

## Why this is not a public release

- No public GitHub Release is created.
- No public release tag is created.
- The build is not production-ready.
- It exists solely to verify that the application starts, navigates, and runs basic smoke tests on a clean Windows machine without Rust/Node/npm installed.

## How to build the `.exe`

### Prerequisites

- Windows 10/11
- Rust stable toolchain
- Node.js LTS (v20+)
- Tauri CLI dependencies (WebView2 runtime)

### Build steps

```powershell
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm install
npm.cmd run tauri:build
```

The build may emit PowerShell formatting warnings (exit code 1), but the `.exe` is still produced correctly.

## Where the `.exe` is located

After a successful build:

```text
apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
```

## How to create a ZIP

```powershell
$releaseDir = "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri\src-tauri\target\release"
$exe = Join-Path $releaseDir "hotsas_desktop_tauri.exe"
$zip = Join-Path $releaseDir "HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip"
Compress-Archive -LiteralPath $exe -DestinationPath $zip -Force
```

## How to verify SHA256

```powershell
Get-FileHash $exe -Algorithm SHA256
Get-FileHash $zip -Algorithm SHA256
```

## How to run on another PC

1. Copy `HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip` to the target PC.
2. Extract to a local folder **without Cyrillic characters** in the path, e.g. `C:\HotSAS-alpha\`.
3. Run `hotsas_desktop_tauri.exe`.
4. Verify the main window opens and core screens are navigable.

## Limitations before v2.0

- No PCB editor.
- No routing/Gerber/DRC.
- No proprietary Altium file generation.
- No full KiCad project export.
- No full symbolic solver.
- No production-ready SPICE model manager.
- No Touchstone graph viewer.
- ngspice is optional; the app falls back to a mock simulation engine.

## Known limitations

- Large frontend bundles (>500 KB after gzip) — code splitting will be improved in v2.0.
- Export Center may require ACL permission fixes when new commands are added.
- Component library is built-in only; external library loading is not yet supported.
