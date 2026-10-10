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

// Per-language SBE parity: encode with the *generated* C# code and compare against
// the shared conformance vectors (produced by the Rust/FFI encoder).
static string Hex(byte[] b) => Convert.ToHexString(b).ToLowerInvariant();
static string Want(JsonDocument doc, string id) =>
    doc.RootElement.GetProperty("vectors").EnumerateArray()
        .First(v => v.GetProperty("id").GetString() == id)
        .GetProperty("expected_hex").GetString()!;

var genOrder = Fig.Sbe.NewOrderSingleEncoder.Encode(
    "CONF-001", Fig.Sbe.NewOrderSingleSide.Buy, 100.0, 50.25, null, "AAPL",
    Fig.Sbe.NewOrderSingleOrderType.Limit, Fig.Sbe.NewOrderSingleTimeInForce.Day,
    null, null, null, null, null, null, null, null);
if (Hex(genOrder) != Want(doc, "sbe.new_order_single.limit_buy"))
{
    throw new InvalidOperationException($"generated C# new_order_single mismatch: {Hex(genOrder)}");
}

var genSnapshot = Fig.Sbe.OrderBookSnapshotEncoder.Encode(
    "AAPL", "SIM",
    new List<Fig.Sbe.PriceLevel> { new Fig.Sbe.PriceLevel { Price = 100.0, Qty = 5.0, OrderCount = 1 } },
    new List<Fig.Sbe.PriceLevel>(),
    1700000000000000000L, 42UL, 1);
if (Hex(genSnapshot) != Want(doc, "sbe.order_book_snapshot.demo"))
{
    throw new InvalidOperationException($"generated C# order_book_snapshot mismatch: {Hex(genSnapshot)}");
}

Console.WriteLine("fig-csharp generated SBE parity OK");
