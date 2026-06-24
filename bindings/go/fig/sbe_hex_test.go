//go:build conformance

package fig

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

type conformanceVector struct {
	ID           string `json:"id"`
	Category     string `json:"category"`
	MessageType  string `json:"message_type"`
	ExpectedHex  string `json:"expected_hex"`
}

type conformanceSuite struct {
	Vectors []conformanceVector `json:"vectors"`
}

func TestSbeHexViaFfi(t *testing.T) {
	root := filepath.Clean(filepath.Join("..", "..", ".."))
	data, err := os.ReadFile(filepath.Join(root, "tests/conformance/vectors/v1.json"))
	if err != nil {
		t.Fatalf("read vectors: %v", err)
	}
	var suite conformanceSuite
	if err := json.Unmarshal(data, &suite); err != nil {
		t.Fatalf("parse vectors: %v", err)
	}
	for _, v := range suite.Vectors {
		if v.Category != "sbe" || v.MessageType != "NewOrderSingle" {
			continue
		}
		buf, err := SbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100, 50.25)
		if err != nil {
			t.Fatalf("%s encode: %v", v.ID, err)
		}
		got := hex.EncodeToString(buf.Bytes())
		if got != v.ExpectedHex {
			t.Fatalf("%s hex mismatch\nexpected: %s\ngot:      %s", v.ID, v.ExpectedHex, got)
		}
	}
}
