//go:build !conformance

// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

package figsbe

import (
	"fmt"
	"math"
)

const SCHEMA_ID uint16 = 0x01

func writeU16BE(buf *[]byte, v uint16) {
	*buf = append(*buf, byte(v>>8), byte(v))
}

func writeU32BE(buf *[]byte, v uint32) {
	*buf = append(*buf, byte(v>>24), byte(v>>16), byte(v>>8), byte(v))
}

func writeU64BE(buf *[]byte, v uint64) {
	*buf = append(*buf, byte(v>>56), byte(v>>48), byte(v>>40), byte(v>>32), byte(v>>24), byte(v>>16), byte(v>>8), byte(v))
}

func writeF64BE(buf *[]byte, v float64) { writeU64BE(buf, math.Float64bits(v)) }
func writeI64BE(buf *[]byte, v int64)   { writeU64BE(buf, uint64(v)) }

func writeString(buf *[]byte, s string) {
	writeU16BE(buf, uint16(len(s)))
	*buf = append(*buf, []byte(s)...)
}

func readU16BE(buf []byte, pos *int) uint16 {
	v := uint16(buf[*pos])<<8 | uint16(buf[*pos+1])
	*pos += 2
	return v
}

func readU32BE(buf []byte, pos *int) uint32 {
	v := uint32(buf[*pos])<<24 | uint32(buf[*pos+1])<<16 | uint32(buf[*pos+2])<<8 | uint32(buf[*pos+3])
	*pos += 4
	return v
}

func readU64BE(buf []byte, pos *int) uint64 {
	v := uint64(buf[*pos])<<56 | uint64(buf[*pos+1])<<48 | uint64(buf[*pos+2])<<40 | uint64(buf[*pos+3])<<32 |
		uint64(buf[*pos+4])<<24 | uint64(buf[*pos+5])<<16 | uint64(buf[*pos+6])<<8 | uint64(buf[*pos+7])
	*pos += 8
	return v
}

func readF64BE(buf []byte, pos *int) float64 { return math.Float64frombits(readU64BE(buf, pos)) }
func readI64BE(buf []byte, pos *int) int64   { return int64(readU64BE(buf, pos)) }

func readString(buf []byte, pos *int) string {
	n := int(readU16BE(buf, pos))
	s := string(buf[*pos : *pos+n])
	*pos += n
	return s
}

type NewOrderSingleSide uint8

const (
	NewOrderSingleSideBuy             NewOrderSingleSide = 1
	NewOrderSingleSideSell            NewOrderSingleSide = 2
	NewOrderSingleSideSellShort       NewOrderSingleSide = 3
	NewOrderSingleSideSellShortExempt NewOrderSingleSide = 4
)

func NewOrderSingleSideFromValue(v uint8) (NewOrderSingleSide, error) {
	switch v {
	case 1:
		return NewOrderSingleSideBuy, nil
	case 2:
		return NewOrderSingleSideSell, nil
	case 3:
		return NewOrderSingleSideSellShort, nil
	case 4:
		return NewOrderSingleSideSellShortExempt, nil
	default:
		return 0, fmt.Errorf("invalid NewOrderSingleSide value: %d", v)
	}
}

func (e NewOrderSingleSide) ToValue() uint8 { return uint8(e) }

type NewOrderSingleOrderType uint8

const (
	NewOrderSingleOrderTypeMarket        NewOrderSingleOrderType = 1
	NewOrderSingleOrderTypeLimit         NewOrderSingleOrderType = 2
	NewOrderSingleOrderTypeStop          NewOrderSingleOrderType = 3
	NewOrderSingleOrderTypeStopLimit     NewOrderSingleOrderType = 4
	NewOrderSingleOrderTypeMarketOnClose NewOrderSingleOrderType = 5
	NewOrderSingleOrderTypeLimitOnClose  NewOrderSingleOrderType = 6
	NewOrderSingleOrderTypePegged        NewOrderSingleOrderType = 7
)

func NewOrderSingleOrderTypeFromValue(v uint8) (NewOrderSingleOrderType, error) {
	switch v {
	case 1:
		return NewOrderSingleOrderTypeMarket, nil
	case 2:
		return NewOrderSingleOrderTypeLimit, nil
	case 3:
		return NewOrderSingleOrderTypeStop, nil
	case 4:
		return NewOrderSingleOrderTypeStopLimit, nil
	case 5:
		return NewOrderSingleOrderTypeMarketOnClose, nil
	case 6:
		return NewOrderSingleOrderTypeLimitOnClose, nil
	case 7:
		return NewOrderSingleOrderTypePegged, nil
	default:
		return 0, fmt.Errorf("invalid NewOrderSingleOrderType value: %d", v)
	}
}

func (e NewOrderSingleOrderType) ToValue() uint8 { return uint8(e) }

type NewOrderSingleTimeInForce uint8

const (
	NewOrderSingleTimeInForceDay NewOrderSingleTimeInForce = 1
	NewOrderSingleTimeInForceGtc NewOrderSingleTimeInForce = 2
	NewOrderSingleTimeInForceIoc NewOrderSingleTimeInForce = 3
	NewOrderSingleTimeInForceFok NewOrderSingleTimeInForce = 4
	NewOrderSingleTimeInForceGtd NewOrderSingleTimeInForce = 5
)

func NewOrderSingleTimeInForceFromValue(v uint8) (NewOrderSingleTimeInForce, error) {
	switch v {
	case 1:
		return NewOrderSingleTimeInForceDay, nil
	case 2:
		return NewOrderSingleTimeInForceGtc, nil
	case 3:
		return NewOrderSingleTimeInForceIoc, nil
	case 4:
		return NewOrderSingleTimeInForceFok, nil
	case 5:
		return NewOrderSingleTimeInForceGtd, nil
	default:
		return 0, fmt.Errorf("invalid NewOrderSingleTimeInForce value: %d", v)
	}
}

func (e NewOrderSingleTimeInForce) ToValue() uint8 { return uint8(e) }

type NewOrderSingleIdSource uint8

const (
	NewOrderSingleIdSourceCusip          NewOrderSingleIdSource = 1
	NewOrderSingleIdSourceSedol          NewOrderSingleIdSource = 2
	NewOrderSingleIdSourceIsin           NewOrderSingleIdSource = 3
	NewOrderSingleIdSourceRic            NewOrderSingleIdSource = 4
	NewOrderSingleIdSourceExchangeSymbol NewOrderSingleIdSource = 5
)

func NewOrderSingleIdSourceFromValue(v uint8) (NewOrderSingleIdSource, error) {
	switch v {
	case 1:
		return NewOrderSingleIdSourceCusip, nil
	case 2:
		return NewOrderSingleIdSourceSedol, nil
	case 3:
		return NewOrderSingleIdSourceIsin, nil
	case 4:
		return NewOrderSingleIdSourceRic, nil
	case 5:
		return NewOrderSingleIdSourceExchangeSymbol, nil
	default:
		return 0, fmt.Errorf("invalid NewOrderSingleIdSource value: %d", v)
	}
}

func (e NewOrderSingleIdSource) ToValue() uint8 { return uint8(e) }

type CancelRequestSide uint8

const (
	CancelRequestSideBuy       CancelRequestSide = 1
	CancelRequestSideSell      CancelRequestSide = 2
	CancelRequestSideSellShort CancelRequestSide = 3
)

func CancelRequestSideFromValue(v uint8) (CancelRequestSide, error) {
	switch v {
	case 1:
		return CancelRequestSideBuy, nil
	case 2:
		return CancelRequestSideSell, nil
	case 3:
		return CancelRequestSideSellShort, nil
	default:
		return 0, fmt.Errorf("invalid CancelRequestSide value: %d", v)
	}
}

func (e CancelRequestSide) ToValue() uint8 { return uint8(e) }

type CancelReplaceRequestSide uint8

const (
	CancelReplaceRequestSideBuy       CancelReplaceRequestSide = 1
	CancelReplaceRequestSideSell      CancelReplaceRequestSide = 2
	CancelReplaceRequestSideSellShort CancelReplaceRequestSide = 3
)

func CancelReplaceRequestSideFromValue(v uint8) (CancelReplaceRequestSide, error) {
	switch v {
	case 1:
		return CancelReplaceRequestSideBuy, nil
	case 2:
		return CancelReplaceRequestSideSell, nil
	case 3:
		return CancelReplaceRequestSideSellShort, nil
	default:
		return 0, fmt.Errorf("invalid CancelReplaceRequestSide value: %d", v)
	}
}

func (e CancelReplaceRequestSide) ToValue() uint8 { return uint8(e) }

type ExecutionReportExecType uint8

const (
	ExecutionReportExecTypeNew           ExecutionReportExecType = 1
	ExecutionReportExecTypePartialFill   ExecutionReportExecType = 2
	ExecutionReportExecTypeFill          ExecutionReportExecType = 3
	ExecutionReportExecTypeDoneForDay    ExecutionReportExecType = 4
	ExecutionReportExecTypeCanceled      ExecutionReportExecType = 5
	ExecutionReportExecTypeReplaced      ExecutionReportExecType = 6
	ExecutionReportExecTypePendingCancel ExecutionReportExecType = 7
	ExecutionReportExecTypeStopped       ExecutionReportExecType = 8
	ExecutionReportExecTypeRejected      ExecutionReportExecType = 9
	ExecutionReportExecTypeSuspended     ExecutionReportExecType = 10
	ExecutionReportExecTypePendingNew    ExecutionReportExecType = 11
	ExecutionReportExecTypeExpired       ExecutionReportExecType = 12
)

func ExecutionReportExecTypeFromValue(v uint8) (ExecutionReportExecType, error) {
	switch v {
	case 1:
		return ExecutionReportExecTypeNew, nil
	case 2:
		return ExecutionReportExecTypePartialFill, nil
	case 3:
		return ExecutionReportExecTypeFill, nil
	case 4:
		return ExecutionReportExecTypeDoneForDay, nil
	case 5:
		return ExecutionReportExecTypeCanceled, nil
	case 6:
		return ExecutionReportExecTypeReplaced, nil
	case 7:
		return ExecutionReportExecTypePendingCancel, nil
	case 8:
		return ExecutionReportExecTypeStopped, nil
	case 9:
		return ExecutionReportExecTypeRejected, nil
	case 10:
		return ExecutionReportExecTypeSuspended, nil
	case 11:
		return ExecutionReportExecTypePendingNew, nil
	case 12:
		return ExecutionReportExecTypeExpired, nil
	default:
		return 0, fmt.Errorf("invalid ExecutionReportExecType value: %d", v)
	}
}

func (e ExecutionReportExecType) ToValue() uint8 { return uint8(e) }

type ExecutionReportOrdStatus uint8

const (
	ExecutionReportOrdStatusNew             ExecutionReportOrdStatus = 1
	ExecutionReportOrdStatusPartiallyFilled ExecutionReportOrdStatus = 2
	ExecutionReportOrdStatusFilled          ExecutionReportOrdStatus = 3
	ExecutionReportOrdStatusDoneForDay      ExecutionReportOrdStatus = 4
	ExecutionReportOrdStatusCanceled        ExecutionReportOrdStatus = 5
	ExecutionReportOrdStatusPendingCancel   ExecutionReportOrdStatus = 6
	ExecutionReportOrdStatusStopped         ExecutionReportOrdStatus = 7
	ExecutionReportOrdStatusRejected        ExecutionReportOrdStatus = 8
	ExecutionReportOrdStatusSuspended       ExecutionReportOrdStatus = 9
	ExecutionReportOrdStatusPendingNew      ExecutionReportOrdStatus = 10
	ExecutionReportOrdStatusExpired         ExecutionReportOrdStatus = 11
	ExecutionReportOrdStatusReplaced        ExecutionReportOrdStatus = 12
)

