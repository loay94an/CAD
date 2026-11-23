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

configurations.configureEach {
    resolutionStrategy.eachDependency {
        if (requested.group == "org.jetbrains.kotlin" &&
            requested.name in setOf("kotlin-stdlib", "kotlin-stdlib-jdk7", "kotlin-stdlib-jdk8")) {
            useVersion("1.8.22")
            because("Keep Kotlin standard-library artifacts aligned; older jdk7/jdk8 artifacts duplicate classes in kotlin-stdlib 1.8.22.")
        }
    }
}

dependencies {
    implementation("androidx.games:games-activity:4.4.0")
    implementation("androidx.appcompat:appcompat:1.7.0")
}
