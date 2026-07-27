package rpcimpls

import example.app.GameServiceImpl
import rosetta.example.GameService
import rosetta.example.GeneratedServiceFactory
import rosetta.example.ServiceImplementations

/**
 * The one hand-written seam the generated Bootstrap expects: an implementation of
 * ServiceImplementations for every service this language implements (just GameService — Clock is
 * implemented in Rust, see ../../rust/). Must live in package `rpcimpls` as `object
 * AppServiceImplementations` — that fixed, well-known location is what lets the generated
 * Bootstrap.kt reference it without knowing this project's own package layout (mirrors the Rust
 * generator's `crate::rpc_impls` convention).
 */
object AppServiceImplementations : ServiceImplementations {
  override fun gameService(factory: GeneratedServiceFactory): GameService = GameServiceImpl(factory)
}
