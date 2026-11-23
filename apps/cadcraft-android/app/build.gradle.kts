plugins {
    id("com.android.application")
}

android {
    namespace = "ai.storyteller.cadcraft"
    compileSdk = 35

    defaultConfig {
        applicationId = "ai.storyteller.cadcraft"
        minSdk = 23
        targetSdk = 35
        versionCode = 1
        versionName = "0.21.0-android"
        ndk {
            abiFilters += "arm64-v8a"
        }
    }

    sourceSets["main"].jniLibs.srcDirs("src/main/jniLibs")
}

dependencies {
    implementation("androidx.games:games-activity:4.4.0")
}
