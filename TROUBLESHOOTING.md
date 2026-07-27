# Troubleshooting

## Godot Kotlin integration

- **The `com.utopia-rise.godot-kotlin-jvm` plugin's
  `entryGenerationIndexExistingRegistrationFiles` task doesn't declare a
  dependency on the compiled output of Kotlin library project dependencies it
  reads (like `:kotlin-runtime`)**, which Gradle 8.10.2 flags as an
  implicit-dependency validation error and fails the build on. Worked around
  in the consuming project with:

  ```kotlin
  tasks.matching { it.name == "entryGenerationIndexExistingRegistrationFiles" }.configureEach {
    dependsOn(":kotlin-runtime:compileKotlin")
  }
  ```
