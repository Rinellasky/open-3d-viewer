# SubstanceViewerAlt

A lightweight local-first 3D viewer to fill the gap left by Adobe Substance 3D Viewer.

## How to use

1. **First-time setup**: open PowerShell in this folder and run `.\setup.ps1`. This downloads the OpenSCAD-WASM vendor files (~8 MB, one time). Skip if you don't plan to use text-to-3D.
2. Double-click `index.html` — opens in your default browser. Works offline once cached. (First load fetches Three.js from unpkg; after that it stays in browser cache. To make it fully offline, see *Vendoring* below.)
3. Drag a 3D file onto the window, or click **Open File...**.
4. Drag an `.hdr` or `.exr` onto the window to use it as the environment, or click **Load HDR/EXR...**.
5. Click **Generate (OpenSCAD)** to open the parametric text-to-3D panel.

## Supported formats

| Type | Extensions | Notes |
|------|------------|-------|
| Real-time meshes | `.glb` `.gltf` (with DRACO) `.fbx` `.obj` `.stl` `.ply` | Three.js native loaders |
| USDZ | `.usdz` | Three.js native loader |
| USD (raw) | `.usd` `.usda` `.usdc` | Phase 3: wrapped into USDZ in memory. Works for self-contained files. Files with external asset references need full Pixar USD support (planned). |
| CAD (open standards) | `.step` `.stp` `.iges` `.igs` `.brep` | Phase 2: OpenCascade compiled to WASM via [`occt-import-js`](https://github.com/kovacsv/occt-import-js). |
| Environments | `.hdr` `.exr` | RGBE / OpenEXR |
| Generation | text + OpenSCAD code | Phase 4: rendered with [openscad-wasm](https://github.com/openscad/openscad-wasm) to STL → loaded into viewer |

## Features

- Orbit / pan / zoom (mouse drag, right-drag, scroll)
- HDRI environment lighting + procedural fallbacks (Studio / Warehouse / Sunset)
- Environment rotation, intensity, exposure
- Tone mapping presets (ACES Filmic, Neutral, Reinhard, Linear)
- Wireframe toggle, grid toggle, auto-rotate
- PNG screenshot export
- Auto-frame, reset view

## Roadmap

### Phase 2 — CAD interop
- Add **STEP** and **IGES** import via [`occt-import-js`](https://github.com/kovacsv/occt-import-js) (OpenCascade compiled to WASM, runs in the browser, no install).
- Optional: headless FreeCAD pipeline for richer conversion (`freecadcmd` → GLB).

### Phase 3 — USD natively
- Currently USDZ loads via Three.js's experimental loader. For full `.usd / .usda / .usdc` support, options:
  - **`three-usdz-loader`** — lighter but read-only.
  - **`usd-wasm`** — full Pixar USD compiled to WASM, heavy (~30 MB) but real.

### Phase 4 — Text-to-3D (DONE: parametric)
- ✅ Built-in template library (cube, sphere, cylinder, hollow box, rounded box, vase, gear, L-bracket, knurled knob, phone stand).
- ✅ OpenSCAD code editor + browser-side render via openscad-wasm.
- ✅ "Copy Claude Prompt" button generates a structured prompt template for use in a Claude chat — paste the output back into the editor and Render.
- ✅ "Save .scad" to download your code.

Future for this phase: hosted-API generation (Meshy / Tripo / Hyper3D) for organic shapes, optional local ONNX model support for fully-offline generative.

### Phase 5 — Desktop wrap
Wrap in **Tauri** (small binary, Rust core) for:
- File associations (double-click a `.glb` to open in this app)
- Recent files menu
- Native title bar / drag region
- Settings persistence

## Vendoring (going fully offline)

Currently uses `unpkg.com` for Three.js. To make `index.html` truly offline:

```powershell
# In C:\AppDev\SubstanceViewerAlt
mkdir vendor\three
# Download three@0.160.0/build/three.module.js into vendor\three\
# Download three@0.160.0/examples/jsm/* recursively into vendor\three\jsm\
# Update the importmap in index.html to point at ./vendor/three/...
```

A `vendor.ps1` helper script can be added later.

## Notes on Adobe assets

The original Substance 3D Viewer installation at
`C:\Program Files\Adobe\Adobe Substance 3D Viewer (Beta)` has been left untouched.
Its HDRIs (Atelier / Exterior / Studio) and reference USDs are Adobe-owned and not
copied into this project. Recommended free replacements:

- HDRIs → [Poly Haven](https://polyhaven.com/hdris) (CC0 public domain)
- Material ball → [glTF Sample Models](https://github.com/KhronosGroup/glTF-Sample-Models) (`MetalRoughSpheres`, `MaterialBall`)

## Architecture (for future reference)

Original Substance Viewer was Qt 6 Quick 3D + Pixar USD + Hydra rendering, with
Tech Soft 3D's HOOPS Exchange for CAD interop and ONNX Runtime + DirectML for ML.
This project covers the same use cases with open standards (glTF, USD, MaterialX)
and a browser-first runtime. The CAD interop gap is filled with OpenCascade
instead of HOOPS — fewer formats supported, but no licensing concerns.
