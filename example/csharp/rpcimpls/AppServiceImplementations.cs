using Example.App;
using Rosetta.Example;

namespace RpcImpls;

/// <summary>
/// The one hand-written seam the generated Bootstrap expects: an
/// implementation of ServiceImplementations for every service this
/// language implements (just Achievements — Clock is implemented in Rust,
/// see ../../rust/). Must live in namespace `RpcImpls` as
/// `AppServiceImplementations` — that fixed, well-known location is what
/// lets the generated Bootstrap.cs reference it without knowing this
/// project's own namespace layout (mirrors the Rust generator's
/// `crate::rpc_impls` / Kotlin's `rpcimpls` convention).
/// </summary>
public class AppServiceImplementations : ServiceImplementations
{
    public IAchievements? Achievements(GeneratedServiceFactory factory) => new AchievementsServiceImpl(factory);
}
