using Rosetta.Example;

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
}
