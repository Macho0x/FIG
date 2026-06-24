using System.Text.Json;
using Fig;

var bytes = FigNative.SbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100, 50.25);
if (bytes.Length == 0)
{
    throw new InvalidOperationException("empty SBE payload");
}

using var funding = new FundingState(4);

var root = Environment.GetEnvironmentVariable("FIG_REPO_ROOT")
    ?? Path.GetFullPath(Path.Combine(Directory.GetCurrentDirectory(), "..", "..", ".."));
var jsonPath = Path.Combine(root, "tests", "conformance", "vectors", "v1.json");
using var doc = JsonDocument.Parse(File.ReadAllText(jsonPath));
var got = Convert.ToHexString(bytes).ToLowerInvariant();

foreach (var v in doc.RootElement.GetProperty("vectors").EnumerateArray())
{
    if (v.GetProperty("category").GetString() != "sbe") continue;
    if (v.GetProperty("message_type").GetString() != "NewOrderSingle") continue;
    var expected = v.GetProperty("expected_hex").GetString()!;
    if (got != expected)
    {
        throw new InvalidOperationException($"SBE hex mismatch expected={expected} got={got}");
    }
}

Console.WriteLine($"fig-csharp smoke OK ({bytes.Length} bytes, SBE hex verified)");