func ExecutionReportOrdStatusFromValue(v uint8) (ExecutionReportOrdStatus, error) {
	switch v {
	case 1:
		return ExecutionReportOrdStatusNew, nil
	case 2:
		return ExecutionReportOrdStatusPartiallyFilled, nil
	case 3:
		return ExecutionReportOrdStatusFilled, nil
	case 4:
		return ExecutionReportOrdStatusDoneForDay, nil
	case 5:
		return ExecutionReportOrdStatusCanceled, nil
	case 6:
		return ExecutionReportOrdStatusPendingCancel, nil
	case 7:
		return ExecutionReportOrdStatusStopped, nil
	case 8:
		return ExecutionReportOrdStatusRejected, nil
	case 9:
		return ExecutionReportOrdStatusSuspended, nil
	case 10:
		return ExecutionReportOrdStatusPendingNew, nil
	case 11:
		return ExecutionReportOrdStatusExpired, nil
	case 12:
		return ExecutionReportOrdStatusReplaced, nil
	default:
		return 0, fmt.Errorf("invalid ExecutionReportOrdStatus value: %d", v)
	}
}

func (e ExecutionReportOrdStatus) ToValue() uint8 { return uint8(e) }

type ExecutionReportSide uint8

const (
	ExecutionReportSideBuy       ExecutionReportSide = 1
	ExecutionReportSideSell      ExecutionReportSide = 2
	ExecutionReportSideSellShort ExecutionReportSide = 3
)

func ExecutionReportSideFromValue(v uint8) (ExecutionReportSide, error) {
	switch v {
	case 1:
		return ExecutionReportSideBuy, nil
	case 2:
		return ExecutionReportSideSell, nil
	case 3:
		return ExecutionReportSideSellShort, nil
	default:
		return 0, fmt.Errorf("invalid ExecutionReportSide value: %d", v)
	}
}

func (e ExecutionReportSide) ToValue() uint8 { return uint8(e) }

type CancelRejectRejectReason uint8

const (
	CancelRejectRejectReasonOrderNotFound   CancelRejectRejectReason = 1
	CancelRejectRejectReasonAlreadyCanceled CancelRejectRejectReason = 2
	CancelRejectRejectReasonAlreadyFilled   CancelRejectRejectReason = 3
	CancelRejectRejectReasonTooLateToCancel CancelRejectRejectReason = 4
)

func CancelRejectRejectReasonFromValue(v uint8) (CancelRejectRejectReason, error) {
	switch v {
	case 1:
		return CancelRejectRejectReasonOrderNotFound, nil
	case 2:
		return CancelRejectRejectReasonAlreadyCanceled, nil
	case 3:
		return CancelRejectRejectReasonAlreadyFilled, nil
	case 4:
		return CancelRejectRejectReasonTooLateToCancel, nil
	default:
		return 0, fmt.Errorf("invalid CancelRejectRejectReason value: %d", v)
	}
}

func (e CancelRejectRejectReason) ToValue() uint8 { return uint8(e) }

type BalanceUpdateReason uint8

const (
	BalanceUpdateReasonTrade      BalanceUpdateReason = 1
	BalanceUpdateReasonDeposit    BalanceUpdateReason = 2
	BalanceUpdateReasonWithdrawal BalanceUpdateReason = 3
	BalanceUpdateReasonTransfer   BalanceUpdateReason = 4
	BalanceUpdateReasonFee        BalanceUpdateReason = 5
)

func BalanceUpdateReasonFromValue(v uint8) (BalanceUpdateReason, error) {
	switch v {
	case 1:
		return BalanceUpdateReasonTrade, nil
	case 2:
		return BalanceUpdateReasonDeposit, nil
	case 3:
		return BalanceUpdateReasonWithdrawal, nil
	case 4:
		return BalanceUpdateReasonTransfer, nil
	case 5:
		return BalanceUpdateReasonFee, nil
	default:
		return 0, fmt.Errorf("invalid BalanceUpdateReason value: %d", v)
	}
}

func (e BalanceUpdateReason) ToValue() uint8 { return uint8(e) }

type OrderListStatusStatus uint8

const (
	OrderListStatusStatusExecuting OrderListStatusStatus = 1
	OrderListStatusStatusAllDone   OrderListStatusStatus = 2
	OrderListStatusStatusReject    OrderListStatusStatus = 3
)

func OrderListStatusStatusFromValue(v uint8) (OrderListStatusStatus, error) {
	switch v {
	case 1:
		return OrderListStatusStatusExecuting, nil
	case 2:
		return OrderListStatusStatusAllDone, nil
	case 3:
		return OrderListStatusStatusReject, nil
	default:
		return 0, fmt.Errorf("invalid OrderListStatusStatus value: %d", v)
	}
}

func (e OrderListStatusStatus) ToValue() uint8 { return uint8(e) }

type LedgerUpdateKind uint8

const (
	LedgerUpdateKindDeposit    LedgerUpdateKind = 1
	LedgerUpdateKindWithdrawal LedgerUpdateKind = 2
	LedgerUpdateKindTransfer   LedgerUpdateKind = 3
	LedgerUpdateKindFee        LedgerUpdateKind = 4
	LedgerUpdateKindFunding    LedgerUpdateKind = 5
)

func LedgerUpdateKindFromValue(v uint8) (LedgerUpdateKind, error) {
	switch v {
	case 1:
		return LedgerUpdateKindDeposit, nil
	case 2:
		return LedgerUpdateKindWithdrawal, nil
	case 3:
		return LedgerUpdateKindTransfer, nil
	case 4:
		return LedgerUpdateKindFee, nil
	case 5:
		return LedgerUpdateKindFunding, nil
	default:
		return 0, fmt.Errorf("invalid LedgerUpdateKind value: %d", v)
	}
}

func (e LedgerUpdateKind) ToValue() uint8 { return uint8(e) }

type AggregateTradeSide uint8

const (
	AggregateTradeSideBuy  AggregateTradeSide = 1
	AggregateTradeSideSell AggregateTradeSide = 2
)

func AggregateTradeSideFromValue(v uint8) (AggregateTradeSide, error) {
	switch v {
	case 1:
		return AggregateTradeSideBuy, nil
	case 2:
		return AggregateTradeSideSell, nil
	default:
		return 0, fmt.Errorf("invalid AggregateTradeSide value: %d", v)
	}
}

func (e AggregateTradeSide) ToValue() uint8 { return uint8(e) }

type LiquidationTradeSide uint8

const (
	LiquidationTradeSideBuy  LiquidationTradeSide = 1
	LiquidationTradeSideSell LiquidationTradeSide = 2
)

func LiquidationTradeSideFromValue(v uint8) (LiquidationTradeSide, error) {
	switch v {
	case 1:
		return LiquidationTradeSideBuy, nil
	case 2:
		return LiquidationTradeSideSell, nil
	default:
		return 0, fmt.Errorf("invalid LiquidationTradeSide value: %d", v)
	}
}

func (e LiquidationTradeSide) ToValue() uint8 { return uint8(e) }

type MarketDataUpdateSide uint8

const (
	MarketDataUpdateSideBuy  MarketDataUpdateSide = 1
	MarketDataUpdateSideSell MarketDataUpdateSide = 2
)

func MarketDataUpdateSideFromValue(v uint8) (MarketDataUpdateSide, error) {
	switch v {
	case 1:
		return MarketDataUpdateSideBuy, nil
	case 2:
		return MarketDataUpdateSideSell, nil
	default:
		return 0, fmt.Errorf("invalid MarketDataUpdateSide value: %d", v)
	}
}

func (e MarketDataUpdateSide) ToValue() uint8 { return uint8(e) }

type MarketDataUpdateAction uint8

const (
	MarketDataUpdateActionNew    MarketDataUpdateAction = 1
	MarketDataUpdateActionChange MarketDataUpdateAction = 2
	MarketDataUpdateActionDelete MarketDataUpdateAction = 3
)

func MarketDataUpdateActionFromValue(v uint8) (MarketDataUpdateAction, error) {
	switch v {
	case 1:
		return MarketDataUpdateActionNew, nil
	case 2:
		return MarketDataUpdateActionChange, nil
	case 3:
		return MarketDataUpdateActionDelete, nil
	default:
		return 0, fmt.Errorf("invalid MarketDataUpdateAction value: %d", v)
	}
}

func (e MarketDataUpdateAction) ToValue() uint8 { return uint8(e) }

type PublicTradeSide uint8

const (
	PublicTradeSideBuy  PublicTradeSide = 1
	PublicTradeSideSell PublicTradeSide = 2
)

func PublicTradeSideFromValue(v uint8) (PublicTradeSide, error) {
	switch v {
	case 1:
		return PublicTradeSideBuy, nil
	case 2:
		return PublicTradeSideSell, nil
	default:
		return 0, fmt.Errorf("invalid PublicTradeSide value: %d", v)
	}
}

func (e PublicTradeSide) ToValue() uint8 { return uint8(e) }

type CapabilityPathPattern uint8

const (
	CapabilityPathPatternPubSub          CapabilityPathPattern = 1
	CapabilityPathPatternRequestResponse CapabilityPathPattern = 2
	CapabilityPathPatternRequestStream   CapabilityPathPattern = 3
)

func CapabilityPathPatternFromValue(v uint8) (CapabilityPathPattern, error) {
	switch v {
	case 1:
		return CapabilityPathPatternPubSub, nil
	case 2:
		return CapabilityPathPatternRequestResponse, nil
	case 3:
		return CapabilityPathPatternRequestStream, nil
	default:
		return 0, fmt.Errorf("invalid CapabilityPathPattern value: %d", v)
	}
}

func (e CapabilityPathPattern) ToValue() uint8 { return uint8(e) }

// SBE encoder for NewOrderSingle
type NewOrderSingleEncoder struct{}

func (NewOrderSingleEncoder) Encode(ClOrdId string, Side NewOrderSingleSide, OrderQty float64, Price *float64, StopPrice *float64, Symbol string, OrderType NewOrderSingleOrderType, TimeInForce NewOrderSingleTimeInForce, ExpireTime *int64, Account *string, StrategyId *string, SecurityId *string, IdSource *NewOrderSingleIdSource, SecurityExchange *string, PostOnly *uint8, ReduceOnly *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 1)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 38)

	// Fixed fields
	writeString(&buf, ClOrdId)
	buf = append(buf, byte(Side.ToValue()))
	writeF64BE(&buf, OrderQty)
	v := 0.0
	if Price != nil {
		v = *Price
	}
	writeF64BE(&buf, v)
	if Price != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := 0.0
	if StopPrice != nil {
		v = *StopPrice
	}
	writeF64BE(&buf, v)
	if StopPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeString(&buf, Symbol)
	buf = append(buf, byte(OrderType.ToValue()))
	buf = append(buf, byte(TimeInForce.ToValue()))
	if ExpireTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *ExpireTime)
	} else {
		buf = append(buf, 0)
	}
	if Account != nil {
		buf = append(buf, 1)
		writeString(&buf, *Account)
	} else {
		buf = append(buf, 0)
	}
	if StrategyId != nil {
		buf = append(buf, 1)
		writeString(&buf, *StrategyId)
	} else {
		buf = append(buf, 0)
	}
	if SecurityId != nil {
		buf = append(buf, 1)
		writeString(&buf, *SecurityId)
	} else {
		buf = append(buf, 0)
	}
	if IdSource != nil {
		buf = append(buf, byte(IdSource.ToValue()))
	} else {
		buf = append(buf, 0)
	}
	if SecurityExchange != nil {
		buf = append(buf, 1)
		writeString(&buf, *SecurityExchange)
	} else {
		buf = append(buf, 0)
	}
	v := uint8(0)
	if PostOnly != nil {
		v = *PostOnly
	}
	buf = append(buf, v)
	if PostOnly != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := uint8(0)
	if ReduceOnly != nil {
		v = *ReduceOnly
	}
	buf = append(buf, v)
	if ReduceOnly != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for NewOrderSingle
