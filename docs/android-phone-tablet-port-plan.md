# Android phone + tablet port plan

Status: Android ARM64 debug APK builds successfully in CI. Package structure is verified; device runtime, mobile UX, and Android document access remain unverified/incomplete.

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


## Implementation checkpoint — Android shell scaffold

The following files now exist under `apps/cadcraft-android/` on `feat/android-phone-tablet`:

- A separate Rust `cdylib` package with an Android GameActivity entry point.
- A thin eframe host that constructs the original `CadApp(Session::new(), Services::default())`, forwards logic/input/UI to it, and installs the original CAD IO codecs.
- Android manifest and Java GameActivity launcher.
- Gradle app configuration restricted to `arm64-v8a`.
- A local build script and an Android debug-APK GitHub Actions workflow.

Important limits of this checkpoint:

- Android ARM64 debug APK build was verified by GitHub Actions run #17 and run #18; run #18 (`623712d`) completed successfully and its APK ZIP/package contents were inspected.
- SAF document picking and URI-backed open/save are not connected; the host currently uses default empty services.
- DWG worker support is not enabled or claimed.
- Responsive phone/tablet layout and real-device GPU/lifecycle validation remain outstanding.
- CI now checks the produced APK for its manifest, DEX bytecode, ARM64 `libmain.so`, unexpected native ABIs, and ZIP integrity before publishing the artifact. This is a packaging gate only, not a device runtime test.

The Android PR build workflow is also installed on the base branch so GitHub Actions can evaluate pull requests. The PR head was updated after installing the workflow to request the first CI run.


## Verified build checkpoint — 2026-11-24

- GitHub Actions run #18: [45291444858](https://github.com/loay94an/CAD/actions/runs/45291444858), commit `623712d264c36cb6d081dbc2b90b70cc86fab797`, completed successfully.
- Artifact: `cadcraft-android-arm64-debug`, artifact ID `14874273475`.
- Downloaded APK size: 76,742,935 bytes.
- ZIP integrity: passed; manifest and DEX files present; `lib/arm64-v8a/libmain.so` present; ARM64 is the only packaged native ABI.
- APK SHA-256: `5025887ff99af8ca448e6396f68c93332d27d71181d36c7e151e80f755df0c03`.
- No installation or runtime test on a physical Android device has been completed. Do not describe the app as device-tested or feature-complete.

## Next implementation order

1. Add Android Storage Access Framework integration with asynchronous URI selection and byte-stream read/write. The current engine file hooks and path-oriented opening flow need an adapter; a Java picker alone is insufficient.
2. Add Android-specific compact phone and expanded tablet presentation without changing shared CAD commands or document semantics.
3. Validate touch gestures and stylus behavior, Arabic shaping/RTL, and Android pause/resume/rotation.
4. Exercise core drawing operations and DXF round-trip on actual Android hardware; treat DWG as unsupported until its worker/bridge is independently validated.
