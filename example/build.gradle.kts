plugins { alias(libs.plugins.godot.kotlin.jvm) }

repositories { mavenCentral() }

sourceSets { main { java.srcDir("kotlin/generated/java") } }

kotlin {
  jvmToolchain(17)
  sourceSets {
    main { kotlin.srcDirs("kotlin", "kotlin/generated/kotlin", "kotlin/generated/rpc") }
  }
}

godot {
  registrationFilesDirectory.set(projectDir.resolve("kotlin/gdj"))
  isGodotCoroutinesEnabled.set(true)
}

dependencies {
  implementation(project(":kotlin-runtime"))
  implementation(libs.protobuf.java)
  implementation(libs.protobuf.kotlin)
}