type NewOrderSingleDecoder struct {
	ClOrdId          string
	Side             NewOrderSingleSide
	OrderQty         float64
	Price            *float64
	StopPrice        *float64
	Symbol           string
	OrderType        NewOrderSingleOrderType
	TimeInForce      NewOrderSingleTimeInForce
	ExpireTime       *int64
	Account          *string
	StrategyId       *string
	SecurityId       *string
	IdSource         *NewOrderSingleIdSource
	SecurityExchange *string
	PostOnly         *uint8
	ReduceOnly       *uint8
}

func (NewOrderSingleDecoder) EncodedLen() int { return 0 }

func NewOrderSingleDecoderDecode(buf []byte) (NewOrderSingleDecoder, error) {
	if len(buf) < 8 {
		return NewOrderSingleDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return NewOrderSingleDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 1 {
		return NewOrderSingleDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	sideRaw := buf[pos]
	pos++
	side, err := NewOrderSingleSideFromValue(sideRaw)
	if err != nil {
		return NewOrderSingleDecoder{}, err
	}
	order_qty := readF64BE(buf, &pos)
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		price = &raw
	} else {
		pos++
		price = nil
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		stop_price = &raw
	} else {
		pos++
		stop_price = nil
	}
	symbol := readString(buf, &pos)
	order_typeRaw := buf[pos]
	pos++
	order_type, err := NewOrderSingleOrderTypeFromValue(order_typeRaw)
	if err != nil {
		return NewOrderSingleDecoder{}, err
	}
	time_in_forceRaw := buf[pos]
	pos++
	time_in_force, err := NewOrderSingleTimeInForceFromValue(time_in_forceRaw)
	if err != nil {
		return NewOrderSingleDecoder{}, err
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		expire_time = &v
	} else {
		pos++
		expire_time = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		account = &s
	} else {
		pos++
		account = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		strategy_id = &s
	} else {
		pos++
		strategy_id = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		security_id = &s
	} else {
		pos++
		security_id = nil
	}
	id_sourceRaw := buf[pos]
	pos++
	id_source, err := NewOrderSingleIdSourceFromValue(id_sourceRaw)
	if err != nil {
		return NewOrderSingleDecoder{}, err
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		security_exchange = &s
	} else {
		pos++
		security_exchange = nil
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		post_only = &v
	} else {
		pos++
		post_only = nil
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		reduce_only = &v
	} else {
		pos++
		reduce_only = nil
	}

	return NewOrderSingleDecoder{
		ClOrdId:          cl_ord_id,
		Side:             side,
		OrderQty:         order_qty,
		Price:            price,
		StopPrice:        stop_price,
		Symbol:           symbol,
		OrderType:        order_type,
		TimeInForce:      time_in_force,
		ExpireTime:       expire_time,
		Account:          account,
		StrategyId:       strategy_id,
		SecurityId:       security_id,
		IdSource:         id_source,
		SecurityExchange: security_exchange,
		PostOnly:         post_only,
		ReduceOnly:       reduce_only,
	}, nil
}

// SBE encoder for CancelRequest
type CancelRequestEncoder struct{}

func (CancelRequestEncoder) Encode(ClOrdId string, OrigClOrdId string, Symbol string, Side CancelRequestSide, OrderQty *float64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 2)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 9)

	// Fixed fields
	writeString(&buf, ClOrdId)
	writeString(&buf, OrigClOrdId)
	writeString(&buf, Symbol)
	buf = append(buf, byte(Side.ToValue()))
	v := 0.0
	if OrderQty != nil {
		v = *OrderQty
	}
	writeF64BE(&buf, v)
	if OrderQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for CancelRequest
type CancelRequestDecoder struct {
	ClOrdId     string
	OrigClOrdId string
	Symbol      string
	Side        CancelRequestSide
	OrderQty    *float64
}

func (CancelRequestDecoder) EncodedLen() int { return 0 }

func CancelRequestDecoderDecode(buf []byte) (CancelRequestDecoder, error) {
	if len(buf) < 8 {
		return CancelRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CancelRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 2 {
		return CancelRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	sideRaw := buf[pos]
	pos++
	side, err := CancelRequestSideFromValue(sideRaw)
	if err != nil {
		return CancelRequestDecoder{}, err
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		order_qty = &raw
	} else {
		pos++
		order_qty = nil
	}

	return CancelRequestDecoder{
		ClOrdId:     cl_ord_id,
		OrigClOrdId: orig_cl_ord_id,
		Symbol:      symbol,
		Side:        side,
		OrderQty:    order_qty,
	}, nil
}

// SBE encoder for CancelReplaceRequest
type CancelReplaceRequestEncoder struct{}

func (CancelReplaceRequestEncoder) Encode(ClOrdId string, OrigClOrdId string, Symbol string, Side CancelReplaceRequestSide, OrderQty float64, Price *float64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 3)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)

	// Fixed fields
	writeString(&buf, ClOrdId)
	writeString(&buf, OrigClOrdId)
	writeString(&buf, Symbol)
	buf = append(buf, byte(Side.ToValue()))
	writeF64BE(&buf, OrderQty)
	v := 0.0
	if Price != nil {
		v = *Price
	}
	writeF64BE(&buf, v)
	if Price != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for CancelReplaceRequest
type CancelReplaceRequestDecoder struct {
	ClOrdId     string
	OrigClOrdId string
	Symbol      string
	Side        CancelReplaceRequestSide
	OrderQty    float64
	Price       *float64
}

func (CancelReplaceRequestDecoder) EncodedLen() int { return 0 }

func CancelReplaceRequestDecoderDecode(buf []byte) (CancelReplaceRequestDecoder, error) {
	if len(buf) < 8 {
		return CancelReplaceRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CancelReplaceRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 3 {
		return CancelReplaceRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	sideRaw := buf[pos]
	pos++
	side, err := CancelReplaceRequestSideFromValue(sideRaw)
	if err != nil {
		return CancelReplaceRequestDecoder{}, err
	}
	order_qty := readF64BE(buf, &pos)
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		price = &raw
	} else {
		pos++
		price = nil
	}

	return CancelReplaceRequestDecoder{
		ClOrdId:     cl_ord_id,
		OrigClOrdId: orig_cl_ord_id,
		Symbol:      symbol,
		Side:        side,
		OrderQty:    order_qty,
		Price:       price,
	}, nil
}

// SBE encoder for ExecutionReport
type ExecutionReportEncoder struct{}

func (ExecutionReportEncoder) Encode(ClOrdId string, OrderId string, ExecId string, ExecType ExecutionReportExecType, OrdStatus ExecutionReportOrdStatus, Side ExecutionReportSide, LastQty *float64, LastPrice *float64, LeavesQty float64, CumQty float64, AvgPrice float64, Symbol string, TransactTime int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 4)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 51)

	// Fixed fields
	writeString(&buf, ClOrdId)
	writeString(&buf, OrderId)
	writeString(&buf, ExecId)
	buf = append(buf, byte(ExecType.ToValue()))
	buf = append(buf, byte(OrdStatus.ToValue()))
	buf = append(buf, byte(Side.ToValue()))
	v := 0.0
	if LastQty != nil {
		v = *LastQty
	}
	writeF64BE(&buf, v)
	if LastQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := 0.0
	if LastPrice != nil {
		v = *LastPrice
	}
	writeF64BE(&buf, v)
	if LastPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeF64BE(&buf, LeavesQty)
	writeF64BE(&buf, CumQty)
	writeF64BE(&buf, AvgPrice)
	writeString(&buf, Symbol)
	writeI64BE(&buf, TransactTime)

	return buf, nil
}

// SBE decoder for ExecutionReport
type ExecutionReportDecoder struct {
	ClOrdId      string
	OrderId      string
	ExecId       string
	ExecType     ExecutionReportExecType
	OrdStatus    ExecutionReportOrdStatus
	Side         ExecutionReportSide
	LastQty      *float64
	LastPrice    *float64
	LeavesQty    float64
	CumQty       float64
	AvgPrice     float64
	Symbol       string
	TransactTime int64
}

func (ExecutionReportDecoder) EncodedLen() int { return 0 }

func ExecutionReportDecoderDecode(buf []byte) (ExecutionReportDecoder, error) {
	if len(buf) < 8 {
		return ExecutionReportDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return ExecutionReportDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 4 {
		return ExecutionReportDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	order_id := readString(buf, &pos)
	exec_id := readString(buf, &pos)
	exec_typeRaw := buf[pos]
	pos++
	exec_type, err := ExecutionReportExecTypeFromValue(exec_typeRaw)
	if err != nil {
		return ExecutionReportDecoder{}, err
	}
	ord_statusRaw := buf[pos]
	pos++
	ord_status, err := ExecutionReportOrdStatusFromValue(ord_statusRaw)
	if err != nil {
		return ExecutionReportDecoder{}, err
	}
	sideRaw := buf[pos]
	pos++
	side, err := ExecutionReportSideFromValue(sideRaw)
	if err != nil {
		return ExecutionReportDecoder{}, err
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		last_qty = &raw
	} else {
		pos++
		last_qty = nil
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		last_price = &raw
	} else {
		pos++
		last_price = nil
	}
	leaves_qty := readF64BE(buf, &pos)
	cum_qty := readF64BE(buf, &pos)
	avg_price := readF64BE(buf, &pos)
	symbol := readString(buf, &pos)
	transact_time := readI64BE(buf, &pos)

	return ExecutionReportDecoder{
		ClOrdId:      cl_ord_id,
		OrderId:      order_id,
		ExecId:       exec_id,
		ExecType:     exec_type,
		OrdStatus:    ord_status,
		Side:         side,
		LastQty:      last_qty,
		LastPrice:    last_price,
		LeavesQty:    leaves_qty,
		CumQty:       cum_qty,
		AvgPrice:     avg_price,
		Symbol:       symbol,
		TransactTime: transact_time,
	}, nil
}

// SBE encoder for CancelReject
type CancelRejectEncoder struct{}

func (CancelRejectEncoder) Encode(ClOrdId string, OrigClOrdId string, RejectReason CancelRejectRejectReason, Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 5)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, ClOrdId)
	writeString(&buf, OrigClOrdId)
	buf = append(buf, byte(RejectReason.ToValue()))
	writeString(&buf, Symbol)

	return buf, nil
}

// SBE decoder for CancelReject
type CancelRejectDecoder struct {
	ClOrdId      string
	OrigClOrdId  string
	RejectReason CancelRejectRejectReason
	Symbol       string
}

func (CancelRejectDecoder) EncodedLen() int { return 0 }

