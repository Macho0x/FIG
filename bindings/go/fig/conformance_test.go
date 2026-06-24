//go:build conformance

package fig

import (
	"math"
	"testing"
)

func TestVersionLinked(t *testing.T) {
	if Version() == "" {
		t.Fatal("expected non-empty fig version from FFI")
	}
}

func TestSbeNewOrderSingleCompiles(t *testing.T) {
	_, err := SbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100, 50.25)
	if err != nil {
		t.Fatalf("SbeEncodeNewOrderSingle: %v", err)
	}
}

func TestMergeStateConstructors(t *testing.T) {
	funding := NewFundingState(8)
	defer funding.Close()
	agg := NewAggTradesState(8)
	defer agg.Close()
	if funding.Len() != 0 || !math.IsNaN(agg.LatestPrice()) {
		t.Fatal("expected empty initial merge state")
	}
}
