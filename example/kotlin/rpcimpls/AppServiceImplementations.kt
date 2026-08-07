package rpcimpls

import example.app.BrokenKotlinImpl
import example.app.GameServiceImpl
import rosetta.example.BrokenKotlin
import rosetta.example.GameService
import rosetta.example.GeneratedServiceFactory
import rosetta.example.ServiceImplementations

/**
 * The one hand-written seam the generated Bootstrap expects: an implementation of
 * ServiceImplementations for every service this language implements (GameService and BrokenKotlin).
 * Must live in package `rpcimpls` as `object AppServiceImplementations`, so that Bootstrap.kt can
 * reference it without knowing this project's own package layout.
 */
object AppServiceImplementations : ServiceImplementations {
  override fun gameService(factory: GeneratedServiceFactory): GameService = GameServiceImpl(factory)

  override fun brokenKotlin(factory: GeneratedServiceFactory): BrokenKotlin = BrokenKotlinImpl()
}
