# Credits & Licenses

This project uses the following third-party assets and libraries.

## Bundled sample assets (committed)

### `web/assets/samples/Duck.glb`
- Source: [Khronos glTF Sample Assets](https://github.com/KhronosGroup/glTF-Sample-Assets)
- License: SCEA Shared Source License 1.0 (free for non-commercial and commercial use; attribution required)
- Copyright © 2006 Sony Computer Entertainment Inc.

### `web/assets/hdris/studio_small_03_1k.hdr`
- Source: [Poly Haven](https://polyhaven.com/a/studio_small_03)
- License: [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) (public domain)
- Author: Greg Zaal / Poly Haven

## Vendored libraries (downloaded by setup.ps1 into `web/vendor/`, gitignored)

### [Three.js](https://threejs.org/) v0.160.0
- License: [MIT](https://github.com/mrdoob/three.js/blob/master/LICENSE)
- Source: `https://registry.npmjs.org/three/-/three-0.160.0.tgz`
- Includes the `examples/jsm/` modules used by the viewer (loaders, controls, environments, libs, utils).

### DRACO decoder
- Bundled with Three.js examples.
- License: Apache 2.0 (Google).

### [TinyUSDZ wasm](https://github.com/lighttransport/tinyusdz) v0.9.1
- License: Apache 2.0 + MIT
- Source: `https://registry.npmjs.org/tinyusdz/-/tinyusdz-0.9.1.tgz`
- Used as the primary loader for `.usd` / `.usda` / `.usdc` files. Reads both text and binary USD natively in WebAssembly, with no external binary dependency.

### [openscad-wasm](https://github.com/openscad/openscad-wasm) 2022.03.20
- License: GPL-2.0 (matches OpenSCAD itself)
- Source: GitHub Releases for [`openscad/openscad-wasm`](https://github.com/openscad/openscad-wasm/releases/tag/2022.03.20)

## Runtime libraries (lazy-loaded from CDN, not bundled)

### [occt-import-js](https://github.com/kovacsv/occt-import-js) 0.0.22
- License: LGPL-2.1 (matches OpenCascade)
- Loaded from `cdn.jsdelivr.net` only when a `.step` / `.iges` / `.brep` file is opened.

## Build-time dependencies

### [Tauri 2](https://tauri.app)
- Used as the desktop wrapper.
- License: MIT / Apache-2.0.

### Pixar USD tools (optional fallback, called at runtime)
- If TinyUSDZ can't parse a particular USD file (e.g. uses features the smaller
  implementation doesn't cover yet), the desktop app falls back to shelling out
  to `usdcat` to convert via the `usdGltf` plugin.
- This is now a fallback path, not the primary one. License: Apache-2.0 (Modified).
- This project does NOT bundle usdcat. It discovers an existing installation
  (preferring `pip install usd-core`, falling back to Adobe Substance 3D Viewer's
  bundled copy if present).

## Notes on Adobe Substance 3D Viewer

This project is a from-scratch alternative inspired by Adobe Substance 3D Viewer
(discontinued 2025). No Adobe code, binaries, HDRIs, models, or icons are
reused or redistributed. The user's local copy at `C:\Program Files\Adobe\Adobe
Substance 3D Viewer (Beta)` was inspected for reference only, and is now only
relevant if the `usdcat` fallback is needed (rare).
