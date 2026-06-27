# Open 3D Viewer

**A free, lightweight, offline-first 3D file viewer for Windows — built for game developers and 3D artists.**

Adobe Substance 3D Viewer was discontinued in 2025. Open 3D Viewer fills that gap: drag a file onto it and you're looking at your mesh in under a second, no account, no cloud, no subscription.

![Open 3D Viewer](https://github.com/Rinellasky/open-3d-viewer/raw/main/web/assets/samples/screenshot.png)

---

## Why it exists

Every 3D artist and game dev needs a fast "just show me this file" tool. Blender is great — but it's a full DCC, not a viewer. Windows Explorer thumbnails don't render PBR materials or CAD formats. Open 3D Viewer is the missing piece: instant preview, real lighting, zero overhead.

---

## Supported formats

| Category | Extensions |
|---|---|
| Real-time meshes | `.glb` `.gltf` (with DRACO) `.fbx` `.obj` `.stl` `.ply` |
| USD / USDZ | `.usdz` `.usd` `.usda` `.usdc` |
| CAD (open standards) | `.step` `.stp` `.iges` `.igs` `.brep` |
| Environments | `.hdr` `.exr` |
| Parametric generation | OpenSCAD (`.scad`) |

USD support uses [TinyUSDZ](https://github.com/lighttransport/tinyusdz) compiled to WebAssembly — no external binary needed. CAD formats use [OpenCascade via occt-import-js](https://github.com/kovacsv/occt-import-js), lazy-loaded only when needed.

---

## Features

- **Drag and drop** any supported file directly onto the window — or use **Open File...**
- **File associations** registered on install — double-click any `.glb`, `.fbx`, `.step`, etc. in Explorer
- **HDRI environment lighting** — load any `.hdr` / `.exr`, or pick from 5 built-in presets (Studio, Warehouse, Sunset, Neutral, Black)
- **PBR rendering** via Three.js — tone mapping (ACES Filmic, Neutral, Reinhard, Linear), exposure, environment intensity & rotation
- **Orbit / pan / zoom** — left-drag, right-drag, scroll
- **Wireframe, grid, auto-rotate** toggles
- **Auto-frame on load**, manual Frame Object, Reset View
- **PNG screenshot** export
- **Recent Files** — last 10 opens, click to reopen
- **Settings persistence** — env preset, tone mapping, grid, exposure, etc. restored on next launch
- **OpenSCAD generator** — 16 built-in templates (Primitives / Decorative / Mechanical / Practical) + a "Copy Claude Prompt" button for natural-language → OpenSCAD via Claude chat

---

## Install

### Option A — Download the installer (recommended)

Go to [Releases](https://github.com/Rinellasky/open-3d-viewer/releases) and download `Open3DViewer_0.1.0_x64-setup.exe`.

Run the installer (no admin required — installs to your user folder), and file associations are registered automatically.

### Option B — Build from source

**Prerequisites:** [Rust](https://rustup.rs/) · [Node.js](https://nodejs.org/) (optional, only for tooling) · [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

```powershell
git clone https://github.com/Rinellasky/open-3d-viewer.git
cd open-3d-viewer

# Download vendored JS libraries (Three.js, OpenSCAD-WASM)
.\setup.ps1

# Run in dev mode
cd src-tauri
cargo run

# Build release installer
tauri build
# → installer at: src-tauri/target/release/bundle/nsis/Open3DViewer_0.1.0_x64-setup.exe
```

---

## USD notes

`.usdc` / `.usda` / `.usd` files are loaded in-browser via TinyUSDZ (WebAssembly, ~2 MB, no install). If TinyUSDZ can't parse a specific file (e.g. advanced skinning or shader features), the app falls back to shelling out to `usdcat` if it's available on your system — either via `pip install usd-core` or from an existing Adobe Substance 3D Viewer installation.

---

## Settings & state storage

```
%APPDATA%\com.rinellasky.open3dviewer\state.json
```

---

## License

MIT — see [LICENSE](LICENSE).

Third-party library licenses are documented in [CREDITS.md](CREDITS.md).

---

## Credits

Built with [Tauri](https://tauri.app), [Three.js](https://threejs.org), [TinyUSDZ](https://github.com/lighttransport/tinyusdz), [occt-import-js](https://github.com/kovacsv/occt-import-js), and [openscad-wasm](https://github.com/openscad/openscad-wasm).

Sample model: [Duck.glb](https://github.com/KhronosGroup/glTF-Sample-Assets) © Sony Computer Entertainment Inc. (SCEA Shared Source License).  
Sample HDRI: [studio_small_03](https://polyhaven.com/a/studio_small_03) by Greg Zaal / Poly Haven (CC0).