func CancelRejectDecoderDecode(buf []byte) (CancelRejectDecoder, error) {
	if len(buf) < 8 {
		return CancelRejectDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CancelRejectDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 5 {
		return CancelRejectDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	reject_reasonRaw := buf[pos]
	pos++
	reject_reason, err := CancelRejectRejectReasonFromValue(reject_reasonRaw)
	if err != nil {
		return CancelRejectDecoder{}, err
	}
	symbol := readString(buf, &pos)

	return CancelRejectDecoder{
		ClOrdId:      cl_ord_id,
		OrigClOrdId:  orig_cl_ord_id,
		RejectReason: reject_reason,
		Symbol:       symbol,
	}, nil
}

// SBE encoder for MarketDataSnapshot
type MarketDataSnapshotEncoder struct{}

func (MarketDataSnapshotEncoder) Encode(Symbol string, Exchange string, Bids []string, Asks []string, Timestamp int64, Sequence *int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 6)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)

	// Fixed fields
	writeString(&buf, Symbol)
	writeString(&buf, Exchange)
	writeU32BE(&buf, uint32(len(Bids)))
	for _, item := range Bids {
		PriceLevelEncoder{}.Encode(item, &buf)
	}
	writeU32BE(&buf, uint32(len(Asks)))
	for _, item := range Asks {
		PriceLevelEncoder{}.Encode(item, &buf)
	}
	writeI64BE(&buf, Timestamp)
	if Sequence != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *Sequence)
	} else {
		buf = append(buf, 0)
	}
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for MarketDataSnapshot
type MarketDataSnapshotDecoder struct {
	Symbol     string
	Exchange   string
	Bids       []string
	Asks       []string
	Timestamp  int64
	Sequence   *int64
	IsSnapshot *uint8
}

func (MarketDataSnapshotDecoder) EncodedLen() int { return 0 }

func MarketDataSnapshotDecoderDecode(buf []byte) (MarketDataSnapshotDecoder, error) {
	if len(buf) < 8 {
		return MarketDataSnapshotDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarketDataSnapshotDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 6 {
		return MarketDataSnapshotDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	exchange := readString(buf, &pos)
	bidsCount := int(readU32BE(buf, &pos))
	bids := make([]PriceLevel, 0, bidsCount)
	for i := 0; i < bidsCount; i++ {
		item, err := PriceLevelDecoder{}.Decode(buf[pos:])
		if err != nil {
			return MarketDataSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		bids = append(bids, item)
	}
	asksCount := int(readU32BE(buf, &pos))
	asks := make([]PriceLevel, 0, asksCount)
	for i := 0; i < asksCount; i++ {
		item, err := PriceLevelDecoder{}.Decode(buf[pos:])
		if err != nil {
			return MarketDataSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		asks = append(asks, item)
	}
	timestamp := readI64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		sequence = &v
	} else {
		pos++
		sequence = nil
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return MarketDataSnapshotDecoder{
		Symbol:     symbol,
		Exchange:   exchange,
		Bids:       bids,
		Asks:       asks,
		Timestamp:  timestamp,
		Sequence:   sequence,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for MarketDataIncrementalRefresh
type MarketDataIncrementalRefreshEncoder struct{}

func (MarketDataIncrementalRefreshEncoder) Encode(Symbol string, Updates []string, Timestamp int64, Sequence *int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 7)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 16)

	// Fixed fields
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Updates)))
	for _, item := range Updates {
		MarketDataUpdateEncoder{}.Encode(item, &buf)
	}
	writeI64BE(&buf, Timestamp)
	if Sequence != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *Sequence)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for MarketDataIncrementalRefresh
type MarketDataIncrementalRefreshDecoder struct {
	Symbol    string
	Updates   []string
	Timestamp int64
	Sequence  *int64
}

func (MarketDataIncrementalRefreshDecoder) EncodedLen() int { return 0 }

func MarketDataIncrementalRefreshDecoderDecode(buf []byte) (MarketDataIncrementalRefreshDecoder, error) {
	if len(buf) < 8 {
		return MarketDataIncrementalRefreshDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarketDataIncrementalRefreshDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 7 {
		return MarketDataIncrementalRefreshDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	updatesCount := int(readU32BE(buf, &pos))
	updates := make([]MarketDataUpdate, 0, updatesCount)
	for i := 0; i < updatesCount; i++ {
		item, err := MarketDataUpdateDecoder{}.Decode(buf[pos:])
		if err != nil {
			return MarketDataIncrementalRefreshDecoder{}, err
		}
		pos += item.EncodedLen()
		updates = append(updates, item)
	}
	timestamp := readI64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		sequence = &v
	} else {
		pos++
		sequence = nil
	}

	return MarketDataIncrementalRefreshDecoder{
		Symbol:    symbol,
		Updates:   updates,
		Timestamp: timestamp,
		Sequence:  sequence,
	}, nil
}

// SBE encoder for OrderBookRequest
type OrderBookRequestEncoder struct{}

func (OrderBookRequestEncoder) Encode(Symbol string, Depth *u32, AtTime *int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 8)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 12)

	// Fixed fields
	writeString(&buf, Symbol)
	// TODO: encode depth as u32
	if AtTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *AtTime)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OrderBookRequest
type OrderBookRequestDecoder struct {
	Symbol string
	Depth  *u32
	AtTime *int64
}

func (OrderBookRequestDecoder) EncodedLen() int { return 0 }

func OrderBookRequestDecoderDecode(buf []byte) (OrderBookRequestDecoder, error) {
	if len(buf) < 8 {
		return OrderBookRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderBookRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 8 {
		return OrderBookRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	// TODO: decode depth
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		at_time = &v
	} else {
		pos++
		at_time = nil
	}

	return OrderBookRequestDecoder{
		Symbol: symbol,
		Depth:  depth,
		AtTime: at_time,
	}, nil
}

// SBE encoder for OrderBookSnapshot
type OrderBookSnapshotEncoder struct{}

func (OrderBookSnapshotEncoder) Encode(Symbol string, Exchange string, Bids []string, Asks []string, Timestamp int64, Sequence *int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 9)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)

	// Fixed fields
	writeString(&buf, Symbol)
	writeString(&buf, Exchange)
	writeU32BE(&buf, uint32(len(Bids)))
	for _, item := range Bids {
		PriceLevelEncoder{}.Encode(item, &buf)
	}
	writeU32BE(&buf, uint32(len(Asks)))
	for _, item := range Asks {
		PriceLevelEncoder{}.Encode(item, &buf)
	}
	writeI64BE(&buf, Timestamp)
	if Sequence != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *Sequence)
	} else {
		buf = append(buf, 0)
	}
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OrderBookSnapshot
type OrderBookSnapshotDecoder struct {
	Symbol     string
	Exchange   string
	Bids       []string
	Asks       []string
	Timestamp  int64
	Sequence   *int64
	IsSnapshot *uint8
}

func (OrderBookSnapshotDecoder) EncodedLen() int { return 0 }

func OrderBookSnapshotDecoderDecode(buf []byte) (OrderBookSnapshotDecoder, error) {
	if len(buf) < 8 {
		return OrderBookSnapshotDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderBookSnapshotDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 9 {
		return OrderBookSnapshotDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	exchange := readString(buf, &pos)
	bidsCount := int(readU32BE(buf, &pos))
	bids := make([]PriceLevel, 0, bidsCount)
	for i := 0; i < bidsCount; i++ {
		item, err := PriceLevelDecoder{}.Decode(buf[pos:])
		if err != nil {
			return OrderBookSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		bids = append(bids, item)
	}
	asksCount := int(readU32BE(buf, &pos))
	asks := make([]PriceLevel, 0, asksCount)
	for i := 0; i < asksCount; i++ {
		item, err := PriceLevelDecoder{}.Decode(buf[pos:])
		if err != nil {
			return OrderBookSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		asks = append(asks, item)
	}
	timestamp := readI64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		sequence = &v
	} else {
		pos++
		sequence = nil
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return OrderBookSnapshotDecoder{
		Symbol:     symbol,
		Exchange:   exchange,
		Bids:       bids,
		Asks:       asks,
		Timestamp:  timestamp,
		Sequence:   sequence,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for OrderBookDelta
type OrderBookDeltaEncoder struct{}

func (OrderBookDeltaEncoder) Encode(Symbol string, Updates []string, Timestamp int64, Sequence *int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 10)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 16)

	// Fixed fields
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Updates)))
	for _, item := range Updates {
		MarketDataUpdateEncoder{}.Encode(item, &buf)
	}
	writeI64BE(&buf, Timestamp)
	if Sequence != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *Sequence)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OrderBookDelta
type OrderBookDeltaDecoder struct {
	Symbol    string
	Updates   []string
	Timestamp int64
	Sequence  *int64
}

func (OrderBookDeltaDecoder) EncodedLen() int { return 0 }

func OrderBookDeltaDecoderDecode(buf []byte) (OrderBookDeltaDecoder, error) {
	if len(buf) < 8 {
		return OrderBookDeltaDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderBookDeltaDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 10 {
		return OrderBookDeltaDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	updatesCount := int(readU32BE(buf, &pos))
	updates := make([]MarketDataUpdate, 0, updatesCount)
	for i := 0; i < updatesCount; i++ {
		item, err := MarketDataUpdateDecoder{}.Decode(buf[pos:])
		if err != nil {
			return OrderBookDeltaDecoder{}, err
		}
		pos += item.EncodedLen()
		updates = append(updates, item)
	}
	timestamp := readI64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		sequence = &v
	} else {
		pos++
		sequence = nil
	}

	return OrderBookDeltaDecoder{
		Symbol:    symbol,
		Updates:   updates,
		Timestamp: timestamp,
		Sequence:  sequence,
	}, nil
}

// SBE encoder for AggregateTradeEvent
type AggregateTradeEventEncoder struct{}

func (AggregateTradeEventEncoder) Encode(Trade string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 11)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Trade)

	return buf, nil
}

// SBE decoder for AggregateTradeEvent
type AggregateTradeEventDecoder struct {
	Trade string
}

func (AggregateTradeEventDecoder) EncodedLen() int { return 0 }

