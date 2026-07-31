// The Kotlin runtime for the godot-rosetta-rpc cross-language
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
  // When built standalone, the version comes from this module's own
  // `settings.gradle.kts`.
  kotlin("jvm")
  `java-library`
  id("com.vanniktech.maven.publish") version "0.37.0"
}

group = "io.github.acamadeo"

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

mavenPublishing {
  publishToMavenCentral()
  signAllPublications()

  coordinates(group.toString(), "godot-rosetta-rpc", version.toString())

  pom {
    name.set("godot-rosetta-rpc")
    description.set(
      "Kotlin runtime (RpcMethodDescriptor, ServiceRegistry, RpcClient) for " +
        "the godot-rosetta-rpc cross-language RPC framework."
    )
    url.set("https://github.com/acamadeo/godot-rosetta-rpc")
    licenses {
      license {
        name.set("MIT")
        url.set("https://github.com/acamadeo/godot-rosetta-rpc/blob/main/LICENSE")
      }
    }
    developers {
      developer { id.set("acamadeo") }
    }
    scm { url.set("https://github.com/acamadeo/godot-rosetta-rpc") }
  }
}
