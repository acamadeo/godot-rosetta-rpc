using System;
using Rosetta.Example;
using RosettaRpc;

namespace Example.App;

/// <summary>
/// Achievements is implemented in C# and calls Clock — implemented in
/// Rust — purely through the generated ClockClient, with no idea Clock is
/// implemented in a different language.
/// </summary>
public class AchievementsServiceImpl : IAchievements
{
    private readonly GeneratedServiceFactory _factory;

    public AchievementsServiceImpl(GeneratedServiceFactory factory) => _factory = factory;

    public UnlockAchievementResponse UnlockAchievement(UnlockAchievementRequest request)
    {
        var clock = _factory.Clock();
        var currentTime = clock.CurrentTime(new CurrentTimeRequest());
        return new UnlockAchievementResponse
        {
            Message = $"unlocked: {request.AchievementId} @ {currentTime.Millis}ms",
        };
    }

    /// <summary>
    /// Calls the BrokenXXX service named by `request.Target` and reports the
    /// RpcException it caught.
    /// 
    /// Note that C# can only call Rust across language boundaries.
    /// </summary>
    public ProbeBrokenServiceResult ProbeBrokenService(ProbeBrokenServiceRequest request)
    {
        try
        {
            switch (request.Target)
            {
                case "BrokenRust":
                    _factory.BrokenRust().Fail(new FailRequest());
                    break;
                default:
                    throw new ArgumentException($"incompatible probe target: {request.Target}");
            }
            return new ProbeBrokenServiceResult { ErrorMessage = "" };
        }
        catch (RpcException e)
        {
            return new ProbeBrokenServiceResult { ErrorMessage = e.Message };
        }
    }
}
