namespace RosettaRpc;

/// <summary>
/// A generated adapter: decodes a request, invokes a concrete service
/// implementation, encodes the response.
/// </summary>
public interface IAdapter
{
    byte[] Invoke(string methodId, byte[] requestBytes);
}

/// <summary>
/// Holds every locally-implemented service's adapter, keyed by service id,
/// and dispatches incoming (serviceId, methodId, bytes) calls to the right
/// one. Registered by a project's generated <c>Bootstrap.Register(...)</c>.
/// </summary>
public sealed class ServiceRegistry
{
    private readonly Dictionary<string, IAdapter> _services = new();

    public void Register(string serviceId, IAdapter adapter) => _services[serviceId] = adapter;

    public byte[] Dispatch(string serviceId, string methodId, byte[] requestBytes)
    {
        if (!_services.TryGetValue(serviceId, out var adapter))
        {
            throw new RpcException(RpcErrorCode.UnknownService, $"no service registered for service id: {serviceId}");
        }
        return adapter.Invoke(methodId, requestBytes);
    }

    public IReadOnlyList<string> RegisteredServiceIds() => _services.Keys.ToList();
}
