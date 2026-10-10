package figsbe

// Parity tests: the generated Go SBE code must produce exactly the bytes of the
// shared conformance vectors, which are produced by the Rust/FFI encoder.

import (
	"encoding/hex"
	"testing"
)

func TestNewOrderSingleMatchesConformanceVector(t *testing.T) {
	const want = "00010001000000260008434f4e462d30303101405900000000000040492000000000000100000000000000000000044141504c020100000000000000000000000000000000000000"
	price := 50.25
	got, err := NewOrderSingleEncoder{}.Encode(
		"CONF-001", NewOrderSingleSideBuy, 100.0, &price, nil, "AAPL",
		NewOrderSingleOrderTypeLimit, NewOrderSingleTimeInForceDay,
		nil, nil, nil, nil, nil, nil, nil, nil,
	)
	if err != nil {
		t.Fatalf("encode: %v", err)
	}
	if hex.EncodeToString(got) != want {
		t.Fatalf("hex mismatch\n got %s\nwant %s", hex.EncodeToString(got), want)
	}
}

func TestOrderBookSnapshotMatchesConformanceVector(t *testing.T) {
	const want = "000100090000001100044141504c000353494d00000001000000154059000000000000401400000000000000000001010000000017979cfe362a0000000000000000002a010101"
	orderCount := uint32(1)
	seq := uint64(42)
	snap := uint8(1)
	got, err := OrderBookSnapshotEncoder{}.Encode(
		"AAPL", "SIM",
		[]PriceLevel{{Price: 100.0, Qty: 5.0, OrderCount: &orderCount}},
		nil,
		1700000000000000000,
		&seq,
		&snap,
	)
	if err != nil {
		t.Fatalf("encode: %v", err)
	}
	if hex.EncodeToString(got) != want {
		t.Fatalf("hex mismatch\n got %s\nwant %s", hex.EncodeToString(got), want)
	}
}

func TestRoundTrips(t *testing.T) {
	price := 12.5
	raw, err := NewOrderSingleEncoder{}.Encode(
		"RT-001", NewOrderSingleSideBuy, 7, &price, nil, "MSFT",
		NewOrderSingleOrderTypeLimit, NewOrderSingleTimeInForceDay,
		nil, nil, nil, nil, nil, nil, nil, nil,
	)
	if err != nil {
		t.Fatalf("encode: %v", err)
	}
	got, consumed, err := NewOrderSingleDecoderDecode(raw)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}
	if consumed != len(raw) {
		t.Fatalf("consumed %d of %d bytes", consumed, len(raw))
	}
	if got.ClOrdId != "RT-001" || got.Symbol != "MSFT" || got.OrderQty != 7 {
		t.Fatalf("round trip mismatch: %+v", got)
	}
	if got.Price == nil || *got.Price != price {
		t.Fatalf("price round trip mismatch: %v", got.Price)
	}

	oc := uint32(3)
	seq := uint64(9)
	snapRaw, err := OrderBookSnapshotEncoder{}.Encode(
		"AAPL", "SIM",
		[]PriceLevel{{Price: 1.5, Qty: 2.5, OrderCount: &oc}},
		[]PriceLevel{{Price: 3.5, Qty: 4.5}},
		1234567890, &seq, nil,
	)
	if err != nil {
		t.Fatalf("snapshot encode: %v", err)
	}
	snapGot, snapConsumed, err := OrderBookSnapshotDecoderDecode(snapRaw)
	if err != nil {
		t.Fatalf("snapshot decode: %v", err)
	}
	if snapConsumed != len(snapRaw) {
		t.Fatalf("snapshot consumed %d of %d", snapConsumed, len(snapRaw))
	}
	if len(snapGot.Bids) != 1 || snapGot.Bids[0].Qty != 2.5 || snapGot.Bids[0].OrderCount == nil || *snapGot.Bids[0].OrderCount != 3 {
		t.Fatalf("bids round trip mismatch: %+v", snapGot.Bids)
	}
	if len(snapGot.Asks) != 1 || snapGot.Asks[0].Price != 3.5 || snapGot.Asks[0].OrderCount != nil {
		t.Fatalf("asks round trip mismatch: %+v", snapGot.Asks)
	}
	if snapGot.Sequence == nil || *snapGot.Sequence != 9 || snapGot.IsSnapshot != nil {
		t.Fatalf("optionals round trip mismatch: %+v", snapGot)
	}
}

func TestCapabilitiesResponseMatchesConformanceVector(t *testing.T) {
	// Exercises the list-of-scalar layout: list<uint8>, list<string>, list<enum>.
	const want = "000100350000000000000003010203000000010000001e001a6d61726b6574646174612f4141504c2f63616e646c65732f356d010000000001000000044141504c0000000100000002356d"
	got, err := CapabilitiesResponseEncoder{}.Encode(
		[]uint8{1, 2, 3},
		[]CapabilityPath{{Path: "marketdata/AAPL/candles/5m", Pattern: CapabilityPathPatternPubSub, AuthRequired: 0}},
		[]string{"AAPL"},
		[]string{"5m"},
	)
	if err != nil {
		t.Fatalf("encode: %v", err)
	}
	if hex.EncodeToString(got) != want {
		t.Fatalf("hex mismatch\n got %s\nwant %s", hex.EncodeToString(got), want)
	}
}
