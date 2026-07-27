plugins {
  // to automatically download the toolchain jdk if missing
  id("org.gradle.toolchains.foojay-resolver-convention") version "0.9.0"
}

rootProject.name = "godot-rosetta-rpc-example"

include(":kotlin-runtime")

project(":kotlin-runtime").projectDir = file("../kotlin-runtime")
