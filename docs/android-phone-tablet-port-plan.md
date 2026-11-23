# Android phone + tablet port plan

Status: architecture and compatibility assessment only. No production source files have been changed.

## Goal

Ship one Arabic CADCraft Android APK that adapts to phones and tablets while reusing the original CADCraft engine, document model, renderer, command system, and file-format crates. Keep the desktop and browser applications intact.

## Verified source architecture

- Rust workspace, edition 2024, minimum Rust 1.95.
- Shared crates: geom, color, doc, constraints, fonts, render, io, engine, ui-egui, mcp.
- Desktop binary: apps/cadcraft; current entry point is a desktop eframe/winit application.
- Browser binary: apps/cadcraft-web; uses the wasm32 eframe runner.
- UI state is owned by cadcraft-ui-egui::CadApp; drawing data and commands are owned by cadcraft-engine::Session.
- The IO crate contains DXF/DWG and SVG import plus SVG/PNG/PDF output.
- The workspace pins reviewed forks/revisions of egui/winit and craft-libs. Preserve these pins during the port.
- Arabic translation catalog exists at crates/ui-egui/src/i18n/ar.tsv; translation coverage is incomplete and RTL layout needs a separate UI audit.

## Android integration approach

Create a separate apps/cadcraft-android package. Do not turn the desktop package into a single cross-platform binary and do not replace the CAD engine with a WebView.

Preferred initial integration:
- eframe 0.37 with wgpu and the android-game-activity backend.
- Android arm64-v8a first; add armeabi-v7a only if dependency and performance tests justify it.
- Android-specific application entry point and packaging/manifest configuration.
- Reuse CadApp, Session, render, geometry, document and IO code wherever their dependencies are platform-neutral.
- Keep Android-only lifecycle, storage, picker and UI adaptation in the new package or narrow platform abstractions.

Why GameActivity: eframe exposes android-game-activity, and its current source documents an AccessKit compatibility limitation for android-native-activity. Re-evaluate accessibility feature selection during the first compile.

## Known blockers to resolve before a valid APK

1. apps/cadcraft/src/main.rs is desktop-specific: desktop window options, desktop event-loop configuration, OS services, and process/worker setup cannot be copied unchanged.
2. apps/cadcraft/Cargo.toml enables desktop features and dependencies including Wayland/X11, native windows, rfd, sys-locale, HID backends and platform-specific services. Android needs its own dependency graph and cfg boundaries.
3. craft-ui native floating windows are not an appropriate assumption on phones; panel docking/floating behavior must be constrained or adapted for Android.
4. File open/save currently relies on desktop paths/dialogs. Android needs Storage Access Framework document selection and URI/stream-based access, or a carefully tested Android bridge. Do not assume ordinary filesystem paths for selected documents.
5. DWG uses craft-dwg worker/process behavior in the desktop startup path. Android process launching and the DWG bridge must be validated independently; do not advertise DWG read/write until it passes device tests.
6. GPU canvas, stencil/clipping, texture limits, context loss and app pause/resume need real-device validation.
7. UI layout currently assumes desktop window dimensions (minimum 900x560 points). Add responsive layout modes without changing engine behavior.
8. The Arabic catalog is partial; validate Arabic shaping, bidirectional text, RTL navigation and font fallback in both app chrome and drawing text.

## Responsive UI targets

- Phone: compact command/tool bars, maximum practical canvas area, bottom/side tool drawer, touch-first selection and grips, two-finger pan/zoom, deliberate gesture arbitration, larger touch targets, optional stylus pressure only if supported.
- Tablet: expanded CAD workspace with persistent toolsets, layer/property panels, command line and status bar; panels may collapse or dock but must not require desktop-style floating OS windows.
- Both: same document model, command IDs, drawing semantics, file formats and settings wherever supported.

## Delivery gates

### Gate A — Android shell
- Android package compiles for arm64-v8a and installs.
- Launch, pause/resume, rotate, background/foreground and clean exit work.
- Arabic font and basic UI render without GPU validation errors.

### Gate B — original engine integration
- Create/open drawing; line, polyline, circle, arc, selection, grips, snap, layers, properties, undo/redo, zoom and pan.
- Save/reopen DXF and compare geometry/document state.
- Confirm no regressions in desktop CI.

### Gate C — Android file access and export
- SAF open/save round trips for DXF.
- SVG/PNG/PDF export through Android-compatible destinations.
- DWG is a separate gate and remains marked unsupported until worker/bridge tests pass.

### Gate D — phone/tablet UX and release
- Test compact phone and large tablet breakpoints, touch, stylus if available, RTL, keyboard, accessibility and memory/performance.
- Build signed release APK and attach reproducible build information.

## Non-goals

- Do not rewrite or replace CADCraft's engine.
- Do not remove original desktop/web apps.
- Do not silently drop file-format capabilities.
- Do not claim DWG support, smooth GPU performance or a working APK based only on a successful Rust compile.
