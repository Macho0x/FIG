// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' v1.0.0
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

func writeStringU32(buf *[]byte, s string) {
	writeU32BE(buf, uint32(len(s)))
	*buf = append(*buf, []byte(s)...)
}

func readStringU32(buf []byte, pos *int) string {
	n := int(readU32BE(buf, pos))
	s := string(buf[*pos : *pos+n])
	*pos += n
	return s
}

func readU8(buf []byte, pos *int) uint8 {
	v := buf[*pos]
	*pos = *pos + 1
	return v
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

type TradingSessionStatusStatus uint8

const (
	TradingSessionStatusStatusPreOpen TradingSessionStatusStatus = 1
	TradingSessionStatusStatusOpen    TradingSessionStatusStatus = 2
	TradingSessionStatusStatusBreak   TradingSessionStatusStatus = 3
	TradingSessionStatusStatusClose   TradingSessionStatusStatus = 4
	TradingSessionStatusStatusHalt    TradingSessionStatusStatus = 5
	TradingSessionStatusStatusAuction TradingSessionStatusStatus = 6
)

func TradingSessionStatusStatusFromValue(v uint8) (TradingSessionStatusStatus, error) {
	switch v {
	case 1:
		return TradingSessionStatusStatusPreOpen, nil
	case 2:
		return TradingSessionStatusStatusOpen, nil
	case 3:
		return TradingSessionStatusStatusBreak, nil
	case 4:
		return TradingSessionStatusStatusClose, nil
	case 5:
		return TradingSessionStatusStatusHalt, nil
	case 6:
		return TradingSessionStatusStatusAuction, nil
	default:
		return 0, fmt.Errorf("invalid TradingSessionStatusStatus value: %d", v)
	}
}

func (e TradingSessionStatusStatus) ToValue() uint8 { return uint8(e) }

type TradingSessionStatusHaltReason uint8

const (
	TradingSessionStatusHaltReasonNone           TradingSessionStatusHaltReason = 1
	TradingSessionStatusHaltReasonNews           TradingSessionStatusHaltReason = 2
	TradingSessionStatusHaltReasonMatchingEngine TradingSessionStatusHaltReason = 3
	TradingSessionStatusHaltReasonRegulatory     TradingSessionStatusHaltReason = 4
	TradingSessionStatusHaltReasonAuction        TradingSessionStatusHaltReason = 5
	TradingSessionStatusHaltReasonOther          TradingSessionStatusHaltReason = 6
)

func TradingSessionStatusHaltReasonFromValue(v uint8) (TradingSessionStatusHaltReason, error) {
	switch v {
	case 1:
		return TradingSessionStatusHaltReasonNone, nil
	case 2:
		return TradingSessionStatusHaltReasonNews, nil
	case 3:
		return TradingSessionStatusHaltReasonMatchingEngine, nil
	case 4:
		return TradingSessionStatusHaltReasonRegulatory, nil
	case 5:
		return TradingSessionStatusHaltReasonAuction, nil
	case 6:
		return TradingSessionStatusHaltReasonOther, nil
	default:
		return 0, fmt.Errorf("invalid TradingSessionStatusHaltReason value: %d", v)
	}
}

func (e TradingSessionStatusHaltReason) ToValue() uint8 { return uint8(e) }

type SecurityStatusStatus uint8

const (
	SecurityStatusStatusTrading   SecurityStatusStatus = 1
	SecurityStatusStatusHalt      SecurityStatusStatus = 2
	SecurityStatusStatusAuction   SecurityStatusStatus = 3
	SecurityStatusStatusSuspended SecurityStatusStatus = 4
)

func SecurityStatusStatusFromValue(v uint8) (SecurityStatusStatus, error) {
	switch v {
	case 1:
		return SecurityStatusStatusTrading, nil
	case 2:
		return SecurityStatusStatusHalt, nil
	case 3:
		return SecurityStatusStatusAuction, nil
	case 4:
		return SecurityStatusStatusSuspended, nil
	default:
		return 0, fmt.Errorf("invalid SecurityStatusStatus value: %d", v)
	}
}

func (e SecurityStatusStatus) ToValue() uint8 { return uint8(e) }

type SecurityStatusHaltReason uint8

const (
	SecurityStatusHaltReasonNone           SecurityStatusHaltReason = 1
	SecurityStatusHaltReasonNews           SecurityStatusHaltReason = 2
	SecurityStatusHaltReasonMatchingEngine SecurityStatusHaltReason = 3
	SecurityStatusHaltReasonRegulatory     SecurityStatusHaltReason = 4
	SecurityStatusHaltReasonAuction        SecurityStatusHaltReason = 5
	SecurityStatusHaltReasonOther          SecurityStatusHaltReason = 6
)

func SecurityStatusHaltReasonFromValue(v uint8) (SecurityStatusHaltReason, error) {
	switch v {
	case 1:
		return SecurityStatusHaltReasonNone, nil
	case 2:
		return SecurityStatusHaltReasonNews, nil
	case 3:
		return SecurityStatusHaltReasonMatchingEngine, nil
	case 4:
		return SecurityStatusHaltReasonRegulatory, nil
	case 5:
		return SecurityStatusHaltReasonAuction, nil
	case 6:
		return SecurityStatusHaltReasonOther, nil
	default:
		return 0, fmt.Errorf("invalid SecurityStatusHaltReason value: %d", v)
	}
}

func (e SecurityStatusHaltReason) ToValue() uint8 { return uint8(e) }

type MassCancelRequestSide uint8

const (
	MassCancelRequestSideBuy             MassCancelRequestSide = 1
	MassCancelRequestSideSell            MassCancelRequestSide = 2
	MassCancelRequestSideSellShort       MassCancelRequestSide = 3
	MassCancelRequestSideSellShortExempt MassCancelRequestSide = 4
)

func MassCancelRequestSideFromValue(v uint8) (MassCancelRequestSide, error) {
	switch v {
	case 1:
		return MassCancelRequestSideBuy, nil
	case 2:
		return MassCancelRequestSideSell, nil
	case 3:
		return MassCancelRequestSideSellShort, nil
	case 4:
		return MassCancelRequestSideSellShortExempt, nil
	default:
		return 0, fmt.Errorf("invalid MassCancelRequestSide value: %d", v)
	}
}

func (e MassCancelRequestSide) ToValue() uint8 { return uint8(e) }

type MassCancelReportStatus uint8

const (
	MassCancelReportStatusAccepted  MassCancelReportStatus = 1
	MassCancelReportStatusCompleted MassCancelReportStatus = 2
	MassCancelReportStatusRejected  MassCancelReportStatus = 3
)

func MassCancelReportStatusFromValue(v uint8) (MassCancelReportStatus, error) {
	switch v {
	case 1:
		return MassCancelReportStatusAccepted, nil
	case 2:
		return MassCancelReportStatusCompleted, nil
	case 3:
		return MassCancelReportStatusRejected, nil
	default:
		return 0, fmt.Errorf("invalid MassCancelReportStatus value: %d", v)
	}
}

func (e MassCancelReportStatus) ToValue() uint8 { return uint8(e) }

type BatchReportOutcome uint8

const (
	BatchReportOutcomeApplied  BatchReportOutcome = 1
	BatchReportOutcomeRejected BatchReportOutcome = 2
)

func BatchReportOutcomeFromValue(v uint8) (BatchReportOutcome, error) {
	switch v {
	case 1:
		return BatchReportOutcomeApplied, nil
	case 2:
		return BatchReportOutcomeRejected, nil
	default:
		return 0, fmt.Errorf("invalid BatchReportOutcome value: %d", v)
	}
}

func (e BatchReportOutcome) ToValue() uint8 { return uint8(e) }

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

type BatchActionKind uint8

const (
	BatchActionKindNewOrder BatchActionKind = 1
	BatchActionKindCancel   BatchActionKind = 2
	BatchActionKindReplace  BatchActionKind = 3
)

func BatchActionKindFromValue(v uint8) (BatchActionKind, error) {
	switch v {
	case 1:
		return BatchActionKindNewOrder, nil
	case 2:
		return BatchActionKindCancel, nil
	case 3:
		return BatchActionKindReplace, nil
	default:
		return 0, fmt.Errorf("invalid BatchActionKind value: %d", v)
	}
}

func (e BatchActionKind) ToValue() uint8 { return uint8(e) }

type BatchResultKind uint8

const (
	BatchResultKindNewOrder BatchResultKind = 1
	BatchResultKindCancel   BatchResultKind = 2
	BatchResultKindReplace  BatchResultKind = 3
)

func BatchResultKindFromValue(v uint8) (BatchResultKind, error) {
	switch v {
	case 1:
		return BatchResultKindNewOrder, nil
	case 2:
		return BatchResultKindCancel, nil
	case 3:
		return BatchResultKindReplace, nil
	default:
		return 0, fmt.Errorf("invalid BatchResultKind value: %d", v)
	}
}

func (e BatchResultKind) ToValue() uint8 { return uint8(e) }

type BatchResultStatus uint8

const (
	BatchResultStatusApplied  BatchResultStatus = 1
	BatchResultStatusRejected BatchResultStatus = 2
)

func BatchResultStatusFromValue(v uint8) (BatchResultStatus, error) {
	switch v {
	case 1:
		return BatchResultStatusApplied, nil
	case 2:
		return BatchResultStatusRejected, nil
	default:
		return 0, fmt.Errorf("invalid BatchResultStatus value: %d", v)
	}
}

func (e BatchResultStatus) ToValue() uint8 { return uint8(e) }

// SBE struct AggregateTrade
type AggregateTrade struct {
	Symbol       string
	AggTradeId   string
	Price        float64
	Qty          float64
	Side         AggregateTradeSide
	FirstTradeId string
	LastTradeId  string
	Timestamp    int64
}

// Encode writes AggregateTrade fields (no header) into buf.
type AggregateTradeEncoder struct{}

func (AggregateTradeEncoder) Encode(value AggregateTrade, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.AggTradeId)
	writeF64BE(&local, value.Price)
	writeF64BE(&local, value.Qty)
	local = append(local, byte(value.Side.ToValue()))
	writeString(&local, value.FirstTradeId)
	writeString(&local, value.LastTradeId)
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// DecodeAggregateTradeDecoder reads AggregateTrade fields starting at pos and returns the new offset.
func AggregateTradeDecoderDecode(buf []byte, pos int) (AggregateTrade, int, error) {
	symbol := readString(buf, &pos)
	agg_trade_id := readString(buf, &pos)
	price := readF64BE(buf, &pos)
	qty := readF64BE(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := AggregateTradeSideFromValue(side_raw)
	if side_err != nil {
		return AggregateTrade{}, pos, side_err
	}
	first_trade_id := readString(buf, &pos)
	last_trade_id := readString(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	value := AggregateTrade{
		Symbol:       symbol,
		AggTradeId:   agg_trade_id,
		Price:        price,
		Qty:          qty,
		Side:         side,
		FirstTradeId: first_trade_id,
		LastTradeId:  last_trade_id,
		Timestamp:    timestamp,
	}
	return value, pos, nil
}

// SBE struct LiquidationTrade
type LiquidationTrade struct {
	Symbol    string
	Side      LiquidationTradeSide
	Price     float64
	Qty       float64
	Timestamp int64
}

// Encode writes LiquidationTrade fields (no header) into buf.
type LiquidationTradeEncoder struct{}

func (LiquidationTradeEncoder) Encode(value LiquidationTrade, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	local = append(local, byte(value.Side.ToValue()))
	writeF64BE(&local, value.Price)
	writeF64BE(&local, value.Qty)
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// DecodeLiquidationTradeDecoder reads LiquidationTrade fields starting at pos and returns the new offset.
func LiquidationTradeDecoderDecode(buf []byte, pos int) (LiquidationTrade, int, error) {
	symbol := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := LiquidationTradeSideFromValue(side_raw)
	if side_err != nil {
		return LiquidationTrade{}, pos, side_err
	}
	price := readF64BE(buf, &pos)
	qty := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	value := LiquidationTrade{
		Symbol:    symbol,
		Side:      side,
		Price:     price,
		Qty:       qty,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE struct PriceLevel
type PriceLevel struct {
	Price      float64
	Qty        float64
	OrderCount *uint32
}

// Encode writes PriceLevel fields (no header) into buf.
type PriceLevelEncoder struct{}

func (PriceLevelEncoder) Encode(value PriceLevel, buf *[]byte) {
	local := *buf
	writeF64BE(&local, value.Price)
	writeF64BE(&local, value.Qty)
	var value_OrderCount_v uint32
	if value.OrderCount != nil {
		value_OrderCount_v = *value.OrderCount
	}
	writeU32BE(&local, uint32(value_OrderCount_v))
	if value.OrderCount != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// DecodePriceLevelDecoder reads PriceLevel fields starting at pos and returns the new offset.
func PriceLevelDecoderDecode(buf []byte, pos int) (PriceLevel, int, error) {
	price := readF64BE(buf, &pos)
	qty := readF64BE(buf, &pos)
	order_count_v := readU32BE(buf, &pos)
	var order_count *uint32
	if buf[pos] == 1 {
		pos++
		order_count = &order_count_v
	} else {
		pos++
		order_count = nil
	}
	value := PriceLevel{
		Price:      price,
		Qty:        qty,
		OrderCount: order_count,
	}
	return value, pos, nil
}

// SBE struct MarketDataUpdate
type MarketDataUpdate struct {
	Side   MarketDataUpdateSide
	Action MarketDataUpdateAction
	Price  float64
	Qty    float64
}

// Encode writes MarketDataUpdate fields (no header) into buf.
type MarketDataUpdateEncoder struct{}

func (MarketDataUpdateEncoder) Encode(value MarketDataUpdate, buf *[]byte) {
	local := *buf
	local = append(local, byte(value.Side.ToValue()))
	local = append(local, byte(value.Action.ToValue()))
	writeF64BE(&local, value.Price)
	writeF64BE(&local, value.Qty)
	*buf = local
}

// DecodeMarketDataUpdateDecoder reads MarketDataUpdate fields starting at pos and returns the new offset.
func MarketDataUpdateDecoderDecode(buf []byte, pos int) (MarketDataUpdate, int, error) {
	side_raw := readU8(buf, &pos)
	side, side_err := MarketDataUpdateSideFromValue(side_raw)
	if side_err != nil {
		return MarketDataUpdate{}, pos, side_err
	}
	action_raw := readU8(buf, &pos)
	action, action_err := MarketDataUpdateActionFromValue(action_raw)
	if action_err != nil {
		return MarketDataUpdate{}, pos, action_err
	}
	price := readF64BE(buf, &pos)
	qty := readF64BE(buf, &pos)
	value := MarketDataUpdate{
		Side:   side,
		Action: action,
		Price:  price,
		Qty:    qty,
	}
	return value, pos, nil
}

// SBE struct CandleBar
type CandleBar struct {
	Symbol     string
	Interval   string
	Open       float64
	High       float64
	Low        float64
	Close      float64
	Volume     float64
	BarStart   int64
	BarEnd     int64
	IsFinal    uint8
	IsSnapshot *uint8
}

// Encode writes CandleBar fields (no header) into buf.
type CandleBarEncoder struct{}

func (CandleBarEncoder) Encode(value CandleBar, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.Interval)
	writeF64BE(&local, value.Open)
	writeF64BE(&local, value.High)
	writeF64BE(&local, value.Low)
	writeF64BE(&local, value.Close)
	writeF64BE(&local, value.Volume)
	writeU64BE(&local, uint64(value.BarStart))
	writeU64BE(&local, uint64(value.BarEnd))
	local = append(local, uint8(value.IsFinal))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// DecodeCandleBarDecoder reads CandleBar fields starting at pos and returns the new offset.
func CandleBarDecoderDecode(buf []byte, pos int) (CandleBar, int, error) {
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	open := readF64BE(buf, &pos)
	high := readF64BE(buf, &pos)
	low := readF64BE(buf, &pos)
	close := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	bar_start := int64(readU64BE(buf, &pos))
	bar_end := int64(readU64BE(buf, &pos))
	is_final := readU8(buf, &pos)
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := CandleBar{
		Symbol:     symbol,
		Interval:   interval,
		Open:       open,
		High:       high,
		Low:        low,
		Close:      close,
		Volume:     volume,
		BarStart:   bar_start,
		BarEnd:     bar_end,
		IsFinal:    is_final,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE struct PublicTrade
type PublicTrade struct {
	Symbol    string
	TradeId   string
	Price     float64
	Qty       float64
	Side      PublicTradeSide
	Timestamp int64
}

// Encode writes PublicTrade fields (no header) into buf.
type PublicTradeEncoder struct{}

func (PublicTradeEncoder) Encode(value PublicTrade, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.TradeId)
	writeF64BE(&local, value.Price)
	writeF64BE(&local, value.Qty)
	local = append(local, byte(value.Side.ToValue()))
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// DecodePublicTradeDecoder reads PublicTrade fields starting at pos and returns the new offset.
func PublicTradeDecoderDecode(buf []byte, pos int) (PublicTrade, int, error) {
	symbol := readString(buf, &pos)
	trade_id := readString(buf, &pos)
	price := readF64BE(buf, &pos)
	qty := readF64BE(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := PublicTradeSideFromValue(side_raw)
	if side_err != nil {
		return PublicTrade{}, pos, side_err
	}
	timestamp := int64(readU64BE(buf, &pos))
	value := PublicTrade{
		Symbol:    symbol,
		TradeId:   trade_id,
		Price:     price,
		Qty:       qty,
		Side:      side,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE struct InstrumentMetadata
type InstrumentMetadata struct {
	InstrumentId string
	Symbol       string
	ProductKind  string
	MarginAsset  *string
	DisplayName  *string
	TickSize     *float64
	LotSize      *float64
}

// Encode writes InstrumentMetadata fields (no header) into buf.
type InstrumentMetadataEncoder struct{}

func (InstrumentMetadataEncoder) Encode(value InstrumentMetadata, buf *[]byte) {
	local := *buf
	writeString(&local, value.InstrumentId)
	writeString(&local, value.Symbol)
	writeString(&local, value.ProductKind)
	if value.MarginAsset != nil {
		local = append(local, 1)
		writeString(&local, *value.MarginAsset)
	} else {
		local = append(local, 0)
	}
	if value.DisplayName != nil {
		local = append(local, 1)
		writeString(&local, *value.DisplayName)
	} else {
		local = append(local, 0)
	}
	var value_TickSize_v float64
	if value.TickSize != nil {
		value_TickSize_v = *value.TickSize
	}
	writeF64BE(&local, value_TickSize_v)
	if value.TickSize != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_LotSize_v float64
	if value.LotSize != nil {
		value_LotSize_v = *value.LotSize
	}
	writeF64BE(&local, value_LotSize_v)
	if value.LotSize != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// DecodeInstrumentMetadataDecoder reads InstrumentMetadata fields starting at pos and returns the new offset.
func InstrumentMetadataDecoderDecode(buf []byte, pos int) (InstrumentMetadata, int, error) {
	var margin_asset *string
	var display_name *string
	instrument_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	product_kind := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		margin_asset = &s
	} else {
		pos++
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		display_name = &s
	} else {
		pos++
	}
	tick_size_v := readF64BE(buf, &pos)
	var tick_size *float64
	if buf[pos] == 1 {
		pos++
		tick_size = &tick_size_v
	} else {
		pos++
		tick_size = nil
	}
	lot_size_v := readF64BE(buf, &pos)
	var lot_size *float64
	if buf[pos] == 1 {
		pos++
		lot_size = &lot_size_v
	} else {
		pos++
		lot_size = nil
	}
	value := InstrumentMetadata{
		InstrumentId: instrument_id,
		Symbol:       symbol,
		ProductKind:  product_kind,
		MarginAsset:  margin_asset,
		DisplayName:  display_name,
		TickSize:     tick_size,
		LotSize:      lot_size,
	}
	return value, pos, nil
}

// SBE struct BalanceEntry
type BalanceEntry struct {
	Asset     string
	Total     float64
	Available float64
	Hold      float64
}

// Encode writes BalanceEntry fields (no header) into buf.
type BalanceEntryEncoder struct{}

func (BalanceEntryEncoder) Encode(value BalanceEntry, buf *[]byte) {
	local := *buf
	writeString(&local, value.Asset)
	writeF64BE(&local, value.Total)
	writeF64BE(&local, value.Available)
	writeF64BE(&local, value.Hold)
	*buf = local
}

// DecodeBalanceEntryDecoder reads BalanceEntry fields starting at pos and returns the new offset.
func BalanceEntryDecoderDecode(buf []byte, pos int) (BalanceEntry, int, error) {
	asset := readString(buf, &pos)
	total := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	hold := readF64BE(buf, &pos)
	value := BalanceEntry{
		Asset:     asset,
		Total:     total,
		Available: available,
		Hold:      hold,
	}
	return value, pos, nil
}

// SBE struct PositionEntry
type PositionEntry struct {
	Symbol        string
	Qty           float64
	EntryPrice    float64
	UnrealizedPnl float64
}

// Encode writes PositionEntry fields (no header) into buf.
type PositionEntryEncoder struct{}

func (PositionEntryEncoder) Encode(value PositionEntry, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeF64BE(&local, value.Qty)
	writeF64BE(&local, value.EntryPrice)
	writeF64BE(&local, value.UnrealizedPnl)
	*buf = local
}

// DecodePositionEntryDecoder reads PositionEntry fields starting at pos and returns the new offset.
func PositionEntryDecoderDecode(buf []byte, pos int) (PositionEntry, int, error) {
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	entry_price := readF64BE(buf, &pos)
	unrealized_pnl := readF64BE(buf, &pos)
	value := PositionEntry{
		Symbol:        symbol,
		Qty:           qty,
		EntryPrice:    entry_price,
		UnrealizedPnl: unrealized_pnl,
	}
	return value, pos, nil
}

// SBE struct PageInfo
type PageInfo struct {
	HasMore    uint8
	NextCursor *string
}

// Encode writes PageInfo fields (no header) into buf.
type PageInfoEncoder struct{}

func (PageInfoEncoder) Encode(value PageInfo, buf *[]byte) {
	local := *buf
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// DecodePageInfoDecoder reads PageInfo fields starting at pos and returns the new offset.
func PageInfoDecoderDecode(buf []byte, pos int) (PageInfo, int, error) {
	var next_cursor *string
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := PageInfo{
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE struct CapabilityPath
type CapabilityPath struct {
	Path         string
	Pattern      CapabilityPathPattern
	AuthRequired uint8
}

// Encode writes CapabilityPath fields (no header) into buf.
type CapabilityPathEncoder struct{}

func (CapabilityPathEncoder) Encode(value CapabilityPath, buf *[]byte) {
	local := *buf
	writeString(&local, value.Path)
	local = append(local, byte(value.Pattern.ToValue()))
	local = append(local, uint8(value.AuthRequired))
	*buf = local
}

// DecodeCapabilityPathDecoder reads CapabilityPath fields starting at pos and returns the new offset.
func CapabilityPathDecoderDecode(buf []byte, pos int) (CapabilityPath, int, error) {
	path := readString(buf, &pos)
	pattern_raw := readU8(buf, &pos)
	pattern, pattern_err := CapabilityPathPatternFromValue(pattern_raw)
	if pattern_err != nil {
		return CapabilityPath{}, pos, pattern_err
	}
	auth_required := readU8(buf, &pos)
	value := CapabilityPath{
		Path:         path,
		Pattern:      pattern,
		AuthRequired: auth_required,
	}
	return value, pos, nil
}

// SBE struct BatchResult
type BatchResult struct {
	Index        uint32
	Kind         BatchResultKind
	ClOrdId      string
	Status       BatchResultStatus
	RejectReason *string
}

// Encode writes BatchResult fields (no header) into buf.
type BatchResultEncoder struct{}

func (BatchResultEncoder) Encode(value BatchResult, buf *[]byte) {
	local := *buf
	writeU32BE(&local, uint32(value.Index))
	local = append(local, byte(value.Kind.ToValue()))
	writeString(&local, value.ClOrdId)
	local = append(local, byte(value.Status.ToValue()))
	if value.RejectReason != nil {
		local = append(local, 1)
		writeString(&local, *value.RejectReason)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// DecodeBatchResultDecoder reads BatchResult fields starting at pos and returns the new offset.
func BatchResultDecoderDecode(buf []byte, pos int) (BatchResult, int, error) {
	var reject_reason *string
	index := readU32BE(buf, &pos)
	kind_raw := readU8(buf, &pos)
	kind, kind_err := BatchResultKindFromValue(kind_raw)
	if kind_err != nil {
		return BatchResult{}, pos, kind_err
	}
	cl_ord_id := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := BatchResultStatusFromValue(status_raw)
	if status_err != nil {
		return BatchResult{}, pos, status_err
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		reject_reason = &s
	} else {
		pos++
	}
	value := BatchResult{
		Index:        index,
		Kind:         kind,
		ClOrdId:      cl_ord_id,
		Status:       status,
		RejectReason: reject_reason,
	}
	return value, pos, nil
}

// SBE message NewOrderSingle
type NewOrderSingle struct {
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

// SBE body encoder for NewOrderSingle (fields only, no header).
type NewOrderSingleBodyEncoder struct{}

func (NewOrderSingleBodyEncoder) Encode(value NewOrderSingle, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	local = append(local, byte(value.Side.ToValue()))
	writeF64BE(&local, value.OrderQty)
	var value_Price_v float64
	if value.Price != nil {
		value_Price_v = *value.Price
	}
	writeF64BE(&local, value_Price_v)
	if value.Price != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_StopPrice_v float64
	if value.StopPrice != nil {
		value_StopPrice_v = *value.StopPrice
	}
	writeF64BE(&local, value_StopPrice_v)
	if value.StopPrice != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeString(&local, value.Symbol)
	local = append(local, byte(value.OrderType.ToValue()))
	local = append(local, byte(value.TimeInForce.ToValue()))
	var value_ExpireTime_v int64
	if value.ExpireTime != nil {
		value_ExpireTime_v = *value.ExpireTime
	}
	writeU64BE(&local, uint64(value_ExpireTime_v))
	if value.ExpireTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Account != nil {
		local = append(local, 1)
		writeString(&local, *value.Account)
	} else {
		local = append(local, 0)
	}
	if value.StrategyId != nil {
		local = append(local, 1)
		writeString(&local, *value.StrategyId)
	} else {
		local = append(local, 0)
	}
	if value.SecurityId != nil {
		local = append(local, 1)
		writeString(&local, *value.SecurityId)
	} else {
		local = append(local, 0)
	}
	value_IdSource_v := uint8(0)
	if value.IdSource != nil {
		value_IdSource_v = byte(value.IdSource.ToValue())
	}
	local = append(local, value_IdSource_v)
	if value.IdSource != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.SecurityExchange != nil {
		local = append(local, 1)
		writeString(&local, *value.SecurityExchange)
	} else {
		local = append(local, 0)
	}
	var value_PostOnly_v uint8
	if value.PostOnly != nil {
		value_PostOnly_v = *value.PostOnly
	}
	local = append(local, uint8(value_PostOnly_v))
	if value.PostOnly != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_ReduceOnly_v uint8
	if value.ReduceOnly != nil {
		value_ReduceOnly_v = *value.ReduceOnly
	}
	local = append(local, uint8(value_ReduceOnly_v))
	if value.ReduceOnly != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for NewOrderSingle: reads fields at pos, returns the value and the new offset.
func NewOrderSingleBodyDecoderDecode(buf []byte, pos int) (NewOrderSingle, int, error) {
	var account *string
	var strategy_id *string
	var security_id *string
	var security_exchange *string
	cl_ord_id := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := NewOrderSingleSideFromValue(side_raw)
	if side_err != nil {
		return NewOrderSingle{}, pos, side_err
	}
	order_qty := readF64BE(buf, &pos)
	price_v := readF64BE(buf, &pos)
	var price *float64
	if buf[pos] == 1 {
		pos++
		price = &price_v
	} else {
		pos++
		price = nil
	}
	stop_price_v := readF64BE(buf, &pos)
	var stop_price *float64
	if buf[pos] == 1 {
		pos++
		stop_price = &stop_price_v
	} else {
		pos++
		stop_price = nil
	}
	symbol := readString(buf, &pos)
	order_type_raw := readU8(buf, &pos)
	order_type, order_type_err := NewOrderSingleOrderTypeFromValue(order_type_raw)
	if order_type_err != nil {
		return NewOrderSingle{}, pos, order_type_err
	}
	time_in_force_raw := readU8(buf, &pos)
	time_in_force, time_in_force_err := NewOrderSingleTimeInForceFromValue(time_in_force_raw)
	if time_in_force_err != nil {
		return NewOrderSingle{}, pos, time_in_force_err
	}
	expire_time_v := int64(readU64BE(buf, &pos))
	var expire_time *int64
	if buf[pos] == 1 {
		pos++
		expire_time = &expire_time_v
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
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		strategy_id = &s
	} else {
		pos++
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		security_id = &s
	} else {
		pos++
	}
	id_source_raw := readU8(buf, &pos)
	var id_source *NewOrderSingleIdSource
	if buf[pos] == 1 {
		pos++
		e, id_source_err := NewOrderSingleIdSourceFromValue(id_source_raw)
		if id_source_err != nil {
			return NewOrderSingle{}, pos, id_source_err
		}
		id_source = &e
	} else {
		pos++
		id_source = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		security_exchange = &s
	} else {
		pos++
	}
	post_only_v := readU8(buf, &pos)
	var post_only *uint8
	if buf[pos] == 1 {
		pos++
		post_only = &post_only_v
	} else {
		pos++
		post_only = nil
	}
	reduce_only_v := readU8(buf, &pos)
	var reduce_only *uint8
	if buf[pos] == 1 {
		pos++
		reduce_only = &reduce_only_v
	} else {
		pos++
		reduce_only = nil
	}
	value := NewOrderSingle{
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
	}
	return value, pos, nil
}

// SBE encoder for NewOrderSingle
type NewOrderSingleEncoder struct{}

// Encode builds a complete message (header + body).
func (NewOrderSingleEncoder) Encode(ClOrdId string, Side NewOrderSingleSide, OrderQty float64, Price *float64, StopPrice *float64, Symbol string, OrderType NewOrderSingleOrderType, TimeInForce NewOrderSingleTimeInForce, ExpireTime *int64, Account *string, StrategyId *string, SecurityId *string, IdSource *NewOrderSingleIdSource, SecurityExchange *string, PostOnly *uint8, ReduceOnly *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 1)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 38)
	writeString(&buf, ClOrdId)
	buf = append(buf, byte(Side.ToValue()))
	writeF64BE(&buf, OrderQty)
	var Price_v float64
	if Price != nil {
		Price_v = *Price
	}
	writeF64BE(&buf, Price_v)
	if Price != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var StopPrice_v float64
	if StopPrice != nil {
		StopPrice_v = *StopPrice
	}
	writeF64BE(&buf, StopPrice_v)
	if StopPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeString(&buf, Symbol)
	buf = append(buf, byte(OrderType.ToValue()))
	buf = append(buf, byte(TimeInForce.ToValue()))
	var ExpireTime_v int64
	if ExpireTime != nil {
		ExpireTime_v = *ExpireTime
	}
	writeU64BE(&buf, uint64(ExpireTime_v))
	if ExpireTime != nil {
		buf = append(buf, 1)
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
	IdSource_v := uint8(0)
	if IdSource != nil {
		IdSource_v = byte(IdSource.ToValue())
	}
	buf = append(buf, IdSource_v)
	if IdSource != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if SecurityExchange != nil {
		buf = append(buf, 1)
		writeString(&buf, *SecurityExchange)
	} else {
		buf = append(buf, 0)
	}
	var PostOnly_v uint8
	if PostOnly != nil {
		PostOnly_v = *PostOnly
	}
	buf = append(buf, uint8(PostOnly_v))
	if PostOnly != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var ReduceOnly_v uint8
	if ReduceOnly != nil {
		ReduceOnly_v = *ReduceOnly
	}
	buf = append(buf, uint8(ReduceOnly_v))
	if ReduceOnly != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for NewOrderSingle: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func NewOrderSingleDecoderDecode(buf []byte) (NewOrderSingle, int, error) {
	if len(buf) < 8 {
		return NewOrderSingle{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return NewOrderSingle{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 1 {
		return NewOrderSingle{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var account *string
	var strategy_id *string
	var security_id *string
	var security_exchange *string
	cl_ord_id := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := NewOrderSingleSideFromValue(side_raw)
	if side_err != nil {
		return NewOrderSingle{}, pos, side_err
	}
	order_qty := readF64BE(buf, &pos)
	price_v := readF64BE(buf, &pos)
	var price *float64
	if buf[pos] == 1 {
		pos++
		price = &price_v
	} else {
		pos++
		price = nil
	}
	stop_price_v := readF64BE(buf, &pos)
	var stop_price *float64
	if buf[pos] == 1 {
		pos++
		stop_price = &stop_price_v
	} else {
		pos++
		stop_price = nil
	}
	symbol := readString(buf, &pos)
	order_type_raw := readU8(buf, &pos)
	order_type, order_type_err := NewOrderSingleOrderTypeFromValue(order_type_raw)
	if order_type_err != nil {
		return NewOrderSingle{}, pos, order_type_err
	}
	time_in_force_raw := readU8(buf, &pos)
	time_in_force, time_in_force_err := NewOrderSingleTimeInForceFromValue(time_in_force_raw)
	if time_in_force_err != nil {
		return NewOrderSingle{}, pos, time_in_force_err
	}
	expire_time_v := int64(readU64BE(buf, &pos))
	var expire_time *int64
	if buf[pos] == 1 {
		pos++
		expire_time = &expire_time_v
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
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		strategy_id = &s
	} else {
		pos++
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		security_id = &s
	} else {
		pos++
	}
	id_source_raw := readU8(buf, &pos)
	var id_source *NewOrderSingleIdSource
	if buf[pos] == 1 {
		pos++
		e, id_source_err := NewOrderSingleIdSourceFromValue(id_source_raw)
		if id_source_err != nil {
			return NewOrderSingle{}, pos, id_source_err
		}
		id_source = &e
	} else {
		pos++
		id_source = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		security_exchange = &s
	} else {
		pos++
	}
	post_only_v := readU8(buf, &pos)
	var post_only *uint8
	if buf[pos] == 1 {
		pos++
		post_only = &post_only_v
	} else {
		pos++
		post_only = nil
	}
	reduce_only_v := readU8(buf, &pos)
	var reduce_only *uint8
	if buf[pos] == 1 {
		pos++
		reduce_only = &reduce_only_v
	} else {
		pos++
		reduce_only = nil
	}
	value := NewOrderSingle{
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
	}
	return value, pos, nil
}

// SBE message CancelRequest
type CancelRequest struct {
	ClOrdId     string
	OrigClOrdId string
	Symbol      string
	Side        CancelRequestSide
	OrderQty    *float64
}

// SBE body encoder for CancelRequest (fields only, no header).
type CancelRequestBodyEncoder struct{}

func (CancelRequestBodyEncoder) Encode(value CancelRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	writeString(&local, value.OrigClOrdId)
	writeString(&local, value.Symbol)
	local = append(local, byte(value.Side.ToValue()))
	var value_OrderQty_v float64
	if value.OrderQty != nil {
		value_OrderQty_v = *value.OrderQty
	}
	writeF64BE(&local, value_OrderQty_v)
	if value.OrderQty != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for CancelRequest: reads fields at pos, returns the value and the new offset.
func CancelRequestBodyDecoderDecode(buf []byte, pos int) (CancelRequest, int, error) {
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := CancelRequestSideFromValue(side_raw)
	if side_err != nil {
		return CancelRequest{}, pos, side_err
	}
	order_qty_v := readF64BE(buf, &pos)
	var order_qty *float64
	if buf[pos] == 1 {
		pos++
		order_qty = &order_qty_v
	} else {
		pos++
		order_qty = nil
	}
	value := CancelRequest{
		ClOrdId:     cl_ord_id,
		OrigClOrdId: orig_cl_ord_id,
		Symbol:      symbol,
		Side:        side,
		OrderQty:    order_qty,
	}
	return value, pos, nil
}

// SBE encoder for CancelRequest
type CancelRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (CancelRequestEncoder) Encode(ClOrdId string, OrigClOrdId string, Symbol string, Side CancelRequestSide, OrderQty *float64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 2)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 9)
	writeString(&buf, ClOrdId)
	writeString(&buf, OrigClOrdId)
	writeString(&buf, Symbol)
	buf = append(buf, byte(Side.ToValue()))
	var OrderQty_v float64
	if OrderQty != nil {
		OrderQty_v = *OrderQty
	}
	writeF64BE(&buf, OrderQty_v)
	if OrderQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for CancelRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CancelRequestDecoderDecode(buf []byte) (CancelRequest, int, error) {
	if len(buf) < 8 {
		return CancelRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CancelRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 2 {
		return CancelRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := CancelRequestSideFromValue(side_raw)
	if side_err != nil {
		return CancelRequest{}, pos, side_err
	}
	order_qty_v := readF64BE(buf, &pos)
	var order_qty *float64
	if buf[pos] == 1 {
		pos++
		order_qty = &order_qty_v
	} else {
		pos++
		order_qty = nil
	}
	value := CancelRequest{
		ClOrdId:     cl_ord_id,
		OrigClOrdId: orig_cl_ord_id,
		Symbol:      symbol,
		Side:        side,
		OrderQty:    order_qty,
	}
	return value, pos, nil
}

// SBE message CancelReplaceRequest
type CancelReplaceRequest struct {
	ClOrdId     string
	OrigClOrdId string
	Symbol      string
	Side        CancelReplaceRequestSide
	OrderQty    float64
	Price       *float64
}

// SBE body encoder for CancelReplaceRequest (fields only, no header).
type CancelReplaceRequestBodyEncoder struct{}

func (CancelReplaceRequestBodyEncoder) Encode(value CancelReplaceRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	writeString(&local, value.OrigClOrdId)
	writeString(&local, value.Symbol)
	local = append(local, byte(value.Side.ToValue()))
	writeF64BE(&local, value.OrderQty)
	var value_Price_v float64
	if value.Price != nil {
		value_Price_v = *value.Price
	}
	writeF64BE(&local, value_Price_v)
	if value.Price != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for CancelReplaceRequest: reads fields at pos, returns the value and the new offset.
func CancelReplaceRequestBodyDecoderDecode(buf []byte, pos int) (CancelReplaceRequest, int, error) {
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := CancelReplaceRequestSideFromValue(side_raw)
	if side_err != nil {
		return CancelReplaceRequest{}, pos, side_err
	}
	order_qty := readF64BE(buf, &pos)
	price_v := readF64BE(buf, &pos)
	var price *float64
	if buf[pos] == 1 {
		pos++
		price = &price_v
	} else {
		pos++
		price = nil
	}
	value := CancelReplaceRequest{
		ClOrdId:     cl_ord_id,
		OrigClOrdId: orig_cl_ord_id,
		Symbol:      symbol,
		Side:        side,
		OrderQty:    order_qty,
		Price:       price,
	}
	return value, pos, nil
}

// SBE encoder for CancelReplaceRequest
type CancelReplaceRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (CancelReplaceRequestEncoder) Encode(ClOrdId string, OrigClOrdId string, Symbol string, Side CancelReplaceRequestSide, OrderQty float64, Price *float64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 3)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)
	writeString(&buf, ClOrdId)
	writeString(&buf, OrigClOrdId)
	writeString(&buf, Symbol)
	buf = append(buf, byte(Side.ToValue()))
	writeF64BE(&buf, OrderQty)
	var Price_v float64
	if Price != nil {
		Price_v = *Price
	}
	writeF64BE(&buf, Price_v)
	if Price != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for CancelReplaceRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CancelReplaceRequestDecoderDecode(buf []byte) (CancelReplaceRequest, int, error) {
	if len(buf) < 8 {
		return CancelReplaceRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CancelReplaceRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 3 {
		return CancelReplaceRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	symbol := readString(buf, &pos)
	side_raw := readU8(buf, &pos)
	side, side_err := CancelReplaceRequestSideFromValue(side_raw)
	if side_err != nil {
		return CancelReplaceRequest{}, pos, side_err
	}
	order_qty := readF64BE(buf, &pos)
	price_v := readF64BE(buf, &pos)
	var price *float64
	if buf[pos] == 1 {
		pos++
		price = &price_v
	} else {
		pos++
		price = nil
	}
	value := CancelReplaceRequest{
		ClOrdId:     cl_ord_id,
		OrigClOrdId: orig_cl_ord_id,
		Symbol:      symbol,
		Side:        side,
		OrderQty:    order_qty,
		Price:       price,
	}
	return value, pos, nil
}

// SBE message ExecutionReport
type ExecutionReport struct {
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

// SBE body encoder for ExecutionReport (fields only, no header).
type ExecutionReportBodyEncoder struct{}

func (ExecutionReportBodyEncoder) Encode(value ExecutionReport, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	writeString(&local, value.OrderId)
	writeString(&local, value.ExecId)
	local = append(local, byte(value.ExecType.ToValue()))
	local = append(local, byte(value.OrdStatus.ToValue()))
	local = append(local, byte(value.Side.ToValue()))
	var value_LastQty_v float64
	if value.LastQty != nil {
		value_LastQty_v = *value.LastQty
	}
	writeF64BE(&local, value_LastQty_v)
	if value.LastQty != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_LastPrice_v float64
	if value.LastPrice != nil {
		value_LastPrice_v = *value.LastPrice
	}
	writeF64BE(&local, value_LastPrice_v)
	if value.LastPrice != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeF64BE(&local, value.LeavesQty)
	writeF64BE(&local, value.CumQty)
	writeF64BE(&local, value.AvgPrice)
	writeString(&local, value.Symbol)
	writeU64BE(&local, uint64(value.TransactTime))
	*buf = local
}

// SBE body decoder for ExecutionReport: reads fields at pos, returns the value and the new offset.
func ExecutionReportBodyDecoderDecode(buf []byte, pos int) (ExecutionReport, int, error) {
	cl_ord_id := readString(buf, &pos)
	order_id := readString(buf, &pos)
	exec_id := readString(buf, &pos)
	exec_type_raw := readU8(buf, &pos)
	exec_type, exec_type_err := ExecutionReportExecTypeFromValue(exec_type_raw)
	if exec_type_err != nil {
		return ExecutionReport{}, pos, exec_type_err
	}
	ord_status_raw := readU8(buf, &pos)
	ord_status, ord_status_err := ExecutionReportOrdStatusFromValue(ord_status_raw)
	if ord_status_err != nil {
		return ExecutionReport{}, pos, ord_status_err
	}
	side_raw := readU8(buf, &pos)
	side, side_err := ExecutionReportSideFromValue(side_raw)
	if side_err != nil {
		return ExecutionReport{}, pos, side_err
	}
	last_qty_v := readF64BE(buf, &pos)
	var last_qty *float64
	if buf[pos] == 1 {
		pos++
		last_qty = &last_qty_v
	} else {
		pos++
		last_qty = nil
	}
	last_price_v := readF64BE(buf, &pos)
	var last_price *float64
	if buf[pos] == 1 {
		pos++
		last_price = &last_price_v
	} else {
		pos++
		last_price = nil
	}
	leaves_qty := readF64BE(buf, &pos)
	cum_qty := readF64BE(buf, &pos)
	avg_price := readF64BE(buf, &pos)
	symbol := readString(buf, &pos)
	transact_time := int64(readU64BE(buf, &pos))
	value := ExecutionReport{
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
	}
	return value, pos, nil
}

// SBE encoder for ExecutionReport
type ExecutionReportEncoder struct{}

// Encode builds a complete message (header + body).
func (ExecutionReportEncoder) Encode(ClOrdId string, OrderId string, ExecId string, ExecType ExecutionReportExecType, OrdStatus ExecutionReportOrdStatus, Side ExecutionReportSide, LastQty *float64, LastPrice *float64, LeavesQty float64, CumQty float64, AvgPrice float64, Symbol string, TransactTime int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 4)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 51)
	writeString(&buf, ClOrdId)
	writeString(&buf, OrderId)
	writeString(&buf, ExecId)
	buf = append(buf, byte(ExecType.ToValue()))
	buf = append(buf, byte(OrdStatus.ToValue()))
	buf = append(buf, byte(Side.ToValue()))
	var LastQty_v float64
	if LastQty != nil {
		LastQty_v = *LastQty
	}
	writeF64BE(&buf, LastQty_v)
	if LastQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var LastPrice_v float64
	if LastPrice != nil {
		LastPrice_v = *LastPrice
	}
	writeF64BE(&buf, LastPrice_v)
	if LastPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeF64BE(&buf, LeavesQty)
	writeF64BE(&buf, CumQty)
	writeF64BE(&buf, AvgPrice)
	writeString(&buf, Symbol)
	writeU64BE(&buf, uint64(TransactTime))
	return buf, nil
}

// SBE decoder for ExecutionReport: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func ExecutionReportDecoderDecode(buf []byte) (ExecutionReport, int, error) {
	if len(buf) < 8 {
		return ExecutionReport{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return ExecutionReport{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 4 {
		return ExecutionReport{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	order_id := readString(buf, &pos)
	exec_id := readString(buf, &pos)
	exec_type_raw := readU8(buf, &pos)
	exec_type, exec_type_err := ExecutionReportExecTypeFromValue(exec_type_raw)
	if exec_type_err != nil {
		return ExecutionReport{}, pos, exec_type_err
	}
	ord_status_raw := readU8(buf, &pos)
	ord_status, ord_status_err := ExecutionReportOrdStatusFromValue(ord_status_raw)
	if ord_status_err != nil {
		return ExecutionReport{}, pos, ord_status_err
	}
	side_raw := readU8(buf, &pos)
	side, side_err := ExecutionReportSideFromValue(side_raw)
	if side_err != nil {
		return ExecutionReport{}, pos, side_err
	}
	last_qty_v := readF64BE(buf, &pos)
	var last_qty *float64
	if buf[pos] == 1 {
		pos++
		last_qty = &last_qty_v
	} else {
		pos++
		last_qty = nil
	}
	last_price_v := readF64BE(buf, &pos)
	var last_price *float64
	if buf[pos] == 1 {
		pos++
		last_price = &last_price_v
	} else {
		pos++
		last_price = nil
	}
	leaves_qty := readF64BE(buf, &pos)
	cum_qty := readF64BE(buf, &pos)
	avg_price := readF64BE(buf, &pos)
	symbol := readString(buf, &pos)
	transact_time := int64(readU64BE(buf, &pos))
	value := ExecutionReport{
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
	}
	return value, pos, nil
}

// SBE message CancelReject
type CancelReject struct {
	ClOrdId      string
	OrigClOrdId  string
	RejectReason CancelRejectRejectReason
	Symbol       string
}

// SBE body encoder for CancelReject (fields only, no header).
type CancelRejectBodyEncoder struct{}

func (CancelRejectBodyEncoder) Encode(value CancelReject, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	writeString(&local, value.OrigClOrdId)
	local = append(local, byte(value.RejectReason.ToValue()))
	writeString(&local, value.Symbol)
	*buf = local
}

// SBE body decoder for CancelReject: reads fields at pos, returns the value and the new offset.
func CancelRejectBodyDecoderDecode(buf []byte, pos int) (CancelReject, int, error) {
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	reject_reason_raw := readU8(buf, &pos)
	reject_reason, reject_reason_err := CancelRejectRejectReasonFromValue(reject_reason_raw)
	if reject_reason_err != nil {
		return CancelReject{}, pos, reject_reason_err
	}
	symbol := readString(buf, &pos)
	value := CancelReject{
		ClOrdId:      cl_ord_id,
		OrigClOrdId:  orig_cl_ord_id,
		RejectReason: reject_reason,
		Symbol:       symbol,
	}
	return value, pos, nil
}

// SBE encoder for CancelReject
type CancelRejectEncoder struct{}

// Encode builds a complete message (header + body).
func (CancelRejectEncoder) Encode(ClOrdId string, OrigClOrdId string, RejectReason CancelRejectRejectReason, Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 5)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, ClOrdId)
	writeString(&buf, OrigClOrdId)
	buf = append(buf, byte(RejectReason.ToValue()))
	writeString(&buf, Symbol)
	return buf, nil
}

// SBE decoder for CancelReject: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CancelRejectDecoderDecode(buf []byte) (CancelReject, int, error) {
	if len(buf) < 8 {
		return CancelReject{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CancelReject{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 5 {
		return CancelReject{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	cl_ord_id := readString(buf, &pos)
	orig_cl_ord_id := readString(buf, &pos)
	reject_reason_raw := readU8(buf, &pos)
	reject_reason, reject_reason_err := CancelRejectRejectReasonFromValue(reject_reason_raw)
	if reject_reason_err != nil {
		return CancelReject{}, pos, reject_reason_err
	}
	symbol := readString(buf, &pos)
	value := CancelReject{
		ClOrdId:      cl_ord_id,
		OrigClOrdId:  orig_cl_ord_id,
		RejectReason: reject_reason,
		Symbol:       symbol,
	}
	return value, pos, nil
}

// SBE message OrderBookRequest
type OrderBookRequest struct {
	Symbol string
	Depth  *uint32
	AtTime *int64
}

// SBE body encoder for OrderBookRequest (fields only, no header).
type OrderBookRequestBodyEncoder struct{}

func (OrderBookRequestBodyEncoder) Encode(value OrderBookRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	var value_Depth_v uint32
	if value.Depth != nil {
		value_Depth_v = *value.Depth
	}
	writeU32BE(&local, uint32(value_Depth_v))
	if value.Depth != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_AtTime_v int64
	if value.AtTime != nil {
		value_AtTime_v = *value.AtTime
	}
	writeU64BE(&local, uint64(value_AtTime_v))
	if value.AtTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OrderBookRequest: reads fields at pos, returns the value and the new offset.
func OrderBookRequestBodyDecoderDecode(buf []byte, pos int) (OrderBookRequest, int, error) {
	symbol := readString(buf, &pos)
	depth_v := readU32BE(buf, &pos)
	var depth *uint32
	if buf[pos] == 1 {
		pos++
		depth = &depth_v
	} else {
		pos++
		depth = nil
	}
	at_time_v := int64(readU64BE(buf, &pos))
	var at_time *int64
	if buf[pos] == 1 {
		pos++
		at_time = &at_time_v
	} else {
		pos++
		at_time = nil
	}
	value := OrderBookRequest{
		Symbol: symbol,
		Depth:  depth,
		AtTime: at_time,
	}
	return value, pos, nil
}

// SBE encoder for OrderBookRequest
type OrderBookRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (OrderBookRequestEncoder) Encode(Symbol string, Depth *uint32, AtTime *int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 8)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 12)
	writeString(&buf, Symbol)
	var Depth_v uint32
	if Depth != nil {
		Depth_v = *Depth
	}
	writeU32BE(&buf, uint32(Depth_v))
	if Depth != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var AtTime_v int64
	if AtTime != nil {
		AtTime_v = *AtTime
	}
	writeU64BE(&buf, uint64(AtTime_v))
	if AtTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OrderBookRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OrderBookRequestDecoderDecode(buf []byte) (OrderBookRequest, int, error) {
	if len(buf) < 8 {
		return OrderBookRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderBookRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 8 {
		return OrderBookRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	depth_v := readU32BE(buf, &pos)
	var depth *uint32
	if buf[pos] == 1 {
		pos++
		depth = &depth_v
	} else {
		pos++
		depth = nil
	}
	at_time_v := int64(readU64BE(buf, &pos))
	var at_time *int64
	if buf[pos] == 1 {
		pos++
		at_time = &at_time_v
	} else {
		pos++
		at_time = nil
	}
	value := OrderBookRequest{
		Symbol: symbol,
		Depth:  depth,
		AtTime: at_time,
	}
	return value, pos, nil
}

// SBE message AggregateTradeRequest
type AggregateTradeRequest struct {
	Symbol    string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for AggregateTradeRequest (fields only, no header).
type AggregateTradeRequestBodyEncoder struct{}

func (AggregateTradeRequestBodyEncoder) Encode(value AggregateTradeRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for AggregateTradeRequest: reads fields at pos, returns the value and the new offset.
func AggregateTradeRequestBodyDecoderDecode(buf []byte, pos int) (AggregateTradeRequest, int, error) {
	var cursor *string
	symbol := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := AggregateTradeRequest{
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for AggregateTradeRequest
type AggregateTradeRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (AggregateTradeRequestEncoder) Encode(Symbol string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 12)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Symbol)
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for AggregateTradeRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func AggregateTradeRequestDecoderDecode(buf []byte) (AggregateTradeRequest, int, error) {
	if len(buf) < 8 {
		return AggregateTradeRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AggregateTradeRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 12 {
		return AggregateTradeRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var cursor *string
	symbol := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := AggregateTradeRequest{
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message MiniTicker
type MiniTicker struct {
	Symbol     string
	LastPrice  float64
	Volume     float64
	Timestamp  int64
	IsSnapshot *uint8
}

// SBE body encoder for MiniTicker (fields only, no header).
type MiniTickerBodyEncoder struct{}

func (MiniTickerBodyEncoder) Encode(value MiniTicker, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeF64BE(&local, value.LastPrice)
	writeF64BE(&local, value.Volume)
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for MiniTicker: reads fields at pos, returns the value and the new offset.
func MiniTickerBodyDecoderDecode(buf []byte, pos int) (MiniTicker, int, error) {
	symbol := readString(buf, &pos)
	last_price := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MiniTicker{
		Symbol:     symbol,
		LastPrice:  last_price,
		Volume:     volume,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for MiniTicker
type MiniTickerEncoder struct{}

// Encode builds a complete message (header + body).
func (MiniTickerEncoder) Encode(Symbol string, LastPrice float64, Volume float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 14)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 25)
	writeString(&buf, Symbol)
	writeF64BE(&buf, LastPrice)
	writeF64BE(&buf, Volume)
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for MiniTicker: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MiniTickerDecoderDecode(buf []byte) (MiniTicker, int, error) {
	if len(buf) < 8 {
		return MiniTicker{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MiniTicker{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 14 {
		return MiniTicker{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	last_price := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MiniTicker{
		Symbol:     symbol,
		LastPrice:  last_price,
		Volume:     volume,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message AllMidsRequest
type AllMidsRequest struct {
}

// SBE body encoder for AllMidsRequest (fields only, no header).
type AllMidsRequestBodyEncoder struct{}

func (AllMidsRequestBodyEncoder) Encode(value AllMidsRequest, buf *[]byte) {
	local := *buf
	*buf = local
}

// SBE body decoder for AllMidsRequest: reads fields at pos, returns the value and the new offset.
func AllMidsRequestBodyDecoderDecode(buf []byte, pos int) (AllMidsRequest, int, error) {
	value := AllMidsRequest{}
	return value, pos, nil
}

// SBE encoder for AllMidsRequest
type AllMidsRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (AllMidsRequestEncoder) Encode() ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 15)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	return buf, nil
}

// SBE decoder for AllMidsRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func AllMidsRequestDecoderDecode(buf []byte) (AllMidsRequest, int, error) {
	if len(buf) < 8 {
		return AllMidsRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AllMidsRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 15 {
		return AllMidsRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	value := AllMidsRequest{}
	return value, pos, nil
}

// SBE message MarkPriceUpdate
type MarkPriceUpdate struct {
	Symbol      string
	MarkPrice   float64
	IndexPrice  *float64
	FundingRate *float64
	Timestamp   int64
	IsSnapshot  *uint8
}

// SBE body encoder for MarkPriceUpdate (fields only, no header).
type MarkPriceUpdateBodyEncoder struct{}

func (MarkPriceUpdateBodyEncoder) Encode(value MarkPriceUpdate, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeF64BE(&local, value.MarkPrice)
	var value_IndexPrice_v float64
	if value.IndexPrice != nil {
		value_IndexPrice_v = *value.IndexPrice
	}
	writeF64BE(&local, value_IndexPrice_v)
	if value.IndexPrice != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_FundingRate_v float64
	if value.FundingRate != nil {
		value_FundingRate_v = *value.FundingRate
	}
	writeF64BE(&local, value_FundingRate_v)
	if value.FundingRate != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for MarkPriceUpdate: reads fields at pos, returns the value and the new offset.
func MarkPriceUpdateBodyDecoderDecode(buf []byte, pos int) (MarkPriceUpdate, int, error) {
	symbol := readString(buf, &pos)
	mark_price := readF64BE(buf, &pos)
	index_price_v := readF64BE(buf, &pos)
	var index_price *float64
	if buf[pos] == 1 {
		pos++
		index_price = &index_price_v
	} else {
		pos++
		index_price = nil
	}
	funding_rate_v := readF64BE(buf, &pos)
	var funding_rate *float64
	if buf[pos] == 1 {
		pos++
		funding_rate = &funding_rate_v
	} else {
		pos++
		funding_rate = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MarkPriceUpdate{
		Symbol:      symbol,
		MarkPrice:   mark_price,
		IndexPrice:  index_price,
		FundingRate: funding_rate,
		Timestamp:   timestamp,
		IsSnapshot:  is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for MarkPriceUpdate
type MarkPriceUpdateEncoder struct{}

// Encode builds a complete message (header + body).
func (MarkPriceUpdateEncoder) Encode(Symbol string, MarkPrice float64, IndexPrice *float64, FundingRate *float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 17)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 33)
	writeString(&buf, Symbol)
	writeF64BE(&buf, MarkPrice)
	var IndexPrice_v float64
	if IndexPrice != nil {
		IndexPrice_v = *IndexPrice
	}
	writeF64BE(&buf, IndexPrice_v)
	if IndexPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var FundingRate_v float64
	if FundingRate != nil {
		FundingRate_v = *FundingRate
	}
	writeF64BE(&buf, FundingRate_v)
	if FundingRate != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for MarkPriceUpdate: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MarkPriceUpdateDecoderDecode(buf []byte) (MarkPriceUpdate, int, error) {
	if len(buf) < 8 {
		return MarkPriceUpdate{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarkPriceUpdate{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 17 {
		return MarkPriceUpdate{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	mark_price := readF64BE(buf, &pos)
	index_price_v := readF64BE(buf, &pos)
	var index_price *float64
	if buf[pos] == 1 {
		pos++
		index_price = &index_price_v
	} else {
		pos++
		index_price = nil
	}
	funding_rate_v := readF64BE(buf, &pos)
	var funding_rate *float64
	if buf[pos] == 1 {
		pos++
		funding_rate = &funding_rate_v
	} else {
		pos++
		funding_rate = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MarkPriceUpdate{
		Symbol:      symbol,
		MarkPrice:   mark_price,
		IndexPrice:  index_price,
		FundingRate: funding_rate,
		Timestamp:   timestamp,
		IsSnapshot:  is_snapshot,
	}
	return value, pos, nil
}

// SBE message MarkPriceRequest
type MarkPriceRequest struct {
	Symbol string
}

// SBE body encoder for MarkPriceRequest (fields only, no header).
type MarkPriceRequestBodyEncoder struct{}

func (MarkPriceRequestBodyEncoder) Encode(value MarkPriceRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	*buf = local
}

// SBE body decoder for MarkPriceRequest: reads fields at pos, returns the value and the new offset.
func MarkPriceRequestBodyDecoderDecode(buf []byte, pos int) (MarkPriceRequest, int, error) {
	symbol := readString(buf, &pos)
	value := MarkPriceRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE encoder for MarkPriceRequest
type MarkPriceRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (MarkPriceRequestEncoder) Encode(Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 18)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, Symbol)
	return buf, nil
}

// SBE decoder for MarkPriceRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MarkPriceRequestDecoderDecode(buf []byte) (MarkPriceRequest, int, error) {
	if len(buf) < 8 {
		return MarkPriceRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarkPriceRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 18 {
		return MarkPriceRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	value := MarkPriceRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE message CandleBarRequest
type CandleBarRequest struct {
	Symbol    string
	Interval  string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for CandleBarRequest (fields only, no header).
type CandleBarRequestBodyEncoder struct{}

func (CandleBarRequestBodyEncoder) Encode(value CandleBarRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.Interval)
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for CandleBarRequest: reads fields at pos, returns the value and the new offset.
func CandleBarRequestBodyDecoderDecode(buf []byte, pos int) (CandleBarRequest, int, error) {
	var cursor *string
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := CandleBarRequest{
		Symbol:    symbol,
		Interval:  interval,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for CandleBarRequest
type CandleBarRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (CandleBarRequestEncoder) Encode(Symbol string, Interval string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 21)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Symbol)
	writeString(&buf, Interval)
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for CandleBarRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CandleBarRequestDecoderDecode(buf []byte) (CandleBarRequest, int, error) {
	if len(buf) < 8 {
		return CandleBarRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CandleBarRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 21 {
		return CandleBarRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var cursor *string
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := CandleBarRequest{
		Symbol:    symbol,
		Interval:  interval,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message TradeHistoryRequest
type TradeHistoryRequest struct {
	Symbol    string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for TradeHistoryRequest (fields only, no header).
type TradeHistoryRequestBodyEncoder struct{}

func (TradeHistoryRequestBodyEncoder) Encode(value TradeHistoryRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for TradeHistoryRequest: reads fields at pos, returns the value and the new offset.
func TradeHistoryRequestBodyDecoderDecode(buf []byte, pos int) (TradeHistoryRequest, int, error) {
	var cursor *string
	symbol := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := TradeHistoryRequest{
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for TradeHistoryRequest
type TradeHistoryRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (TradeHistoryRequestEncoder) Encode(Symbol string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 24)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Symbol)
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for TradeHistoryRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func TradeHistoryRequestDecoderDecode(buf []byte) (TradeHistoryRequest, int, error) {
	if len(buf) < 8 {
		return TradeHistoryRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return TradeHistoryRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 24 {
		return TradeHistoryRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var cursor *string
	symbol := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := TradeHistoryRequest{
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message BestBidOffer
type BestBidOffer struct {
	Symbol     string
	BidPrice   *float64
	BidQty     *float64
	AskPrice   *float64
	AskQty     *float64
	Timestamp  int64
	IsSnapshot *uint8
}

// SBE body encoder for BestBidOffer (fields only, no header).
type BestBidOfferBodyEncoder struct{}

func (BestBidOfferBodyEncoder) Encode(value BestBidOffer, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	var value_BidPrice_v float64
	if value.BidPrice != nil {
		value_BidPrice_v = *value.BidPrice
	}
	writeF64BE(&local, value_BidPrice_v)
	if value.BidPrice != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_BidQty_v float64
	if value.BidQty != nil {
		value_BidQty_v = *value.BidQty
	}
	writeF64BE(&local, value_BidQty_v)
	if value.BidQty != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_AskPrice_v float64
	if value.AskPrice != nil {
		value_AskPrice_v = *value.AskPrice
	}
	writeF64BE(&local, value_AskPrice_v)
	if value.AskPrice != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_AskQty_v float64
	if value.AskQty != nil {
		value_AskQty_v = *value.AskQty
	}
	writeF64BE(&local, value_AskQty_v)
	if value.AskQty != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for BestBidOffer: reads fields at pos, returns the value and the new offset.
func BestBidOfferBodyDecoderDecode(buf []byte, pos int) (BestBidOffer, int, error) {
	symbol := readString(buf, &pos)
	bid_price_v := readF64BE(buf, &pos)
	var bid_price *float64
	if buf[pos] == 1 {
		pos++
		bid_price = &bid_price_v
	} else {
		pos++
		bid_price = nil
	}
	bid_qty_v := readF64BE(buf, &pos)
	var bid_qty *float64
	if buf[pos] == 1 {
		pos++
		bid_qty = &bid_qty_v
	} else {
		pos++
		bid_qty = nil
	}
	ask_price_v := readF64BE(buf, &pos)
	var ask_price *float64
	if buf[pos] == 1 {
		pos++
		ask_price = &ask_price_v
	} else {
		pos++
		ask_price = nil
	}
	ask_qty_v := readF64BE(buf, &pos)
	var ask_qty *float64
	if buf[pos] == 1 {
		pos++
		ask_qty = &ask_qty_v
	} else {
		pos++
		ask_qty = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := BestBidOffer{
		Symbol:     symbol,
		BidPrice:   bid_price,
		BidQty:     bid_qty,
		AskPrice:   ask_price,
		AskQty:     ask_qty,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for BestBidOffer
type BestBidOfferEncoder struct{}

// Encode builds a complete message (header + body).
func (BestBidOfferEncoder) Encode(Symbol string, BidPrice *float64, BidQty *float64, AskPrice *float64, AskQty *float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 26)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 41)
	writeString(&buf, Symbol)
	var BidPrice_v float64
	if BidPrice != nil {
		BidPrice_v = *BidPrice
	}
	writeF64BE(&buf, BidPrice_v)
	if BidPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var BidQty_v float64
	if BidQty != nil {
		BidQty_v = *BidQty
	}
	writeF64BE(&buf, BidQty_v)
	if BidQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var AskPrice_v float64
	if AskPrice != nil {
		AskPrice_v = *AskPrice
	}
	writeF64BE(&buf, AskPrice_v)
	if AskPrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var AskQty_v float64
	if AskQty != nil {
		AskQty_v = *AskQty
	}
	writeF64BE(&buf, AskQty_v)
	if AskQty != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for BestBidOffer: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func BestBidOfferDecoderDecode(buf []byte) (BestBidOffer, int, error) {
	if len(buf) < 8 {
		return BestBidOffer{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BestBidOffer{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 26 {
		return BestBidOffer{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	bid_price_v := readF64BE(buf, &pos)
	var bid_price *float64
	if buf[pos] == 1 {
		pos++
		bid_price = &bid_price_v
	} else {
		pos++
		bid_price = nil
	}
	bid_qty_v := readF64BE(buf, &pos)
	var bid_qty *float64
	if buf[pos] == 1 {
		pos++
		bid_qty = &bid_qty_v
	} else {
		pos++
		bid_qty = nil
	}
	ask_price_v := readF64BE(buf, &pos)
	var ask_price *float64
	if buf[pos] == 1 {
		pos++
		ask_price = &ask_price_v
	} else {
		pos++
		ask_price = nil
	}
	ask_qty_v := readF64BE(buf, &pos)
	var ask_qty *float64
	if buf[pos] == 1 {
		pos++
		ask_qty = &ask_qty_v
	} else {
		pos++
		ask_qty = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := BestBidOffer{
		Symbol:     symbol,
		BidPrice:   bid_price,
		BidQty:     bid_qty,
		AskPrice:   ask_price,
		AskQty:     ask_qty,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message SymbolTicker
type SymbolTicker struct {
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

// SBE body encoder for SymbolTicker (fields only, no header).
type SymbolTickerBodyEncoder struct{}

func (SymbolTickerBodyEncoder) Encode(value SymbolTicker, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeF64BE(&local, value.LastPrice)
	writeF64BE(&local, value.PriceChange)
	writeF64BE(&local, value.PriceChangePct)
	writeF64BE(&local, value.Volume)
	writeF64BE(&local, value.High)
	writeF64BE(&local, value.Low)
	writeF64BE(&local, value.Open)
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for SymbolTicker: reads fields at pos, returns the value and the new offset.
func SymbolTickerBodyDecoderDecode(buf []byte, pos int) (SymbolTicker, int, error) {
	symbol := readString(buf, &pos)
	last_price := readF64BE(buf, &pos)
	price_change := readF64BE(buf, &pos)
	price_change_pct := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	high := readF64BE(buf, &pos)
	low := readF64BE(buf, &pos)
	open := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := SymbolTicker{
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
	}
	return value, pos, nil
}

// SBE encoder for SymbolTicker
type SymbolTickerEncoder struct{}

// Encode builds a complete message (header + body).
func (SymbolTickerEncoder) Encode(Symbol string, LastPrice float64, PriceChange float64, PriceChangePct float64, Volume float64, High float64, Low float64, Open float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 27)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 65)
	writeString(&buf, Symbol)
	writeF64BE(&buf, LastPrice)
	writeF64BE(&buf, PriceChange)
	writeF64BE(&buf, PriceChangePct)
	writeF64BE(&buf, Volume)
	writeF64BE(&buf, High)
	writeF64BE(&buf, Low)
	writeF64BE(&buf, Open)
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for SymbolTicker: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func SymbolTickerDecoderDecode(buf []byte) (SymbolTicker, int, error) {
	if len(buf) < 8 {
		return SymbolTicker{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return SymbolTicker{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 27 {
		return SymbolTicker{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	last_price := readF64BE(buf, &pos)
	price_change := readF64BE(buf, &pos)
	price_change_pct := readF64BE(buf, &pos)
	volume := readF64BE(buf, &pos)
	high := readF64BE(buf, &pos)
	low := readF64BE(buf, &pos)
	open := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := SymbolTicker{
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
	}
	return value, pos, nil
}

// SBE message TickerRequest
type TickerRequest struct {
	Symbol string
}

// SBE body encoder for TickerRequest (fields only, no header).
type TickerRequestBodyEncoder struct{}

func (TickerRequestBodyEncoder) Encode(value TickerRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	*buf = local
}

// SBE body decoder for TickerRequest: reads fields at pos, returns the value and the new offset.
func TickerRequestBodyDecoderDecode(buf []byte, pos int) (TickerRequest, int, error) {
	symbol := readString(buf, &pos)
	value := TickerRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE encoder for TickerRequest
type TickerRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (TickerRequestEncoder) Encode(Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 28)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, Symbol)
	return buf, nil
}

// SBE decoder for TickerRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func TickerRequestDecoderDecode(buf []byte) (TickerRequest, int, error) {
	if len(buf) < 8 {
		return TickerRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return TickerRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 28 {
		return TickerRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	value := TickerRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE message InstrumentCatalogRequest
type InstrumentCatalogRequest struct {
}

// SBE body encoder for InstrumentCatalogRequest (fields only, no header).
type InstrumentCatalogRequestBodyEncoder struct{}

func (InstrumentCatalogRequestBodyEncoder) Encode(value InstrumentCatalogRequest, buf *[]byte) {
	local := *buf
	*buf = local
}

// SBE body decoder for InstrumentCatalogRequest: reads fields at pos, returns the value and the new offset.
func InstrumentCatalogRequestBodyDecoderDecode(buf []byte, pos int) (InstrumentCatalogRequest, int, error) {
	value := InstrumentCatalogRequest{}
	return value, pos, nil
}

// SBE encoder for InstrumentCatalogRequest
type InstrumentCatalogRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (InstrumentCatalogRequestEncoder) Encode() ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 29)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	return buf, nil
}

// SBE decoder for InstrumentCatalogRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func InstrumentCatalogRequestDecoderDecode(buf []byte) (InstrumentCatalogRequest, int, error) {
	if len(buf) < 8 {
		return InstrumentCatalogRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return InstrumentCatalogRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 29 {
		return InstrumentCatalogRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	value := InstrumentCatalogRequest{}
	return value, pos, nil
}

// SBE message AccountSummary
type AccountSummary struct {
	Account     string
	Balance     float64
	BuyingPower float64
	Currency    string
}

// SBE body encoder for AccountSummary (fields only, no header).
type AccountSummaryBodyEncoder struct{}

func (AccountSummaryBodyEncoder) Encode(value AccountSummary, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeF64BE(&local, value.Balance)
	writeF64BE(&local, value.BuyingPower)
	writeString(&local, value.Currency)
	*buf = local
}

// SBE body decoder for AccountSummary: reads fields at pos, returns the value and the new offset.
func AccountSummaryBodyDecoderDecode(buf []byte, pos int) (AccountSummary, int, error) {
	account := readString(buf, &pos)
	balance := readF64BE(buf, &pos)
	buying_power := readF64BE(buf, &pos)
	currency := readString(buf, &pos)
	value := AccountSummary{
		Account:     account,
		Balance:     balance,
		BuyingPower: buying_power,
		Currency:    currency,
	}
	return value, pos, nil
}

// SBE encoder for AccountSummary
type AccountSummaryEncoder struct{}

// Encode builds a complete message (header + body).
func (AccountSummaryEncoder) Encode(Account string, Balance float64, BuyingPower float64, Currency string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 31)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 16)
	writeString(&buf, Account)
	writeF64BE(&buf, Balance)
	writeF64BE(&buf, BuyingPower)
	writeString(&buf, Currency)
	return buf, nil
}

// SBE decoder for AccountSummary: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func AccountSummaryDecoderDecode(buf []byte) (AccountSummary, int, error) {
	if len(buf) < 8 {
		return AccountSummary{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AccountSummary{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 31 {
		return AccountSummary{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	balance := readF64BE(buf, &pos)
	buying_power := readF64BE(buf, &pos)
	currency := readString(buf, &pos)
	value := AccountSummary{
		Account:     account,
		Balance:     balance,
		BuyingPower: buying_power,
		Currency:    currency,
	}
	return value, pos, nil
}

// SBE message MarginSummary
type MarginSummary struct {
	Account     string
	Balance     float64
	BuyingPower float64
	Equity      float64
	MarginUsed  float64
	Available   float64
	Currency    string
}

// SBE body encoder for MarginSummary (fields only, no header).
type MarginSummaryBodyEncoder struct{}

func (MarginSummaryBodyEncoder) Encode(value MarginSummary, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeF64BE(&local, value.Balance)
	writeF64BE(&local, value.BuyingPower)
	writeF64BE(&local, value.Equity)
	writeF64BE(&local, value.MarginUsed)
	writeF64BE(&local, value.Available)
	writeString(&local, value.Currency)
	*buf = local
}

// SBE body decoder for MarginSummary: reads fields at pos, returns the value and the new offset.
func MarginSummaryBodyDecoderDecode(buf []byte, pos int) (MarginSummary, int, error) {
	account := readString(buf, &pos)
	balance := readF64BE(buf, &pos)
	buying_power := readF64BE(buf, &pos)
	equity := readF64BE(buf, &pos)
	margin_used := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	currency := readString(buf, &pos)
	value := MarginSummary{
		Account:     account,
		Balance:     balance,
		BuyingPower: buying_power,
		Equity:      equity,
		MarginUsed:  margin_used,
		Available:   available,
		Currency:    currency,
	}
	return value, pos, nil
}

// SBE encoder for MarginSummary
type MarginSummaryEncoder struct{}

// Encode builds a complete message (header + body).
func (MarginSummaryEncoder) Encode(Account string, Balance float64, BuyingPower float64, Equity float64, MarginUsed float64, Available float64, Currency string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 32)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 40)
	writeString(&buf, Account)
	writeF64BE(&buf, Balance)
	writeF64BE(&buf, BuyingPower)
	writeF64BE(&buf, Equity)
	writeF64BE(&buf, MarginUsed)
	writeF64BE(&buf, Available)
	writeString(&buf, Currency)
	return buf, nil
}

// SBE decoder for MarginSummary: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MarginSummaryDecoderDecode(buf []byte) (MarginSummary, int, error) {
	if len(buf) < 8 {
		return MarginSummary{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarginSummary{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 32 {
		return MarginSummary{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	balance := readF64BE(buf, &pos)
	buying_power := readF64BE(buf, &pos)
	equity := readF64BE(buf, &pos)
	margin_used := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	currency := readString(buf, &pos)
	value := MarginSummary{
		Account:     account,
		Balance:     balance,
		BuyingPower: buying_power,
		Equity:      equity,
		MarginUsed:  margin_used,
		Available:   available,
		Currency:    currency,
	}
	return value, pos, nil
}

// SBE message BalanceUpdate
type BalanceUpdate struct {
	Account   string
	Asset     string
	Delta     float64
	Total     float64
	Available float64
	Reason    BalanceUpdateReason
}

// SBE body encoder for BalanceUpdate (fields only, no header).
type BalanceUpdateBodyEncoder struct{}

func (BalanceUpdateBodyEncoder) Encode(value BalanceUpdate, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeString(&local, value.Asset)
	writeF64BE(&local, value.Delta)
	writeF64BE(&local, value.Total)
	writeF64BE(&local, value.Available)
	local = append(local, byte(value.Reason.ToValue()))
	*buf = local
}

// SBE body decoder for BalanceUpdate: reads fields at pos, returns the value and the new offset.
func BalanceUpdateBodyDecoderDecode(buf []byte, pos int) (BalanceUpdate, int, error) {
	account := readString(buf, &pos)
	asset := readString(buf, &pos)
	delta := readF64BE(buf, &pos)
	total := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	reason_raw := readU8(buf, &pos)
	reason, reason_err := BalanceUpdateReasonFromValue(reason_raw)
	if reason_err != nil {
		return BalanceUpdate{}, pos, reason_err
	}
	value := BalanceUpdate{
		Account:   account,
		Asset:     asset,
		Delta:     delta,
		Total:     total,
		Available: available,
		Reason:    reason,
	}
	return value, pos, nil
}

// SBE encoder for BalanceUpdate
type BalanceUpdateEncoder struct{}

// Encode builds a complete message (header + body).
func (BalanceUpdateEncoder) Encode(Account string, Asset string, Delta float64, Total float64, Available float64, Reason BalanceUpdateReason) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 34)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 25)
	writeString(&buf, Account)
	writeString(&buf, Asset)
	writeF64BE(&buf, Delta)
	writeF64BE(&buf, Total)
	writeF64BE(&buf, Available)
	buf = append(buf, byte(Reason.ToValue()))
	return buf, nil
}

// SBE decoder for BalanceUpdate: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func BalanceUpdateDecoderDecode(buf []byte) (BalanceUpdate, int, error) {
	if len(buf) < 8 {
		return BalanceUpdate{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BalanceUpdate{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 34 {
		return BalanceUpdate{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	asset := readString(buf, &pos)
	delta := readF64BE(buf, &pos)
	total := readF64BE(buf, &pos)
	available := readF64BE(buf, &pos)
	reason_raw := readU8(buf, &pos)
	reason, reason_err := BalanceUpdateReasonFromValue(reason_raw)
	if reason_err != nil {
		return BalanceUpdate{}, pos, reason_err
	}
	value := BalanceUpdate{
		Account:   account,
		Asset:     asset,
		Delta:     delta,
		Total:     total,
		Available: available,
		Reason:    reason,
	}
	return value, pos, nil
}

// SBE message PositionUpdate
type PositionUpdate struct {
	Account       string
	Symbol        string
	Qty           float64
	EntryPrice    float64
	UnrealizedPnl float64
}

// SBE body encoder for PositionUpdate (fields only, no header).
type PositionUpdateBodyEncoder struct{}

func (PositionUpdateBodyEncoder) Encode(value PositionUpdate, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeString(&local, value.Symbol)
	writeF64BE(&local, value.Qty)
	writeF64BE(&local, value.EntryPrice)
	writeF64BE(&local, value.UnrealizedPnl)
	*buf = local
}

// SBE body decoder for PositionUpdate: reads fields at pos, returns the value and the new offset.
func PositionUpdateBodyDecoderDecode(buf []byte, pos int) (PositionUpdate, int, error) {
	account := readString(buf, &pos)
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	entry_price := readF64BE(buf, &pos)
	unrealized_pnl := readF64BE(buf, &pos)
	value := PositionUpdate{
		Account:       account,
		Symbol:        symbol,
		Qty:           qty,
		EntryPrice:    entry_price,
		UnrealizedPnl: unrealized_pnl,
	}
	return value, pos, nil
}

// SBE encoder for PositionUpdate
type PositionUpdateEncoder struct{}

// Encode builds a complete message (header + body).
func (PositionUpdateEncoder) Encode(Account string, Symbol string, Qty float64, EntryPrice float64, UnrealizedPnl float64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 36)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 24)
	writeString(&buf, Account)
	writeString(&buf, Symbol)
	writeF64BE(&buf, Qty)
	writeF64BE(&buf, EntryPrice)
	writeF64BE(&buf, UnrealizedPnl)
	return buf, nil
}

// SBE decoder for PositionUpdate: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PositionUpdateDecoderDecode(buf []byte) (PositionUpdate, int, error) {
	if len(buf) < 8 {
		return PositionUpdate{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PositionUpdate{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 36 {
		return PositionUpdate{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	entry_price := readF64BE(buf, &pos)
	unrealized_pnl := readF64BE(buf, &pos)
	value := PositionUpdate{
		Account:       account,
		Symbol:        symbol,
		Qty:           qty,
		EntryPrice:    entry_price,
		UnrealizedPnl: unrealized_pnl,
	}
	return value, pos, nil
}

// SBE message UserLiquidation
type UserLiquidation struct {
	Account   string
	Symbol    string
	Qty       float64
	Price     float64
	Timestamp int64
}

// SBE body encoder for UserLiquidation (fields only, no header).
type UserLiquidationBodyEncoder struct{}

func (UserLiquidationBodyEncoder) Encode(value UserLiquidation, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeString(&local, value.Symbol)
	writeF64BE(&local, value.Qty)
	writeF64BE(&local, value.Price)
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// SBE body decoder for UserLiquidation: reads fields at pos, returns the value and the new offset.
func UserLiquidationBodyDecoderDecode(buf []byte, pos int) (UserLiquidation, int, error) {
	account := readString(buf, &pos)
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	price := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	value := UserLiquidation{
		Account:   account,
		Symbol:    symbol,
		Qty:       qty,
		Price:     price,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE encoder for UserLiquidation
type UserLiquidationEncoder struct{}

// Encode builds a complete message (header + body).
func (UserLiquidationEncoder) Encode(Account string, Symbol string, Qty float64, Price float64, Timestamp int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 38)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 24)
	writeString(&buf, Account)
	writeString(&buf, Symbol)
	writeF64BE(&buf, Qty)
	writeF64BE(&buf, Price)
	writeU64BE(&buf, uint64(Timestamp))
	return buf, nil
}

// SBE decoder for UserLiquidation: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func UserLiquidationDecoderDecode(buf []byte) (UserLiquidation, int, error) {
	if len(buf) < 8 {
		return UserLiquidation{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return UserLiquidation{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 38 {
		return UserLiquidation{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	symbol := readString(buf, &pos)
	qty := readF64BE(buf, &pos)
	price := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	value := UserLiquidation{
		Account:   account,
		Symbol:    symbol,
		Qty:       qty,
		Price:     price,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE message OrderListStatus
type OrderListStatus struct {
	Account string
	ListId  string
	Status  OrderListStatusStatus
	Symbol  *string
}

// SBE body encoder for OrderListStatus (fields only, no header).
type OrderListStatusBodyEncoder struct{}

func (OrderListStatusBodyEncoder) Encode(value OrderListStatus, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeString(&local, value.ListId)
	local = append(local, byte(value.Status.ToValue()))
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OrderListStatus: reads fields at pos, returns the value and the new offset.
func OrderListStatusBodyDecoderDecode(buf []byte, pos int) (OrderListStatus, int, error) {
	var symbol *string
	account := readString(buf, &pos)
	list_id := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := OrderListStatusStatusFromValue(status_raw)
	if status_err != nil {
		return OrderListStatus{}, pos, status_err
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	value := OrderListStatus{
		Account: account,
		ListId:  list_id,
		Status:  status,
		Symbol:  symbol,
	}
	return value, pos, nil
}

// SBE encoder for OrderListStatus
type OrderListStatusEncoder struct{}

// Encode builds a complete message (header + body).
func (OrderListStatusEncoder) Encode(Account string, ListId string, Status OrderListStatusStatus, Symbol *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 39)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
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

// SBE decoder for OrderListStatus: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OrderListStatusDecoderDecode(buf []byte) (OrderListStatus, int, error) {
	if len(buf) < 8 {
		return OrderListStatus{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderListStatus{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 39 {
		return OrderListStatus{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	account := readString(buf, &pos)
	list_id := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := OrderListStatusStatusFromValue(status_raw)
	if status_err != nil {
		return OrderListStatus{}, pos, status_err
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	value := OrderListStatus{
		Account: account,
		ListId:  list_id,
		Status:  status,
		Symbol:  symbol,
	}
	return value, pos, nil
}

// SBE message FillHistoryRequest
type FillHistoryRequest struct {
	Account   string
	Symbol    *string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for FillHistoryRequest (fields only, no header).
type FillHistoryRequestBodyEncoder struct{}

func (FillHistoryRequestBodyEncoder) Encode(value FillHistoryRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for FillHistoryRequest: reads fields at pos, returns the value and the new offset.
func FillHistoryRequestBodyDecoderDecode(buf []byte, pos int) (FillHistoryRequest, int, error) {
	var symbol *string
	var cursor *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := FillHistoryRequest{
		Account:   account,
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for FillHistoryRequest
type FillHistoryRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (FillHistoryRequestEncoder) Encode(Account string, Symbol *string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 40)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for FillHistoryRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func FillHistoryRequestDecoderDecode(buf []byte) (FillHistoryRequest, int, error) {
	if len(buf) < 8 {
		return FillHistoryRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FillHistoryRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 40 {
		return FillHistoryRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	var cursor *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := FillHistoryRequest{
		Account:   account,
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message FundingPayment
type FundingPayment struct {
	Account   string
	Symbol    *string
	Amount    float64
	Rate      float64
	Timestamp int64
}

// SBE body encoder for FundingPayment (fields only, no header).
type FundingPaymentBodyEncoder struct{}

func (FundingPaymentBodyEncoder) Encode(value FundingPayment, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	writeF64BE(&local, value.Amount)
	writeF64BE(&local, value.Rate)
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// SBE body decoder for FundingPayment: reads fields at pos, returns the value and the new offset.
func FundingPaymentBodyDecoderDecode(buf []byte, pos int) (FundingPayment, int, error) {
	var symbol *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	amount := readF64BE(buf, &pos)
	rate := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	value := FundingPayment{
		Account:   account,
		Symbol:    symbol,
		Amount:    amount,
		Rate:      rate,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE encoder for FundingPayment
type FundingPaymentEncoder struct{}

// Encode builds a complete message (header + body).
func (FundingPaymentEncoder) Encode(Account string, Symbol *string, Amount float64, Rate float64, Timestamp int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 42)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 24)
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	writeF64BE(&buf, Amount)
	writeF64BE(&buf, Rate)
	writeU64BE(&buf, uint64(Timestamp))
	return buf, nil
}

// SBE decoder for FundingPayment: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func FundingPaymentDecoderDecode(buf []byte) (FundingPayment, int, error) {
	if len(buf) < 8 {
		return FundingPayment{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FundingPayment{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 42 {
		return FundingPayment{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	amount := readF64BE(buf, &pos)
	rate := readF64BE(buf, &pos)
	timestamp := int64(readU64BE(buf, &pos))
	value := FundingPayment{
		Account:   account,
		Symbol:    symbol,
		Amount:    amount,
		Rate:      rate,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE message FundingHistoryRequest
type FundingHistoryRequest struct {
	Account   string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for FundingHistoryRequest (fields only, no header).
type FundingHistoryRequestBodyEncoder struct{}

func (FundingHistoryRequestBodyEncoder) Encode(value FundingHistoryRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for FundingHistoryRequest: reads fields at pos, returns the value and the new offset.
func FundingHistoryRequestBodyDecoderDecode(buf []byte, pos int) (FundingHistoryRequest, int, error) {
	var cursor *string
	account := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := FundingHistoryRequest{
		Account:   account,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for FundingHistoryRequest
type FundingHistoryRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (FundingHistoryRequestEncoder) Encode(Account string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 43)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Account)
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for FundingHistoryRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func FundingHistoryRequestDecoderDecode(buf []byte) (FundingHistoryRequest, int, error) {
	if len(buf) < 8 {
		return FundingHistoryRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FundingHistoryRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 43 {
		return FundingHistoryRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var cursor *string
	account := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := FundingHistoryRequest{
		Account:   account,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message LedgerUpdate
type LedgerUpdate struct {
	Account     string
	Asset       string
	Delta       float64
	Kind        LedgerUpdateKind
	Timestamp   int64
	ReferenceId *string
}

// SBE body encoder for LedgerUpdate (fields only, no header).
type LedgerUpdateBodyEncoder struct{}

func (LedgerUpdateBodyEncoder) Encode(value LedgerUpdate, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeString(&local, value.Asset)
	writeF64BE(&local, value.Delta)
	local = append(local, byte(value.Kind.ToValue()))
	writeU64BE(&local, uint64(value.Timestamp))
	if value.ReferenceId != nil {
		local = append(local, 1)
		writeString(&local, *value.ReferenceId)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for LedgerUpdate: reads fields at pos, returns the value and the new offset.
func LedgerUpdateBodyDecoderDecode(buf []byte, pos int) (LedgerUpdate, int, error) {
	var reference_id *string
	account := readString(buf, &pos)
	asset := readString(buf, &pos)
	delta := readF64BE(buf, &pos)
	kind_raw := readU8(buf, &pos)
	kind, kind_err := LedgerUpdateKindFromValue(kind_raw)
	if kind_err != nil {
		return LedgerUpdate{}, pos, kind_err
	}
	timestamp := int64(readU64BE(buf, &pos))
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		reference_id = &s
	} else {
		pos++
	}
	value := LedgerUpdate{
		Account:     account,
		Asset:       asset,
		Delta:       delta,
		Kind:        kind,
		Timestamp:   timestamp,
		ReferenceId: reference_id,
	}
	return value, pos, nil
}

// SBE encoder for LedgerUpdate
type LedgerUpdateEncoder struct{}

// Encode builds a complete message (header + body).
func (LedgerUpdateEncoder) Encode(Account string, Asset string, Delta float64, Kind LedgerUpdateKind, Timestamp int64, ReferenceId *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 45)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)
	writeString(&buf, Account)
	writeString(&buf, Asset)
	writeF64BE(&buf, Delta)
	buf = append(buf, byte(Kind.ToValue()))
	writeU64BE(&buf, uint64(Timestamp))
	if ReferenceId != nil {
		buf = append(buf, 1)
		writeString(&buf, *ReferenceId)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for LedgerUpdate: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func LedgerUpdateDecoderDecode(buf []byte) (LedgerUpdate, int, error) {
	if len(buf) < 8 {
		return LedgerUpdate{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LedgerUpdate{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 45 {
		return LedgerUpdate{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var reference_id *string
	account := readString(buf, &pos)
	asset := readString(buf, &pos)
	delta := readF64BE(buf, &pos)
	kind_raw := readU8(buf, &pos)
	kind, kind_err := LedgerUpdateKindFromValue(kind_raw)
	if kind_err != nil {
		return LedgerUpdate{}, pos, kind_err
	}
	timestamp := int64(readU64BE(buf, &pos))
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		reference_id = &s
	} else {
		pos++
	}
	value := LedgerUpdate{
		Account:     account,
		Asset:       asset,
		Delta:       delta,
		Kind:        kind,
		Timestamp:   timestamp,
		ReferenceId: reference_id,
	}
	return value, pos, nil
}

// SBE message LedgerHistoryRequest
type LedgerHistoryRequest struct {
	Account   string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for LedgerHistoryRequest (fields only, no header).
type LedgerHistoryRequestBodyEncoder struct{}

func (LedgerHistoryRequestBodyEncoder) Encode(value LedgerHistoryRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for LedgerHistoryRequest: reads fields at pos, returns the value and the new offset.
func LedgerHistoryRequestBodyDecoderDecode(buf []byte, pos int) (LedgerHistoryRequest, int, error) {
	var cursor *string
	account := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := LedgerHistoryRequest{
		Account:   account,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for LedgerHistoryRequest
type LedgerHistoryRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (LedgerHistoryRequestEncoder) Encode(Account string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 46)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Account)
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for LedgerHistoryRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func LedgerHistoryRequestDecoderDecode(buf []byte) (LedgerHistoryRequest, int, error) {
	if len(buf) < 8 {
		return LedgerHistoryRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LedgerHistoryRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 46 {
		return LedgerHistoryRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var cursor *string
	account := readString(buf, &pos)
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := LedgerHistoryRequest{
		Account:   account,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message OpenOrdersRequest
type OpenOrdersRequest struct {
	Account string
	Symbol  *string
}

// SBE body encoder for OpenOrdersRequest (fields only, no header).
type OpenOrdersRequestBodyEncoder struct{}

func (OpenOrdersRequestBodyEncoder) Encode(value OpenOrdersRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OpenOrdersRequest: reads fields at pos, returns the value and the new offset.
func OpenOrdersRequestBodyDecoderDecode(buf []byte, pos int) (OpenOrdersRequest, int, error) {
	var symbol *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	value := OpenOrdersRequest{
		Account: account,
		Symbol:  symbol,
	}
	return value, pos, nil
}

// SBE encoder for OpenOrdersRequest
type OpenOrdersRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (OpenOrdersRequestEncoder) Encode(Account string, Symbol *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 48)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OpenOrdersRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OpenOrdersRequestDecoderDecode(buf []byte) (OpenOrdersRequest, int, error) {
	if len(buf) < 8 {
		return OpenOrdersRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OpenOrdersRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 48 {
		return OpenOrdersRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	value := OpenOrdersRequest{
		Account: account,
		Symbol:  symbol,
	}
	return value, pos, nil
}

// SBE message OrderHistoryRequest
type OrderHistoryRequest struct {
	Account   string
	Symbol    *string
	StartTime *int64
	EndTime   *int64
	Limit     *uint32
	Cursor    *string
}

// SBE body encoder for OrderHistoryRequest (fields only, no header).
type OrderHistoryRequestBodyEncoder struct{}

func (OrderHistoryRequestBodyEncoder) Encode(value OrderHistoryRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	var value_StartTime_v int64
	if value.StartTime != nil {
		value_StartTime_v = *value.StartTime
	}
	writeU64BE(&local, uint64(value_StartTime_v))
	if value.StartTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_EndTime_v int64
	if value.EndTime != nil {
		value_EndTime_v = *value.EndTime
	}
	writeU64BE(&local, uint64(value_EndTime_v))
	if value.EndTime != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_Limit_v uint32
	if value.Limit != nil {
		value_Limit_v = *value.Limit
	}
	writeU32BE(&local, uint32(value_Limit_v))
	if value.Limit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cursor != nil {
		local = append(local, 1)
		writeString(&local, *value.Cursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OrderHistoryRequest: reads fields at pos, returns the value and the new offset.
func OrderHistoryRequestBodyDecoderDecode(buf []byte, pos int) (OrderHistoryRequest, int, error) {
	var symbol *string
	var cursor *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := OrderHistoryRequest{
		Account:   account,
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE encoder for OrderHistoryRequest
type OrderHistoryRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (OrderHistoryRequestEncoder) Encode(Account string, Symbol *string, StartTime *int64, EndTime *int64, Limit *uint32, Cursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 50)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 20)
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	var StartTime_v int64
	if StartTime != nil {
		StartTime_v = *StartTime
	}
	writeU64BE(&buf, uint64(StartTime_v))
	if StartTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var EndTime_v int64
	if EndTime != nil {
		EndTime_v = *EndTime
	}
	writeU64BE(&buf, uint64(EndTime_v))
	if EndTime != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var Limit_v uint32
	if Limit != nil {
		Limit_v = *Limit
	}
	writeU32BE(&buf, uint32(Limit_v))
	if Limit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	if Cursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *Cursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OrderHistoryRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OrderHistoryRequestDecoderDecode(buf []byte) (OrderHistoryRequest, int, error) {
	if len(buf) < 8 {
		return OrderHistoryRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderHistoryRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 50 {
		return OrderHistoryRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	var cursor *string
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	start_time_v := int64(readU64BE(buf, &pos))
	var start_time *int64
	if buf[pos] == 1 {
		pos++
		start_time = &start_time_v
	} else {
		pos++
		start_time = nil
	}
	end_time_v := int64(readU64BE(buf, &pos))
	var end_time *int64
	if buf[pos] == 1 {
		pos++
		end_time = &end_time_v
	} else {
		pos++
		end_time = nil
	}
	limit_v := readU32BE(buf, &pos)
	var limit *uint32
	if buf[pos] == 1 {
		pos++
		limit = &limit_v
	} else {
		pos++
		limit = nil
	}
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		cursor = &s
	} else {
		pos++
	}
	value := OrderHistoryRequest{
		Account:   account,
		Symbol:    symbol,
		StartTime: start_time,
		EndTime:   end_time,
		Limit:     limit,
		Cursor:    cursor,
	}
	return value, pos, nil
}

// SBE message CapabilitiesRequest
type CapabilitiesRequest struct {
}

// SBE body encoder for CapabilitiesRequest (fields only, no header).
type CapabilitiesRequestBodyEncoder struct{}

func (CapabilitiesRequestBodyEncoder) Encode(value CapabilitiesRequest, buf *[]byte) {
	local := *buf
	*buf = local
}

// SBE body decoder for CapabilitiesRequest: reads fields at pos, returns the value and the new offset.
func CapabilitiesRequestBodyDecoderDecode(buf []byte, pos int) (CapabilitiesRequest, int, error) {
	value := CapabilitiesRequest{}
	return value, pos, nil
}

// SBE encoder for CapabilitiesRequest
type CapabilitiesRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (CapabilitiesRequestEncoder) Encode() ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 52)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	return buf, nil
}

// SBE decoder for CapabilitiesRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CapabilitiesRequestDecoderDecode(buf []byte) (CapabilitiesRequest, int, error) {
	if len(buf) < 8 {
		return CapabilitiesRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CapabilitiesRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 52 {
		return CapabilitiesRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	value := CapabilitiesRequest{}
	return value, pos, nil
}

// SBE message PositionRequest
type PositionRequest struct {
	Account string
}

// SBE body encoder for PositionRequest (fields only, no header).
type PositionRequestBodyEncoder struct{}

func (PositionRequestBodyEncoder) Encode(value PositionRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	*buf = local
}

// SBE body decoder for PositionRequest: reads fields at pos, returns the value and the new offset.
func PositionRequestBodyDecoderDecode(buf []byte, pos int) (PositionRequest, int, error) {
	account := readString(buf, &pos)
	value := PositionRequest{
		Account: account,
	}
	return value, pos, nil
}

// SBE encoder for PositionRequest
type PositionRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (PositionRequestEncoder) Encode(Account string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 54)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, Account)
	return buf, nil
}

// SBE decoder for PositionRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PositionRequestDecoderDecode(buf []byte) (PositionRequest, int, error) {
	if len(buf) < 8 {
		return PositionRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PositionRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 54 {
		return PositionRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	value := PositionRequest{
		Account: account,
	}
	return value, pos, nil
}

// SBE message TradingSessionStatusRequest
type TradingSessionStatusRequest struct {
	Symbol *string
}

// SBE body encoder for TradingSessionStatusRequest (fields only, no header).
type TradingSessionStatusRequestBodyEncoder struct{}

func (TradingSessionStatusRequestBodyEncoder) Encode(value TradingSessionStatusRequest, buf *[]byte) {
	local := *buf
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for TradingSessionStatusRequest: reads fields at pos, returns the value and the new offset.
func TradingSessionStatusRequestBodyDecoderDecode(buf []byte, pos int) (TradingSessionStatusRequest, int, error) {
	var symbol *string
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	value := TradingSessionStatusRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE encoder for TradingSessionStatusRequest
type TradingSessionStatusRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (TradingSessionStatusRequestEncoder) Encode(Symbol *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 55)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for TradingSessionStatusRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func TradingSessionStatusRequestDecoderDecode(buf []byte) (TradingSessionStatusRequest, int, error) {
	if len(buf) < 8 {
		return TradingSessionStatusRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return TradingSessionStatusRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 55 {
		return TradingSessionStatusRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	value := TradingSessionStatusRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE message TradingSessionStatus
type TradingSessionStatus struct {
	Symbol           *string
	TradingSessionId string
	Status           TradingSessionStatusStatus
	HaltReason       *TradingSessionStatusHaltReason
	Timestamp        int64
	IsSnapshot       *uint8
}

// SBE body encoder for TradingSessionStatus (fields only, no header).
type TradingSessionStatusBodyEncoder struct{}

func (TradingSessionStatusBodyEncoder) Encode(value TradingSessionStatus, buf *[]byte) {
	local := *buf
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	writeString(&local, value.TradingSessionId)
	local = append(local, byte(value.Status.ToValue()))
	value_HaltReason_v := uint8(0)
	if value.HaltReason != nil {
		value_HaltReason_v = byte(value.HaltReason.ToValue())
	}
	local = append(local, value_HaltReason_v)
	if value.HaltReason != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for TradingSessionStatus: reads fields at pos, returns the value and the new offset.
func TradingSessionStatusBodyDecoderDecode(buf []byte, pos int) (TradingSessionStatus, int, error) {
	var symbol *string
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	trading_session_id := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := TradingSessionStatusStatusFromValue(status_raw)
	if status_err != nil {
		return TradingSessionStatus{}, pos, status_err
	}
	halt_reason_raw := readU8(buf, &pos)
	var halt_reason *TradingSessionStatusHaltReason
	if buf[pos] == 1 {
		pos++
		e, halt_reason_err := TradingSessionStatusHaltReasonFromValue(halt_reason_raw)
		if halt_reason_err != nil {
			return TradingSessionStatus{}, pos, halt_reason_err
		}
		halt_reason = &e
	} else {
		pos++
		halt_reason = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := TradingSessionStatus{
		Symbol:           symbol,
		TradingSessionId: trading_session_id,
		Status:           status,
		HaltReason:       halt_reason,
		Timestamp:        timestamp,
		IsSnapshot:       is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for TradingSessionStatus
type TradingSessionStatusEncoder struct{}

// Encode builds a complete message (header + body).
func (TradingSessionStatusEncoder) Encode(Symbol *string, TradingSessionId string, Status TradingSessionStatusStatus, HaltReason *TradingSessionStatusHaltReason, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 56)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 11)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	writeString(&buf, TradingSessionId)
	buf = append(buf, byte(Status.ToValue()))
	HaltReason_v := uint8(0)
	if HaltReason != nil {
		HaltReason_v = byte(HaltReason.ToValue())
	}
	buf = append(buf, HaltReason_v)
	if HaltReason != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for TradingSessionStatus: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func TradingSessionStatusDecoderDecode(buf []byte) (TradingSessionStatus, int, error) {
	if len(buf) < 8 {
		return TradingSessionStatus{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return TradingSessionStatus{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 56 {
		return TradingSessionStatus{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	trading_session_id := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := TradingSessionStatusStatusFromValue(status_raw)
	if status_err != nil {
		return TradingSessionStatus{}, pos, status_err
	}
	halt_reason_raw := readU8(buf, &pos)
	var halt_reason *TradingSessionStatusHaltReason
	if buf[pos] == 1 {
		pos++
		e, halt_reason_err := TradingSessionStatusHaltReasonFromValue(halt_reason_raw)
		if halt_reason_err != nil {
			return TradingSessionStatus{}, pos, halt_reason_err
		}
		halt_reason = &e
	} else {
		pos++
		halt_reason = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := TradingSessionStatus{
		Symbol:           symbol,
		TradingSessionId: trading_session_id,
		Status:           status,
		HaltReason:       halt_reason,
		Timestamp:        timestamp,
		IsSnapshot:       is_snapshot,
	}
	return value, pos, nil
}

// SBE message SecurityStatusRequest
type SecurityStatusRequest struct {
	Symbol string
}

// SBE body encoder for SecurityStatusRequest (fields only, no header).
type SecurityStatusRequestBodyEncoder struct{}

func (SecurityStatusRequestBodyEncoder) Encode(value SecurityStatusRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	*buf = local
}

// SBE body decoder for SecurityStatusRequest: reads fields at pos, returns the value and the new offset.
func SecurityStatusRequestBodyDecoderDecode(buf []byte, pos int) (SecurityStatusRequest, int, error) {
	symbol := readString(buf, &pos)
	value := SecurityStatusRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE encoder for SecurityStatusRequest
type SecurityStatusRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (SecurityStatusRequestEncoder) Encode(Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 57)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, Symbol)
	return buf, nil
}

// SBE decoder for SecurityStatusRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func SecurityStatusRequestDecoderDecode(buf []byte) (SecurityStatusRequest, int, error) {
	if len(buf) < 8 {
		return SecurityStatusRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return SecurityStatusRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 57 {
		return SecurityStatusRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	value := SecurityStatusRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE message SecurityStatus
type SecurityStatus struct {
	Symbol     string
	Status     SecurityStatusStatus
	HaltReason *SecurityStatusHaltReason
	Timestamp  int64
	IsSnapshot *uint8
}

// SBE body encoder for SecurityStatus (fields only, no header).
type SecurityStatusBodyEncoder struct{}

func (SecurityStatusBodyEncoder) Encode(value SecurityStatus, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	local = append(local, byte(value.Status.ToValue()))
	value_HaltReason_v := uint8(0)
	if value.HaltReason != nil {
		value_HaltReason_v = byte(value.HaltReason.ToValue())
	}
	local = append(local, value_HaltReason_v)
	if value.HaltReason != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for SecurityStatus: reads fields at pos, returns the value and the new offset.
func SecurityStatusBodyDecoderDecode(buf []byte, pos int) (SecurityStatus, int, error) {
	symbol := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := SecurityStatusStatusFromValue(status_raw)
	if status_err != nil {
		return SecurityStatus{}, pos, status_err
	}
	halt_reason_raw := readU8(buf, &pos)
	var halt_reason *SecurityStatusHaltReason
	if buf[pos] == 1 {
		pos++
		e, halt_reason_err := SecurityStatusHaltReasonFromValue(halt_reason_raw)
		if halt_reason_err != nil {
			return SecurityStatus{}, pos, halt_reason_err
		}
		halt_reason = &e
	} else {
		pos++
		halt_reason = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := SecurityStatus{
		Symbol:     symbol,
		Status:     status,
		HaltReason: halt_reason,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for SecurityStatus
type SecurityStatusEncoder struct{}

// Encode builds a complete message (header + body).
func (SecurityStatusEncoder) Encode(Symbol string, Status SecurityStatusStatus, HaltReason *SecurityStatusHaltReason, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 58)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 11)
	writeString(&buf, Symbol)
	buf = append(buf, byte(Status.ToValue()))
	HaltReason_v := uint8(0)
	if HaltReason != nil {
		HaltReason_v = byte(HaltReason.ToValue())
	}
	buf = append(buf, HaltReason_v)
	if HaltReason != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for SecurityStatus: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func SecurityStatusDecoderDecode(buf []byte) (SecurityStatus, int, error) {
	if len(buf) < 8 {
		return SecurityStatus{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return SecurityStatus{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 58 {
		return SecurityStatus{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := SecurityStatusStatusFromValue(status_raw)
	if status_err != nil {
		return SecurityStatus{}, pos, status_err
	}
	halt_reason_raw := readU8(buf, &pos)
	var halt_reason *SecurityStatusHaltReason
	if buf[pos] == 1 {
		pos++
		e, halt_reason_err := SecurityStatusHaltReasonFromValue(halt_reason_raw)
		if halt_reason_err != nil {
			return SecurityStatus{}, pos, halt_reason_err
		}
		halt_reason = &e
	} else {
		pos++
		halt_reason = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := SecurityStatus{
		Symbol:     symbol,
		Status:     status,
		HaltReason: halt_reason,
		Timestamp:  timestamp,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message MassCancelRequest
type MassCancelRequest struct {
	ClOrdId string
	Account string
	Symbol  *string
	Side    *MassCancelRequestSide
}

// SBE body encoder for MassCancelRequest (fields only, no header).
type MassCancelRequestBodyEncoder struct{}

func (MassCancelRequestBodyEncoder) Encode(value MassCancelRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	writeString(&local, value.Account)
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	value_Side_v := uint8(0)
	if value.Side != nil {
		value_Side_v = byte(value.Side.ToValue())
	}
	local = append(local, value_Side_v)
	if value.Side != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for MassCancelRequest: reads fields at pos, returns the value and the new offset.
func MassCancelRequestBodyDecoderDecode(buf []byte, pos int) (MassCancelRequest, int, error) {
	var symbol *string
	cl_ord_id := readString(buf, &pos)
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	side_raw := readU8(buf, &pos)
	var side *MassCancelRequestSide
	if buf[pos] == 1 {
		pos++
		e, side_err := MassCancelRequestSideFromValue(side_raw)
		if side_err != nil {
			return MassCancelRequest{}, pos, side_err
		}
		side = &e
	} else {
		pos++
		side = nil
	}
	value := MassCancelRequest{
		ClOrdId: cl_ord_id,
		Account: account,
		Symbol:  symbol,
		Side:    side,
	}
	return value, pos, nil
}

// SBE encoder for MassCancelRequest
type MassCancelRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (MassCancelRequestEncoder) Encode(ClOrdId string, Account string, Symbol *string, Side *MassCancelRequestSide) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 59)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, ClOrdId)
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	Side_v := uint8(0)
	if Side != nil {
		Side_v = byte(Side.ToValue())
	}
	buf = append(buf, Side_v)
	if Side != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for MassCancelRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MassCancelRequestDecoderDecode(buf []byte) (MassCancelRequest, int, error) {
	if len(buf) < 8 {
		return MassCancelRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MassCancelRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 59 {
		return MassCancelRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	cl_ord_id := readString(buf, &pos)
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	side_raw := readU8(buf, &pos)
	var side *MassCancelRequestSide
	if buf[pos] == 1 {
		pos++
		e, side_err := MassCancelRequestSideFromValue(side_raw)
		if side_err != nil {
			return MassCancelRequest{}, pos, side_err
		}
		side = &e
	} else {
		pos++
		side = nil
	}
	value := MassCancelRequest{
		ClOrdId: cl_ord_id,
		Account: account,
		Symbol:  symbol,
		Side:    side,
	}
	return value, pos, nil
}

// SBE message MassCancelReport
type MassCancelReport struct {
	ClOrdId       string
	Account       string
	Symbol        *string
	TotalAffected uint32
	Status        MassCancelReportStatus
	Timestamp     int64
}

// SBE body encoder for MassCancelReport (fields only, no header).
type MassCancelReportBodyEncoder struct{}

func (MassCancelReportBodyEncoder) Encode(value MassCancelReport, buf *[]byte) {
	local := *buf
	writeString(&local, value.ClOrdId)
	writeString(&local, value.Account)
	if value.Symbol != nil {
		local = append(local, 1)
		writeString(&local, *value.Symbol)
	} else {
		local = append(local, 0)
	}
	writeU32BE(&local, uint32(value.TotalAffected))
	local = append(local, byte(value.Status.ToValue()))
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// SBE body decoder for MassCancelReport: reads fields at pos, returns the value and the new offset.
func MassCancelReportBodyDecoderDecode(buf []byte, pos int) (MassCancelReport, int, error) {
	var symbol *string
	cl_ord_id := readString(buf, &pos)
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	total_affected := readU32BE(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := MassCancelReportStatusFromValue(status_raw)
	if status_err != nil {
		return MassCancelReport{}, pos, status_err
	}
	timestamp := int64(readU64BE(buf, &pos))
	value := MassCancelReport{
		ClOrdId:       cl_ord_id,
		Account:       account,
		Symbol:        symbol,
		TotalAffected: total_affected,
		Status:        status,
		Timestamp:     timestamp,
	}
	return value, pos, nil
}

// SBE encoder for MassCancelReport
type MassCancelReportEncoder struct{}

// Encode builds a complete message (header + body).
func (MassCancelReportEncoder) Encode(ClOrdId string, Account string, Symbol *string, TotalAffected uint32, Status MassCancelReportStatus, Timestamp int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 60)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 13)
	writeString(&buf, ClOrdId)
	writeString(&buf, Account)
	if Symbol != nil {
		buf = append(buf, 1)
		writeString(&buf, *Symbol)
	} else {
		buf = append(buf, 0)
	}
	writeU32BE(&buf, uint32(TotalAffected))
	buf = append(buf, byte(Status.ToValue()))
	writeU64BE(&buf, uint64(Timestamp))
	return buf, nil
}

// SBE decoder for MassCancelReport: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MassCancelReportDecoderDecode(buf []byte) (MassCancelReport, int, error) {
	if len(buf) < 8 {
		return MassCancelReport{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MassCancelReport{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 60 {
		return MassCancelReport{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var symbol *string
	cl_ord_id := readString(buf, &pos)
	account := readString(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		symbol = &s
	} else {
		pos++
	}
	total_affected := readU32BE(buf, &pos)
	status_raw := readU8(buf, &pos)
	status, status_err := MassCancelReportStatusFromValue(status_raw)
	if status_err != nil {
		return MassCancelReport{}, pos, status_err
	}
	timestamp := int64(readU64BE(buf, &pos))
	value := MassCancelReport{
		ClOrdId:       cl_ord_id,
		Account:       account,
		Symbol:        symbol,
		TotalAffected: total_affected,
		Status:        status,
		Timestamp:     timestamp,
	}
	return value, pos, nil
}

// SBE message PriceLimitsRequest
type PriceLimitsRequest struct {
	Symbol string
}

// SBE body encoder for PriceLimitsRequest (fields only, no header).
type PriceLimitsRequestBodyEncoder struct{}

func (PriceLimitsRequestBodyEncoder) Encode(value PriceLimitsRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	*buf = local
}

// SBE body decoder for PriceLimitsRequest: reads fields at pos, returns the value and the new offset.
func PriceLimitsRequestBodyDecoderDecode(buf []byte, pos int) (PriceLimitsRequest, int, error) {
	symbol := readString(buf, &pos)
	value := PriceLimitsRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE encoder for PriceLimitsRequest
type PriceLimitsRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (PriceLimitsRequestEncoder) Encode(Symbol string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 61)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, Symbol)
	return buf, nil
}

// SBE decoder for PriceLimitsRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PriceLimitsRequestDecoderDecode(buf []byte) (PriceLimitsRequest, int, error) {
	if len(buf) < 8 {
		return PriceLimitsRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PriceLimitsRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 61 {
		return PriceLimitsRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	value := PriceLimitsRequest{
		Symbol: symbol,
	}
	return value, pos, nil
}

// SBE message PriceLimits
type PriceLimits struct {
	Symbol         string
	LowerLimit     *float64
	UpperLimit     *float64
	ReferencePrice *float64
	Timestamp      int64
	IsSnapshot     *uint8
}

// SBE body encoder for PriceLimits (fields only, no header).
type PriceLimitsBodyEncoder struct{}

func (PriceLimitsBodyEncoder) Encode(value PriceLimits, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	var value_LowerLimit_v float64
	if value.LowerLimit != nil {
		value_LowerLimit_v = *value.LowerLimit
	}
	writeF64BE(&local, value_LowerLimit_v)
	if value.LowerLimit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_UpperLimit_v float64
	if value.UpperLimit != nil {
		value_UpperLimit_v = *value.UpperLimit
	}
	writeF64BE(&local, value_UpperLimit_v)
	if value.UpperLimit != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_ReferencePrice_v float64
	if value.ReferencePrice != nil {
		value_ReferencePrice_v = *value.ReferencePrice
	}
	writeF64BE(&local, value_ReferencePrice_v)
	if value.ReferencePrice != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for PriceLimits: reads fields at pos, returns the value and the new offset.
func PriceLimitsBodyDecoderDecode(buf []byte, pos int) (PriceLimits, int, error) {
	symbol := readString(buf, &pos)
	lower_limit_v := readF64BE(buf, &pos)
	var lower_limit *float64
	if buf[pos] == 1 {
		pos++
		lower_limit = &lower_limit_v
	} else {
		pos++
		lower_limit = nil
	}
	upper_limit_v := readF64BE(buf, &pos)
	var upper_limit *float64
	if buf[pos] == 1 {
		pos++
		upper_limit = &upper_limit_v
	} else {
		pos++
		upper_limit = nil
	}
	reference_price_v := readF64BE(buf, &pos)
	var reference_price *float64
	if buf[pos] == 1 {
		pos++
		reference_price = &reference_price_v
	} else {
		pos++
		reference_price = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := PriceLimits{
		Symbol:         symbol,
		LowerLimit:     lower_limit,
		UpperLimit:     upper_limit,
		ReferencePrice: reference_price,
		Timestamp:      timestamp,
		IsSnapshot:     is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for PriceLimits
type PriceLimitsEncoder struct{}

// Encode builds a complete message (header + body).
func (PriceLimitsEncoder) Encode(Symbol string, LowerLimit *float64, UpperLimit *float64, ReferencePrice *float64, Timestamp int64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 62)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 33)
	writeString(&buf, Symbol)
	var LowerLimit_v float64
	if LowerLimit != nil {
		LowerLimit_v = *LowerLimit
	}
	writeF64BE(&buf, LowerLimit_v)
	if LowerLimit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var UpperLimit_v float64
	if UpperLimit != nil {
		UpperLimit_v = *UpperLimit
	}
	writeF64BE(&buf, UpperLimit_v)
	if UpperLimit != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var ReferencePrice_v float64
	if ReferencePrice != nil {
		ReferencePrice_v = *ReferencePrice
	}
	writeF64BE(&buf, ReferencePrice_v)
	if ReferencePrice != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for PriceLimits: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PriceLimitsDecoderDecode(buf []byte) (PriceLimits, int, error) {
	if len(buf) < 8 {
		return PriceLimits{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PriceLimits{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 62 {
		return PriceLimits{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	lower_limit_v := readF64BE(buf, &pos)
	var lower_limit *float64
	if buf[pos] == 1 {
		pos++
		lower_limit = &lower_limit_v
	} else {
		pos++
		lower_limit = nil
	}
	upper_limit_v := readF64BE(buf, &pos)
	var upper_limit *float64
	if buf[pos] == 1 {
		pos++
		upper_limit = &upper_limit_v
	} else {
		pos++
		upper_limit = nil
	}
	reference_price_v := readF64BE(buf, &pos)
	var reference_price *float64
	if buf[pos] == 1 {
		pos++
		reference_price = &reference_price_v
	} else {
		pos++
		reference_price = nil
	}
	timestamp := int64(readU64BE(buf, &pos))
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := PriceLimits{
		Symbol:         symbol,
		LowerLimit:     lower_limit,
		UpperLimit:     upper_limit,
		ReferencePrice: reference_price,
		Timestamp:      timestamp,
		IsSnapshot:     is_snapshot,
	}
	return value, pos, nil
}

// SBE message AggregateTradeEvent
type AggregateTradeEvent struct {
	Trade AggregateTrade
}

// SBE body encoder for AggregateTradeEvent (fields only, no header).
type AggregateTradeEventBodyEncoder struct{}

func (AggregateTradeEventBodyEncoder) Encode(value AggregateTradeEvent, buf *[]byte) {
	local := *buf
	AggregateTradeEncoder{}.Encode(value.Trade, &local)
	*buf = local
}

// SBE body decoder for AggregateTradeEvent: reads fields at pos, returns the value and the new offset.
func AggregateTradeEventBodyDecoderDecode(buf []byte, pos int) (AggregateTradeEvent, int, error) {
	trade, trade_pos, trade_err := AggregateTradeDecoderDecode(buf, pos)
	if trade_err != nil {
		return AggregateTradeEvent{}, pos, trade_err
	}
	pos = trade_pos
	value := AggregateTradeEvent{
		Trade: trade,
	}
	return value, pos, nil
}

// SBE encoder for AggregateTradeEvent
type AggregateTradeEventEncoder struct{}

// Encode builds a complete message (header + body).
func (AggregateTradeEventEncoder) Encode(Trade AggregateTrade) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 11)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	AggregateTradeEncoder{}.Encode(Trade, &buf)
	return buf, nil
}

// SBE decoder for AggregateTradeEvent: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func AggregateTradeEventDecoderDecode(buf []byte) (AggregateTradeEvent, int, error) {
	if len(buf) < 8 {
		return AggregateTradeEvent{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AggregateTradeEvent{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 11 {
		return AggregateTradeEvent{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	trade, trade_pos, trade_err := AggregateTradeDecoderDecode(buf, pos)
	if trade_err != nil {
		return AggregateTradeEvent{}, pos, trade_err
	}
	pos = trade_pos
	value := AggregateTradeEvent{
		Trade: trade,
	}
	return value, pos, nil
}

// SBE message AggregateTradeBatch
type AggregateTradeBatch struct {
	Symbol     string
	Trades     []AggregateTrade
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for AggregateTradeBatch (fields only, no header).
type AggregateTradeBatchBodyEncoder struct{}

func (AggregateTradeBatchBodyEncoder) Encode(value AggregateTradeBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeU32BE(&local, uint32(len(value.Trades)))
	for _, item := range value.Trades {
		var item_buf []byte
		AggregateTradeEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for AggregateTradeBatch: reads fields at pos, returns the value and the new offset.
func AggregateTradeBatchBodyDecoderDecode(buf []byte, pos int) (AggregateTradeBatch, int, error) {
	var next_cursor *string
	symbol := readString(buf, &pos)
	trades_count := readU32BE(buf, &pos)
	trades := make([]AggregateTrade, 0, trades_count)
	for i := 0; i < int(trades_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := AggregateTradeDecoderDecode(buf, pos)
		if err != nil {
			return AggregateTradeBatch{}, pos, err
		}
		pos = item_end
		trades = append(trades, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := AggregateTradeBatch{
		Symbol:     symbol,
		Trades:     trades,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for AggregateTradeBatch
type AggregateTradeBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (AggregateTradeBatchEncoder) Encode(Symbol string, Trades []AggregateTrade, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 13)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Trades)))
	for _, item := range Trades {
		var item_buf []byte
		AggregateTradeEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for AggregateTradeBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func AggregateTradeBatchDecoderDecode(buf []byte) (AggregateTradeBatch, int, error) {
	if len(buf) < 8 {
		return AggregateTradeBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AggregateTradeBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 13 {
		return AggregateTradeBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	symbol := readString(buf, &pos)
	trades_count := readU32BE(buf, &pos)
	trades := make([]AggregateTrade, 0, trades_count)
	for i := 0; i < int(trades_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := AggregateTradeDecoderDecode(buf, pos)
		if err != nil {
			return AggregateTradeBatch{}, pos, err
		}
		pos = item_end
		trades = append(trades, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := AggregateTradeBatch{
		Symbol:     symbol,
		Trades:     trades,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message LiquidationTradeEvent
type LiquidationTradeEvent struct {
	Trade LiquidationTrade
}

// SBE body encoder for LiquidationTradeEvent (fields only, no header).
type LiquidationTradeEventBodyEncoder struct{}

func (LiquidationTradeEventBodyEncoder) Encode(value LiquidationTradeEvent, buf *[]byte) {
	local := *buf
	LiquidationTradeEncoder{}.Encode(value.Trade, &local)
	*buf = local
}

// SBE body decoder for LiquidationTradeEvent: reads fields at pos, returns the value and the new offset.
func LiquidationTradeEventBodyDecoderDecode(buf []byte, pos int) (LiquidationTradeEvent, int, error) {
	trade, trade_pos, trade_err := LiquidationTradeDecoderDecode(buf, pos)
	if trade_err != nil {
		return LiquidationTradeEvent{}, pos, trade_err
	}
	pos = trade_pos
	value := LiquidationTradeEvent{
		Trade: trade,
	}
	return value, pos, nil
}

// SBE encoder for LiquidationTradeEvent
type LiquidationTradeEventEncoder struct{}

// Encode builds a complete message (header + body).
func (LiquidationTradeEventEncoder) Encode(Trade LiquidationTrade) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 19)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	LiquidationTradeEncoder{}.Encode(Trade, &buf)
	return buf, nil
}

// SBE decoder for LiquidationTradeEvent: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func LiquidationTradeEventDecoderDecode(buf []byte) (LiquidationTradeEvent, int, error) {
	if len(buf) < 8 {
		return LiquidationTradeEvent{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LiquidationTradeEvent{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 19 {
		return LiquidationTradeEvent{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	trade, trade_pos, trade_err := LiquidationTradeDecoderDecode(buf, pos)
	if trade_err != nil {
		return LiquidationTradeEvent{}, pos, trade_err
	}
	pos = trade_pos
	value := LiquidationTradeEvent{
		Trade: trade,
	}
	return value, pos, nil
}

// SBE message MarketDataSnapshot
type MarketDataSnapshot struct {
	Symbol     string
	Exchange   string
	Bids       []PriceLevel
	Asks       []PriceLevel
	Timestamp  int64
	Sequence   *uint64
	IsSnapshot *uint8
}

// SBE body encoder for MarketDataSnapshot (fields only, no header).
type MarketDataSnapshotBodyEncoder struct{}

func (MarketDataSnapshotBodyEncoder) Encode(value MarketDataSnapshot, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.Exchange)
	writeU32BE(&local, uint32(len(value.Bids)))
	for _, item := range value.Bids {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU32BE(&local, uint32(len(value.Asks)))
	for _, item := range value.Asks {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_Sequence_v uint64
	if value.Sequence != nil {
		value_Sequence_v = *value.Sequence
	}
	writeU64BE(&local, uint64(value_Sequence_v))
	if value.Sequence != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for MarketDataSnapshot: reads fields at pos, returns the value and the new offset.
func MarketDataSnapshotBodyDecoderDecode(buf []byte, pos int) (MarketDataSnapshot, int, error) {
	symbol := readString(buf, &pos)
	exchange := readString(buf, &pos)
	bids_count := readU32BE(buf, &pos)
	bids := make([]PriceLevel, 0, bids_count)
	for i := 0; i < int(bids_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return MarketDataSnapshot{}, pos, err
		}
		pos = item_end
		bids = append(bids, item)
	}
	asks_count := readU32BE(buf, &pos)
	asks := make([]PriceLevel, 0, asks_count)
	for i := 0; i < int(asks_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return MarketDataSnapshot{}, pos, err
		}
		pos = item_end
		asks = append(asks, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MarketDataSnapshot{
		Symbol:     symbol,
		Exchange:   exchange,
		Bids:       bids,
		Asks:       asks,
		Timestamp:  timestamp,
		Sequence:   sequence,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for MarketDataSnapshot
type MarketDataSnapshotEncoder struct{}

// Encode builds a complete message (header + body).
func (MarketDataSnapshotEncoder) Encode(Symbol string, Exchange string, Bids []PriceLevel, Asks []PriceLevel, Timestamp int64, Sequence *uint64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 6)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)
	writeString(&buf, Symbol)
	writeString(&buf, Exchange)
	writeU32BE(&buf, uint32(len(Bids)))
	for _, item := range Bids {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU32BE(&buf, uint32(len(Asks)))
	for _, item := range Asks {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var Sequence_v uint64
	if Sequence != nil {
		Sequence_v = *Sequence
	}
	writeU64BE(&buf, uint64(Sequence_v))
	if Sequence != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for MarketDataSnapshot: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MarketDataSnapshotDecoderDecode(buf []byte) (MarketDataSnapshot, int, error) {
	if len(buf) < 8 {
		return MarketDataSnapshot{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarketDataSnapshot{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 6 {
		return MarketDataSnapshot{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	exchange := readString(buf, &pos)
	bids_count := readU32BE(buf, &pos)
	bids := make([]PriceLevel, 0, bids_count)
	for i := 0; i < int(bids_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return MarketDataSnapshot{}, pos, err
		}
		pos = item_end
		bids = append(bids, item)
	}
	asks_count := readU32BE(buf, &pos)
	asks := make([]PriceLevel, 0, asks_count)
	for i := 0; i < int(asks_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return MarketDataSnapshot{}, pos, err
		}
		pos = item_end
		asks = append(asks, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MarketDataSnapshot{
		Symbol:     symbol,
		Exchange:   exchange,
		Bids:       bids,
		Asks:       asks,
		Timestamp:  timestamp,
		Sequence:   sequence,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message OrderBookSnapshot
type OrderBookSnapshot struct {
	Symbol     string
	Exchange   string
	Bids       []PriceLevel
	Asks       []PriceLevel
	Timestamp  int64
	Sequence   *uint64
	IsSnapshot *uint8
}

// SBE body encoder for OrderBookSnapshot (fields only, no header).
type OrderBookSnapshotBodyEncoder struct{}

func (OrderBookSnapshotBodyEncoder) Encode(value OrderBookSnapshot, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.Exchange)
	writeU32BE(&local, uint32(len(value.Bids)))
	for _, item := range value.Bids {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU32BE(&local, uint32(len(value.Asks)))
	for _, item := range value.Asks {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_Sequence_v uint64
	if value.Sequence != nil {
		value_Sequence_v = *value.Sequence
	}
	writeU64BE(&local, uint64(value_Sequence_v))
	if value.Sequence != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OrderBookSnapshot: reads fields at pos, returns the value and the new offset.
func OrderBookSnapshotBodyDecoderDecode(buf []byte, pos int) (OrderBookSnapshot, int, error) {
	symbol := readString(buf, &pos)
	exchange := readString(buf, &pos)
	bids_count := readU32BE(buf, &pos)
	bids := make([]PriceLevel, 0, bids_count)
	for i := 0; i < int(bids_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return OrderBookSnapshot{}, pos, err
		}
		pos = item_end
		bids = append(bids, item)
	}
	asks_count := readU32BE(buf, &pos)
	asks := make([]PriceLevel, 0, asks_count)
	for i := 0; i < int(asks_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return OrderBookSnapshot{}, pos, err
		}
		pos = item_end
		asks = append(asks, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := OrderBookSnapshot{
		Symbol:     symbol,
		Exchange:   exchange,
		Bids:       bids,
		Asks:       asks,
		Timestamp:  timestamp,
		Sequence:   sequence,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for OrderBookSnapshot
type OrderBookSnapshotEncoder struct{}

// Encode builds a complete message (header + body).
func (OrderBookSnapshotEncoder) Encode(Symbol string, Exchange string, Bids []PriceLevel, Asks []PriceLevel, Timestamp int64, Sequence *uint64, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 9)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 17)
	writeString(&buf, Symbol)
	writeString(&buf, Exchange)
	writeU32BE(&buf, uint32(len(Bids)))
	for _, item := range Bids {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU32BE(&buf, uint32(len(Asks)))
	for _, item := range Asks {
		var item_buf []byte
		PriceLevelEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var Sequence_v uint64
	if Sequence != nil {
		Sequence_v = *Sequence
	}
	writeU64BE(&buf, uint64(Sequence_v))
	if Sequence != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OrderBookSnapshot: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OrderBookSnapshotDecoderDecode(buf []byte) (OrderBookSnapshot, int, error) {
	if len(buf) < 8 {
		return OrderBookSnapshot{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderBookSnapshot{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 9 {
		return OrderBookSnapshot{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	exchange := readString(buf, &pos)
	bids_count := readU32BE(buf, &pos)
	bids := make([]PriceLevel, 0, bids_count)
	for i := 0; i < int(bids_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return OrderBookSnapshot{}, pos, err
		}
		pos = item_end
		bids = append(bids, item)
	}
	asks_count := readU32BE(buf, &pos)
	asks := make([]PriceLevel, 0, asks_count)
	for i := 0; i < int(asks_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PriceLevelDecoderDecode(buf, pos)
		if err != nil {
			return OrderBookSnapshot{}, pos, err
		}
		pos = item_end
		asks = append(asks, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := OrderBookSnapshot{
		Symbol:     symbol,
		Exchange:   exchange,
		Bids:       bids,
		Asks:       asks,
		Timestamp:  timestamp,
		Sequence:   sequence,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message MarketDataIncrementalRefresh
type MarketDataIncrementalRefresh struct {
	Symbol    string
	Updates   []MarketDataUpdate
	Timestamp int64
	Sequence  *uint64
}

// SBE body encoder for MarketDataIncrementalRefresh (fields only, no header).
type MarketDataIncrementalRefreshBodyEncoder struct{}

func (MarketDataIncrementalRefreshBodyEncoder) Encode(value MarketDataIncrementalRefresh, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeU32BE(&local, uint32(len(value.Updates)))
	for _, item := range value.Updates {
		var item_buf []byte
		MarketDataUpdateEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_Sequence_v uint64
	if value.Sequence != nil {
		value_Sequence_v = *value.Sequence
	}
	writeU64BE(&local, uint64(value_Sequence_v))
	if value.Sequence != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for MarketDataIncrementalRefresh: reads fields at pos, returns the value and the new offset.
func MarketDataIncrementalRefreshBodyDecoderDecode(buf []byte, pos int) (MarketDataIncrementalRefresh, int, error) {
	symbol := readString(buf, &pos)
	updates_count := readU32BE(buf, &pos)
	updates := make([]MarketDataUpdate, 0, updates_count)
	for i := 0; i < int(updates_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := MarketDataUpdateDecoderDecode(buf, pos)
		if err != nil {
			return MarketDataIncrementalRefresh{}, pos, err
		}
		pos = item_end
		updates = append(updates, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	value := MarketDataIncrementalRefresh{
		Symbol:    symbol,
		Updates:   updates,
		Timestamp: timestamp,
		Sequence:  sequence,
	}
	return value, pos, nil
}

// SBE encoder for MarketDataIncrementalRefresh
type MarketDataIncrementalRefreshEncoder struct{}

// Encode builds a complete message (header + body).
func (MarketDataIncrementalRefreshEncoder) Encode(Symbol string, Updates []MarketDataUpdate, Timestamp int64, Sequence *uint64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 7)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 16)
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Updates)))
	for _, item := range Updates {
		var item_buf []byte
		MarketDataUpdateEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var Sequence_v uint64
	if Sequence != nil {
		Sequence_v = *Sequence
	}
	writeU64BE(&buf, uint64(Sequence_v))
	if Sequence != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for MarketDataIncrementalRefresh: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MarketDataIncrementalRefreshDecoderDecode(buf []byte) (MarketDataIncrementalRefresh, int, error) {
	if len(buf) < 8 {
		return MarketDataIncrementalRefresh{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarketDataIncrementalRefresh{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 7 {
		return MarketDataIncrementalRefresh{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	updates_count := readU32BE(buf, &pos)
	updates := make([]MarketDataUpdate, 0, updates_count)
	for i := 0; i < int(updates_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := MarketDataUpdateDecoderDecode(buf, pos)
		if err != nil {
			return MarketDataIncrementalRefresh{}, pos, err
		}
		pos = item_end
		updates = append(updates, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	value := MarketDataIncrementalRefresh{
		Symbol:    symbol,
		Updates:   updates,
		Timestamp: timestamp,
		Sequence:  sequence,
	}
	return value, pos, nil
}

// SBE message OrderBookDelta
type OrderBookDelta struct {
	Symbol    string
	Updates   []MarketDataUpdate
	Timestamp int64
	Sequence  *uint64
}

// SBE body encoder for OrderBookDelta (fields only, no header).
type OrderBookDeltaBodyEncoder struct{}

func (OrderBookDeltaBodyEncoder) Encode(value OrderBookDelta, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeU32BE(&local, uint32(len(value.Updates)))
	for _, item := range value.Updates {
		var item_buf []byte
		MarketDataUpdateEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	var value_Sequence_v uint64
	if value.Sequence != nil {
		value_Sequence_v = *value.Sequence
	}
	writeU64BE(&local, uint64(value_Sequence_v))
	if value.Sequence != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OrderBookDelta: reads fields at pos, returns the value and the new offset.
func OrderBookDeltaBodyDecoderDecode(buf []byte, pos int) (OrderBookDelta, int, error) {
	symbol := readString(buf, &pos)
	updates_count := readU32BE(buf, &pos)
	updates := make([]MarketDataUpdate, 0, updates_count)
	for i := 0; i < int(updates_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := MarketDataUpdateDecoderDecode(buf, pos)
		if err != nil {
			return OrderBookDelta{}, pos, err
		}
		pos = item_end
		updates = append(updates, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	value := OrderBookDelta{
		Symbol:    symbol,
		Updates:   updates,
		Timestamp: timestamp,
		Sequence:  sequence,
	}
	return value, pos, nil
}

// SBE encoder for OrderBookDelta
type OrderBookDeltaEncoder struct{}

// Encode builds a complete message (header + body).
func (OrderBookDeltaEncoder) Encode(Symbol string, Updates []MarketDataUpdate, Timestamp int64, Sequence *uint64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 10)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 16)
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Updates)))
	for _, item := range Updates {
		var item_buf []byte
		MarketDataUpdateEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU64BE(&buf, uint64(Timestamp))
	var Sequence_v uint64
	if Sequence != nil {
		Sequence_v = *Sequence
	}
	writeU64BE(&buf, uint64(Sequence_v))
	if Sequence != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OrderBookDelta: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OrderBookDeltaDecoderDecode(buf []byte) (OrderBookDelta, int, error) {
	if len(buf) < 8 {
		return OrderBookDelta{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderBookDelta{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 10 {
		return OrderBookDelta{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	symbol := readString(buf, &pos)
	updates_count := readU32BE(buf, &pos)
	updates := make([]MarketDataUpdate, 0, updates_count)
	for i := 0; i < int(updates_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := MarketDataUpdateDecoderDecode(buf, pos)
		if err != nil {
			return OrderBookDelta{}, pos, err
		}
		pos = item_end
		updates = append(updates, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	sequence_v := readU64BE(buf, &pos)
	var sequence *uint64
	if buf[pos] == 1 {
		pos++
		sequence = &sequence_v
	} else {
		pos++
		sequence = nil
	}
	value := OrderBookDelta{
		Symbol:    symbol,
		Updates:   updates,
		Timestamp: timestamp,
		Sequence:  sequence,
	}
	return value, pos, nil
}

// SBE message CandleBarEvent
type CandleBarEvent struct {
	Bar CandleBar
}

// SBE body encoder for CandleBarEvent (fields only, no header).
type CandleBarEventBodyEncoder struct{}

func (CandleBarEventBodyEncoder) Encode(value CandleBarEvent, buf *[]byte) {
	local := *buf
	CandleBarEncoder{}.Encode(value.Bar, &local)
	*buf = local
}

// SBE body decoder for CandleBarEvent: reads fields at pos, returns the value and the new offset.
func CandleBarEventBodyDecoderDecode(buf []byte, pos int) (CandleBarEvent, int, error) {
	bar, bar_pos, bar_err := CandleBarDecoderDecode(buf, pos)
	if bar_err != nil {
		return CandleBarEvent{}, pos, bar_err
	}
	pos = bar_pos
	value := CandleBarEvent{
		Bar: bar,
	}
	return value, pos, nil
}

// SBE encoder for CandleBarEvent
type CandleBarEventEncoder struct{}

// Encode builds a complete message (header + body).
func (CandleBarEventEncoder) Encode(Bar CandleBar) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 20)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	CandleBarEncoder{}.Encode(Bar, &buf)
	return buf, nil
}

// SBE decoder for CandleBarEvent: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CandleBarEventDecoderDecode(buf []byte) (CandleBarEvent, int, error) {
	if len(buf) < 8 {
		return CandleBarEvent{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CandleBarEvent{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 20 {
		return CandleBarEvent{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	bar, bar_pos, bar_err := CandleBarDecoderDecode(buf, pos)
	if bar_err != nil {
		return CandleBarEvent{}, pos, bar_err
	}
	pos = bar_pos
	value := CandleBarEvent{
		Bar: bar,
	}
	return value, pos, nil
}

// SBE message CandleBarBatch
type CandleBarBatch struct {
	Symbol     string
	Interval   string
	Bars       []CandleBar
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for CandleBarBatch (fields only, no header).
type CandleBarBatchBodyEncoder struct{}

func (CandleBarBatchBodyEncoder) Encode(value CandleBarBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeString(&local, value.Interval)
	writeU32BE(&local, uint32(len(value.Bars)))
	for _, item := range value.Bars {
		var item_buf []byte
		CandleBarEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for CandleBarBatch: reads fields at pos, returns the value and the new offset.
func CandleBarBatchBodyDecoderDecode(buf []byte, pos int) (CandleBarBatch, int, error) {
	var next_cursor *string
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	bars_count := readU32BE(buf, &pos)
	bars := make([]CandleBar, 0, bars_count)
	for i := 0; i < int(bars_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := CandleBarDecoderDecode(buf, pos)
		if err != nil {
			return CandleBarBatch{}, pos, err
		}
		pos = item_end
		bars = append(bars, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := CandleBarBatch{
		Symbol:     symbol,
		Interval:   interval,
		Bars:       bars,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for CandleBarBatch
type CandleBarBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (CandleBarBatchEncoder) Encode(Symbol string, Interval string, Bars []CandleBar, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 22)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Symbol)
	writeString(&buf, Interval)
	writeU32BE(&buf, uint32(len(Bars)))
	for _, item := range Bars {
		var item_buf []byte
		CandleBarEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for CandleBarBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CandleBarBatchDecoderDecode(buf []byte) (CandleBarBatch, int, error) {
	if len(buf) < 8 {
		return CandleBarBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CandleBarBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 22 {
		return CandleBarBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	symbol := readString(buf, &pos)
	interval := readString(buf, &pos)
	bars_count := readU32BE(buf, &pos)
	bars := make([]CandleBar, 0, bars_count)
	for i := 0; i < int(bars_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := CandleBarDecoderDecode(buf, pos)
		if err != nil {
			return CandleBarBatch{}, pos, err
		}
		pos = item_end
		bars = append(bars, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := CandleBarBatch{
		Symbol:     symbol,
		Interval:   interval,
		Bars:       bars,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message PublicTradeEvent
type PublicTradeEvent struct {
	Trade PublicTrade
}

// SBE body encoder for PublicTradeEvent (fields only, no header).
type PublicTradeEventBodyEncoder struct{}

func (PublicTradeEventBodyEncoder) Encode(value PublicTradeEvent, buf *[]byte) {
	local := *buf
	PublicTradeEncoder{}.Encode(value.Trade, &local)
	*buf = local
}

// SBE body decoder for PublicTradeEvent: reads fields at pos, returns the value and the new offset.
func PublicTradeEventBodyDecoderDecode(buf []byte, pos int) (PublicTradeEvent, int, error) {
	trade, trade_pos, trade_err := PublicTradeDecoderDecode(buf, pos)
	if trade_err != nil {
		return PublicTradeEvent{}, pos, trade_err
	}
	pos = trade_pos
	value := PublicTradeEvent{
		Trade: trade,
	}
	return value, pos, nil
}

// SBE encoder for PublicTradeEvent
type PublicTradeEventEncoder struct{}

// Encode builds a complete message (header + body).
func (PublicTradeEventEncoder) Encode(Trade PublicTrade) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 23)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	PublicTradeEncoder{}.Encode(Trade, &buf)
	return buf, nil
}

// SBE decoder for PublicTradeEvent: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PublicTradeEventDecoderDecode(buf []byte) (PublicTradeEvent, int, error) {
	if len(buf) < 8 {
		return PublicTradeEvent{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PublicTradeEvent{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 23 {
		return PublicTradeEvent{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	trade, trade_pos, trade_err := PublicTradeDecoderDecode(buf, pos)
	if trade_err != nil {
		return PublicTradeEvent{}, pos, trade_err
	}
	pos = trade_pos
	value := PublicTradeEvent{
		Trade: trade,
	}
	return value, pos, nil
}

// SBE message PublicTradeBatch
type PublicTradeBatch struct {
	Symbol     string
	Trades     []PublicTrade
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for PublicTradeBatch (fields only, no header).
type PublicTradeBatchBodyEncoder struct{}

func (PublicTradeBatchBodyEncoder) Encode(value PublicTradeBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Symbol)
	writeU32BE(&local, uint32(len(value.Trades)))
	for _, item := range value.Trades {
		var item_buf []byte
		PublicTradeEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for PublicTradeBatch: reads fields at pos, returns the value and the new offset.
func PublicTradeBatchBodyDecoderDecode(buf []byte, pos int) (PublicTradeBatch, int, error) {
	var next_cursor *string
	symbol := readString(buf, &pos)
	trades_count := readU32BE(buf, &pos)
	trades := make([]PublicTrade, 0, trades_count)
	for i := 0; i < int(trades_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PublicTradeDecoderDecode(buf, pos)
		if err != nil {
			return PublicTradeBatch{}, pos, err
		}
		pos = item_end
		trades = append(trades, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := PublicTradeBatch{
		Symbol:     symbol,
		Trades:     trades,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for PublicTradeBatch
type PublicTradeBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (PublicTradeBatchEncoder) Encode(Symbol string, Trades []PublicTrade, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 25)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Symbol)
	writeU32BE(&buf, uint32(len(Trades)))
	for _, item := range Trades {
		var item_buf []byte
		PublicTradeEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for PublicTradeBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PublicTradeBatchDecoderDecode(buf []byte) (PublicTradeBatch, int, error) {
	if len(buf) < 8 {
		return PublicTradeBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PublicTradeBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 25 {
		return PublicTradeBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	symbol := readString(buf, &pos)
	trades_count := readU32BE(buf, &pos)
	trades := make([]PublicTrade, 0, trades_count)
	for i := 0; i < int(trades_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PublicTradeDecoderDecode(buf, pos)
		if err != nil {
			return PublicTradeBatch{}, pos, err
		}
		pos = item_end
		trades = append(trades, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := PublicTradeBatch{
		Symbol:     symbol,
		Trades:     trades,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message InstrumentCatalogResponse
type InstrumentCatalogResponse struct {
	Instruments []InstrumentMetadata
}

// SBE body encoder for InstrumentCatalogResponse (fields only, no header).
type InstrumentCatalogResponseBodyEncoder struct{}

func (InstrumentCatalogResponseBodyEncoder) Encode(value InstrumentCatalogResponse, buf *[]byte) {
	local := *buf
	writeU32BE(&local, uint32(len(value.Instruments)))
	for _, item := range value.Instruments {
		var item_buf []byte
		InstrumentMetadataEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	*buf = local
}

// SBE body decoder for InstrumentCatalogResponse: reads fields at pos, returns the value and the new offset.
func InstrumentCatalogResponseBodyDecoderDecode(buf []byte, pos int) (InstrumentCatalogResponse, int, error) {
	instruments_count := readU32BE(buf, &pos)
	instruments := make([]InstrumentMetadata, 0, instruments_count)
	for i := 0; i < int(instruments_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := InstrumentMetadataDecoderDecode(buf, pos)
		if err != nil {
			return InstrumentCatalogResponse{}, pos, err
		}
		pos = item_end
		instruments = append(instruments, item)
	}
	value := InstrumentCatalogResponse{
		Instruments: instruments,
	}
	return value, pos, nil
}

// SBE encoder for InstrumentCatalogResponse
type InstrumentCatalogResponseEncoder struct{}

// Encode builds a complete message (header + body).
func (InstrumentCatalogResponseEncoder) Encode(Instruments []InstrumentMetadata) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 30)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeU32BE(&buf, uint32(len(Instruments)))
	for _, item := range Instruments {
		var item_buf []byte
		InstrumentMetadataEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	return buf, nil
}

// SBE decoder for InstrumentCatalogResponse: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func InstrumentCatalogResponseDecoderDecode(buf []byte) (InstrumentCatalogResponse, int, error) {
	if len(buf) < 8 {
		return InstrumentCatalogResponse{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return InstrumentCatalogResponse{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 30 {
		return InstrumentCatalogResponse{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	instruments_count := readU32BE(buf, &pos)
	instruments := make([]InstrumentMetadata, 0, instruments_count)
	for i := 0; i < int(instruments_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := InstrumentMetadataDecoderDecode(buf, pos)
		if err != nil {
			return InstrumentCatalogResponse{}, pos, err
		}
		pos = item_end
		instruments = append(instruments, item)
	}
	value := InstrumentCatalogResponse{
		Instruments: instruments,
	}
	return value, pos, nil
}

// SBE message BalanceSnapshot
type BalanceSnapshot struct {
	Account    string
	Balances   []BalanceEntry
	IsSnapshot *uint8
}

// SBE body encoder for BalanceSnapshot (fields only, no header).
type BalanceSnapshotBodyEncoder struct{}

func (BalanceSnapshotBodyEncoder) Encode(value BalanceSnapshot, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Balances)))
	for _, item := range value.Balances {
		var item_buf []byte
		BalanceEntryEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for BalanceSnapshot: reads fields at pos, returns the value and the new offset.
func BalanceSnapshotBodyDecoderDecode(buf []byte, pos int) (BalanceSnapshot, int, error) {
	account := readString(buf, &pos)
	balances_count := readU32BE(buf, &pos)
	balances := make([]BalanceEntry, 0, balances_count)
	for i := 0; i < int(balances_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := BalanceEntryDecoderDecode(buf, pos)
		if err != nil {
			return BalanceSnapshot{}, pos, err
		}
		pos = item_end
		balances = append(balances, item)
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := BalanceSnapshot{
		Account:    account,
		Balances:   balances,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for BalanceSnapshot
type BalanceSnapshotEncoder struct{}

// Encode builds a complete message (header + body).
func (BalanceSnapshotEncoder) Encode(Account string, Balances []BalanceEntry, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 33)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Balances)))
	for _, item := range Balances {
		var item_buf []byte
		BalanceEntryEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for BalanceSnapshot: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func BalanceSnapshotDecoderDecode(buf []byte) (BalanceSnapshot, int, error) {
	if len(buf) < 8 {
		return BalanceSnapshot{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BalanceSnapshot{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 33 {
		return BalanceSnapshot{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	balances_count := readU32BE(buf, &pos)
	balances := make([]BalanceEntry, 0, balances_count)
	for i := 0; i < int(balances_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := BalanceEntryDecoderDecode(buf, pos)
		if err != nil {
			return BalanceSnapshot{}, pos, err
		}
		pos = item_end
		balances = append(balances, item)
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := BalanceSnapshot{
		Account:    account,
		Balances:   balances,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message PositionSnapshot
type PositionSnapshot struct {
	Account    string
	Positions  []PositionEntry
	IsSnapshot *uint8
}

// SBE body encoder for PositionSnapshot (fields only, no header).
type PositionSnapshotBodyEncoder struct{}

func (PositionSnapshotBodyEncoder) Encode(value PositionSnapshot, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Positions)))
	for _, item := range value.Positions {
		var item_buf []byte
		PositionEntryEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for PositionSnapshot: reads fields at pos, returns the value and the new offset.
func PositionSnapshotBodyDecoderDecode(buf []byte, pos int) (PositionSnapshot, int, error) {
	account := readString(buf, &pos)
	positions_count := readU32BE(buf, &pos)
	positions := make([]PositionEntry, 0, positions_count)
	for i := 0; i < int(positions_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PositionEntryDecoderDecode(buf, pos)
		if err != nil {
			return PositionSnapshot{}, pos, err
		}
		pos = item_end
		positions = append(positions, item)
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := PositionSnapshot{
		Account:    account,
		Positions:  positions,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for PositionSnapshot
type PositionSnapshotEncoder struct{}

// Encode builds a complete message (header + body).
func (PositionSnapshotEncoder) Encode(Account string, Positions []PositionEntry, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 35)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Positions)))
	for _, item := range Positions {
		var item_buf []byte
		PositionEntryEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for PositionSnapshot: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func PositionSnapshotDecoderDecode(buf []byte) (PositionSnapshot, int, error) {
	if len(buf) < 8 {
		return PositionSnapshot{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return PositionSnapshot{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 35 {
		return PositionSnapshot{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	positions_count := readU32BE(buf, &pos)
	positions := make([]PositionEntry, 0, positions_count)
	for i := 0; i < int(positions_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := PositionEntryDecoderDecode(buf, pos)
		if err != nil {
			return PositionSnapshot{}, pos, err
		}
		pos = item_end
		positions = append(positions, item)
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := PositionSnapshot{
		Account:    account,
		Positions:  positions,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message CapabilitiesResponse
type CapabilitiesResponse struct {
	SchemaIds []uint8
	Paths     []CapabilityPath
	Symbols   []string
	Intervals []string
}

// SBE body encoder for CapabilitiesResponse (fields only, no header).
type CapabilitiesResponseBodyEncoder struct{}

func (CapabilitiesResponseBodyEncoder) Encode(value CapabilitiesResponse, buf *[]byte) {
	local := *buf
	writeU32BE(&local, uint32(len(value.SchemaIds)))
	for _, item := range value.SchemaIds {
		local = append(local, uint8(item))
	}
	writeU32BE(&local, uint32(len(value.Paths)))
	for _, item := range value.Paths {
		var item_buf []byte
		CapabilityPathEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU32BE(&local, uint32(len(value.Symbols)))
	for _, item := range value.Symbols {
		writeStringU32(&local, item)
	}
	writeU32BE(&local, uint32(len(value.Intervals)))
	for _, item := range value.Intervals {
		writeStringU32(&local, item)
	}
	*buf = local
}

// SBE body decoder for CapabilitiesResponse: reads fields at pos, returns the value and the new offset.
func CapabilitiesResponseBodyDecoderDecode(buf []byte, pos int) (CapabilitiesResponse, int, error) {
	schema_ids_count := readU32BE(buf, &pos)
	schema_ids := make([]uint8, 0, schema_ids_count)
	for i := 0; i < int(schema_ids_count); i++ {
		schema_ids = append(schema_ids, readU8(buf, &pos))
	}
	paths_count := readU32BE(buf, &pos)
	paths := make([]CapabilityPath, 0, paths_count)
	for i := 0; i < int(paths_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := CapabilityPathDecoderDecode(buf, pos)
		if err != nil {
			return CapabilitiesResponse{}, pos, err
		}
		pos = item_end
		paths = append(paths, item)
	}
	symbols_count := readU32BE(buf, &pos)
	symbols := make([]string, 0, symbols_count)
	for i := 0; i < int(symbols_count); i++ {
		symbols = append(symbols, readStringU32(buf, &pos))
	}
	intervals_count := readU32BE(buf, &pos)
	intervals := make([]string, 0, intervals_count)
	for i := 0; i < int(intervals_count); i++ {
		intervals = append(intervals, readStringU32(buf, &pos))
	}
	value := CapabilitiesResponse{
		SchemaIds: schema_ids,
		Paths:     paths,
		Symbols:   symbols,
		Intervals: intervals,
	}
	return value, pos, nil
}

// SBE encoder for CapabilitiesResponse
type CapabilitiesResponseEncoder struct{}

// Encode builds a complete message (header + body).
func (CapabilitiesResponseEncoder) Encode(SchemaIds []uint8, Paths []CapabilityPath, Symbols []string, Intervals []string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 53)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeU32BE(&buf, uint32(len(SchemaIds)))
	for _, item := range SchemaIds {
		buf = append(buf, uint8(item))
	}
	writeU32BE(&buf, uint32(len(Paths)))
	for _, item := range Paths {
		var item_buf []byte
		CapabilityPathEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU32BE(&buf, uint32(len(Symbols)))
	for _, item := range Symbols {
		writeStringU32(&buf, item)
	}
	writeU32BE(&buf, uint32(len(Intervals)))
	for _, item := range Intervals {
		writeStringU32(&buf, item)
	}
	return buf, nil
}

// SBE decoder for CapabilitiesResponse: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func CapabilitiesResponseDecoderDecode(buf []byte) (CapabilitiesResponse, int, error) {
	if len(buf) < 8 {
		return CapabilitiesResponse{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return CapabilitiesResponse{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 53 {
		return CapabilitiesResponse{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	schema_ids_count := readU32BE(buf, &pos)
	schema_ids := make([]uint8, 0, schema_ids_count)
	for i := 0; i < int(schema_ids_count); i++ {
		schema_ids = append(schema_ids, readU8(buf, &pos))
	}
	paths_count := readU32BE(buf, &pos)
	paths := make([]CapabilityPath, 0, paths_count)
	for i := 0; i < int(paths_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := CapabilityPathDecoderDecode(buf, pos)
		if err != nil {
			return CapabilitiesResponse{}, pos, err
		}
		pos = item_end
		paths = append(paths, item)
	}
	symbols_count := readU32BE(buf, &pos)
	symbols := make([]string, 0, symbols_count)
	for i := 0; i < int(symbols_count); i++ {
		symbols = append(symbols, readStringU32(buf, &pos))
	}
	intervals_count := readU32BE(buf, &pos)
	intervals := make([]string, 0, intervals_count)
	for i := 0; i < int(intervals_count); i++ {
		intervals = append(intervals, readStringU32(buf, &pos))
	}
	value := CapabilitiesResponse{
		SchemaIds: schema_ids,
		Paths:     paths,
		Symbols:   symbols,
		Intervals: intervals,
	}
	return value, pos, nil
}

// SBE message BatchReport
type BatchReport struct {
	BatchId   string
	Account   string
	Outcome   BatchReportOutcome
	Results   []BatchResult
	Timestamp int64
}

// SBE body encoder for BatchReport (fields only, no header).
type BatchReportBodyEncoder struct{}

func (BatchReportBodyEncoder) Encode(value BatchReport, buf *[]byte) {
	local := *buf
	writeString(&local, value.BatchId)
	writeString(&local, value.Account)
	local = append(local, byte(value.Outcome.ToValue()))
	writeU32BE(&local, uint32(len(value.Results)))
	for _, item := range value.Results {
		var item_buf []byte
		BatchResultEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	writeU64BE(&local, uint64(value.Timestamp))
	*buf = local
}

// SBE body decoder for BatchReport: reads fields at pos, returns the value and the new offset.
func BatchReportBodyDecoderDecode(buf []byte, pos int) (BatchReport, int, error) {
	batch_id := readString(buf, &pos)
	account := readString(buf, &pos)
	outcome_raw := readU8(buf, &pos)
	outcome, outcome_err := BatchReportOutcomeFromValue(outcome_raw)
	if outcome_err != nil {
		return BatchReport{}, pos, outcome_err
	}
	results_count := readU32BE(buf, &pos)
	results := make([]BatchResult, 0, results_count)
	for i := 0; i < int(results_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := BatchResultDecoderDecode(buf, pos)
		if err != nil {
			return BatchReport{}, pos, err
		}
		pos = item_end
		results = append(results, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	value := BatchReport{
		BatchId:   batch_id,
		Account:   account,
		Outcome:   outcome,
		Results:   results,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE encoder for BatchReport
type BatchReportEncoder struct{}

// Encode builds a complete message (header + body).
func (BatchReportEncoder) Encode(BatchId string, Account string, Outcome BatchReportOutcome, Results []BatchResult, Timestamp int64) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 64)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 9)
	writeString(&buf, BatchId)
	writeString(&buf, Account)
	buf = append(buf, byte(Outcome.ToValue()))
	writeU32BE(&buf, uint32(len(Results)))
	for _, item := range Results {
		var item_buf []byte
		BatchResultEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	writeU64BE(&buf, uint64(Timestamp))
	return buf, nil
}

// SBE decoder for BatchReport: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func BatchReportDecoderDecode(buf []byte) (BatchReport, int, error) {
	if len(buf) < 8 {
		return BatchReport{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BatchReport{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 64 {
		return BatchReport{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	batch_id := readString(buf, &pos)
	account := readString(buf, &pos)
	outcome_raw := readU8(buf, &pos)
	outcome, outcome_err := BatchReportOutcomeFromValue(outcome_raw)
	if outcome_err != nil {
		return BatchReport{}, pos, outcome_err
	}
	results_count := readU32BE(buf, &pos)
	results := make([]BatchResult, 0, results_count)
	for i := 0; i < int(results_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := BatchResultDecoderDecode(buf, pos)
		if err != nil {
			return BatchReport{}, pos, err
		}
		pos = item_end
		results = append(results, item)
	}
	timestamp := int64(readU64BE(buf, &pos))
	value := BatchReport{
		BatchId:   batch_id,
		Account:   account,
		Outcome:   outcome,
		Results:   results,
		Timestamp: timestamp,
	}
	return value, pos, nil
}

// SBE struct BatchAction
type BatchAction struct {
	Kind     BatchActionKind
	NewOrder *NewOrderSingle
	Cancel   *CancelRequest
	Replace  *CancelReplaceRequest
}

// Encode writes BatchAction fields (no header) into buf.
type BatchActionEncoder struct{}

func (BatchActionEncoder) Encode(value BatchAction, buf *[]byte) {
	local := *buf
	local = append(local, byte(value.Kind.ToValue()))
	if value.NewOrder != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.NewOrder != nil {
		NewOrderSingleBodyEncoder{}.Encode(*value.NewOrder, &local)
	}
	if value.Cancel != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Cancel != nil {
		CancelRequestBodyEncoder{}.Encode(*value.Cancel, &local)
	}
	if value.Replace != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	if value.Replace != nil {
		CancelReplaceRequestBodyEncoder{}.Encode(*value.Replace, &local)
	}
	*buf = local
}

// DecodeBatchActionDecoder reads BatchAction fields starting at pos and returns the new offset.
func BatchActionDecoderDecode(buf []byte, pos int) (BatchAction, int, error) {
	kind_raw := readU8(buf, &pos)
	kind, kind_err := BatchActionKindFromValue(kind_raw)
	if kind_err != nil {
		return BatchAction{}, pos, kind_err
	}
	var new_order *NewOrderSingle
	if readU8(buf, &pos) == 1 {
		new_order_inner, new_order_inner_pos, new_order_inner_err := NewOrderSingleBodyDecoderDecode(buf, pos)
		if new_order_inner_err != nil {
			return BatchAction{}, pos, new_order_inner_err
		}
		pos = new_order_inner_pos
		new_order = &new_order_inner
	} else {
		new_order = nil
	}
	var cancel *CancelRequest
	if readU8(buf, &pos) == 1 {
		cancel_inner, cancel_inner_pos, cancel_inner_err := CancelRequestBodyDecoderDecode(buf, pos)
		if cancel_inner_err != nil {
			return BatchAction{}, pos, cancel_inner_err
		}
		pos = cancel_inner_pos
		cancel = &cancel_inner
	} else {
		cancel = nil
	}
	var replace *CancelReplaceRequest
	if readU8(buf, &pos) == 1 {
		replace_inner, replace_inner_pos, replace_inner_err := CancelReplaceRequestBodyDecoderDecode(buf, pos)
		if replace_inner_err != nil {
			return BatchAction{}, pos, replace_inner_err
		}
		pos = replace_inner_pos
		replace = &replace_inner
	} else {
		replace = nil
	}
	value := BatchAction{
		Kind:     kind,
		NewOrder: new_order,
		Cancel:   cancel,
		Replace:  replace,
	}
	return value, pos, nil
}

// SBE message FillHistoryBatch
type FillHistoryBatch struct {
	Account    string
	Fills      []ExecutionReport
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for FillHistoryBatch (fields only, no header).
type FillHistoryBatchBodyEncoder struct{}

func (FillHistoryBatchBodyEncoder) Encode(value FillHistoryBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Fills)))
	for _, item := range value.Fills {
		var item_buf []byte
		ExecutionReportBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for FillHistoryBatch: reads fields at pos, returns the value and the new offset.
func FillHistoryBatchBodyDecoderDecode(buf []byte, pos int) (FillHistoryBatch, int, error) {
	var next_cursor *string
	account := readString(buf, &pos)
	fills_count := readU32BE(buf, &pos)
	fills := make([]ExecutionReport, 0, fills_count)
	for i := 0; i < int(fills_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := ExecutionReportBodyDecoderDecode(buf, pos)
		if err != nil {
			return FillHistoryBatch{}, pos, err
		}
		pos = item_end
		fills = append(fills, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := FillHistoryBatch{
		Account:    account,
		Fills:      fills,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for FillHistoryBatch
type FillHistoryBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (FillHistoryBatchEncoder) Encode(Account string, Fills []ExecutionReport, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 41)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Fills)))
	for _, item := range Fills {
		var item_buf []byte
		ExecutionReportBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for FillHistoryBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func FillHistoryBatchDecoderDecode(buf []byte) (FillHistoryBatch, int, error) {
	if len(buf) < 8 {
		return FillHistoryBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FillHistoryBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 41 {
		return FillHistoryBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	account := readString(buf, &pos)
	fills_count := readU32BE(buf, &pos)
	fills := make([]ExecutionReport, 0, fills_count)
	for i := 0; i < int(fills_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := ExecutionReportBodyDecoderDecode(buf, pos)
		if err != nil {
			return FillHistoryBatch{}, pos, err
		}
		pos = item_end
		fills = append(fills, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := FillHistoryBatch{
		Account:    account,
		Fills:      fills,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message OpenOrdersSnapshot
type OpenOrdersSnapshot struct {
	Account    string
	Orders     []ExecutionReport
	IsSnapshot *uint8
}

// SBE body encoder for OpenOrdersSnapshot (fields only, no header).
type OpenOrdersSnapshotBodyEncoder struct{}

func (OpenOrdersSnapshotBodyEncoder) Encode(value OpenOrdersSnapshot, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Orders)))
	for _, item := range value.Orders {
		var item_buf []byte
		ExecutionReportBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OpenOrdersSnapshot: reads fields at pos, returns the value and the new offset.
func OpenOrdersSnapshotBodyDecoderDecode(buf []byte, pos int) (OpenOrdersSnapshot, int, error) {
	account := readString(buf, &pos)
	orders_count := readU32BE(buf, &pos)
	orders := make([]ExecutionReport, 0, orders_count)
	for i := 0; i < int(orders_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := ExecutionReportBodyDecoderDecode(buf, pos)
		if err != nil {
			return OpenOrdersSnapshot{}, pos, err
		}
		pos = item_end
		orders = append(orders, item)
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := OpenOrdersSnapshot{
		Account:    account,
		Orders:     orders,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for OpenOrdersSnapshot
type OpenOrdersSnapshotEncoder struct{}

// Encode builds a complete message (header + body).
func (OpenOrdersSnapshotEncoder) Encode(Account string, Orders []ExecutionReport, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 49)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Orders)))
	for _, item := range Orders {
		var item_buf []byte
		ExecutionReportBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OpenOrdersSnapshot: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OpenOrdersSnapshotDecoderDecode(buf []byte) (OpenOrdersSnapshot, int, error) {
	if len(buf) < 8 {
		return OpenOrdersSnapshot{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OpenOrdersSnapshot{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 49 {
		return OpenOrdersSnapshot{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	orders_count := readU32BE(buf, &pos)
	orders := make([]ExecutionReport, 0, orders_count)
	for i := 0; i < int(orders_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := ExecutionReportBodyDecoderDecode(buf, pos)
		if err != nil {
			return OpenOrdersSnapshot{}, pos, err
		}
		pos = item_end
		orders = append(orders, item)
	}
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := OpenOrdersSnapshot{
		Account:    account,
		Orders:     orders,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message OrderHistoryBatch
type OrderHistoryBatch struct {
	Account    string
	Orders     []ExecutionReport
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for OrderHistoryBatch (fields only, no header).
type OrderHistoryBatchBodyEncoder struct{}

func (OrderHistoryBatchBodyEncoder) Encode(value OrderHistoryBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Orders)))
	for _, item := range value.Orders {
		var item_buf []byte
		ExecutionReportBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for OrderHistoryBatch: reads fields at pos, returns the value and the new offset.
func OrderHistoryBatchBodyDecoderDecode(buf []byte, pos int) (OrderHistoryBatch, int, error) {
	var next_cursor *string
	account := readString(buf, &pos)
	orders_count := readU32BE(buf, &pos)
	orders := make([]ExecutionReport, 0, orders_count)
	for i := 0; i < int(orders_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := ExecutionReportBodyDecoderDecode(buf, pos)
		if err != nil {
			return OrderHistoryBatch{}, pos, err
		}
		pos = item_end
		orders = append(orders, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := OrderHistoryBatch{
		Account:    account,
		Orders:     orders,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for OrderHistoryBatch
type OrderHistoryBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (OrderHistoryBatchEncoder) Encode(Account string, Orders []ExecutionReport, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 51)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Orders)))
	for _, item := range Orders {
		var item_buf []byte
		ExecutionReportBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for OrderHistoryBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func OrderHistoryBatchDecoderDecode(buf []byte) (OrderHistoryBatch, int, error) {
	if len(buf) < 8 {
		return OrderHistoryBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return OrderHistoryBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 51 {
		return OrderHistoryBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	account := readString(buf, &pos)
	orders_count := readU32BE(buf, &pos)
	orders := make([]ExecutionReport, 0, orders_count)
	for i := 0; i < int(orders_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := ExecutionReportBodyDecoderDecode(buf, pos)
		if err != nil {
			return OrderHistoryBatch{}, pos, err
		}
		pos = item_end
		orders = append(orders, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := OrderHistoryBatch{
		Account:    account,
		Orders:     orders,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message AllMidsBatch
type AllMidsBatch struct {
	Tickers []MiniTicker
}

// SBE body encoder for AllMidsBatch (fields only, no header).
type AllMidsBatchBodyEncoder struct{}

func (AllMidsBatchBodyEncoder) Encode(value AllMidsBatch, buf *[]byte) {
	local := *buf
	writeU32BE(&local, uint32(len(value.Tickers)))
	for _, item := range value.Tickers {
		var item_buf []byte
		MiniTickerBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	*buf = local
}

// SBE body decoder for AllMidsBatch: reads fields at pos, returns the value and the new offset.
func AllMidsBatchBodyDecoderDecode(buf []byte, pos int) (AllMidsBatch, int, error) {
	tickers_count := readU32BE(buf, &pos)
	tickers := make([]MiniTicker, 0, tickers_count)
	for i := 0; i < int(tickers_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := MiniTickerBodyDecoderDecode(buf, pos)
		if err != nil {
			return AllMidsBatch{}, pos, err
		}
		pos = item_end
		tickers = append(tickers, item)
	}
	value := AllMidsBatch{
		Tickers: tickers,
	}
	return value, pos, nil
}

// SBE encoder for AllMidsBatch
type AllMidsBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (AllMidsBatchEncoder) Encode(Tickers []MiniTicker) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 16)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeU32BE(&buf, uint32(len(Tickers)))
	for _, item := range Tickers {
		var item_buf []byte
		MiniTickerBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	return buf, nil
}

// SBE decoder for AllMidsBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func AllMidsBatchDecoderDecode(buf []byte) (AllMidsBatch, int, error) {
	if len(buf) < 8 {
		return AllMidsBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return AllMidsBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 16 {
		return AllMidsBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	tickers_count := readU32BE(buf, &pos)
	tickers := make([]MiniTicker, 0, tickers_count)
	for i := 0; i < int(tickers_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := MiniTickerBodyDecoderDecode(buf, pos)
		if err != nil {
			return AllMidsBatch{}, pos, err
		}
		pos = item_end
		tickers = append(tickers, item)
	}
	value := AllMidsBatch{
		Tickers: tickers,
	}
	return value, pos, nil
}

// SBE message MarginUpdate
type MarginUpdate struct {
	Account    string
	Summary    MarginSummary
	IsSnapshot *uint8
}

// SBE body encoder for MarginUpdate (fields only, no header).
type MarginUpdateBodyEncoder struct{}

func (MarginUpdateBodyEncoder) Encode(value MarginUpdate, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	MarginSummaryBodyEncoder{}.Encode(value.Summary, &local)
	var value_IsSnapshot_v uint8
	if value.IsSnapshot != nil {
		value_IsSnapshot_v = *value.IsSnapshot
	}
	local = append(local, uint8(value_IsSnapshot_v))
	if value.IsSnapshot != nil {
		local = append(local, 1)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for MarginUpdate: reads fields at pos, returns the value and the new offset.
func MarginUpdateBodyDecoderDecode(buf []byte, pos int) (MarginUpdate, int, error) {
	account := readString(buf, &pos)
	summary, summary_pos, summary_err := MarginSummaryBodyDecoderDecode(buf, pos)
	if summary_err != nil {
		return MarginUpdate{}, pos, summary_err
	}
	pos = summary_pos
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MarginUpdate{
		Account:    account,
		Summary:    summary,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE encoder for MarginUpdate
type MarginUpdateEncoder struct{}

// Encode builds a complete message (header + body).
func (MarginUpdateEncoder) Encode(Account string, Summary MarginSummary, IsSnapshot *uint8) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 37)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	MarginSummaryBodyEncoder{}.Encode(Summary, &buf)
	var IsSnapshot_v uint8
	if IsSnapshot != nil {
		IsSnapshot_v = *IsSnapshot
	}
	buf = append(buf, uint8(IsSnapshot_v))
	if IsSnapshot != nil {
		buf = append(buf, 1)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for MarginUpdate: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func MarginUpdateDecoderDecode(buf []byte) (MarginUpdate, int, error) {
	if len(buf) < 8 {
		return MarginUpdate{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return MarginUpdate{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 37 {
		return MarginUpdate{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	account := readString(buf, &pos)
	summary, summary_pos, summary_err := MarginSummaryBodyDecoderDecode(buf, pos)
	if summary_err != nil {
		return MarginUpdate{}, pos, summary_err
	}
	pos = summary_pos
	is_snapshot_v := readU8(buf, &pos)
	var is_snapshot *uint8
	if buf[pos] == 1 {
		pos++
		is_snapshot = &is_snapshot_v
	} else {
		pos++
		is_snapshot = nil
	}
	value := MarginUpdate{
		Account:    account,
		Summary:    summary,
		IsSnapshot: is_snapshot,
	}
	return value, pos, nil
}

// SBE message FundingHistoryBatch
type FundingHistoryBatch struct {
	Account    string
	Payments   []FundingPayment
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for FundingHistoryBatch (fields only, no header).
type FundingHistoryBatchBodyEncoder struct{}

func (FundingHistoryBatchBodyEncoder) Encode(value FundingHistoryBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Payments)))
	for _, item := range value.Payments {
		var item_buf []byte
		FundingPaymentBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for FundingHistoryBatch: reads fields at pos, returns the value and the new offset.
func FundingHistoryBatchBodyDecoderDecode(buf []byte, pos int) (FundingHistoryBatch, int, error) {
	var next_cursor *string
	account := readString(buf, &pos)
	payments_count := readU32BE(buf, &pos)
	payments := make([]FundingPayment, 0, payments_count)
	for i := 0; i < int(payments_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := FundingPaymentBodyDecoderDecode(buf, pos)
		if err != nil {
			return FundingHistoryBatch{}, pos, err
		}
		pos = item_end
		payments = append(payments, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := FundingHistoryBatch{
		Account:    account,
		Payments:   payments,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for FundingHistoryBatch
type FundingHistoryBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (FundingHistoryBatchEncoder) Encode(Account string, Payments []FundingPayment, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 44)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Payments)))
	for _, item := range Payments {
		var item_buf []byte
		FundingPaymentBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for FundingHistoryBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func FundingHistoryBatchDecoderDecode(buf []byte) (FundingHistoryBatch, int, error) {
	if len(buf) < 8 {
		return FundingHistoryBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return FundingHistoryBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 44 {
		return FundingHistoryBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	account := readString(buf, &pos)
	payments_count := readU32BE(buf, &pos)
	payments := make([]FundingPayment, 0, payments_count)
	for i := 0; i < int(payments_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := FundingPaymentBodyDecoderDecode(buf, pos)
		if err != nil {
			return FundingHistoryBatch{}, pos, err
		}
		pos = item_end
		payments = append(payments, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := FundingHistoryBatch{
		Account:    account,
		Payments:   payments,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message LedgerHistoryBatch
type LedgerHistoryBatch struct {
	Account    string
	Entries    []LedgerUpdate
	HasMore    uint8
	NextCursor *string
}

// SBE body encoder for LedgerHistoryBatch (fields only, no header).
type LedgerHistoryBatchBodyEncoder struct{}

func (LedgerHistoryBatchBodyEncoder) Encode(value LedgerHistoryBatch, buf *[]byte) {
	local := *buf
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Entries)))
	for _, item := range value.Entries {
		var item_buf []byte
		LedgerUpdateBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	local = append(local, uint8(value.HasMore))
	if value.NextCursor != nil {
		local = append(local, 1)
		writeString(&local, *value.NextCursor)
	} else {
		local = append(local, 0)
	}
	*buf = local
}

// SBE body decoder for LedgerHistoryBatch: reads fields at pos, returns the value and the new offset.
func LedgerHistoryBatchBodyDecoderDecode(buf []byte, pos int) (LedgerHistoryBatch, int, error) {
	var next_cursor *string
	account := readString(buf, &pos)
	entries_count := readU32BE(buf, &pos)
	entries := make([]LedgerUpdate, 0, entries_count)
	for i := 0; i < int(entries_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := LedgerUpdateBodyDecoderDecode(buf, pos)
		if err != nil {
			return LedgerHistoryBatch{}, pos, err
		}
		pos = item_end
		entries = append(entries, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := LedgerHistoryBatch{
		Account:    account,
		Entries:    entries,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE encoder for LedgerHistoryBatch
type LedgerHistoryBatchEncoder struct{}

// Encode builds a complete message (header + body).
func (LedgerHistoryBatchEncoder) Encode(Account string, Entries []LedgerUpdate, HasMore uint8, NextCursor *string) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 47)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 1)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Entries)))
	for _, item := range Entries {
		var item_buf []byte
		LedgerUpdateBodyEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	buf = append(buf, uint8(HasMore))
	if NextCursor != nil {
		buf = append(buf, 1)
		writeString(&buf, *NextCursor)
	} else {
		buf = append(buf, 0)
	}
	return buf, nil
}

// SBE decoder for LedgerHistoryBatch: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func LedgerHistoryBatchDecoderDecode(buf []byte) (LedgerHistoryBatch, int, error) {
	if len(buf) < 8 {
		return LedgerHistoryBatch{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return LedgerHistoryBatch{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 47 {
		return LedgerHistoryBatch{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	var next_cursor *string
	account := readString(buf, &pos)
	entries_count := readU32BE(buf, &pos)
	entries := make([]LedgerUpdate, 0, entries_count)
	for i := 0; i < int(entries_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := LedgerUpdateBodyDecoderDecode(buf, pos)
		if err != nil {
			return LedgerHistoryBatch{}, pos, err
		}
		pos = item_end
		entries = append(entries, item)
	}
	has_more := readU8(buf, &pos)
	if buf[pos] == 1 {
		pos++
		s := readString(buf, &pos)
		next_cursor = &s
	} else {
		pos++
	}
	value := LedgerHistoryBatch{
		Account:    account,
		Entries:    entries,
		HasMore:    has_more,
		NextCursor: next_cursor,
	}
	return value, pos, nil
}

// SBE message BatchRequest
type BatchRequest struct {
	BatchId string
	Account string
	Actions []BatchAction
}

// SBE body encoder for BatchRequest (fields only, no header).
type BatchRequestBodyEncoder struct{}

func (BatchRequestBodyEncoder) Encode(value BatchRequest, buf *[]byte) {
	local := *buf
	writeString(&local, value.BatchId)
	writeString(&local, value.Account)
	writeU32BE(&local, uint32(len(value.Actions)))
	for _, item := range value.Actions {
		var item_buf []byte
		BatchActionEncoder{}.Encode(item, &item_buf)
		writeU32BE(&local, uint32(len(item_buf)))
		local = append(local, item_buf...)
	}
	*buf = local
}

// SBE body decoder for BatchRequest: reads fields at pos, returns the value and the new offset.
func BatchRequestBodyDecoderDecode(buf []byte, pos int) (BatchRequest, int, error) {
	batch_id := readString(buf, &pos)
	account := readString(buf, &pos)
	actions_count := readU32BE(buf, &pos)
	actions := make([]BatchAction, 0, actions_count)
	for i := 0; i < int(actions_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := BatchActionDecoderDecode(buf, pos)
		if err != nil {
			return BatchRequest{}, pos, err
		}
		pos = item_end
		actions = append(actions, item)
	}
	value := BatchRequest{
		BatchId: batch_id,
		Account: account,
		Actions: actions,
	}
	return value, pos, nil
}

// SBE encoder for BatchRequest
type BatchRequestEncoder struct{}

// Encode builds a complete message (header + body).
func (BatchRequestEncoder) Encode(BatchId string, Account string, Actions []BatchAction) ([]byte, error) {
	buf := make([]byte, 0, 256)
	writeU16BE(&buf, SCHEMA_ID)
	writeU16BE(&buf, 63)
	writeU16BE(&buf, 0)
	writeU16BE(&buf, 0)
	writeString(&buf, BatchId)
	writeString(&buf, Account)
	writeU32BE(&buf, uint32(len(Actions)))
	for _, item := range Actions {
		var item_buf []byte
		BatchActionEncoder{}.Encode(item, &item_buf)
		writeU32BE(&buf, uint32(len(item_buf)))
		buf = append(buf, item_buf...)
	}
	return buf, nil
}

// SBE decoder for BatchRequest: validates the header and reads the body.
// Returns the value, the offset just past the message, and an error.
func BatchRequestDecoderDecode(buf []byte) (BatchRequest, int, error) {
	if len(buf) < 8 {
		return BatchRequest{}, 0, fmt.Errorf("buffer too short for SBE header")
	}
	pos := 0
	schemaID := readU16BE(buf, &pos)
	tmplID := readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	_ = readU16BE(buf, &pos)
	if schemaID != SCHEMA_ID {
		return BatchRequest{}, pos, fmt.Errorf("invalid schema_id: %d", schemaID)
	}
	if tmplID != 63 {
		return BatchRequest{}, pos, fmt.Errorf("invalid template_id: %d", tmplID)
	}
	batch_id := readString(buf, &pos)
	account := readString(buf, &pos)
	actions_count := readU32BE(buf, &pos)
	actions := make([]BatchAction, 0, actions_count)
	for i := 0; i < int(actions_count); i++ {
		item_len := int(readU32BE(buf, &pos))
		item_end := pos + item_len
		item, _, err := BatchActionDecoderDecode(buf, pos)
		if err != nil {
			return BatchRequest{}, pos, err
		}
		pos = item_end
		actions = append(actions, item)
	}
	value := BatchRequest{
		BatchId: batch_id,
		Account: account,
		Actions: actions,
	}
	return value, pos, nil
}
