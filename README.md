# SubstanceViewerAlt

A lightweight local-first 3D viewer to fill the gap left by Adobe Substance 3D Viewer.

## How to use

1. Double-click `index.html` — opens in your default browser. Works offline once cached. (First load fetches Three.js from unpkg; after that it stays in browser cache. To make it fully offline, see *Vendoring* below.)
2. Drag a 3D file onto the window, or click **Open File...**.
3. Drag an `.hdr` or `.exr` onto the window to use it as the environment, or click **Load HDR/EXR...**.

## Supported formats (Phase 1)

| Type | Extensions |
|------|------------|
| Models | `.glb` `.gltf` (with DRACO) `.fbx` `.obj` `.stl` `.ply` `.usdz` |
| Environments | `.hdr` `.exr` |

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

### Phase 4 — Text-to-3D
Three viable approaches, in order of effort:
1. **Parametric LLM-driven** — Claude writes OpenSCAD or CadQuery scripts from your prompt → render to STL/GLB → viewer loads. Best for mechanical/geometric drafts.
2. **Hosted API** — Meshy / CSM / Hyper3D / Tripo. Paste their API key, paste prompt, get GLB back.
3. **Local ONNX models** — what Substance Viewer did. Heavy (1–5 GB weights, GPU recommended) but fully offline.

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
