//go:build conformance

package fig

import "testing"

func TestVersionLinked(t *testing.T) {
	if Version() == "" {
		t.Fatal("expected non-empty fig version from FFI")
	}
}
