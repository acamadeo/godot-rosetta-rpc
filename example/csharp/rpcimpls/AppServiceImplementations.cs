using Example.App;
using Rosetta.Example;

namespace RpcImpls;

/// <summary>
/// The one hand-written seam the generated Bootstrap expects: an
/// implementation of ServiceImplementations for every service this
/// language implements (Achievements and BrokenCSharp). Must live in
/// namespace `RpcImpls` as `AppServiceImplementations`, so that
/// Bootstrap.cs can reference it without knowing this project's own
/// namespace layout.
/// </summary>
public class AppServiceImplementations : ServiceImplementations
{
    public IAchievements? Achievements(GeneratedServiceFactory factory) => new AchievementsServiceImpl(factory);

    public IBrokenCSharp? BrokenCSharp(GeneratedServiceFactory factory) => new BrokenCSharpServiceImpl();
}
