# Android phone/tablet product audit and implementation plan

**Branch:** `feat/android-phone-tablet`  
**Audited revision:** `59cff36462dda197a434f55835d2f4e327df890a`  
**Audit date:** 2026-11-24  
**Scope:** Android host, shared egui UI, command line, file workflows, export/plot, mobile interaction, Arabic UI.

## Executive finding

The Android ARM64 debug APK builds in CI, but the Android product is not yet a complete mobile CAD application. The current host constructs the original desktop-oriented `CadApp` and reuses the original engine, which is the correct architectural foundation. Android-specific work so far is mostly dock visibility/sizing, horizontal menu scrolling, and touch navigation/event routing. It does not yet provide a complete mobile workspace or Android document workflows.

The most important functional blocker is file access: `apps/cadcraft-android/src/lib.rs` constructs `CadApp` with `Services::default()`. The shared UI's Open, Save As, Export, Import and Publish routes request host picker services through `pick_then`; those services are not supplied by the Android host. The engine codecs are installed, but codecs alone do not grant Android access to user documents.

## Verified current state

| Area | Source evidence | Current assessment |
|---|---|---|
| Engine integration | Android host creates original `Session` and `CadApp`; installs CAD IO codecs | Reuse architecture is sound |
| APK build | Android workflow builds ARM64 APK and validates ZIP, manifest, DEX and `libmain.so` | CI packaging passes; not a device test |
| Android document picker | Host uses `Services::default()`; README says SAF is not wired | Open/save picker unavailable |
| Save/export/import/publish | Shared menu and exchange routes use `pick_then` for file destinations/sources | UI routes exist but Android host cannot complete picker-based flows |
| Command line | Shared command line has prompt keywords, history, autocomplete, IME handling and command submission | Feature-rich desktop implementation exists; mobile ergonomics and keyboard behavior need dedicated validation |
| Workspace | Android defaults hide desktop docks; panels are opened from Window/menu; Android dock widths are still desktop-scale | Not a phone-first workspace |
| Menus | Android menu row scrolls horizontally | Reachability improved, but primary workflows remain menu-centric |
| Touch | Native touch is converted to press steps; multi-touch cancels pending press and navigates | CI compiles; drawing/selection gesture semantics still need device validation |
| Arabic | Arabic catalog exists and Android forces Arabic language | Catalog is partial; RTL/shaping and mixed Arabic/Latin command strings need audit |
| DWG | Android host does not enable/validate the desktop worker bridge | Do not claim DWG support on Android until separately verified |

## Product design target

### Phone workspace
- Canvas-first screen with a compact top bar for document, undo/redo, selection mode and overflow actions.
- A bottom tool dock with large touch targets, grouped by Draw / Modify / Annotate / Measure.
- A command console that can collapse to a one-line prompt and expand into history + prompt + suggestions; it must not permanently consume drawing space.
- A dedicated command-entry action that opens the Android IME and preserves CAD prompt focus.
- Layers, Properties, Tool Sets, Blocks and other palettes shown as modal side sheets/bottom sheets, not desktop docks that compete with the canvas.
- A clear gesture model: one finger draws/selects according to active tool; two fingers pan/zoom; UI controls never leak gestures into the canvas.
- Contextual selection handles and touch-sized snap targets.

### Tablet workspace
- Expanded workspace with canvas, optional persistent tool palette, command line and docked Layers/Properties.
- Resizable/collapsible panels within available width; no floating native desktop windows.
- Keyboard and stylus support without removing touch controls.

### Shared behavior
- Keep one CAD document model, command registry, geometry engine, snapping, layers, properties, undo/redo and codecs.
- Do not fork or rewrite the CAD engine for Android.
- Keep desktop and web behavior unchanged unless a shared bug fix is required.

## Implementation workstreams and acceptance criteria

### P0 — Android document access and export
1. Implement Android Storage Access Framework open/create-document flows using Activity Result APIs.
2. Bridge selected content URIs to byte streams; do not pretend a content URI is a filesystem path.
3. Adapt the shared host file service boundary so Open, Save, Save As, Import, APPLOAD and related workflows can use asynchronous Android picker results safely.
4. Write through the URI output stream, flush/close, and report failures without losing the current drawing.
5. Connect SVG, PNG and PDF export to Android create-document destinations; expose format choice and a meaningful filename.
6. Add round-trip tests for DXF (open → edit → save → reopen), plus export checks for SVG/PNG/PDF.
7. Keep DWG disabled or clearly marked unsupported on Android until its reader/writer bridge is proven.

