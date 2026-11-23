# CADCraft Android

This is a separate Android host for the original CADCraft engine and egui UI.

## Current scope

- Native Rust cdylib entry point using winit's Android GameActivity integration.
- Original CADCraft `CadApp` and `Session` are linked into the Android host.
- Verified GitHub Actions run #18 (`623712d`) produced a structurally valid ARM64 debug APK; package inspection checks are now part of CI.
- Original CadApp, Session, rendering and CAD format codecs are reused.
- Initial target: arm64-v8a, for both phones and tablets.
- Android document picker / Storage Access Framework is not wired yet. The current host passes `Services::default()`, so document open/save through Android storage is not yet available.
- DWG worker support is not enabled or claimed on Android.
- Responsive phone/tablet layout and real-device lifecycle/GPU validation remain pending.

## Build

From the repository root, with Rust 1.95+, Android SDK/NDK, Java 17, and cargo-ndk installed:

    cargo ndk -t arm64-v8a -o apps/cadcraft-android/app/src/main/jniLibs build --release -p cadcraft-android
    cd apps/cadcraft-android
    gradle assembleDebug

The debug APK is written to app/build/outputs/apk/debug/app-debug.apk. CI verifies that the package contains the manifest, DEX bytecode, the ARM64 `libmain.so`, no unexpected native ABIs, and no corrupt ZIP entries.

The GitHub Actions workflow `.github/workflows/android-build.yml` builds the arm64-v8a debug APK and publishes it as the `cadcraft-android-arm64-debug` artifact when the build succeeds. The Android SDK setup uses the maintained setup-android v4 action; pull-request builds validate the arm64 Android package. The CI workflow is intentionally triggered by this documentation path as well as Android source changes.

This first shell is an integration milestone, not a feature-complete mobile release. Do not treat the APK as device-validated until it has been installed and tested on Android hardware.
