package fig

/*
#include "../../../crates/fig-ffi/include/fig.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"unsafe"
)

// FundingState merges CBOR FundingPayment stream payloads.
type FundingState struct {
	handle *C.struct_FigFundingHandle
}

func NewFundingState(capacity uintptr) *FundingState {
	return &FundingState{handle: C.fig_funding_new(C.uintptr_t(capacity))}
}

func (s *FundingState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_funding_free(s.handle)
	s.handle = nil
}

func (s *FundingState) Apply(payload []byte) error {
	if s == nil || s.handle == nil || len(payload) == 0 {
		return ErrClosed
	}
	rc := C.fig_funding_apply(s.handle, (*C.uint8_t)(unsafe.Pointer(&payload[0])), C.uintptr_t(len(payload)))
	if rc != 0 {
		return ErrApplyFailed
	}
	return nil
}

func (s *FundingState) Len() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_funding_len(s.handle))
}

func (s *FundingState) LatestAmount() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_funding_latest_amount(s.handle))
}

// AggTradesState merges CBOR AggregateTradeEvent payloads.
type AggTradesState struct {
	handle *C.struct_FigAggTradesHandle
}

func NewAggTradesState(capacity uintptr) *AggTradesState {
	return &AggTradesState{handle: C.fig_agg_trades_new(C.uintptr_t(capacity))}
}

func (s *AggTradesState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_agg_trades_free(s.handle)
	s.handle = nil
}

func (s *AggTradesState) Apply(payload []byte) error {
	if s == nil || s.handle == nil || len(payload) == 0 {
		return ErrClosed
	}
	rc := C.fig_agg_trades_apply(s.handle, (*C.uint8_t)(unsafe.Pointer(&payload[0])), C.uintptr_t(len(payload)))
	if rc != 0 {
		return ErrApplyFailed
	}
	return nil
}

func (s *AggTradesState) LatestPrice() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_agg_trades_latest_price(s.handle))
}

// LedgerState merges CBOR LedgerUpdate stream payloads.
type LedgerState struct {
	handle *C.struct_FigLedgerHandle
}

func NewLedgerState(capacity uintptr) *LedgerState {
	return &LedgerState{handle: C.fig_ledger_new(C.uintptr_t(capacity))}
}

func (s *LedgerState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_ledger_free(s.handle)
	s.handle = nil
}

func (s *LedgerState) Apply(payload []byte) error {
	if s == nil || s.handle == nil || len(payload) == 0 {
		return ErrClosed
	}
	rc := C.fig_ledger_apply(s.handle, (*C.uint8_t)(unsafe.Pointer(&payload[0])), C.uintptr_t(len(payload)))
	if rc != 0 {
		return ErrApplyFailed
	}
	return nil
}

func (s *LedgerState) Len() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_ledger_len(s.handle))
}

// LiquidationState merges CBOR UserLiquidation and LiquidationTradeEvent payloads.
type LiquidationState struct {
	handle *C.struct_FigLiquidationHandle
}

func NewLiquidationState(capacity uintptr) *LiquidationState {
	return &LiquidationState{handle: C.fig_liquidation_new(C.uintptr_t(capacity))}
}

func (s *LiquidationState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_liquidation_free(s.handle)
	s.handle = nil
}

func (s *LiquidationState) ApplyUser(payload []byte) error {
	if s == nil || s.handle == nil || len(payload) == 0 {
		return ErrClosed
	}
	rc := C.fig_liquidation_apply_user(s.handle, (*C.uint8_t)(unsafe.Pointer(&payload[0])), C.uintptr_t(len(payload)))
	if rc != 0 {
		return ErrApplyFailed
	}
	return nil
}

func (s *LiquidationState) ApplyPublic(payload []byte) error {
	if s == nil || s.handle == nil || len(payload) == 0 {
		return ErrClosed
	}
	rc := C.fig_liquidation_apply_public(s.handle, (*C.uint8_t)(unsafe.Pointer(&payload[0])), C.uintptr_t(len(payload)))
	if rc != 0 {
		return ErrApplyFailed
	}
	return nil
}

func (s *LiquidationState) UserCount() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_liquidation_user_count(s.handle))
}

var (
	ErrClosed      = errors.New("fig merge state closed")
	ErrApplyFailed = errors.New("fig merge apply failed")
)
