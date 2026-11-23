# CADCraft Android

This is a separate Android host for the original CADCraft engine and egui UI.

## Current scope

- Native Rust cdylib entry point using winit's Android GameActivity integration.
- Original CadApp, Session, rendering and CAD format codecs are reused.
- Initial target: arm64-v8a, for both phones and tablets.
- Android document picker / Storage Access Framework is not wired yet. File open/save actions are therefore not advertised as available in this shell.
- DWG worker support is not enabled or claimed on Android.

## Build

From the repository root, with Rust 1.95+, Android SDK/NDK, Java 17, and cargo-ndk installed:

    cargo ndk -t arm64-v8a -o apps/cadcraft-android/app/src/main/jniLibs build --release -p cadcraft-android
    cd apps/cadcraft-android
    gradle assembleDebug

The debug APK is written to app/build/outputs/apk/debug/app-debug.apk.

This first shell is an integration milestone, not a feature-complete mobile release.
