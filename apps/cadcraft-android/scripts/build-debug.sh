#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"

: "${ANDROID_HOME:?Set ANDROID_HOME to the Android SDK directory}"

if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
    if [[ ! -d "$ANDROID_HOME/ndk" ]]; then
        echo "Android NDK not found under $ANDROID_HOME/ndk" >&2
        exit 1
    fi
    ANDROID_NDK_HOME="$(find "$ANDROID_HOME/ndk" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)"
    if [[ -z "$ANDROID_NDK_HOME" ]]; then
        echo "No installed Android NDK version was found" >&2
        exit 1
    fi
    export ANDROID_NDK_HOME
fi

command -v cargo-ndk >/dev/null || {
    echo "cargo-ndk is required (install with: cargo install cargo-ndk --locked)" >&2
    exit 1
}
command -v gradle >/dev/null || {
    echo "Gradle 8.9 or newer is required" >&2
    exit 1
}

cargo ndk -t arm64-v8a -o apps/cadcraft-android/app/src/main/jniLibs build --release -p cadcraft-android
cd apps/cadcraft-android
gradle --no-daemon assembleDebug

APK="$PWD/app/build/outputs/apk/debug/app-debug.apk"
test -s "$APK"
printf '\nAPK: %s\n' "$APK"
