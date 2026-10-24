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

func (s *AggTradesState) Len() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_agg_trades_len(s.handle))
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

func applyBytes(handle unsafe.Pointer, payload []byte, call func(unsafe.Pointer, *C.uint8_t, C.uintptr_t) C.int32_t) error {
	if handle == nil || len(payload) == 0 {
		return ErrClosed
	}
	rc := call(handle, (*C.uint8_t)(unsafe.Pointer(&payload[0])), C.uintptr_t(len(payload)))
	if rc != 0 {
		return ErrApplyFailed
	}
	return nil
}

// OrderBookState merges CBOR OrderBookSnapshot and OrderBookDelta payloads.
type OrderBookState struct {
	handle *C.struct_FigOrderBookHandle
}

func NewOrderBookState() *OrderBookState {
	return &OrderBookState{handle: C.fig_order_book_new()}
}

func (s *OrderBookState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_order_book_free(s.handle)
	s.handle = nil
}

func (s *OrderBookState) ApplySnapshot(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_order_book_apply_snapshot((*C.struct_FigOrderBookHandle)(h), p, n)
	})
}

func (s *OrderBookState) ApplyDelta(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_order_book_apply_delta((*C.struct_FigOrderBookHandle)(h), p, n)
	})
}

func (s *OrderBookState) BestBid() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_order_book_best_bid(s.handle))
}

// MidsState merges CBOR MiniTicker and AllMidsBatch payloads.
type MidsState struct {
	handle *C.struct_FigMidsHandle
}

func NewMidsState() *MidsState {
	return &MidsState{handle: C.fig_mids_new()}
}

func (s *MidsState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_mids_free(s.handle)
	s.handle = nil
}

func (s *MidsState) ApplyTicker(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_mids_apply_ticker((*C.struct_FigMidsHandle)(h), p, n)
	})
}

func (s *MidsState) ApplyBatch(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_mids_apply_batch((*C.struct_FigMidsHandle)(h), p, n)
	})
}

func (s *MidsState) Mid(symbol string) float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	cSymbol := C.CString(symbol)
	defer C.free(unsafe.Pointer(cSymbol))
	return float64(C.fig_mids_mid(s.handle, cSymbol))
}

// BboState merges CBOR BestBidOffer payloads.
type BboState struct {
	handle *C.struct_FigBboHandle
}

func NewBboState() *BboState {
	return &BboState{handle: C.fig_bbo_new()}
}

func (s *BboState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_bbo_free(s.handle)
	s.handle = nil
}

func (s *BboState) Apply(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_bbo_apply((*C.struct_FigBboHandle)(h), p, n)
	})
}

func (s *BboState) ImpliedMid() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_bbo_implied_mid(s.handle))
}

func (s *BboState) BestBid() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_bbo_best_bid(s.handle))
}

func (s *BboState) BestAsk() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_bbo_best_ask(s.handle))
}

// TradeTape merges CBOR PublicTradeEvent payloads.
type TradeTape struct {
	handle *C.struct_FigTradeTapeHandle
}

func NewTradeTape(capacity uintptr) *TradeTape {
	return &TradeTape{handle: C.fig_trade_tape_new(C.uintptr_t(capacity))}
}

func (s *TradeTape) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_trade_tape_free(s.handle)
	s.handle = nil
}

func (s *TradeTape) Push(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_trade_tape_push((*C.struct_FigTradeTapeHandle)(h), p, n)
	})
}

func (s *TradeTape) Len() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_trade_tape_len(s.handle))
}

func (s *TradeTape) LatestPrice() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_trade_tape_latest_price(s.handle))
}

// MarkPriceState merges CBOR MarkPriceUpdate payloads.
type MarkPriceState struct {
	handle *C.struct_FigMarkPriceHandle
}

func NewMarkPriceState() *MarkPriceState {
	return &MarkPriceState{handle: C.fig_mark_price_new()}
}

func (s *MarkPriceState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_mark_price_free(s.handle)
	s.handle = nil
}

func (s *MarkPriceState) Apply(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_mark_price_apply((*C.struct_FigMarkPriceHandle)(h), p, n)
	})
}

func (s *MarkPriceState) Get(symbol string) float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	cSymbol := C.CString(symbol)
	defer C.free(unsafe.Pointer(cSymbol))
	return float64(C.fig_mark_price_get(s.handle, cSymbol))
}

// OrdersState merges CBOR OpenOrdersSnapshot and ExecutionReport payloads.
type OrdersState struct {
	handle *C.struct_FigOrdersHandle
}

func NewOrdersState(execCapacity uintptr) *OrdersState {
	return &OrdersState{handle: C.fig_orders_new(C.uintptr_t(execCapacity))}
}

func (s *OrdersState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_orders_free(s.handle)
	s.handle = nil
}

func (s *OrdersState) ApplySnapshot(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_orders_apply_snapshot((*C.struct_FigOrdersHandle)(h), p, n)
	})
}

func (s *OrdersState) ApplyExecution(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_orders_apply_execution((*C.struct_FigOrdersHandle)(h), p, n)
	})
}

func (s *OrdersState) OpenCount() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_orders_open_count(s.handle))
}

func (s *OrdersState) ExecutionCount() int {
	if s == nil || s.handle == nil {
		return 0
	}
	return int(C.fig_orders_execution_count(s.handle))
}

// CandleState merges CBOR CandleBarEvent payloads.
type CandleState struct {
	handle *C.struct_FigCandleHandle
}

func NewCandleState() *CandleState {
	return &CandleState{handle: C.fig_candle_new()}
}

func (s *CandleState) Close() {
	if s == nil || s.handle == nil {
		return
	}
	C.fig_candle_free(s.handle)
	s.handle = nil
}

func (s *CandleState) Apply(payload []byte) error {
	if s == nil {
		return ErrClosed
	}
	return applyBytes(unsafe.Pointer(s.handle), payload, func(h unsafe.Pointer, p *C.uint8_t, n C.uintptr_t) C.int32_t {
		return C.fig_candle_apply((*C.struct_FigCandleHandle)(h), p, n)
	})
}

func (s *CandleState) ClosePrice() float64 {
	if s == nil || s.handle == nil {
		return 0
	}
	return float64(C.fig_candle_close(s.handle))
}

func (s *CandleState) IsFinal() int {
	if s == nil || s.handle == nil {
		return -1
	}
	return int(C.fig_candle_is_final(s.handle))
}

func (s *CandleState) Symbol() string {
	if s == nil || s.handle == nil {
		return ""
	}
	var out *C.char
	if rc := C.fig_candle_symbol(s.handle, &out); rc != 0 {
		return ""
	}
	return takeCString(out)
}

func (s *CandleState) Interval() string {
	if s == nil || s.handle == nil {
		return ""
	}
	var out *C.char
	if rc := C.fig_candle_interval(s.handle, &out); rc != 0 {
		return ""
	}
	return takeCString(out)
}

var (
	ErrClosed      = errors.New("fig merge state closed")
	ErrApplyFailed = errors.New("fig merge apply failed")
)
