pluginManagement {
  plugins {
    // Kept in sync with the Kotlin version the godot-kotlin-jvm Gradle
    // plugin resolves.
    kotlin("jvm") version "2.3.20"
  }
}

plugins {
  // to automatically download the toolchain jdk if missing
  id("org.gradle.toolchains.foojay-resolver-convention") version "0.9.0"
}

dependencyResolutionManagement {
  versionCatalogs { create("libs") { from(files("../gradle/libs.versions.toml")) } }
}

rootProject.name = "godot-rosetta-rpc"
