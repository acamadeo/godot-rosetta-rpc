// The handwritten Kotlin runtime for the godot-rosetta-rpc cross-language
// RPC framework — the Kotlin mirror of ../rust-runtime. A plain Kotlin/JVM
// library (not the full com.utopia-rise.godot-kotlin-jvm app plugin, which
// in this plugin version has no library mode — see
// https://godot-kotl.in/en/stable/develop-libraries/ for a newer version
// that does): it depends directly on the same godot-api/core-library jars
// the plugin itself resolves, since it only needs the `godot.api.Node` /
// `godot.core.Packed*Array` *types*, not entry scanning, .gdj generation, or
// any of the app-packaging machinery. It never references any
// project-specific generated type (see this module's README), which is what
// keeps it publishable/reusable across projects.

plugins {
  kotlin("jvm")
  `java-library`
}

group = "io.github.godot-rosetta-rpc"

version = "0.1.0"

repositories { mavenCentral() }

kotlin { jvmToolchain(17) }

dependencies {
  // Host application can supply its own Godot API / Godot Core.
  compileOnly(libs.godot.api.library.debug)
  compileOnly(libs.godot.core.library.debug)
  // `api`, since this module exposes `com.google.protobuf.Message/Parser` in
  // its public API (RpcClient, RpcMethodDescriptor).
  api(libs.protobuf.java)
}