func AggregateTradeEventDecoderDecode(buf []byte) (AggregateTradeEventDecoder, error) {
	if len(buf) < 8 {
		return AggregateTradeEventDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AggregateTradeEventDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 11 {
		return AggregateTradeEventDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	trade := readString(buf, &pos)

	return AggregateTradeEventDecoder{
		Trade: trade,
	}, nil
}

// SBE encoder for AggregateTradeRequest
type AggregateTradeRequestEncoder struct{}

func (AggregateTradeRequestEncoder) Encode(Symbol string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 12)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Symbol)
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for AggregateTradeRequest
type AggregateTradeRequestDecoder struct {
	Symbol    string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (AggregateTradeRequestDecoder) EncodedLen() int { return 0 }

func AggregateTradeRequestDecoderDecode(buf []byte) (AggregateTradeRequestDecoder, error) {
	if len(buf) < 8 {
		return AggregateTradeRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AggregateTradeRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 12 {
		return AggregateTradeRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return AggregateTradeRequestDecoder{
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for AggregateTradeBatch
type AggregateTradeBatchEncoder struct{}

func (AggregateTradeBatchEncoder) Encode(Symbol string, Trades []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 13)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Trades)))
	for _, item := range Trades {
		AggregateTradeEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for AggregateTradeBatch
type AggregateTradeBatchDecoder struct {
	Symbol     string
	Trades     []string
	HasMore    uint8
	NextCursor *string
}

func (AggregateTradeBatchDecoder) EncodedLen() int { return 0 }

func AggregateTradeBatchDecoderDecode(buf []byte) (AggregateTradeBatchDecoder, error) {
	if len(buf) < 8 {
		return AggregateTradeBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AggregateTradeBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 13 {
		return AggregateTradeBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	tradesCount := int(readU32BE(buf, &pos))
	trades := make([]AggregateTrade, 0, tradesCount)
	for i := 0; i < tradesCount; i++ {
		item, err := AggregateTradeDecoder{}.Decode(buf[pos:])
		if err != nil {
			return AggregateTradeBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		trades = append(trades, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return AggregateTradeBatchDecoder{
		Symbol:     symbol,
		Trades:     trades,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for MiniTicker
type MiniTickerEncoder struct{}

func (MiniTickerEncoder) Encode(Symbol string, LastPrice float64, Volume float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 14)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 25)

	// Fixed fields
	writeString(&buf, Symbol)
	writeF64BE(&buf, LastPrice)
	writeF64BE(&buf, Volume)
	writeI64BE(&buf, Timestamp)
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for MiniTicker
type MiniTickerDecoder struct {
	Symbol     string
	LastPrice  float64
	Volume     float64
	Timestamp  int64
	IsSnapshot *uint8
}

func (MiniTickerDecoder) EncodedLen() int { return 0 }

func MiniTickerDecoderDecode(buf []byte) (MiniTickerDecoder, error) {
	if len(buf) < 8 {
		return MiniTickerDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MiniTickerDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 14 {
		return MiniTickerDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	last_price := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	timestamp := readI64BE(buf, &pos)
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return MiniTickerDecoder{
		Symbol:     symbol,
		LastPrice:  last_price,
		Volume:     volume,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for AllMidsRequest
type AllMidsRequestEncoder struct{}

func (AllMidsRequestEncoder) Encode() ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 15)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields

	return buf, nil
}

// SBE decoder for AllMidsRequest
type AllMidsRequestDecoder struct {
}

func (AllMidsRequestDecoder) EncodedLen() int { return 0 }

func AllMidsRequestDecoderDecode(buf []byte) (AllMidsRequestDecoder, error) {
	if len(buf) < 8 {
		return AllMidsRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AllMidsRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 15 {
		return AllMidsRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}

	return AllMidsRequestDecoder{}, nil
}

// SBE encoder for AllMidsBatch
type AllMidsBatchEncoder struct{}

func (AllMidsBatchEncoder) Encode(Tickers []string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 16)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeU32BE(&buf, uint32(len(Tickers)))
	for _, item := range Tickers {
		MiniTickerEncoder{}.Encode(item, &buf)
	}

	return buf, nil
}

// SBE decoder for AllMidsBatch
type AllMidsBatchDecoder struct {
	Tickers []string
}

func (AllMidsBatchDecoder) EncodedLen() int { return 0 }

func AllMidsBatchDecoderDecode(buf []byte) (AllMidsBatchDecoder, error) {
	if len(buf) < 8 {
		return AllMidsBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AllMidsBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 16 {
		return AllMidsBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	tickersCount := int(readU32BE(buf, &pos))
	tickers := make([]MiniTicker, 0, tickersCount)
	for i := 0; i < tickersCount; i++ {
		item, err := MiniTickerDecoder{}.Decode(buf[pos:])
		if err != nil {
			return AllMidsBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		tickers = append(tickers, item)
	}

	return AllMidsBatchDecoder{
		Tickers: tickers,
	}, nil
}

// SBE encoder for MarkPriceUpdate
type MarkPriceUpdateEncoder struct{}

func (MarkPriceUpdateEncoder) Encode(Symbol string, MarkPrice float64, IndexPrice *float64, FundingRate *float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 17)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 33)

	// Fixed fields
	writeString(&buf, Symbol)
	writeF64BE(&buf, MarkPrice)
	v := 0.0
	if IndexPrice != nil {
		v = *IndexPrice
	}
	writeF64BE(&buf, v)
	if IndexPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := 0.0
	if FundingRate != nil {
		v = *FundingRate
	}
	writeF64BE(&buf, v)
	if FundingRate != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeI64BE(&buf, Timestamp)
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for MarkPriceUpdate
type MarkPriceUpdateDecoder struct {
	Symbol      string
	MarkPrice   float64
	IndexPrice  *float64
	FundingRate *float64
	Timestamp   int64
	IsSnapshot  *uint8
}

func (MarkPriceUpdateDecoder) EncodedLen() int { return 0 }

func MarkPriceUpdateDecoderDecode(buf []byte) (MarkPriceUpdateDecoder, error) {
	if len(buf) < 8 {
		return MarkPriceUpdateDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarkPriceUpdateDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 17 {
		return MarkPriceUpdateDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	mark_price := readF64BE(buf, &pos)
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		index_price = &raw
	} else {
		pos++
		index_price = nil
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		funding_rate = &raw
	} else {
		pos++
		funding_rate = nil
	}
	timestamp := readI64BE(buf, &pos)
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return MarkPriceUpdateDecoder{
		Symbol:      symbol,
		MarkPrice:   mark_price,
		IndexPrice:  index_price,
		FundingRate: funding_rate,
		Timestamp:   timestamp,
		IsSnapshot:  is_snapshot,
	}, nil
}

// SBE encoder for MarkPriceRequest
type MarkPriceRequestEncoder struct{}

func (MarkPriceRequestEncoder) Encode(Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 18)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Symbol)

	return buf, nil
}

// SBE decoder for MarkPriceRequest
type MarkPriceRequestDecoder struct {
	Symbol string
}

func (MarkPriceRequestDecoder) EncodedLen() int { return 0 }

func MarkPriceRequestDecoderDecode(buf []byte) (MarkPriceRequestDecoder, error) {
	if len(buf) < 8 {
		return MarkPriceRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarkPriceRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 18 {
		return MarkPriceRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)

	return MarkPriceRequestDecoder{
		Symbol: symbol,
	}, nil
}

// SBE encoder for LiquidationTradeEvent
type LiquidationTradeEventEncoder struct{}

func (LiquidationTradeEventEncoder) Encode(Trade string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 19)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Trade)

	return buf, nil
}

// SBE decoder for LiquidationTradeEvent
type LiquidationTradeEventDecoder struct {
	Trade string
}

func (LiquidationTradeEventDecoder) EncodedLen() int { return 0 }

func LiquidationTradeEventDecoderDecode(buf []byte) (LiquidationTradeEventDecoder, error) {
	if len(buf) < 8 {
		return LiquidationTradeEventDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LiquidationTradeEventDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 19 {
		return LiquidationTradeEventDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	trade := readString(buf, &pos)

	return LiquidationTradeEventDecoder{
		Trade: trade,
	}, nil
}

// SBE encoder for CandleBarEvent
type CandleBarEventEncoder struct{}

func (CandleBarEventEncoder) Encode(Bar string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 20)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Bar)

	return buf, nil
}

// SBE decoder for CandleBarEvent
type CandleBarEventDecoder struct {
	Bar string
}

func (CandleBarEventDecoder) EncodedLen() int { return 0 }

func CandleBarEventDecoderDecode(buf []byte) (CandleBarEventDecoder, error) {
	if len(buf) < 8 {
		return CandleBarEventDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CandleBarEventDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 20 {
		return CandleBarEventDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	bar := readString(buf, &pos)

	return CandleBarEventDecoder{
		Bar: bar,
	}, nil
}

// SBE encoder for CandleBarRequest
type CandleBarRequestEncoder struct{}

func (CandleBarRequestEncoder) Encode(Symbol string, Interval string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 21)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Symbol)
	writeString(&buf, Interval)
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for CandleBarRequest
type CandleBarRequestDecoder struct {
	Symbol    string
	Interval  string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (CandleBarRequestDecoder) EncodedLen() int { return 0 }

func CandleBarRequestDecoderDecode(buf []byte) (CandleBarRequestDecoder, error) {
	if len(buf) < 8 {
		return CandleBarRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CandleBarRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 21 {
		return CandleBarRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return CandleBarRequestDecoder{
		Symbol:    symbol,
		Interval:  interval,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for CandleBarBatch
type CandleBarBatchEncoder struct{}

func (CandleBarBatchEncoder) Encode(Symbol string, Interval string, Bars []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 22)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Symbol)
	writeString(&buf, Interval)
	writeU32BE(&buf, uint32(len(Bars)))
	for _, item := range Bars {
		CandleBarEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for CandleBarBatch
type CandleBarBatchDecoder struct {
	Symbol     string
	Interval   string
	Bars       []string
	HasMore    uint8
	NextCursor *string
}

func (CandleBarBatchDecoder) EncodedLen() int { return 0 }

func CandleBarBatchDecoderDecode(buf []byte) (CandleBarBatchDecoder, error) {
	if len(buf) < 8 {
		return CandleBarBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CandleBarBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 22 {
		return CandleBarBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	barsCount := int(readU32BE(buf, &pos))
	bars := make([]CandleBar, 0, barsCount)
	for i := 0; i < barsCount; i++ {
		item, err := CandleBarDecoder{}.Decode(buf[pos:])
		if err != nil {
			return CandleBarBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		bars = append(bars, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return CandleBarBatchDecoder{
		Symbol:     symbol,
		Interval:   interval,
		Bars:       bars,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for PublicTradeEvent
type PublicTradeEventEncoder struct{}

func (PublicTradeEventEncoder) Encode(Trade string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 23)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Trade)

	return buf, nil
}

// SBE decoder for PublicTradeEvent
type PublicTradeEventDecoder struct {
	Trade string
}

func (PublicTradeEventDecoder) EncodedLen() int { return 0 }

func PublicTradeEventDecoderDecode(buf []byte) (PublicTradeEventDecoder, error) {
	if len(buf) < 8 {
		return PublicTradeEventDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PublicTradeEventDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 23 {
		return PublicTradeEventDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	trade := readString(buf, &pos)

	return PublicTradeEventDecoder{
		Trade: trade,
	}, nil
}

// SBE encoder for TradeHistoryRequest
type TradeHistoryRequestEncoder struct{}

func (TradeHistoryRequestEncoder) Encode(Symbol string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 24)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Symbol)
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for TradeHistoryRequest
type TradeHistoryRequestDecoder struct {
	Symbol    string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (TradeHistoryRequestDecoder) EncodedLen() int { return 0 }

func TradeHistoryRequestDecoderDecode(buf []byte) (TradeHistoryRequestDecoder, error) {
	if len(buf) < 8 {
		return TradeHistoryRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return TradeHistoryRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 24 {
		return TradeHistoryRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return TradeHistoryRequestDecoder{
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for PublicTradeBatch
type PublicTradeBatchEncoder struct{}

func (PublicTradeBatchEncoder) Encode(Symbol string, Trades []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 25)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Trades)))
	for _, item := range Trades {
		PublicTradeEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for PublicTradeBatch
type PublicTradeBatchDecoder struct {
	Symbol     string
	Trades     []string
	HasMore    uint8
	NextCursor *string
}

func (PublicTradeBatchDecoder) EncodedLen() int { return 0 }

func PublicTradeBatchDecoderDecode(buf []byte) (PublicTradeBatchDecoder, error) {
	if len(buf) < 8 {
		return PublicTradeBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PublicTradeBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 25 {
		return PublicTradeBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	tradesCount := int(readU32BE(buf, &pos))
	trades := make([]PublicTrade, 0, tradesCount)
	for i := 0; i < tradesCount; i++ {
		item, err := PublicTradeDecoder{}.Decode(buf[pos:])
		if err != nil {
			return PublicTradeBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		trades = append(trades, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return PublicTradeBatchDecoder{
		Symbol:     symbol,
		Trades:     trades,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for BestBidOffer
type BestBidOfferEncoder struct{}

func (BestBidOfferEncoder) Encode(Symbol string, BidPrice *float64, BidQty *float64, AskPrice *float64, AskQty *float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 26)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 41)

	// Fixed fields
	writeString(&buf, Symbol)
	v := 0.0
	if BidPrice != nil {
		v = *BidPrice
	}
	writeF64BE(&buf, v)
	if BidPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := 0.0
	if BidQty != nil {
		v = *BidQty
	}
	writeF64BE(&buf, v)
	if BidQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := 0.0
	if AskPrice != nil {
		v = *AskPrice
	}
	writeF64BE(&buf, v)
	if AskPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	v := 0.0
	if AskQty != nil {
		v = *AskQty
	}
	writeF64BE(&buf, v)
	if AskQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeI64BE(&buf, Timestamp)
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for BestBidOffer
type BestBidOfferDecoder struct {
	Symbol     string
	BidPrice   *float64
	BidQty     *float64
	AskPrice   *float64
	AskQty     *float64
	Timestamp  int64
	IsSnapshot *uint8
}

func (BestBidOfferDecoder) EncodedLen() int { return 0 }

func BestBidOfferDecoderDecode(buf []byte) (BestBidOfferDecoder, error) {
	if len(buf) < 8 {
		return BestBidOfferDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BestBidOfferDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 26 {
		return BestBidOfferDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		bid_price = &raw
	} else {
		pos++
		bid_price = nil
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		bid_qty = &raw
	} else {
		pos++
		bid_qty = nil
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		ask_price = &raw
	} else {
		pos++
		ask_price = nil
	}
	raw := readF64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		ask_qty = &raw
	} else {
		pos++
		ask_qty = nil
	}
	timestamp := readI64BE(buf, &pos)
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return BestBidOfferDecoder{
		Symbol:     symbol,
		BidPrice:   bid_price,
		BidQty:     bid_qty,
		AskPrice:   ask_price,
		AskQty:     ask_qty,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for SymbolTicker
type SymbolTickerEncoder struct{}

func (SymbolTickerEncoder) Encode(Symbol string, LastPrice float64, PriceChange float64, PriceChangePct float64, Volume float64, High float64, Low float64, Open float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 27)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 65)

	// Fixed fields
	writeString(&buf, Symbol)
	writeF64BE(&buf, LastPrice)
	writeF64BE(&buf, PriceChange)
	writeF64BE(&buf, PriceChangePct)
	writeF64BE(&buf, Volume)
	writeF64BE(&buf, High)
	writeF64BE(&buf, Low)
	writeF64BE(&buf, Open)
	writeI64BE(&buf, Timestamp)
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for SymbolTicker
type SymbolTickerDecoder struct {
	Symbol         string
	LastPrice      float64
	PriceChange    float64
	PriceChangePct float64
	Volume         float64
	High           float64
	Low            float64
	Open           float64
	Timestamp      int64
	IsSnapshot     *uint8
}

func (SymbolTickerDecoder) EncodedLen() int { return 0 }

func SymbolTickerDecoderDecode(buf []byte) (SymbolTickerDecoder, error) {
	if len(buf) < 8 {
		return SymbolTickerDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return SymbolTickerDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 27 {
		return SymbolTickerDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	last_price := readF64BE(buf, &pos)
	price_change := readF64BE(buf, &pos)
	price_change_pct := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	high := readF64BE(buf, &pos)
	low := readF64BE(buf, &pos)
	open := readF64BE(buf, &pos)
	timestamp := readI64BE(buf, &pos)
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return SymbolTickerDecoder{
		Symbol:         symbol,
		LastPrice:      last_price,
		PriceChange:    price_change,
		PriceChangePct: price_change_pct,
		Volume:         volume,
		High:           high,
		Low:            low,
		Open:           open,
		Timestamp:      timestamp,
		IsSnapshot:     is_snapshot,
	}, nil
}

// SBE encoder for TickerRequest
type TickerRequestEncoder struct{}

func (TickerRequestEncoder) Encode(Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 28)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Symbol)

	return buf, nil
}

// SBE decoder for TickerRequest
type TickerRequestDecoder struct {
	Symbol string
}

func (TickerRequestDecoder) EncodedLen() int { return 0 }

func TickerRequestDecoderDecode(buf []byte) (TickerRequestDecoder, error) {
	if len(buf) < 8 {
		return TickerRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return TickerRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 28 {
		return TickerRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)

	return TickerRequestDecoder{
		Symbol: symbol,
	}, nil
}

// SBE encoder for InstrumentCatalogRequest
type InstrumentCatalogRequestEncoder struct{}

func (InstrumentCatalogRequestEncoder) Encode() ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 29)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields

	return buf, nil
}

// SBE decoder for InstrumentCatalogRequest
type InstrumentCatalogRequestDecoder struct {
}

func (InstrumentCatalogRequestDecoder) EncodedLen() int { return 0 }

func InstrumentCatalogRequestDecoderDecode(buf []byte) (InstrumentCatalogRequestDecoder, error) {
	if len(buf) < 8 {
		return InstrumentCatalogRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return InstrumentCatalogRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 29 {
		return InstrumentCatalogRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}

	return InstrumentCatalogRequestDecoder{}, nil
}

// SBE encoder for InstrumentCatalogResponse
type InstrumentCatalogResponseEncoder struct{}

func (InstrumentCatalogResponseEncoder) Encode(Instruments []string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 30)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeU32BE(&buf, uint32(len(Instruments)))
	for _, item := range Instruments {
		InstrumentMetadataEncoder{}.Encode(item, &buf)
	}

	return buf, nil
}

// SBE decoder for InstrumentCatalogResponse
type InstrumentCatalogResponseDecoder struct {
	Instruments []string
}

func (InstrumentCatalogResponseDecoder) EncodedLen() int { return 0 }

func InstrumentCatalogResponseDecoderDecode(buf []byte) (InstrumentCatalogResponseDecoder, error) {
	if len(buf) < 8 {
		return InstrumentCatalogResponseDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return InstrumentCatalogResponseDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 30 {
		return InstrumentCatalogResponseDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	instrumentsCount := int(readU32BE(buf, &pos))
	instruments := make([]InstrumentMetadata, 0, instrumentsCount)
	for i := 0; i < instrumentsCount; i++ {
		item, err := InstrumentMetadataDecoder{}.Decode(buf[pos:])
		if err != nil {
			return InstrumentCatalogResponseDecoder{}, err
		}
		pos += item.EncodedLen()
		instruments = append(instruments, item)
	}

	return InstrumentCatalogResponseDecoder{
		Instruments: instruments,
	}, nil
}

// SBE encoder for AccountSummary
type AccountSummaryEncoder struct{}

func (AccountSummaryEncoder) Encode(Account string, Balance float64, BuyingPower float64, Currency string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 31)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 16)

	// Fixed fields
	writeString(&buf, Account)
	writeF64BE(&buf, Balance)
	writeF64BE(&buf, BuyingPower)
	writeString(&buf, Currency)

	return buf, nil
}

// SBE decoder for AccountSummary
type AccountSummaryDecoder struct {
	Account     string
	Balance     float64
	BuyingPower float64
	Currency    string
}

func (AccountSummaryDecoder) EncodedLen() int { return 0 }

func AccountSummaryDecoderDecode(buf []byte) (AccountSummaryDecoder, error) {
	if len(buf) < 8 {
		return AccountSummaryDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AccountSummaryDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 31 {
		return AccountSummaryDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	balance := readF64BE(buf, &pos)
	buying_power := readF64BE(buf, &pos)
	currency := readString(buf, &pos)

	return AccountSummaryDecoder{
		Account:     account,
		Balance:     balance,
		BuyingPower: buying_power,
		Currency:    currency,
	}, nil
}

// SBE encoder for MarginSummary
type MarginSummaryEncoder struct{}

func (MarginSummaryEncoder) Encode(Account string, Balance float64, BuyingPower float64, Equity float64, MarginUsed float64, Available float64, Currency string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 32)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 40)

	// Fixed fields
	writeString(&buf, Account)
	writeF64BE(&buf, Balance)
	writeF64BE(&buf, BuyingPower)
	writeF64BE(&buf, Equity)
	writeF64BE(&buf, MarginUsed)
	writeF64BE(&buf, Available)
	writeString(&buf, Currency)

	return buf, nil
}

// SBE decoder for MarginSummary
type MarginSummaryDecoder struct {
	Account     string
	Balance     float64
	BuyingPower float64
	Equity      float64
	MarginUsed  float64
	Available   float64
	Currency    string
}

func (MarginSummaryDecoder) EncodedLen() int { return 0 }

func MarginSummaryDecoderDecode(buf []byte) (MarginSummaryDecoder, error) {
	if len(buf) < 8 {
		return MarginSummaryDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarginSummaryDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 32 {
		return MarginSummaryDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	balance := readF64BE(buf, &pos)
	buying_power := readF64BE(buf, &pos)
	equity := readF64BE(buf, &pos)
	margin_used := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	currency := readString(buf, &pos)

	return MarginSummaryDecoder{
		Account:     account,
		Balance:     balance,
		BuyingPower: buying_power,
		Equity:      equity,
		MarginUsed:  margin_used,
		Available:   available,
		Currency:    currency,
	}, nil
}

// SBE encoder for BalanceSnapshot
type BalanceSnapshotEncoder struct{}

func (BalanceSnapshotEncoder) Encode(Account string, Balances []string, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 33)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Balances)))
	for _, item := range Balances {
		BalanceEntryEncoder{}.Encode(item, &buf)
	}
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for BalanceSnapshot
type BalanceSnapshotDecoder struct {
	Account    string
	Balances   []string
	IsSnapshot *uint8
}

func (BalanceSnapshotDecoder) EncodedLen() int { return 0 }

func BalanceSnapshotDecoderDecode(buf []byte) (BalanceSnapshotDecoder, error) {
	if len(buf) < 8 {
		return BalanceSnapshotDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BalanceSnapshotDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 33 {
		return BalanceSnapshotDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	balancesCount := int(readU32BE(buf, &pos))
	balances := make([]BalanceEntry, 0, balancesCount)
	for i := 0; i < balancesCount; i++ {
		item, err := BalanceEntryDecoder{}.Decode(buf[pos:])
		if err != nil {
			return BalanceSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		balances = append(balances, item)
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return BalanceSnapshotDecoder{
		Account:    account,
		Balances:   balances,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for BalanceUpdate
type BalanceUpdateEncoder struct{}

func (BalanceUpdateEncoder) Encode(Account string, Asset string, Delta float64, Total float64, Available float64, Reason BalanceUpdateReason) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 34)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 25)

	// Fixed fields
	writeString(&buf, Account)
	writeString(&buf, Asset)
	writeF64BE(&buf, Delta)
	writeF64BE(&buf, Total)
	writeF64BE(&buf, Available)
	buf = append(buf, byte(Reason.ToValue()))

	return buf, nil
}

// SBE decoder for BalanceUpdate
type BalanceUpdateDecoder struct {
	Account   string
	Asset     string
	Delta     float64
	Total     float64
	Available float64
	Reason    BalanceUpdateReason
}

func (BalanceUpdateDecoder) EncodedLen() int { return 0 }

func BalanceUpdateDecoderDecode(buf []byte) (BalanceUpdateDecoder, error) {
	if len(buf) < 8 {
		return BalanceUpdateDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BalanceUpdateDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 34 {
		return BalanceUpdateDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	asset := readString(buf, &pos)
	delta := readF64BE(buf, &pos)
	total := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	reasonRaw := buf[pos]
	pos++
	reason, err := BalanceUpdateReasonFromValue(reasonRaw)
	if err != nil {
		return BalanceUpdateDecoder{}, err
	}

	return BalanceUpdateDecoder{
		Account:   account,
		Asset:     asset,
		Delta:     delta,
		Total:     total,
		Available: available,
		Reason:    reason,
	}, nil
}

// SBE encoder for PositionSnapshot
type PositionSnapshotEncoder struct{}

func (PositionSnapshotEncoder) Encode(Account string, Positions []string, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 35)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Positions)))
	for _, item := range Positions {
		PositionEntryEncoder{}.Encode(item, &buf)
	}
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for PositionSnapshot
type PositionSnapshotDecoder struct {
	Account    string
	Positions  []string
	IsSnapshot *uint8
}

func (PositionSnapshotDecoder) EncodedLen() int { return 0 }

func PositionSnapshotDecoderDecode(buf []byte) (PositionSnapshotDecoder, error) {
	if len(buf) < 8 {
		return PositionSnapshotDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PositionSnapshotDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 35 {
		return PositionSnapshotDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	positionsCount := int(readU32BE(buf, &pos))
	positions := make([]PositionEntry, 0, positionsCount)
	for i := 0; i < positionsCount; i++ {
		item, err := PositionEntryDecoder{}.Decode(buf[pos:])
		if err != nil {
			return PositionSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		positions = append(positions, item)
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return PositionSnapshotDecoder{
		Account:    account,
		Positions:  positions,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for PositionUpdate
type PositionUpdateEncoder struct{}

func (PositionUpdateEncoder) Encode(Account string, Symbol string, Qty float64, EntryPrice float64, UnrealizedPnl float64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 36)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 24)

	// Fixed fields
	writeString(&buf, Account)
	writeString(&buf, Symbol)
	writeF64BE(&buf, Qty)
	writeF64BE(&buf, EntryPrice)
	writeF64BE(&buf, UnrealizedPnl)

	return buf, nil
}

// SBE decoder for PositionUpdate
type PositionUpdateDecoder struct {
	Account       string
	Symbol        string
	Qty           float64
	EntryPrice    float64
	UnrealizedPnl float64
}

func (PositionUpdateDecoder) EncodedLen() int { return 0 }

func PositionUpdateDecoderDecode(buf []byte) (PositionUpdateDecoder, error) {
	if len(buf) < 8 {
		return PositionUpdateDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PositionUpdateDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 36 {
		return PositionUpdateDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	entry_price := readF64BE(buf, &pos)
	unrealized_pnl := readF64BE(buf, &pos)

	return PositionUpdateDecoder{
		Account:       account,
		Symbol:        symbol,
		Qty:           qty,
		EntryPrice:    entry_price,
		UnrealizedPnl: unrealized_pnl,
	}, nil
}

// SBE encoder for MarginUpdate
type MarginUpdateEncoder struct{}

func (MarginUpdateEncoder) Encode(Account string, Summary string, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 37)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeString(&buf, Summary)
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for MarginUpdate
type MarginUpdateDecoder struct {
	Account    string
	Summary    string
	IsSnapshot *uint8
}

func (MarginUpdateDecoder) EncodedLen() int { return 0 }

func MarginUpdateDecoderDecode(buf []byte) (MarginUpdateDecoder, error) {
	if len(buf) < 8 {
		return MarginUpdateDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarginUpdateDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 37 {
		return MarginUpdateDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	summary := readString(buf, &pos)
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return MarginUpdateDecoder{
		Account:    account,
		Summary:    summary,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for UserLiquidation
type UserLiquidationEncoder struct{}

func (UserLiquidationEncoder) Encode(Account string, Symbol string, Qty float64, Price float64, Timestamp int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 38)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 24)

	// Fixed fields
	writeString(&buf, Account)
	writeString(&buf, Symbol)
	writeF64BE(&buf, Qty)
	writeF64BE(&buf, Price)
	writeI64BE(&buf, Timestamp)

	return buf, nil
}

// SBE decoder for UserLiquidation
type UserLiquidationDecoder struct {
	Account   string
	Symbol    string
	Qty       float64
	Price     float64
	Timestamp int64
}

func (UserLiquidationDecoder) EncodedLen() int { return 0 }

func UserLiquidationDecoderDecode(buf []byte) (UserLiquidationDecoder, error) {
	if len(buf) < 8 {
		return UserLiquidationDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return UserLiquidationDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 38 {
		return UserLiquidationDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	price := readF64BE(buf, &pos)
	timestamp := readI64BE(buf, &pos)

	return UserLiquidationDecoder{
		Account:   account,
		Symbol:    symbol,
		Qty:       qty,
		Price:     price,
		Timestamp: timestamp,
	}, nil
}

// SBE encoder for OrderListStatus
type OrderListStatusEncoder struct{}

func (OrderListStatusEncoder) Encode(Account string, ListId string, Status OrderListStatusStatus, Symbol *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 39)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeString(&buf, ListId)
	buf = append(buf, byte(Status.ToValue()))
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OrderListStatus
type OrderListStatusDecoder struct {
	Account string
	ListId  string
	Status  OrderListStatusStatus
	Symbol  *string
}

func (OrderListStatusDecoder) EncodedLen() int { return 0 }

func OrderListStatusDecoderDecode(buf []byte) (OrderListStatusDecoder, error) {
	if len(buf) < 8 {
		return OrderListStatusDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderListStatusDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 39 {
		return OrderListStatusDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	list_id := readString(buf, &pos)
	statusRaw := buf[pos]
	pos++
	status, err := OrderListStatusStatusFromValue(statusRaw)
	if err != nil {
		return OrderListStatusDecoder{}, err
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
		symbol = nil
	}

	return OrderListStatusDecoder{
		Account: account,
		ListId:  list_id,
		Status:  status,
		Symbol:  symbol,
	}, nil
}

// SBE encoder for FillHistoryRequest
type FillHistoryRequestEncoder struct{}

func (FillHistoryRequestEncoder) Encode(Account string, Symbol *string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 40)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for FillHistoryRequest
type FillHistoryRequestDecoder struct {
	Account   string
	Symbol    *string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (FillHistoryRequestDecoder) EncodedLen() int { return 0 }

func FillHistoryRequestDecoderDecode(buf []byte) (FillHistoryRequestDecoder, error) {
	if len(buf) < 8 {
		return FillHistoryRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FillHistoryRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 40 {
		return FillHistoryRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
		symbol = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return FillHistoryRequestDecoder{
		Account:   account,
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for FillHistoryBatch
type FillHistoryBatchEncoder struct{}

func (FillHistoryBatchEncoder) Encode(Account string, Fills []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 41)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Fills)))
	for _, item := range Fills {
		ExecutionReportEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for FillHistoryBatch
type FillHistoryBatchDecoder struct {
	Account    string
	Fills      []string
	HasMore    uint8
	NextCursor *string
}

func (FillHistoryBatchDecoder) EncodedLen() int { return 0 }

func FillHistoryBatchDecoderDecode(buf []byte) (FillHistoryBatchDecoder, error) {
	if len(buf) < 8 {
		return FillHistoryBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FillHistoryBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 41 {
		return FillHistoryBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	fillsCount := int(readU32BE(buf, &pos))
	fills := make([]ExecutionReport, 0, fillsCount)
	for i := 0; i < fillsCount; i++ {
		item, err := ExecutionReportDecoder{}.Decode(buf[pos:])
		if err != nil {
			return FillHistoryBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		fills = append(fills, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return FillHistoryBatchDecoder{
		Account:    account,
		Fills:      fills,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for FundingPayment
type FundingPaymentEncoder struct{}

func (FundingPaymentEncoder) Encode(Account string, Symbol *string, Amount float64, Rate float64, Timestamp int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 42)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 24)

	// Fixed fields
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	writeF64BE(&buf, Amount)
	writeF64BE(&buf, Rate)
	writeI64BE(&buf, Timestamp)

	return buf, nil
}

// SBE decoder for FundingPayment
type FundingPaymentDecoder struct {
	Account   string
	Symbol    *string
	Amount    float64
	Rate      float64
	Timestamp int64
}

func (FundingPaymentDecoder) EncodedLen() int { return 0 }

func FundingPaymentDecoderDecode(buf []byte) (FundingPaymentDecoder, error) {
	if len(buf) < 8 {
		return FundingPaymentDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FundingPaymentDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 42 {
		return FundingPaymentDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
		symbol = nil
	}
	amount := readF64BE(buf, &pos)
	rate := readF64BE(buf, &pos)
	timestamp := readI64BE(buf, &pos)

	return FundingPaymentDecoder{
		Account:   account,
		Symbol:    symbol,
		Amount:    amount,
		Rate:      rate,
		Timestamp: timestamp,
	}, nil
}

// SBE encoder for FundingHistoryRequest
type FundingHistoryRequestEncoder struct{}

func (FundingHistoryRequestEncoder) Encode(Account string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 43)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Account)
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for FundingHistoryRequest
type FundingHistoryRequestDecoder struct {
	Account   string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (FundingHistoryRequestDecoder) EncodedLen() int { return 0 }

func FundingHistoryRequestDecoderDecode(buf []byte) (FundingHistoryRequestDecoder, error) {
	if len(buf) < 8 {
		return FundingHistoryRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FundingHistoryRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 43 {
		return FundingHistoryRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return FundingHistoryRequestDecoder{
		Account:   account,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for FundingHistoryBatch
type FundingHistoryBatchEncoder struct{}

func (FundingHistoryBatchEncoder) Encode(Account string, Payments []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 44)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Payments)))
	for _, item := range Payments {
		FundingPaymentEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for FundingHistoryBatch
type FundingHistoryBatchDecoder struct {
	Account    string
	Payments   []string
	HasMore    uint8
	NextCursor *string
}

func (FundingHistoryBatchDecoder) EncodedLen() int { return 0 }

func FundingHistoryBatchDecoderDecode(buf []byte) (FundingHistoryBatchDecoder, error) {
	if len(buf) < 8 {
		return FundingHistoryBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FundingHistoryBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 44 {
		return FundingHistoryBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	paymentsCount := int(readU32BE(buf, &pos))
	payments := make([]FundingPayment, 0, paymentsCount)
	for i := 0; i < paymentsCount; i++ {
		item, err := FundingPaymentDecoder{}.Decode(buf[pos:])
		if err != nil {
			return FundingHistoryBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		payments = append(payments, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return FundingHistoryBatchDecoder{
		Account:    account,
		Payments:   payments,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for LedgerUpdate
type LedgerUpdateEncoder struct{}

func (LedgerUpdateEncoder) Encode(Account string, Asset string, Delta float64, Kind LedgerUpdateKind, Timestamp int64, ReferenceId *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 45)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)

	// Fixed fields
	writeString(&buf, Account)
	writeString(&buf, Asset)
	writeF64BE(&buf, Delta)
	buf = append(buf, byte(Kind.ToValue()))
	writeI64BE(&buf, Timestamp)
	if ReferenceId != nil {
		buf = append(buf, 1)
		writeString(&buf, *ReferenceId)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for LedgerUpdate
type LedgerUpdateDecoder struct {
	Account     string
	Asset       string
	Delta       float64
	Kind        LedgerUpdateKind
	Timestamp   int64
	ReferenceId *string
}

func (LedgerUpdateDecoder) EncodedLen() int { return 0 }

func LedgerUpdateDecoderDecode(buf []byte) (LedgerUpdateDecoder, error) {
	if len(buf) < 8 {
		return LedgerUpdateDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LedgerUpdateDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 45 {
		return LedgerUpdateDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	asset := readString(buf, &pos)
	delta := readF64BE(buf, &pos)
	kindRaw := buf[pos]
	pos++
	kind, err := LedgerUpdateKindFromValue(kindRaw)
	if err != nil {
		return LedgerUpdateDecoder{}, err
	}
	timestamp := readI64BE(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		reference_id = &s
	} else {
		pos++
		reference_id = nil
	}

	return LedgerUpdateDecoder{
		Account:     account,
		Asset:       asset,
		Delta:       delta,
		Kind:        kind,
		Timestamp:   timestamp,
		ReferenceId: reference_id,
	}, nil
}

// SBE encoder for LedgerHistoryRequest
type LedgerHistoryRequestEncoder struct{}

func (LedgerHistoryRequestEncoder) Encode(Account string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 46)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Account)
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for LedgerHistoryRequest
type LedgerHistoryRequestDecoder struct {
	Account   string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (LedgerHistoryRequestDecoder) EncodedLen() int { return 0 }

func LedgerHistoryRequestDecoderDecode(buf []byte) (LedgerHistoryRequestDecoder, error) {
	if len(buf) < 8 {
		return LedgerHistoryRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LedgerHistoryRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 46 {
		return LedgerHistoryRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return LedgerHistoryRequestDecoder{
		Account:   account,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for LedgerHistoryBatch
type LedgerHistoryBatchEncoder struct{}

func (LedgerHistoryBatchEncoder) Encode(Account string, Entries []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 47)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Entries)))
	for _, item := range Entries {
		LedgerUpdateEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for LedgerHistoryBatch
type LedgerHistoryBatchDecoder struct {
	Account    string
	Entries    []string
	HasMore    uint8
	NextCursor *string
}

func (LedgerHistoryBatchDecoder) EncodedLen() int { return 0 }

func LedgerHistoryBatchDecoderDecode(buf []byte) (LedgerHistoryBatchDecoder, error) {
	if len(buf) < 8 {
		return LedgerHistoryBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LedgerHistoryBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 47 {
		return LedgerHistoryBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	entriesCount := int(readU32BE(buf, &pos))
	entries := make([]LedgerUpdate, 0, entriesCount)
	for i := 0; i < entriesCount; i++ {
		item, err := LedgerUpdateDecoder{}.Decode(buf[pos:])
		if err != nil {
			return LedgerHistoryBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		entries = append(entries, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return LedgerHistoryBatchDecoder{
		Account:    account,
		Entries:    entries,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for OpenOrdersRequest
type OpenOrdersRequestEncoder struct{}

func (OpenOrdersRequestEncoder) Encode(Account string, Symbol *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 48)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OpenOrdersRequest
type OpenOrdersRequestDecoder struct {
	Account string
	Symbol  *string
}

func (OpenOrdersRequestDecoder) EncodedLen() int { return 0 }

func OpenOrdersRequestDecoderDecode(buf []byte) (OpenOrdersRequestDecoder, error) {
	if len(buf) < 8 {
		return OpenOrdersRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OpenOrdersRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 48 {
		return OpenOrdersRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
		symbol = nil
	}

	return OpenOrdersRequestDecoder{
		Account: account,
		Symbol:  symbol,
	}, nil
}

// SBE encoder for OpenOrdersSnapshot
type OpenOrdersSnapshotEncoder struct{}

func (OpenOrdersSnapshotEncoder) Encode(Account string, Orders []string, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 49)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Orders)))
	for _, item := range Orders {
		ExecutionReportEncoder{}.Encode(item, &buf)
	}
	v := uint8(0)
	if IsSnapshot != nil {
		v = *IsSnapshot
	}
	buf = append(buf, v)
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OpenOrdersSnapshot
type OpenOrdersSnapshotDecoder struct {
	Account    string
	Orders     []string
	IsSnapshot *uint8
}

func (OpenOrdersSnapshotDecoder) EncodedLen() int { return 0 }

func OpenOrdersSnapshotDecoderDecode(buf []byte) (OpenOrdersSnapshotDecoder, error) {
	if len(buf) < 8 {
		return OpenOrdersSnapshotDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OpenOrdersSnapshotDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 49 {
		return OpenOrdersSnapshotDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	ordersCount := int(readU32BE(buf, &pos))
	orders := make([]ExecutionReport, 0, ordersCount)
	for i := 0; i < ordersCount; i++ {
		item, err := ExecutionReportDecoder{}.Decode(buf[pos:])
		if err != nil {
			return OpenOrdersSnapshotDecoder{}, err
		}
		pos += item.EncodedLen()
		orders = append(orders, item)
	}
	v := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		is_snapshot = &v
	} else {
		pos++
		is_snapshot = nil
	}

	return OpenOrdersSnapshotDecoder{
		Account:    account,
		Orders:     orders,
		IsSnapshot: is_snapshot,
	}, nil
}

// SBE encoder for OrderHistoryRequest
type OrderHistoryRequestEncoder struct{}

func (OrderHistoryRequestEncoder) Encode(Account string, Symbol *string, StartTime *int64, EndTime *int64, Limit *u32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 50)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)

	// Fixed fields
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	if StartTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *StartTime)
	} else {
		buf = append(buf, 0)
	}
	if EndTime != nil {
		buf = append(buf, 1)
		writeI64BE(&buf, *EndTime)
	} else {
		buf = append(buf, 0)
	}
	// TODO: encode limit as u32
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OrderHistoryRequest
type OrderHistoryRequestDecoder struct {
	Account   string
	Symbol    *string
	StartTime *int64
	EndTime   *int64
	Limit     *u32
	Cursor    *string
}

func (OrderHistoryRequestDecoder) EncodedLen() int { return 0 }

func OrderHistoryRequestDecoderDecode(buf []byte) (OrderHistoryRequestDecoder, error) {
	if len(buf) < 8 {
		return OrderHistoryRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderHistoryRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 50 {
		return OrderHistoryRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
		symbol = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		start_time = &v
	} else {
		pos++
		start_time = nil
	}
	if buf[pos] == 1 {
		pos++
		v := readI64BE(buf, &pos)
		end_time = &v
	} else {
		pos++
		end_time = nil
	}
	// TODO: decode limit
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
		cursor = nil
	}

	return OrderHistoryRequestDecoder{
		Account:   account,
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}, nil
}

// SBE encoder for OrderHistoryBatch
type OrderHistoryBatchEncoder struct{}

func (OrderHistoryBatchEncoder) Encode(Account string, Orders []string, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 51)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)

	// Fixed fields
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Orders)))
	for _, item := range Orders {
		ExecutionReportEncoder{}.Encode(item, &buf)
	}
	buf = append(buf, HasMore)
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}

	return buf, nil
}

// SBE decoder for OrderHistoryBatch
type OrderHistoryBatchDecoder struct {
	Account    string
	Orders     []string
	HasMore    uint8
	NextCursor *string
}

func (OrderHistoryBatchDecoder) EncodedLen() int { return 0 }

func OrderHistoryBatchDecoderDecode(buf []byte) (OrderHistoryBatchDecoder, error) {
	if len(buf) < 8 {
		return OrderHistoryBatchDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderHistoryBatchDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 51 {
		return OrderHistoryBatchDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	ordersCount := int(readU32BE(buf, &pos))
	orders := make([]ExecutionReport, 0, ordersCount)
	for i := 0; i < ordersCount; i++ {
		item, err := ExecutionReportDecoder{}.Decode(buf[pos:])
		if err != nil {
			return OrderHistoryBatchDecoder{}, err
		}
		pos += item.EncodedLen()
		orders = append(orders, item)
	}
	has_more := buf[pos]
	pos++
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
		next_cursor = nil
	}

	return OrderHistoryBatchDecoder{
		Account:    account,
		Orders:     orders,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}, nil
}

// SBE encoder for CapabilitiesRequest
type CapabilitiesRequestEncoder struct{}

func (CapabilitiesRequestEncoder) Encode() ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 52)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields

	return buf, nil
}

// SBE decoder for CapabilitiesRequest
type CapabilitiesRequestDecoder struct {
}

func (CapabilitiesRequestDecoder) EncodedLen() int { return 0 }

func CapabilitiesRequestDecoderDecode(buf []byte) (CapabilitiesRequestDecoder, error) {
	if len(buf) < 8 {
		return CapabilitiesRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CapabilitiesRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 52 {
		return CapabilitiesRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}

	return CapabilitiesRequestDecoder{}, nil
}

// SBE encoder for CapabilitiesResponse
type CapabilitiesResponseEncoder struct{}

func (CapabilitiesResponseEncoder) Encode(SchemaIds []string, Paths []string, Symbols []string, Intervals []string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 53)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeU32BE(&buf, uint32(len(SchemaIds)))
	for _, item := range SchemaIds {
		u8Encoder{}.Encode(item, &buf)
	}
	writeU32BE(&buf, uint32(len(Paths)))
	for _, item := range Paths {
		CapabilityPathEncoder{}.Encode(item, &buf)
	}
	writeU32BE(&buf, uint32(len(Symbols)))
	for _, item := range Symbols {
		SymbolEncoder{}.Encode(item, &buf)
	}
	writeU32BE(&buf, uint32(len(Intervals)))
	for _, item := range Intervals {
		CandleIntervalEncoder{}.Encode(item, &buf)
	}

	return buf, nil
}

// SBE decoder for CapabilitiesResponse
type CapabilitiesResponseDecoder struct {
	SchemaIds []string
	Paths     []string
	Symbols   []string
	Intervals []string
}

func (CapabilitiesResponseDecoder) EncodedLen() int { return 0 }

func CapabilitiesResponseDecoderDecode(buf []byte) (CapabilitiesResponseDecoder, error) {
	if len(buf) < 8 {
		return CapabilitiesResponseDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CapabilitiesResponseDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 53 {
		return CapabilitiesResponseDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	schema_idsCount := int(readU32BE(buf, &pos))
	schema_ids := make([]u8, 0, schema_idsCount)
	for i := 0; i < schema_idsCount; i++ {
		item, err := u8Decoder{}.Decode(buf[pos:])
		if err != nil {
			return CapabilitiesResponseDecoder{}, err
		}
		pos += item.EncodedLen()
		schema_ids = append(schema_ids, item)
	}
	pathsCount := int(readU32BE(buf, &pos))
	paths := make([]CapabilityPath, 0, pathsCount)
	for i := 0; i < pathsCount; i++ {
		item, err := CapabilityPathDecoder{}.Decode(buf[pos:])
		if err != nil {
			return CapabilitiesResponseDecoder{}, err
		}
		pos += item.EncodedLen()
		paths = append(paths, item)
	}
	symbolsCount := int(readU32BE(buf, &pos))
	symbols := make([]Symbol, 0, symbolsCount)
	for i := 0; i < symbolsCount; i++ {
		item, err := SymbolDecoder{}.Decode(buf[pos:])
		if err != nil {
			return CapabilitiesResponseDecoder{}, err
		}
		pos += item.EncodedLen()
		symbols = append(symbols, item)
	}
	intervalsCount := int(readU32BE(buf, &pos))
	intervals := make([]CandleInterval, 0, intervalsCount)
	for i := 0; i < intervalsCount; i++ {
		item, err := CandleIntervalDecoder{}.Decode(buf[pos:])
		if err != nil {
			return CapabilitiesResponseDecoder{}, err
		}
		pos += item.EncodedLen()
		intervals = append(intervals, item)
	}

	return CapabilitiesResponseDecoder{
		SchemaIds: schema_ids,
		Paths:     paths,
		Symbols:   symbols,
		Intervals: intervals,
	}, nil
}

// SBE encoder for PositionRequest
type PositionRequestEncoder struct{}

func (PositionRequestEncoder) Encode(Account string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	// SBE Message Header (8 bytes)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 54)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)

	// Fixed fields
	writeString(&buf, Account)

	return buf, nil
}

// SBE decoder for PositionRequest
type PositionRequestDecoder struct {
	Account string
}

func (PositionRequestDecoder) EncodedLen() int { return 0 }

func PositionRequestDecoderDecode(buf []byte) (PositionRequestDecoder, error) {
	if len(buf) < 8 {
		return PositionRequestDecoder{}, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PositionRequestDecoder{}, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 54 {
		return PositionRequestDecoder{}, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)

	return PositionRequestDecoder{
		Account: account,
	}, nil
}
