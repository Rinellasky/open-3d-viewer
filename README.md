# open-3d-viewer

A lightweight local-first 3D viewer to fill the gap left by Adobe Substance 3D Viewer.

## Layout

```
open-3d-viewer/
  web/              -- the frontend (browser-runnable, served by Tauri)
    index.html
    assets/         -- sample model + sample HDRI (committed)
    vendor/         -- Three.js + openscad-wasm (gitignored, populated by setup.ps1)
  src-tauri/        -- Rust/Tauri desktop wrapper
  setup.ps1         -- one-time vendor downloader
  README.md, CREDITS.md
```

## How to use

1. **First-time setup (required):** open PowerShell in this folder and run:
   ```
   .\setup.ps1
   ```
   Downloads Three.js (~23 MB) and OpenSCAD-WASM (~8 MB) into `web/vendor/`.
   Re-running is a no-op if already installed.

2. **Run as desktop app (recommended):**
   ```
   cd src-tauri
   cargo run          # debug build
   tauri build        # full release installer (.exe + .msi)
   ```
   The release installer registers all file associations on Windows so
   Explorer shows "Open with Open 3D Viewer" for the supported extensions.

3. Drag a 3D file onto the window, click **Open File...**, or — once you've
   built and installed — double-click any registered file in Explorer.

4. Drag an `.hdr` or `.exr` onto the window to use it as the environment, or
   click **Load HDR/EXR...**.

5. Click **Generate (OpenSCAD)** to open the parametric text-to-3D panel.

## Supported formats

| Type | Extensions | Notes |
|------|------------|-------|
| Real-time meshes | `.glb` `.gltf` (with DRACO) `.fbx` `.obj` `.stl` `.ply` | Three.js native loaders |
| USDZ | `.usdz` | Three.js native loader |
| USD (binary) | `.usdc` `.usda` `.usd` | Desktop only. Routed through `usdcat → .glb → GLTFLoader` for robust handling of primvars, materials, UVs, skinning. Plain-browser falls back to wrapping `.usd`/`.usda` as USDZ (best-effort). |
| CAD (open standards) | `.step` `.stp` `.iges` `.igs` `.brep` | OpenCascade compiled to WASM via [`occt-import-js`](https://github.com/kovacsv/occt-import-js) (lazy-loaded). |
| Environments | `.hdr` `.exr` | RGBE / OpenEXR equirect |
| Generation | OpenSCAD source | Rendered with [openscad-wasm](https://github.com/openscad/openscad-wasm) to STL → loaded into viewer |

## Features

- Orbit / pan / zoom (left-drag, right-drag, scroll)
- HDRI environment lighting + 3 procedural presets (Studio / Warehouse / Sunset / Neutral / Black)
- Environment intensity, rotation; tone mapping (ACES Filmic, Neutral, Reinhard, Linear, None); exposure
- Wireframe, grid, auto-rotate toggles
- Auto-frame on load, manual Frame Object, Reset View
- PNG screenshot export
- **Recent Files**: last 10 opens, click to reopen — desktop only
- **Settings persistence**: env preset, intensity, rotation, grid, tone mapping, exposure, background visibility — all restored on next launch
- **OpenSCAD generator**: 16 built-in templates across Primitives / Decorative / Mechanical / Practical, plus a "Copy Claude Prompt" button that generates a structured prompt template for natural-language → OpenSCAD via Claude chat

## Phase history

- **Phase 1** — Browser-based viewer (Three.js, GLB/GLTF/FBX/OBJ/STL/PLY/USDZ, HDR/EXR env) ✅
- **Phase 2** — STEP / IGES / BREP via occt-import-js ✅
- **Phase 3** — USD (`.usd / .usda / .usdc`) via usdcat → .glb pipeline ✅
- **Phase 4** — Text-to-3D via OpenSCAD-WASM (16 templates + Claude prompt helper) ✅
- **Phase 5** — Tauri desktop wrap, file associations, argv-to-WebView open ✅
- **Phase 6** — Recent Files + settings persistence ✅
- **Phase 7** — Vendor Three.js locally (offline-first) ✅
- **Phase 8** — usd-wasm to drop the `usdcat` shell-out — *open*

## How USD support actually works

Three.js's USDZ loader is a USDA (text) parser only. Real-world `.usdc` files
ship from DCCs with features that parser doesn't implement (primvars, complex
shaders, skinning). The workaround: shell out to Pixar's `usdcat` with the
`usdGltf` plugin enabled, which converts any USD variant directly to `.glb`.
That bypasses the limited USDA parser entirely and feeds the result through
Three's well-tested GLTFLoader.

`usdcat` discovery order (in Rust):
1. `usdcat` on PATH (works if you ran `pip install usd-core`)
2. Known Python install Scripts dirs (miniconda3, Anaconda3, Programs/Python*)
3. `C:\Program Files\Adobe\Adobe Substance 3D Viewer (Beta)\usdcat.exe` (works
   as long as the Adobe install is still on disk — open-3d-viewer doesn't ship
   any of Adobe's binaries)

## Settings & recent files storage

State lives in a single JSON file at:
```
%APPDATA%\com.rinellasky.open3dviewer\state.json
```
JS owns the schema, Rust just round-trips it via `save_app_state` /
`load_app_state` commands. Recents are path-keyed; the browser file picker
and browser drag-drop don't yield disk paths, so only argv launches,
Tauri-native drag-drop, and Recent-list clicks contribute to the list.

## Notes on Adobe assets

The original Substance 3D Viewer installation at
`C:\Program Files\Adobe\Adobe Substance 3D Viewer (Beta)` is left untouched.
Adobe's HDRIs, sample models, and binaries are not copied into this project.
The only Adobe-shipped binary this project ever invokes is `usdcat.exe`, and
only as a fallback if no other Pixar USD tools are installed.

## Architecture (for future reference)

Original Substance Viewer was Qt 6 Quick 3D + Pixar USD + Hydra rendering,
with Tech Soft 3D's HOOPS Exchange for CAD interop and ONNX Runtime + DirectML
for ML-based text-to-3D. This project covers the same use cases with open
standards (glTF, USD, MaterialX) and a WebView-first runtime. The CAD interop
gap is filled with OpenCascade instead of HOOPS — fewer formats supported,
but no licensing concerns.
