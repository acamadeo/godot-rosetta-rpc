# godot-rosetta-rpc (Kotlin runtime)

The Kotlin runtime for the [godot-rosetta-rpc](../README.md) cross-language
RPC framework: `RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient`, plus a
`GodotSupport` object to plug in the rest of the library with Godot. This
library never depends on any project-specific generated code.

## Installing

```kotlin
dependencies {
  implementation("io.github.acamadeo:godot-rosetta-rpc:0.1.0")
}
```

### Project setup

Your project's `build.gradle.kts` also needs:

- **godot-kotlin-jvm already applied.** This library depends on
  `godot-api-library`/`godot-core-library` as `compileOnly`, so the host
  project must supply matching jars itself via the
  `com.utopia-rise.godot-kotlin-jvm` plugin. It isn't usable as a standalone
  JVM library.

- **Generated Java sources on the classpath.** `protoc` emits plain Java
  message classes — only the DSL builders (e.g. `pingRequest { ... }`) are
  Kotlin. Add `protobuf-java` and `protobuf-kotlin` as dependencies, and add
  `protoc`'s Java output directory as a Java source set:

  ```kotlin
  sourceSets { main { java.srcDir("kotlin/generated/java") } }
  ```

- **`registrationFilesDirectory` pointed at `kotlin/gdj`.** `RpcRuntime.gd`
  finds the Kotlin runtime by scanning `res://kotlin/gdj` for a generated
  `KotlinRuntime.gdj` file, so the `godot { }` block must write there:

  ```kotlin
  godot {
    registrationFilesDirectory.set(projectDir.resolve("kotlin/gdj"))
  }
  ```

See [example/build.gradle.kts](../example/build.gradle.kts) for a working
configuration.

## Implementing an RPC service

Each `service Foo {}` in your `.proto` file generates a Kotlin `interface Foo`
with one method per RPC. Implement it anywhere in your codebase — the
generated interface doesn't assume any particular package or location:

```kotlin
class BuzzerImpl(private val factory: GeneratedServiceFactory) : Buzzer {
  override fun ping(request: PingRequest): PingResponse {
    val clock = factory.clock()
    val currentTime = clock.currentTime(currentTimeRequest {})
    return pingResponse { message = "pong: ${request.message} @ ${currentTime.millis}ms" }
  }
}
```

A method reports failure by throwing an exception (of any type). The exception will be propagated as an `RpcException`
with `RpcErrorCode.APPLICATION` (see [Error handling](#error-handling)).

## Register an RPC service

Every service implemented in Kotlin must be linked through a single
`object AppServiceImplementations` in package `rpcimpls`, implementing the
generated `ServiceImplementations` interface. That fixed, well-known location
is what lets the generated `Bootstrap.kt` find your implementations without
depending on your project's own package layout:

```kotlin
package rpcimpls

object AppServiceImplementations : ServiceImplementations {
  override fun buzzer(factory: GeneratedServiceFactory): Buzzer = BuzzerImpl(factory)
}
```

Only override the services implemented in Kotlin — every other method defaults
to returning `null`, meaning "not implemented in this language."

## Call an RPC service

From any Godot `Node`, build an `RpcClient` with
`GodotSupport.makeRpcClient(this)`, wrap it in the generated
`GeneratedServiceFactory`, then call through the factory:

```kotlin
@RegisterClass
class MyNode : Node2D() {
  @RegisterFunction
  override fun _ready() {
    val services = GeneratedServiceFactory(GodotSupport.makeRpcClient(this))

    // Without error handling.
    val response = services.buzzer().ping(pingRequest { message = "hello" })
    GD.print(response.message)

    // With error handling.
    try {
      val response = services.buzzer().ping(pingRequest { message = "hello" })
      GD.print(response.message)
    } catch (e: RpcException) {
      GD.printErr("Ping failed (${e.code}): ${e.message}")
    }
  }
}
```

To call one service from another's implementation, reuse the
`GeneratedServiceFactory` passed into your constructor (see above) instead of
building a new one.

## Error handling

`RpcClient.call` throws `RpcException` when a call fails. `RpcException.code`
is an `RpcErrorCode`:

- `UNKNOWN_SERVICE` / `UNKNOWN_METHOD` — no service/method is registered for
  the id the client called with.
- `DECODE` — the request or response protobuf failed to decode.
- `APPLICATION` — the service implementation itself failed: it threw, or an
  otherwise-uncaught exception was caught on its behalf by
  `GodotSupport.dispatchBytes`.
