#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
: "${ANDROID_HOME:?Set ANDROID_HOME to the Android SDK directory}"
: "${ANDROID_NDK_HOME:=$ANDROID_HOME/ndk}"
cargo ndk -t arm64-v8a -o apps/cadcraft-android/app/src/main/jniLibs build --release -p cadcraft-android
cd apps/cadcraft-android
gradle --no-daemon assembleDebug
printf '\nAPK: %s\n' "$PWD/app/build/outputs/apk/debug/app-debug.apk"
