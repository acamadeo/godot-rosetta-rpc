# godot-rosetta-rpc (Kotlin runtime)

The handwritten Kotlin runtime for the [godot-rosetta-rpc](../README.md) cross-language
RPC framework: `RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient` (Godot-free, unit
testable with plain JUnit/`kotlinc`), plus a `GodotSupport` object of thin gdext-facing
helper functions. Never depends on any project-specific generated code — see the
parent README's "Why no dependency cycle" section.

Built as a godot-kotlin-jvm library module (`godot { isLibrary.set(true) }`) rather
than a full application project, so it can be included as a dependency by any
godot-kotlin-jvm project (see `../example/settings.gradle.kts` for how the example
project includes it).

Any consuming project's `build.gradle.kts` must set
`registrationFilesDirectory.set(projectDir.resolve("kotlin/gdj"))` (see
`../example/build.gradle.kts`) — `../godot/RpcRuntime.gd` finds `KotlinRuntime` by
scanning `res://kotlin/gdj` for its generated `.gdj` file at startup, since
godot-kotlin-jvm classes aren't registered in `ClassDB` the way gdext's are.