**Acceptance:** A user can open a DXF from Downloads/Drive/provider storage, edit it, save to a chosen destination, reopen the saved file, and export a PDF/SVG/PNG to a chosen destination on a physical Android device.

### P1 — Command console
1. Replace the assumption of a permanently visible desktop floating command bar with a mobile console component/layout.
2. Collapsed state: prompt, active command/keyword hint, command-entry button and expand affordance.
3. Expanded state: scrollable command history, prompt, autocomplete and clickable keyword chips.
4. Keep command history accessible without obscuring the canvas; support keyboard submit, Escape/cancel, Enter, Tab completion and IME composition.
5. Ensure text selection, cursor movement, paste and hardware keyboard input work.
6. Preserve all existing command syntax, aliases, prompts and command IDs.

**Acceptance:** Start LINE, enter coordinates/distance/angle, complete/cancel the command, inspect history, choose a prompt keyword and repeat a command using only touch and Android keyboard.

### P2 — Phone-first navigation and tool access
1. Introduce explicit phone/tablet breakpoints based on available logical size and orientation.
2. Add a compact top app bar and bottom tool dock on phones.
3. Move palettes into modal sheets on phones; use persistent collapsible docks on tablets.
4. Provide reachable undo/redo, selection, delete, snap, ortho, grid, layer and properties actions.
5. Separate draw/select/pan modes where gesture ambiguity remains; do not overload one-finger gestures.
6. Define touch target sizes, safe-area handling, keyboard-inset handling and landscape behavior.

**Acceptance:** Core actions are reachable without horizontal menu hunting; canvas remains usable with command console and panels opened; phone portrait/landscape and tablet layouts do not clip.

### P3 — File/project and export experience
1. Add a document hub for New, Open, Recent, Save, Save As and recovery.
2. Display dirty/unsaved state and confirm destructive close/switch actions.
3. Create an export sheet with SVG, PNG and PDF options, output scale/resolution, paper/page settings and preview where supported.
4. Distinguish Save (native editable drawing) from Export (derived output).
5. Report unsupported format features and lossy conversions before saving.

**Acceptance:** Every visible file action has a working Android implementation or is explicitly disabled with an explanation.

### P4 — Arabic and accessibility
1. Audit Arabic translations for command console, file dialogs, export, tool names and settings.
2. Validate RTL layout while keeping CAD command IDs, coordinate values, units and file paths readable LTR where appropriate.
3. Test Arabic shaping, mixed Arabic/English, digits, punctuation, negative coordinates and IME composition.
4. Add accessibility labels and large-target alternatives for critical actions.

**Acceptance:** No clipped/reversed critical labels; coordinate and command entry remain predictable in Arabic mode.

### P5 — Reliability and release verification
1. Test app launch, pause/resume, orientation changes, low-memory recovery and GPU context recreation.
2. Test touch drawing, selection, grips, snapping, two-finger pan/zoom and cancellation.
3. Run engine/UI tests and Android build in CI.
4. Validate APK package and install/launch on physical phone and tablet.
5. Publish a release APK only after functional gates pass; record tested devices, Android versions and known limitations.

## Test matrix

| Workflow | Automated coverage | Physical-device coverage |
|---|---|---|
| Draw line/polyline/circle and commit on release | Engine/UI gesture tests | Required |
| Select, move, grips, undo/redo | Existing and new UI tests | Required |
| Two-finger pan/pinch without stray CAD input | Touch event tests | Required |
| Command console, IME, history, autocomplete | UI tests | Required |
| Open/save/save-as DXF through SAF | URI adapter/unit tests | Required |
| Export PDF/SVG/PNG | Codec/export tests | Required |
| Arabic/RTL/mixed coordinate entry | Localization/layout tests | Required |
| Pause/resume/rotation/GPU recovery | Lifecycle instrumentation | Required |

## Work order

1. Implement the Android document-provider bridge first because it unblocks real Open/Save/Export and validates the app's core purpose.
2. Build the mobile command console on top of the existing command engine.
3. Replace phone desktop docks with touch-first sheets and tool dock; keep tablet layout expanded.
4. Complete Arabic/RTL and gesture arbitration.
5. Run full tests, build APK, inspect package, then request real-device verification for the remaining hardware-only gates.

## Non-claims

- A successful APK build does not prove the app launches or works on hardware.
- A registered command ID does not prove the command's full workflow works.
- Installing IO codecs does not provide Android document access.
- Desktop DWG support does not imply Android DWG support.
- Existing Arabic strings do not prove full RTL correctness.
