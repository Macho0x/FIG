// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' v1.0.0
// Standard order entry and execution messages

using System;
using System.Collections.Generic;
using System.Text;

namespace Fig.Sbe
{
    public static class Schema
    {
        public const ushort SCHEMA_ID = 0x01;
    }


    internal static class Wire
    {
        public static void WriteStringU32(List<byte> buf, string s) { var b = System.Text.Encoding.UTF8.GetBytes(s); WriteU32BE(buf, (uint)b.Length); buf.AddRange(b); }
        public static string ReadStringU32(byte[] buf, ref int pos) { uint n = ReadU32BE(buf, ref pos); var s = System.Text.Encoding.UTF8.GetString(buf, pos, (int)n); pos += (int)n; return s; }
        public static byte TakeU8(byte[] buf, ref int pos) => buf[pos++];
        public static void WriteU16BE(List<byte> buf, ushort v) { buf.Add((byte)(v >> 8)); buf.Add((byte)v); }
        public static void WriteU32BE(List<byte> buf, uint v) {
            buf.Add((byte)(v >> 24)); buf.Add((byte)(v >> 16)); buf.Add((byte)(v >> 8)); buf.Add((byte)v);
        }
        public static void WriteU64BE(List<byte> buf, ulong v) {
            for (int i = 7; i >= 0; i--) buf.Add((byte)((v >> (i * 8)) & 0xFF));
        }
        public static void WriteF64BE(List<byte> buf, double v) => WriteU64BE(buf, BitConverter.DoubleToUInt64Bits(v));
        public static void WriteI64BE(List<byte> buf, long v) => WriteU64BE(buf, (ulong)v);
        public static void WriteString(List<byte> buf, string s) {
            var bytes = Encoding.UTF8.GetBytes(s); WriteU16BE(buf, (ushort)bytes.Length); buf.AddRange(bytes);
        }

        public static ushort ReadU16BE(byte[] buf, ref int pos) {
            ushort v = (ushort)((buf[pos] << 8) | buf[pos + 1]); pos += 2; return v;
        }
        public static uint ReadU32BE(byte[] buf, ref int pos) {
            uint v = ((uint)buf[pos] << 24) | ((uint)buf[pos + 1] << 16) | ((uint)buf[pos + 2] << 8) | buf[pos + 3];
            pos += 4; return v;
        }
        public static ulong ReadU64BE(byte[] buf, ref int pos) {
            ulong v = 0; for (int i = 0; i < 8; i++) v = (v << 8) | buf[pos + i]; pos += 8; return v;
        }
        public static double ReadF64BE(byte[] buf, ref int pos) => BitConverter.Int64BitsToDouble((long)ReadU64BE(buf, ref pos));
        public static long ReadI64BE(byte[] buf, ref int pos) => (long)ReadU64BE(buf, ref pos);
        public static string ReadString(byte[] buf, ref int pos) {
            ushort len = ReadU16BE(buf, ref pos); var s = Encoding.UTF8.GetString(buf, pos, len); pos += len; return s;
        }
    }

    public enum NewOrderSingleSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
        SellShortExempt = 4,
    }

    public static class NewOrderSingleSideHelpers
    {
        public static NewOrderSingleSide? FromValue(byte v) => v switch {
        1 => NewOrderSingleSide.Buy,
        2 => NewOrderSingleSide.Sell,
        3 => NewOrderSingleSide.SellShort,
        4 => NewOrderSingleSide.SellShortExempt,
            _ => null,
        };

        public static byte ToValue(NewOrderSingleSide e) => (byte)e;
    }

    public enum NewOrderSingleOrderType : byte
    {
        Market = 1,
        Limit = 2,
        Stop = 3,
        StopLimit = 4,
        MarketOnClose = 5,
        LimitOnClose = 6,
        Pegged = 7,
    }

    public static class NewOrderSingleOrderTypeHelpers
    {
        public static NewOrderSingleOrderType? FromValue(byte v) => v switch {
        1 => NewOrderSingleOrderType.Market,
        2 => NewOrderSingleOrderType.Limit,
        3 => NewOrderSingleOrderType.Stop,
        4 => NewOrderSingleOrderType.StopLimit,
        5 => NewOrderSingleOrderType.MarketOnClose,
        6 => NewOrderSingleOrderType.LimitOnClose,
        7 => NewOrderSingleOrderType.Pegged,
            _ => null,
        };

        public static byte ToValue(NewOrderSingleOrderType e) => (byte)e;
    }

    public enum NewOrderSingleTimeInForce : byte
    {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
        Gtd = 5,
    }

    public static class NewOrderSingleTimeInForceHelpers
    {
        public static NewOrderSingleTimeInForce? FromValue(byte v) => v switch {
        1 => NewOrderSingleTimeInForce.Day,
        2 => NewOrderSingleTimeInForce.Gtc,
        3 => NewOrderSingleTimeInForce.Ioc,
        4 => NewOrderSingleTimeInForce.Fok,
        5 => NewOrderSingleTimeInForce.Gtd,
            _ => null,
        };

        public static byte ToValue(NewOrderSingleTimeInForce e) => (byte)e;
    }

    public enum NewOrderSingleIdSource : byte
    {
        Cusip = 1,
        Sedol = 2,
        Isin = 3,
        Ric = 4,
        ExchangeSymbol = 5,
    }

    public static class NewOrderSingleIdSourceHelpers
    {
        public static NewOrderSingleIdSource? FromValue(byte v) => v switch {
        1 => NewOrderSingleIdSource.Cusip,
        2 => NewOrderSingleIdSource.Sedol,
        3 => NewOrderSingleIdSource.Isin,
        4 => NewOrderSingleIdSource.Ric,
        5 => NewOrderSingleIdSource.ExchangeSymbol,
            _ => null,
        };

        public static byte ToValue(NewOrderSingleIdSource e) => (byte)e;
    }

    public enum CancelRequestSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
    }

    public static class CancelRequestSideHelpers
    {
        public static CancelRequestSide? FromValue(byte v) => v switch {
        1 => CancelRequestSide.Buy,
        2 => CancelRequestSide.Sell,
        3 => CancelRequestSide.SellShort,
            _ => null,
        };

        public static byte ToValue(CancelRequestSide e) => (byte)e;
    }

    public enum CancelReplaceRequestSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
    }

    public static class CancelReplaceRequestSideHelpers
    {
        public static CancelReplaceRequestSide? FromValue(byte v) => v switch {
        1 => CancelReplaceRequestSide.Buy,
        2 => CancelReplaceRequestSide.Sell,
        3 => CancelReplaceRequestSide.SellShort,
            _ => null,
        };

        public static byte ToValue(CancelReplaceRequestSide e) => (byte)e;
    }

    public enum ExecutionReportExecType : byte
    {
        New = 1,
        PartialFill = 2,
        Fill = 3,
        DoneForDay = 4,
        Canceled = 5,
        Replaced = 6,
        PendingCancel = 7,
        Stopped = 8,
        Rejected = 9,
        Suspended = 10,
        PendingNew = 11,
        Expired = 12,
    }

    public static class ExecutionReportExecTypeHelpers
    {
        public static ExecutionReportExecType? FromValue(byte v) => v switch {
        1 => ExecutionReportExecType.New,
        2 => ExecutionReportExecType.PartialFill,
        3 => ExecutionReportExecType.Fill,
        4 => ExecutionReportExecType.DoneForDay,
        5 => ExecutionReportExecType.Canceled,
        6 => ExecutionReportExecType.Replaced,
        7 => ExecutionReportExecType.PendingCancel,
        8 => ExecutionReportExecType.Stopped,
        9 => ExecutionReportExecType.Rejected,
        10 => ExecutionReportExecType.Suspended,
        11 => ExecutionReportExecType.PendingNew,
        12 => ExecutionReportExecType.Expired,
            _ => null,
        };

        public static byte ToValue(ExecutionReportExecType e) => (byte)e;
    }

    public enum ExecutionReportOrdStatus : byte
    {
        New = 1,
        PartiallyFilled = 2,
        Filled = 3,
        DoneForDay = 4,
        Canceled = 5,
        PendingCancel = 6,
        Stopped = 7,
        Rejected = 8,
        Suspended = 9,
        PendingNew = 10,
        Expired = 11,
        Replaced = 12,
    }

    public static class ExecutionReportOrdStatusHelpers
    {
        public static ExecutionReportOrdStatus? FromValue(byte v) => v switch {
        1 => ExecutionReportOrdStatus.New,
        2 => ExecutionReportOrdStatus.PartiallyFilled,
        3 => ExecutionReportOrdStatus.Filled,
        4 => ExecutionReportOrdStatus.DoneForDay,
        5 => ExecutionReportOrdStatus.Canceled,
        6 => ExecutionReportOrdStatus.PendingCancel,
        7 => ExecutionReportOrdStatus.Stopped,
        8 => ExecutionReportOrdStatus.Rejected,
        9 => ExecutionReportOrdStatus.Suspended,
        10 => ExecutionReportOrdStatus.PendingNew,
        11 => ExecutionReportOrdStatus.Expired,
        12 => ExecutionReportOrdStatus.Replaced,
            _ => null,
        };

        public static byte ToValue(ExecutionReportOrdStatus e) => (byte)e;
    }

    public enum ExecutionReportSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
    }

    public static class ExecutionReportSideHelpers
    {
        public static ExecutionReportSide? FromValue(byte v) => v switch {
        1 => ExecutionReportSide.Buy,
        2 => ExecutionReportSide.Sell,
        3 => ExecutionReportSide.SellShort,
            _ => null,
        };

        public static byte ToValue(ExecutionReportSide e) => (byte)e;
    }

    public enum CancelRejectRejectReason : byte
    {
        OrderNotFound = 1,
        AlreadyCanceled = 2,
        AlreadyFilled = 3,
        TooLateToCancel = 4,
    }

    public static class CancelRejectRejectReasonHelpers
    {
        public static CancelRejectRejectReason? FromValue(byte v) => v switch {
        1 => CancelRejectRejectReason.OrderNotFound,
        2 => CancelRejectRejectReason.AlreadyCanceled,
        3 => CancelRejectRejectReason.AlreadyFilled,
        4 => CancelRejectRejectReason.TooLateToCancel,
            _ => null,
        };

        public static byte ToValue(CancelRejectRejectReason e) => (byte)e;
    }

    public enum BalanceUpdateReason : byte
    {
        Trade = 1,
        Deposit = 2,
        Withdrawal = 3,
        Transfer = 4,
        Fee = 5,
    }

    public static class BalanceUpdateReasonHelpers
    {
        public static BalanceUpdateReason? FromValue(byte v) => v switch {
        1 => BalanceUpdateReason.Trade,
        2 => BalanceUpdateReason.Deposit,
        3 => BalanceUpdateReason.Withdrawal,
        4 => BalanceUpdateReason.Transfer,
        5 => BalanceUpdateReason.Fee,
            _ => null,
        };

        public static byte ToValue(BalanceUpdateReason e) => (byte)e;
    }

    public enum OrderListStatusStatus : byte
    {
        Executing = 1,
        AllDone = 2,
        Reject = 3,
    }

    public static class OrderListStatusStatusHelpers
    {
        public static OrderListStatusStatus? FromValue(byte v) => v switch {
        1 => OrderListStatusStatus.Executing,
        2 => OrderListStatusStatus.AllDone,
        3 => OrderListStatusStatus.Reject,
            _ => null,
        };

        public static byte ToValue(OrderListStatusStatus e) => (byte)e;
    }

    public enum LedgerUpdateKind : byte
    {
        Deposit = 1,
        Withdrawal = 2,
        Transfer = 3,
        Fee = 4,
        Funding = 5,
    }

    public static class LedgerUpdateKindHelpers
    {
        public static LedgerUpdateKind? FromValue(byte v) => v switch {
        1 => LedgerUpdateKind.Deposit,
        2 => LedgerUpdateKind.Withdrawal,
        3 => LedgerUpdateKind.Transfer,
        4 => LedgerUpdateKind.Fee,
        5 => LedgerUpdateKind.Funding,
            _ => null,
        };

        public static byte ToValue(LedgerUpdateKind e) => (byte)e;
    }

    public enum TradingSessionStatusStatus : byte
    {
        PreOpen = 1,
        Open = 2,
        Break = 3,
        Close = 4,
        Halt = 5,
        Auction = 6,
    }

    public static class TradingSessionStatusStatusHelpers
    {
        public static TradingSessionStatusStatus? FromValue(byte v) => v switch {
        1 => TradingSessionStatusStatus.PreOpen,
        2 => TradingSessionStatusStatus.Open,
        3 => TradingSessionStatusStatus.Break,
        4 => TradingSessionStatusStatus.Close,
        5 => TradingSessionStatusStatus.Halt,
        6 => TradingSessionStatusStatus.Auction,
            _ => null,
        };

        public static byte ToValue(TradingSessionStatusStatus e) => (byte)e;
    }

    public enum TradingSessionStatusHaltReason : byte
    {
        None = 1,
        News = 2,
        MatchingEngine = 3,
        Regulatory = 4,
        Auction = 5,
        Other = 6,
    }

    public static class TradingSessionStatusHaltReasonHelpers
    {
        public static TradingSessionStatusHaltReason? FromValue(byte v) => v switch {
        1 => TradingSessionStatusHaltReason.None,
        2 => TradingSessionStatusHaltReason.News,
        3 => TradingSessionStatusHaltReason.MatchingEngine,
        4 => TradingSessionStatusHaltReason.Regulatory,
        5 => TradingSessionStatusHaltReason.Auction,
        6 => TradingSessionStatusHaltReason.Other,
            _ => null,
        };

        public static byte ToValue(TradingSessionStatusHaltReason e) => (byte)e;
    }

    public enum SecurityStatusStatus : byte
    {
        Trading = 1,
        Halt = 2,
        Auction = 3,
        Suspended = 4,
    }

    public static class SecurityStatusStatusHelpers
    {
        public static SecurityStatusStatus? FromValue(byte v) => v switch {
        1 => SecurityStatusStatus.Trading,
        2 => SecurityStatusStatus.Halt,
        3 => SecurityStatusStatus.Auction,
        4 => SecurityStatusStatus.Suspended,
            _ => null,
        };

        public static byte ToValue(SecurityStatusStatus e) => (byte)e;
    }

    public enum SecurityStatusHaltReason : byte
    {
        None = 1,
        News = 2,
        MatchingEngine = 3,
        Regulatory = 4,
        Auction = 5,
        Other = 6,
    }

    public static class SecurityStatusHaltReasonHelpers
    {
        public static SecurityStatusHaltReason? FromValue(byte v) => v switch {
        1 => SecurityStatusHaltReason.None,
        2 => SecurityStatusHaltReason.News,
        3 => SecurityStatusHaltReason.MatchingEngine,
        4 => SecurityStatusHaltReason.Regulatory,
        5 => SecurityStatusHaltReason.Auction,
        6 => SecurityStatusHaltReason.Other,
            _ => null,
        };

        public static byte ToValue(SecurityStatusHaltReason e) => (byte)e;
    }

    public enum MassCancelRequestSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
        SellShortExempt = 4,
    }

    public static class MassCancelRequestSideHelpers
    {
        public static MassCancelRequestSide? FromValue(byte v) => v switch {
        1 => MassCancelRequestSide.Buy,
        2 => MassCancelRequestSide.Sell,
        3 => MassCancelRequestSide.SellShort,
        4 => MassCancelRequestSide.SellShortExempt,
            _ => null,
        };

        public static byte ToValue(MassCancelRequestSide e) => (byte)e;
    }

    public enum MassCancelReportStatus : byte
    {
        Accepted = 1,
        Completed = 2,
        Rejected = 3,
    }

    public static class MassCancelReportStatusHelpers
    {
        public static MassCancelReportStatus? FromValue(byte v) => v switch {
        1 => MassCancelReportStatus.Accepted,
        2 => MassCancelReportStatus.Completed,
        3 => MassCancelReportStatus.Rejected,
            _ => null,
        };

        public static byte ToValue(MassCancelReportStatus e) => (byte)e;
    }

    public enum AggregateTradeSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static class AggregateTradeSideHelpers
    {
        public static AggregateTradeSide? FromValue(byte v) => v switch {
        1 => AggregateTradeSide.Buy,
        2 => AggregateTradeSide.Sell,
            _ => null,
        };

        public static byte ToValue(AggregateTradeSide e) => (byte)e;
    }

    public enum LiquidationTradeSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static class LiquidationTradeSideHelpers
    {
        public static LiquidationTradeSide? FromValue(byte v) => v switch {
        1 => LiquidationTradeSide.Buy,
        2 => LiquidationTradeSide.Sell,
            _ => null,
        };

        public static byte ToValue(LiquidationTradeSide e) => (byte)e;
    }

    public enum MarketDataUpdateSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static class MarketDataUpdateSideHelpers
    {
        public static MarketDataUpdateSide? FromValue(byte v) => v switch {
        1 => MarketDataUpdateSide.Buy,
        2 => MarketDataUpdateSide.Sell,
            _ => null,
        };

        public static byte ToValue(MarketDataUpdateSide e) => (byte)e;
    }

    public enum MarketDataUpdateAction : byte
    {
        New = 1,
        Change = 2,
        Delete = 3,
    }

    public static class MarketDataUpdateActionHelpers
    {
        public static MarketDataUpdateAction? FromValue(byte v) => v switch {
        1 => MarketDataUpdateAction.New,
        2 => MarketDataUpdateAction.Change,
        3 => MarketDataUpdateAction.Delete,
            _ => null,
        };

        public static byte ToValue(MarketDataUpdateAction e) => (byte)e;
    }

    public enum PublicTradeSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static class PublicTradeSideHelpers
    {
        public static PublicTradeSide? FromValue(byte v) => v switch {
        1 => PublicTradeSide.Buy,
        2 => PublicTradeSide.Sell,
            _ => null,
        };

        public static byte ToValue(PublicTradeSide e) => (byte)e;
    }

    public enum CapabilityPathPattern : byte
    {
        PubSub = 1,
        RequestResponse = 2,
        RequestStream = 3,
    }

    public static class CapabilityPathPatternHelpers
    {
        public static CapabilityPathPattern? FromValue(byte v) => v switch {
        1 => CapabilityPathPattern.PubSub,
        2 => CapabilityPathPattern.RequestResponse,
        3 => CapabilityPathPattern.RequestStream,
            _ => null,
        };

        public static byte ToValue(CapabilityPathPattern e) => (byte)e;
    }

    // SBE struct AggregateTrade
    public struct AggregateTrade
    {
        public string Symbol;
        public string AggTradeId;
        public double Price;
        public double Qty;
        public AggregateTradeSide Side;
        public string FirstTradeId;
        public string LastTradeId;
        public long Timestamp;
    }

public static class AggregateTradeEncoder
    {
        public static void Encode(AggregateTrade value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.AggTradeId);
            Wire.WriteF64BE(buf, value.Price);
            Wire.WriteF64BE(buf, value.Qty);
            buf.Add(AggregateTradeSideHelpers.ToValue(value.Side));
            Wire.WriteString(buf, value.FirstTradeId);
            Wire.WriteString(buf, value.LastTradeId);
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
        }
    }

public static class AggregateTradeDecoder
    {
        public static (AggregateTrade Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AggregateTrade();
            string symbol = Wire.ReadString(buf, ref pos);
            string agg_trade_id = Wire.ReadString(buf, ref pos);
            double price = Wire.ReadF64BE(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            var side = AggregateTradeSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid AggregateTradeSide");
            string first_trade_id = Wire.ReadString(buf, ref pos);
            string last_trade_id = Wire.ReadString(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Symbol = symbol;
            value.AggTradeId = agg_trade_id;
            value.Price = price;
            value.Qty = qty;
            value.Side = side;
            value.FirstTradeId = first_trade_id;
            value.LastTradeId = last_trade_id;
            value.Timestamp = timestamp;
            return (value, pos);
        }
    }

    // SBE struct LiquidationTrade
    public struct LiquidationTrade
    {
        public string Symbol;
        public LiquidationTradeSide Side;
        public double Price;
        public double Qty;
        public long Timestamp;
    }

public static class LiquidationTradeEncoder
    {
        public static void Encode(LiquidationTrade value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            buf.Add(LiquidationTradeSideHelpers.ToValue(value.Side));
            Wire.WriteF64BE(buf, value.Price);
            Wire.WriteF64BE(buf, value.Qty);
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
        }
    }

public static class LiquidationTradeDecoder
    {
        public static (LiquidationTrade Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new LiquidationTrade();
            string symbol = Wire.ReadString(buf, ref pos);
            var side = LiquidationTradeSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid LiquidationTradeSide");
            double price = Wire.ReadF64BE(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Symbol = symbol;
            value.Side = side;
            value.Price = price;
            value.Qty = qty;
            value.Timestamp = timestamp;
            return (value, pos);
        }
    }

    // SBE struct PriceLevel
    public struct PriceLevel
    {
        public double Price;
        public double Qty;
        public uint? OrderCount;
    }

public static class PriceLevelEncoder
    {
        public static void Encode(PriceLevel value, List<byte> buf)
        {
            Wire.WriteF64BE(buf, value.Price);
            Wire.WriteF64BE(buf, value.Qty);
            Wire.WriteU32BE(buf, value.OrderCount ?? (uint)0);
            buf.Add((byte)(value.OrderCount.HasValue ? 1 : 0));
        }
    }

public static class PriceLevelDecoder
    {
        public static (PriceLevel Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PriceLevel();
            double price = Wire.ReadF64BE(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            uint order_count_v = Wire.ReadU32BE(buf, ref pos);
            uint? order_count = buf[pos++] == 1 ? order_count_v : null;
            value.Price = price;
            value.Qty = qty;
            value.OrderCount = order_count;
            return (value, pos);
        }
    }

    // SBE struct MarketDataUpdate
    public struct MarketDataUpdate
    {
        public MarketDataUpdateSide Side;
        public MarketDataUpdateAction Action;
        public double Price;
        public double Qty;
    }

public static class MarketDataUpdateEncoder
    {
        public static void Encode(MarketDataUpdate value, List<byte> buf)
        {
            buf.Add(MarketDataUpdateSideHelpers.ToValue(value.Side));
            buf.Add(MarketDataUpdateActionHelpers.ToValue(value.Action));
            Wire.WriteF64BE(buf, value.Price);
            Wire.WriteF64BE(buf, value.Qty);
        }
    }

public static class MarketDataUpdateDecoder
    {
        public static (MarketDataUpdate Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarketDataUpdate();
            var side = MarketDataUpdateSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid MarketDataUpdateSide");
            var action = MarketDataUpdateActionHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid MarketDataUpdateAction");
            double price = Wire.ReadF64BE(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            value.Side = side;
            value.Action = action;
            value.Price = price;
            value.Qty = qty;
            return (value, pos);
        }
    }

    // SBE struct CandleBar
    public struct CandleBar
    {
        public string Symbol;
        public string Interval;
        public double Open;
        public double High;
        public double Low;
        public double Close;
        public double Volume;
        public long BarStart;
        public long BarEnd;
        public byte IsFinal;
        public byte? IsSnapshot;
    }

public static class CandleBarEncoder
    {
        public static void Encode(CandleBar value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.Interval);
            Wire.WriteF64BE(buf, value.Open);
            Wire.WriteF64BE(buf, value.High);
            Wire.WriteF64BE(buf, value.Low);
            Wire.WriteF64BE(buf, value.Close);
            Wire.WriteF64BE(buf, value.Volume);
            Wire.WriteU64BE(buf, (ulong)(value.BarStart));
            Wire.WriteU64BE(buf, (ulong)(value.BarEnd));
            buf.Add(value.IsFinal);
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

public static class CandleBarDecoder
    {
        public static (CandleBar Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CandleBar();
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            double open = Wire.ReadF64BE(buf, ref pos);
            double high = Wire.ReadF64BE(buf, ref pos);
            double low = Wire.ReadF64BE(buf, ref pos);
            double close = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            long bar_start = (long)Wire.ReadU64BE(buf, ref pos);
            long bar_end = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_final = Wire.TakeU8(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Interval = interval;
            value.Open = open;
            value.High = high;
            value.Low = low;
            value.Close = close;
            value.Volume = volume;
            value.BarStart = bar_start;
            value.BarEnd = bar_end;
            value.IsFinal = is_final;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    // SBE struct PublicTrade
    public struct PublicTrade
    {
        public string Symbol;
        public string TradeId;
        public double Price;
        public double Qty;
        public PublicTradeSide Side;
        public long Timestamp;
    }

public static class PublicTradeEncoder
    {
        public static void Encode(PublicTrade value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.TradeId);
            Wire.WriteF64BE(buf, value.Price);
            Wire.WriteF64BE(buf, value.Qty);
            buf.Add(PublicTradeSideHelpers.ToValue(value.Side));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
        }
    }

public static class PublicTradeDecoder
    {
        public static (PublicTrade Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PublicTrade();
            string symbol = Wire.ReadString(buf, ref pos);
            string trade_id = Wire.ReadString(buf, ref pos);
            double price = Wire.ReadF64BE(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            var side = PublicTradeSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid PublicTradeSide");
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Symbol = symbol;
            value.TradeId = trade_id;
            value.Price = price;
            value.Qty = qty;
            value.Side = side;
            value.Timestamp = timestamp;
            return (value, pos);
        }
    }

    // SBE struct InstrumentMetadata
    public struct InstrumentMetadata
    {
        public string InstrumentId;
        public string Symbol;
        public string ProductKind;
        public string? MarginAsset;
        public string? DisplayName;
        public double? TickSize;
        public double? LotSize;
    }

public static class InstrumentMetadataEncoder
    {
        public static void Encode(InstrumentMetadata value, List<byte> buf)
        {
            Wire.WriteString(buf, value.InstrumentId);
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.ProductKind);
            if (value.MarginAsset != null) { buf.Add(1); Wire.WriteString(buf, value.MarginAsset); } else { buf.Add(0); }
            if (value.DisplayName != null) { buf.Add(1); Wire.WriteString(buf, value.DisplayName); } else { buf.Add(0); }
            Wire.WriteF64BE(buf, value.TickSize ?? 0.0);
            buf.Add((byte)(value.TickSize.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.LotSize ?? 0.0);
            buf.Add((byte)(value.LotSize.HasValue ? 1 : 0));
        }
    }

public static class InstrumentMetadataDecoder
    {
        public static (InstrumentMetadata Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new InstrumentMetadata();
            string? margin_asset = null;
            string? display_name = null;
            string instrument_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            string product_kind = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) margin_asset = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) display_name = Wire.ReadString(buf, ref pos);
            double tick_size_v = Wire.ReadF64BE(buf, ref pos);
            double? tick_size = buf[pos++] == 1 ? tick_size_v : null;
            double lot_size_v = Wire.ReadF64BE(buf, ref pos);
            double? lot_size = buf[pos++] == 1 ? lot_size_v : null;
            value.InstrumentId = instrument_id;
            value.Symbol = symbol;
            value.ProductKind = product_kind;
            value.MarginAsset = margin_asset;
            value.DisplayName = display_name;
            value.TickSize = tick_size;
            value.LotSize = lot_size;
            return (value, pos);
        }
    }

    // SBE struct BalanceEntry
    public struct BalanceEntry
    {
        public string Asset;
        public double Total;
        public double Available;
        public double Hold;
    }

public static class BalanceEntryEncoder
    {
        public static void Encode(BalanceEntry value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Asset);
            Wire.WriteF64BE(buf, value.Total);
            Wire.WriteF64BE(buf, value.Available);
            Wire.WriteF64BE(buf, value.Hold);
        }
    }

public static class BalanceEntryDecoder
    {
        public static (BalanceEntry Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new BalanceEntry();
            string asset = Wire.ReadString(buf, ref pos);
            double total = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            double hold = Wire.ReadF64BE(buf, ref pos);
            value.Asset = asset;
            value.Total = total;
            value.Available = available;
            value.Hold = hold;
            return (value, pos);
        }
    }

    // SBE struct PositionEntry
    public struct PositionEntry
    {
        public string Symbol;
        public double Qty;
        public double EntryPrice;
        public double UnrealizedPnl;
    }

public static class PositionEntryEncoder
    {
        public static void Encode(PositionEntry value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.Qty);
            Wire.WriteF64BE(buf, value.EntryPrice);
            Wire.WriteF64BE(buf, value.UnrealizedPnl);
        }
    }

public static class PositionEntryDecoder
    {
        public static (PositionEntry Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PositionEntry();
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double entry_price = Wire.ReadF64BE(buf, ref pos);
            double unrealized_pnl = Wire.ReadF64BE(buf, ref pos);
            value.Symbol = symbol;
            value.Qty = qty;
            value.EntryPrice = entry_price;
            value.UnrealizedPnl = unrealized_pnl;
            return (value, pos);
        }
    }

    // SBE struct PageInfo
    public struct PageInfo
    {
        public byte HasMore;
        public string? NextCursor;
    }

public static class PageInfoEncoder
    {
        public static void Encode(PageInfo value, List<byte> buf)
        {
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

public static class PageInfoDecoder
    {
        public static (PageInfo Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PageInfo();
            string? next_cursor = null;
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    // SBE struct CapabilityPath
    public struct CapabilityPath
    {
        public string Path;
        public CapabilityPathPattern Pattern;
        public byte AuthRequired;
    }

public static class CapabilityPathEncoder
    {
        public static void Encode(CapabilityPath value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Path);
            buf.Add(CapabilityPathPatternHelpers.ToValue(value.Pattern));
            buf.Add(value.AuthRequired);
        }
    }

public static class CapabilityPathDecoder
    {
        public static (CapabilityPath Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CapabilityPath();
            string path = Wire.ReadString(buf, ref pos);
            var pattern = CapabilityPathPatternHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CapabilityPathPattern");
            byte auth_required = Wire.TakeU8(buf, ref pos);
            value.Path = path;
            value.Pattern = pattern;
            value.AuthRequired = auth_required;
            return (value, pos);
        }
    }

    /// <summary>SBE message NewOrderSingle</summary>
    public class NewOrderSingle
    {
        public string ClOrdId;
        public NewOrderSingleSide Side;
        public double OrderQty;
        public double? Price;
        public double? StopPrice;
        public string Symbol;
        public NewOrderSingleOrderType OrderType;
        public NewOrderSingleTimeInForce TimeInForce;
        public long? ExpireTime;
        public string? Account;
        public string? StrategyId;
        public string? SecurityId;
        public NewOrderSingleIdSource? IdSource;
        public string? SecurityExchange;
        public byte? PostOnly;
        public byte? ReduceOnly;
    }

    /// <summary>SBE body encoder for NewOrderSingle (fields only, no header).</summary>
    public static class NewOrderSingleBodyEncoder
    {
        public static void Encode(NewOrderSingle value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            buf.Add(NewOrderSingleSideHelpers.ToValue(value.Side));
            Wire.WriteF64BE(buf, value.OrderQty);
            Wire.WriteF64BE(buf, value.Price ?? 0.0);
            buf.Add((byte)(value.Price.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.StopPrice ?? 0.0);
            buf.Add((byte)(value.StopPrice.HasValue ? 1 : 0));
            Wire.WriteString(buf, value.Symbol);
            buf.Add(NewOrderSingleOrderTypeHelpers.ToValue(value.OrderType));
            buf.Add(NewOrderSingleTimeInForceHelpers.ToValue(value.TimeInForce));
            Wire.WriteU64BE(buf, (ulong)(value.ExpireTime ?? (long)0));
            buf.Add((byte)(value.ExpireTime.HasValue ? 1 : 0));
            if (value.Account != null) { buf.Add(1); Wire.WriteString(buf, value.Account); } else { buf.Add(0); }
            if (value.StrategyId != null) { buf.Add(1); Wire.WriteString(buf, value.StrategyId); } else { buf.Add(0); }
            if (value.SecurityId != null) { buf.Add(1); Wire.WriteString(buf, value.SecurityId); } else { buf.Add(0); }
            buf.Add(value.IdSource.HasValue ? NewOrderSingleIdSourceHelpers.ToValue(value.IdSource.Value) : (byte)0);
            buf.Add((byte)(value.IdSource.HasValue ? 1 : 0));
            if (value.SecurityExchange != null) { buf.Add(1); Wire.WriteString(buf, value.SecurityExchange); } else { buf.Add(0); }
            buf.Add(value.PostOnly ?? (byte)0);
            buf.Add((byte)(value.PostOnly.HasValue ? 1 : 0));
            buf.Add(value.ReduceOnly ?? (byte)0);
            buf.Add((byte)(value.ReduceOnly.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for NewOrderSingle: reads fields at pos.</summary>
    public static class NewOrderSingleBodyDecoder
    {
        public static (NewOrderSingle Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new NewOrderSingle();
            string? account = null;
            string? strategy_id = null;
            string? security_id = null;
            string? security_exchange = null;
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            var side = NewOrderSingleSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid NewOrderSingleSide");
            double order_qty = Wire.ReadF64BE(buf, ref pos);
            double price_v = Wire.ReadF64BE(buf, ref pos);
            double? price = buf[pos++] == 1 ? price_v : null;
            double stop_price_v = Wire.ReadF64BE(buf, ref pos);
            double? stop_price = buf[pos++] == 1 ? stop_price_v : null;
            string symbol = Wire.ReadString(buf, ref pos);
            var order_type = NewOrderSingleOrderTypeHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid NewOrderSingleOrderType");
            var time_in_force = NewOrderSingleTimeInForceHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid NewOrderSingleTimeInForce");
            long expire_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? expire_time = buf[pos++] == 1 ? expire_time_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) strategy_id = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) security_id = Wire.ReadString(buf, ref pos);
            var id_sourceRaw = Wire.TakeU8(buf, ref pos);
            NewOrderSingleIdSource? id_source = null;
            if (buf[pos++] == 1) {
                id_source = NewOrderSingleIdSourceHelpers.FromValue(id_sourceRaw) ?? throw new InvalidOperationException("invalid NewOrderSingleIdSource");
            }
            if (Wire.TakeU8(buf, ref pos) == 1) security_exchange = Wire.ReadString(buf, ref pos);
            byte post_only_v = Wire.TakeU8(buf, ref pos);
            byte? post_only = buf[pos++] == 1 ? post_only_v : null;
            byte reduce_only_v = Wire.TakeU8(buf, ref pos);
            byte? reduce_only = buf[pos++] == 1 ? reduce_only_v : null;
            value.ClOrdId = cl_ord_id;
            value.Side = side;
            value.OrderQty = order_qty;
            value.Price = price;
            value.StopPrice = stop_price;
            value.Symbol = symbol;
            value.OrderType = order_type;
            value.TimeInForce = time_in_force;
            value.ExpireTime = expire_time;
            value.Account = account;
            value.StrategyId = strategy_id;
            value.SecurityId = security_id;
            value.IdSource = id_source;
            value.SecurityExchange = security_exchange;
            value.PostOnly = post_only;
            value.ReduceOnly = reduce_only;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for NewOrderSingle</summary>
    public static class NewOrderSingleEncoder
    {
        public static byte[] Encode(string ClOrdId, NewOrderSingleSide Side, double OrderQty, double? Price, double? StopPrice, string Symbol, NewOrderSingleOrderType OrderType, NewOrderSingleTimeInForce TimeInForce, long? ExpireTime, string? Account, string? StrategyId, string? SecurityId, NewOrderSingleIdSource? IdSource, string? SecurityExchange, byte? PostOnly, byte? ReduceOnly)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 38);
            Wire.WriteString(buf, ClOrdId);
            buf.Add(NewOrderSingleSideHelpers.ToValue(Side));
            Wire.WriteF64BE(buf, OrderQty);
            Wire.WriteF64BE(buf, Price ?? 0.0);
            buf.Add((byte)(Price.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, StopPrice ?? 0.0);
            buf.Add((byte)(StopPrice.HasValue ? 1 : 0));
            Wire.WriteString(buf, Symbol);
            buf.Add(NewOrderSingleOrderTypeHelpers.ToValue(OrderType));
            buf.Add(NewOrderSingleTimeInForceHelpers.ToValue(TimeInForce));
            Wire.WriteU64BE(buf, (ulong)(ExpireTime ?? (long)0));
            buf.Add((byte)(ExpireTime.HasValue ? 1 : 0));
            if (Account != null) { buf.Add(1); Wire.WriteString(buf, Account); } else { buf.Add(0); }
            if (StrategyId != null) { buf.Add(1); Wire.WriteString(buf, StrategyId); } else { buf.Add(0); }
            if (SecurityId != null) { buf.Add(1); Wire.WriteString(buf, SecurityId); } else { buf.Add(0); }
            buf.Add(IdSource.HasValue ? NewOrderSingleIdSourceHelpers.ToValue(IdSource.Value) : (byte)0);
            buf.Add((byte)(IdSource.HasValue ? 1 : 0));
            if (SecurityExchange != null) { buf.Add(1); Wire.WriteString(buf, SecurityExchange); } else { buf.Add(0); }
            buf.Add(PostOnly ?? (byte)0);
            buf.Add((byte)(PostOnly.HasValue ? 1 : 0));
            buf.Add(ReduceOnly ?? (byte)0);
            buf.Add((byte)(ReduceOnly.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for NewOrderSingle: validates the header and reads the body.</summary>
    public static class NewOrderSingleDecoder
    {
        public static NewOrderSingle Decode(byte[] buf)
        {
            var value = new NewOrderSingle();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 1) throw new InvalidOperationException("invalid template_id");
            string? account = null;
            string? strategy_id = null;
            string? security_id = null;
            string? security_exchange = null;
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            var side = NewOrderSingleSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid NewOrderSingleSide");
            double order_qty = Wire.ReadF64BE(buf, ref pos);
            double price_v = Wire.ReadF64BE(buf, ref pos);
            double? price = buf[pos++] == 1 ? price_v : null;
            double stop_price_v = Wire.ReadF64BE(buf, ref pos);
            double? stop_price = buf[pos++] == 1 ? stop_price_v : null;
            string symbol = Wire.ReadString(buf, ref pos);
            var order_type = NewOrderSingleOrderTypeHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid NewOrderSingleOrderType");
            var time_in_force = NewOrderSingleTimeInForceHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid NewOrderSingleTimeInForce");
            long expire_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? expire_time = buf[pos++] == 1 ? expire_time_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) strategy_id = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) security_id = Wire.ReadString(buf, ref pos);
            var id_sourceRaw = Wire.TakeU8(buf, ref pos);
            NewOrderSingleIdSource? id_source = null;
            if (buf[pos++] == 1) {
                id_source = NewOrderSingleIdSourceHelpers.FromValue(id_sourceRaw) ?? throw new InvalidOperationException("invalid NewOrderSingleIdSource");
            }
            if (Wire.TakeU8(buf, ref pos) == 1) security_exchange = Wire.ReadString(buf, ref pos);
            byte post_only_v = Wire.TakeU8(buf, ref pos);
            byte? post_only = buf[pos++] == 1 ? post_only_v : null;
            byte reduce_only_v = Wire.TakeU8(buf, ref pos);
            byte? reduce_only = buf[pos++] == 1 ? reduce_only_v : null;
            value.ClOrdId = cl_ord_id;
            value.Side = side;
            value.OrderQty = order_qty;
            value.Price = price;
            value.StopPrice = stop_price;
            value.Symbol = symbol;
            value.OrderType = order_type;
            value.TimeInForce = time_in_force;
            value.ExpireTime = expire_time;
            value.Account = account;
            value.StrategyId = strategy_id;
            value.SecurityId = security_id;
            value.IdSource = id_source;
            value.SecurityExchange = security_exchange;
            value.PostOnly = post_only;
            value.ReduceOnly = reduce_only;
            return value;
        }
    }

    /// <summary>SBE message CancelRequest</summary>
    public class CancelRequest
    {
        public string ClOrdId;
        public string OrigClOrdId;
        public string Symbol;
        public CancelRequestSide Side;
        public double? OrderQty;
    }

    /// <summary>SBE body encoder for CancelRequest (fields only, no header).</summary>
    public static class CancelRequestBodyEncoder
    {
        public static void Encode(CancelRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            Wire.WriteString(buf, value.OrigClOrdId);
            Wire.WriteString(buf, value.Symbol);
            buf.Add(CancelRequestSideHelpers.ToValue(value.Side));
            Wire.WriteF64BE(buf, value.OrderQty ?? 0.0);
            buf.Add((byte)(value.OrderQty.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for CancelRequest: reads fields at pos.</summary>
    public static class CancelRequestBodyDecoder
    {
        public static (CancelRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CancelRequest();
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            var side = CancelRequestSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CancelRequestSide");
            double order_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? order_qty = buf[pos++] == 1 ? order_qty_v : null;
            value.ClOrdId = cl_ord_id;
            value.OrigClOrdId = orig_cl_ord_id;
            value.Symbol = symbol;
            value.Side = side;
            value.OrderQty = order_qty;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CancelRequest</summary>
    public static class CancelRequestEncoder
    {
        public static byte[] Encode(string ClOrdId, string OrigClOrdId, string Symbol, CancelRequestSide Side, double? OrderQty)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 2);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 9);
            Wire.WriteString(buf, ClOrdId);
            Wire.WriteString(buf, OrigClOrdId);
            Wire.WriteString(buf, Symbol);
            buf.Add(CancelRequestSideHelpers.ToValue(Side));
            Wire.WriteF64BE(buf, OrderQty ?? 0.0);
            buf.Add((byte)(OrderQty.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CancelRequest: validates the header and reads the body.</summary>
    public static class CancelRequestDecoder
    {
        public static CancelRequest Decode(byte[] buf)
        {
            var value = new CancelRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 2) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            var side = CancelRequestSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CancelRequestSide");
            double order_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? order_qty = buf[pos++] == 1 ? order_qty_v : null;
            value.ClOrdId = cl_ord_id;
            value.OrigClOrdId = orig_cl_ord_id;
            value.Symbol = symbol;
            value.Side = side;
            value.OrderQty = order_qty;
            return value;
        }
    }

    /// <summary>SBE message CancelReplaceRequest</summary>
    public class CancelReplaceRequest
    {
        public string ClOrdId;
        public string OrigClOrdId;
        public string Symbol;
        public CancelReplaceRequestSide Side;
        public double OrderQty;
        public double? Price;
    }

    /// <summary>SBE body encoder for CancelReplaceRequest (fields only, no header).</summary>
    public static class CancelReplaceRequestBodyEncoder
    {
        public static void Encode(CancelReplaceRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            Wire.WriteString(buf, value.OrigClOrdId);
            Wire.WriteString(buf, value.Symbol);
            buf.Add(CancelReplaceRequestSideHelpers.ToValue(value.Side));
            Wire.WriteF64BE(buf, value.OrderQty);
            Wire.WriteF64BE(buf, value.Price ?? 0.0);
            buf.Add((byte)(value.Price.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for CancelReplaceRequest: reads fields at pos.</summary>
    public static class CancelReplaceRequestBodyDecoder
    {
        public static (CancelReplaceRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CancelReplaceRequest();
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            var side = CancelReplaceRequestSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CancelReplaceRequestSide");
            double order_qty = Wire.ReadF64BE(buf, ref pos);
            double price_v = Wire.ReadF64BE(buf, ref pos);
            double? price = buf[pos++] == 1 ? price_v : null;
            value.ClOrdId = cl_ord_id;
            value.OrigClOrdId = orig_cl_ord_id;
            value.Symbol = symbol;
            value.Side = side;
            value.OrderQty = order_qty;
            value.Price = price;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CancelReplaceRequest</summary>
    public static class CancelReplaceRequestEncoder
    {
        public static byte[] Encode(string ClOrdId, string OrigClOrdId, string Symbol, CancelReplaceRequestSide Side, double OrderQty, double? Price)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 3);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 17);
            Wire.WriteString(buf, ClOrdId);
            Wire.WriteString(buf, OrigClOrdId);
            Wire.WriteString(buf, Symbol);
            buf.Add(CancelReplaceRequestSideHelpers.ToValue(Side));
            Wire.WriteF64BE(buf, OrderQty);
            Wire.WriteF64BE(buf, Price ?? 0.0);
            buf.Add((byte)(Price.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CancelReplaceRequest: validates the header and reads the body.</summary>
    public static class CancelReplaceRequestDecoder
    {
        public static CancelReplaceRequest Decode(byte[] buf)
        {
            var value = new CancelReplaceRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 3) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            var side = CancelReplaceRequestSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CancelReplaceRequestSide");
            double order_qty = Wire.ReadF64BE(buf, ref pos);
            double price_v = Wire.ReadF64BE(buf, ref pos);
            double? price = buf[pos++] == 1 ? price_v : null;
            value.ClOrdId = cl_ord_id;
            value.OrigClOrdId = orig_cl_ord_id;
            value.Symbol = symbol;
            value.Side = side;
            value.OrderQty = order_qty;
            value.Price = price;
            return value;
        }
    }

    /// <summary>SBE message ExecutionReport</summary>
    public class ExecutionReport
    {
        public string ClOrdId;
        public string OrderId;
        public string ExecId;
        public ExecutionReportExecType ExecType;
        public ExecutionReportOrdStatus OrdStatus;
        public ExecutionReportSide Side;
        public double? LastQty;
        public double? LastPrice;
        public double LeavesQty;
        public double CumQty;
        public double AvgPrice;
        public string Symbol;
        public long TransactTime;
    }

    /// <summary>SBE body encoder for ExecutionReport (fields only, no header).</summary>
    public static class ExecutionReportBodyEncoder
    {
        public static void Encode(ExecutionReport value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            Wire.WriteString(buf, value.OrderId);
            Wire.WriteString(buf, value.ExecId);
            buf.Add(ExecutionReportExecTypeHelpers.ToValue(value.ExecType));
            buf.Add(ExecutionReportOrdStatusHelpers.ToValue(value.OrdStatus));
            buf.Add(ExecutionReportSideHelpers.ToValue(value.Side));
            Wire.WriteF64BE(buf, value.LastQty ?? 0.0);
            buf.Add((byte)(value.LastQty.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.LastPrice ?? 0.0);
            buf.Add((byte)(value.LastPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.LeavesQty);
            Wire.WriteF64BE(buf, value.CumQty);
            Wire.WriteF64BE(buf, value.AvgPrice);
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU64BE(buf, (ulong)(value.TransactTime));
        }
    }

    /// <summary>SBE body decoder for ExecutionReport: reads fields at pos.</summary>
    public static class ExecutionReportBodyDecoder
    {
        public static (ExecutionReport Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new ExecutionReport();
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string order_id = Wire.ReadString(buf, ref pos);
            string exec_id = Wire.ReadString(buf, ref pos);
            var exec_type = ExecutionReportExecTypeHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid ExecutionReportExecType");
            var ord_status = ExecutionReportOrdStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid ExecutionReportOrdStatus");
            var side = ExecutionReportSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid ExecutionReportSide");
            double last_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? last_qty = buf[pos++] == 1 ? last_qty_v : null;
            double last_price_v = Wire.ReadF64BE(buf, ref pos);
            double? last_price = buf[pos++] == 1 ? last_price_v : null;
            double leaves_qty = Wire.ReadF64BE(buf, ref pos);
            double cum_qty = Wire.ReadF64BE(buf, ref pos);
            double avg_price = Wire.ReadF64BE(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            long transact_time = (long)Wire.ReadU64BE(buf, ref pos);
            value.ClOrdId = cl_ord_id;
            value.OrderId = order_id;
            value.ExecId = exec_id;
            value.ExecType = exec_type;
            value.OrdStatus = ord_status;
            value.Side = side;
            value.LastQty = last_qty;
            value.LastPrice = last_price;
            value.LeavesQty = leaves_qty;
            value.CumQty = cum_qty;
            value.AvgPrice = avg_price;
            value.Symbol = symbol;
            value.TransactTime = transact_time;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for ExecutionReport</summary>
    public static class ExecutionReportEncoder
    {
        public static byte[] Encode(string ClOrdId, string OrderId, string ExecId, ExecutionReportExecType ExecType, ExecutionReportOrdStatus OrdStatus, ExecutionReportSide Side, double? LastQty, double? LastPrice, double LeavesQty, double CumQty, double AvgPrice, string Symbol, long TransactTime)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 4);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 51);
            Wire.WriteString(buf, ClOrdId);
            Wire.WriteString(buf, OrderId);
            Wire.WriteString(buf, ExecId);
            buf.Add(ExecutionReportExecTypeHelpers.ToValue(ExecType));
            buf.Add(ExecutionReportOrdStatusHelpers.ToValue(OrdStatus));
            buf.Add(ExecutionReportSideHelpers.ToValue(Side));
            Wire.WriteF64BE(buf, LastQty ?? 0.0);
            buf.Add((byte)(LastQty.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, LastPrice ?? 0.0);
            buf.Add((byte)(LastPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, LeavesQty);
            Wire.WriteF64BE(buf, CumQty);
            Wire.WriteF64BE(buf, AvgPrice);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU64BE(buf, (ulong)(TransactTime));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for ExecutionReport: validates the header and reads the body.</summary>
    public static class ExecutionReportDecoder
    {
        public static ExecutionReport Decode(byte[] buf)
        {
            var value = new ExecutionReport();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 4) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string order_id = Wire.ReadString(buf, ref pos);
            string exec_id = Wire.ReadString(buf, ref pos);
            var exec_type = ExecutionReportExecTypeHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid ExecutionReportExecType");
            var ord_status = ExecutionReportOrdStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid ExecutionReportOrdStatus");
            var side = ExecutionReportSideHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid ExecutionReportSide");
            double last_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? last_qty = buf[pos++] == 1 ? last_qty_v : null;
            double last_price_v = Wire.ReadF64BE(buf, ref pos);
            double? last_price = buf[pos++] == 1 ? last_price_v : null;
            double leaves_qty = Wire.ReadF64BE(buf, ref pos);
            double cum_qty = Wire.ReadF64BE(buf, ref pos);
            double avg_price = Wire.ReadF64BE(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            long transact_time = (long)Wire.ReadU64BE(buf, ref pos);
            value.ClOrdId = cl_ord_id;
            value.OrderId = order_id;
            value.ExecId = exec_id;
            value.ExecType = exec_type;
            value.OrdStatus = ord_status;
            value.Side = side;
            value.LastQty = last_qty;
            value.LastPrice = last_price;
            value.LeavesQty = leaves_qty;
            value.CumQty = cum_qty;
            value.AvgPrice = avg_price;
            value.Symbol = symbol;
            value.TransactTime = transact_time;
            return value;
        }
    }

    /// <summary>SBE message CancelReject</summary>
    public class CancelReject
    {
        public string ClOrdId;
        public string OrigClOrdId;
        public CancelRejectRejectReason RejectReason;
        public string Symbol;
    }

    /// <summary>SBE body encoder for CancelReject (fields only, no header).</summary>
    public static class CancelRejectBodyEncoder
    {
        public static void Encode(CancelReject value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            Wire.WriteString(buf, value.OrigClOrdId);
            buf.Add(CancelRejectRejectReasonHelpers.ToValue(value.RejectReason));
            Wire.WriteString(buf, value.Symbol);
        }
    }

    /// <summary>SBE body decoder for CancelReject: reads fields at pos.</summary>
    public static class CancelRejectBodyDecoder
    {
        public static (CancelReject Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CancelReject();
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            var reject_reason = CancelRejectRejectReasonHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CancelRejectRejectReason");
            string symbol = Wire.ReadString(buf, ref pos);
            value.ClOrdId = cl_ord_id;
            value.OrigClOrdId = orig_cl_ord_id;
            value.RejectReason = reject_reason;
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CancelReject</summary>
    public static class CancelRejectEncoder
    {
        public static byte[] Encode(string ClOrdId, string OrigClOrdId, CancelRejectRejectReason RejectReason, string Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 5);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, ClOrdId);
            Wire.WriteString(buf, OrigClOrdId);
            buf.Add(CancelRejectRejectReasonHelpers.ToValue(RejectReason));
            Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CancelReject: validates the header and reads the body.</summary>
    public static class CancelRejectDecoder
    {
        public static CancelReject Decode(byte[] buf)
        {
            var value = new CancelReject();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 5) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            var reject_reason = CancelRejectRejectReasonHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid CancelRejectRejectReason");
            string symbol = Wire.ReadString(buf, ref pos);
            value.ClOrdId = cl_ord_id;
            value.OrigClOrdId = orig_cl_ord_id;
            value.RejectReason = reject_reason;
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message MarketDataSnapshot</summary>
    public class MarketDataSnapshot
    {
        public string Symbol;
        public string Exchange;
        public List<PriceLevel> Bids;
        public List<PriceLevel> Asks;
        public long Timestamp;
        public ulong? Sequence;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for MarketDataSnapshot (fields only, no header).</summary>
    public static class MarketDataSnapshotBodyEncoder
    {
        public static void Encode(MarketDataSnapshot value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.Exchange);
            Wire.WriteU32BE(buf, (uint)value.Bids.Count);
            foreach (var item in value.Bids)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU32BE(buf, (uint)value.Asks.Count);
            foreach (var item in value.Asks)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            Wire.WriteU64BE(buf, value.Sequence ?? (ulong)0);
            buf.Add((byte)(value.Sequence.HasValue ? 1 : 0));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for MarketDataSnapshot: reads fields at pos.</summary>
    public static class MarketDataSnapshotBodyDecoder
    {
        public static (MarketDataSnapshot Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarketDataSnapshot();
            string symbol = Wire.ReadString(buf, ref pos);
            string exchange = Wire.ReadString(buf, ref pos);
            uint bidsCount = Wire.ReadU32BE(buf, ref pos);
            var bids = new List<PriceLevel>((int)bidsCount);
            for (int i = 0; i < (int)bidsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                bids.Add(item);
            }
            uint asksCount = Wire.ReadU32BE(buf, ref pos);
            var asks = new List<PriceLevel>((int)asksCount);
            for (int i = 0; i < (int)asksCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                asks.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Exchange = exchange;
            value.Bids = bids;
            value.Asks = asks;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MarketDataSnapshot</summary>
    public static class MarketDataSnapshotEncoder
    {
        public static byte[] Encode(string Symbol, string Exchange, List<PriceLevel> Bids, List<PriceLevel> Asks, long Timestamp, ulong? Sequence, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 6);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 17);
            Wire.WriteString(buf, Symbol);
            Wire.WriteString(buf, Exchange);
            Wire.WriteU32BE(buf, (uint)Bids.Count);
            foreach (var item in Bids)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU32BE(buf, (uint)Asks.Count);
            foreach (var item in Asks)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            Wire.WriteU64BE(buf, Sequence ?? (ulong)0);
            buf.Add((byte)(Sequence.HasValue ? 1 : 0));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarketDataSnapshot: validates the header and reads the body.</summary>
    public static class MarketDataSnapshotDecoder
    {
        public static MarketDataSnapshot Decode(byte[] buf)
        {
            var value = new MarketDataSnapshot();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 6) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            string exchange = Wire.ReadString(buf, ref pos);
            uint bidsCount = Wire.ReadU32BE(buf, ref pos);
            var bids = new List<PriceLevel>((int)bidsCount);
            for (int i = 0; i < (int)bidsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                bids.Add(item);
            }
            uint asksCount = Wire.ReadU32BE(buf, ref pos);
            var asks = new List<PriceLevel>((int)asksCount);
            for (int i = 0; i < (int)asksCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                asks.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Exchange = exchange;
            value.Bids = bids;
            value.Asks = asks;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message MarketDataIncrementalRefresh</summary>
    public class MarketDataIncrementalRefresh
    {
        public string Symbol;
        public List<MarketDataUpdate> Updates;
        public long Timestamp;
        public ulong? Sequence;
    }

    /// <summary>SBE body encoder for MarketDataIncrementalRefresh (fields only, no header).</summary>
    public static class MarketDataIncrementalRefreshBodyEncoder
    {
        public static void Encode(MarketDataIncrementalRefresh value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU32BE(buf, (uint)value.Updates.Count);
            foreach (var item in value.Updates)
            {
                var itemBuf = new List<byte>();
                MarketDataUpdateEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            Wire.WriteU64BE(buf, value.Sequence ?? (ulong)0);
            buf.Add((byte)(value.Sequence.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for MarketDataIncrementalRefresh: reads fields at pos.</summary>
    public static class MarketDataIncrementalRefreshBodyDecoder
    {
        public static (MarketDataIncrementalRefresh Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarketDataIncrementalRefresh();
            string symbol = Wire.ReadString(buf, ref pos);
            uint updatesCount = Wire.ReadU32BE(buf, ref pos);
            var updates = new List<MarketDataUpdate>((int)updatesCount);
            for (int i = 0; i < (int)updatesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = MarketDataUpdateDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                updates.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            value.Symbol = symbol;
            value.Updates = updates;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MarketDataIncrementalRefresh</summary>
    public static class MarketDataIncrementalRefreshEncoder
    {
        public static byte[] Encode(string Symbol, List<MarketDataUpdate> Updates, long Timestamp, ulong? Sequence)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 7);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 16);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU32BE(buf, (uint)Updates.Count);
            foreach (var item in Updates)
            {
                var itemBuf = new List<byte>();
                MarketDataUpdateEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            Wire.WriteU64BE(buf, Sequence ?? (ulong)0);
            buf.Add((byte)(Sequence.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarketDataIncrementalRefresh: validates the header and reads the body.</summary>
    public static class MarketDataIncrementalRefreshDecoder
    {
        public static MarketDataIncrementalRefresh Decode(byte[] buf)
        {
            var value = new MarketDataIncrementalRefresh();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 7) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            uint updatesCount = Wire.ReadU32BE(buf, ref pos);
            var updates = new List<MarketDataUpdate>((int)updatesCount);
            for (int i = 0; i < (int)updatesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = MarketDataUpdateDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                updates.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            value.Symbol = symbol;
            value.Updates = updates;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            return value;
        }
    }

    /// <summary>SBE message OrderBookRequest</summary>
    public class OrderBookRequest
    {
        public string Symbol;
        public uint? Depth;
        public long? AtTime;
    }

    /// <summary>SBE body encoder for OrderBookRequest (fields only, no header).</summary>
    public static class OrderBookRequestBodyEncoder
    {
        public static void Encode(OrderBookRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU32BE(buf, value.Depth ?? (uint)0);
            buf.Add((byte)(value.Depth.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.AtTime ?? (long)0));
            buf.Add((byte)(value.AtTime.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for OrderBookRequest: reads fields at pos.</summary>
    public static class OrderBookRequestBodyDecoder
    {
        public static (OrderBookRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OrderBookRequest();
            string symbol = Wire.ReadString(buf, ref pos);
            uint depth_v = Wire.ReadU32BE(buf, ref pos);
            uint? depth = buf[pos++] == 1 ? depth_v : null;
            long at_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? at_time = buf[pos++] == 1 ? at_time_v : null;
            value.Symbol = symbol;
            value.Depth = depth;
            value.AtTime = at_time;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OrderBookRequest</summary>
    public static class OrderBookRequestEncoder
    {
        public static byte[] Encode(string Symbol, uint? Depth, long? AtTime)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 8);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 12);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU32BE(buf, Depth ?? (uint)0);
            buf.Add((byte)(Depth.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(AtTime ?? (long)0));
            buf.Add((byte)(AtTime.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderBookRequest: validates the header and reads the body.</summary>
    public static class OrderBookRequestDecoder
    {
        public static OrderBookRequest Decode(byte[] buf)
        {
            var value = new OrderBookRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 8) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            uint depth_v = Wire.ReadU32BE(buf, ref pos);
            uint? depth = buf[pos++] == 1 ? depth_v : null;
            long at_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? at_time = buf[pos++] == 1 ? at_time_v : null;
            value.Symbol = symbol;
            value.Depth = depth;
            value.AtTime = at_time;
            return value;
        }
    }

    /// <summary>SBE message OrderBookSnapshot</summary>
    public class OrderBookSnapshot
    {
        public string Symbol;
        public string Exchange;
        public List<PriceLevel> Bids;
        public List<PriceLevel> Asks;
        public long Timestamp;
        public ulong? Sequence;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for OrderBookSnapshot (fields only, no header).</summary>
    public static class OrderBookSnapshotBodyEncoder
    {
        public static void Encode(OrderBookSnapshot value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.Exchange);
            Wire.WriteU32BE(buf, (uint)value.Bids.Count);
            foreach (var item in value.Bids)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU32BE(buf, (uint)value.Asks.Count);
            foreach (var item in value.Asks)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            Wire.WriteU64BE(buf, value.Sequence ?? (ulong)0);
            buf.Add((byte)(value.Sequence.HasValue ? 1 : 0));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for OrderBookSnapshot: reads fields at pos.</summary>
    public static class OrderBookSnapshotBodyDecoder
    {
        public static (OrderBookSnapshot Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OrderBookSnapshot();
            string symbol = Wire.ReadString(buf, ref pos);
            string exchange = Wire.ReadString(buf, ref pos);
            uint bidsCount = Wire.ReadU32BE(buf, ref pos);
            var bids = new List<PriceLevel>((int)bidsCount);
            for (int i = 0; i < (int)bidsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                bids.Add(item);
            }
            uint asksCount = Wire.ReadU32BE(buf, ref pos);
            var asks = new List<PriceLevel>((int)asksCount);
            for (int i = 0; i < (int)asksCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                asks.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Exchange = exchange;
            value.Bids = bids;
            value.Asks = asks;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OrderBookSnapshot</summary>
    public static class OrderBookSnapshotEncoder
    {
        public static byte[] Encode(string Symbol, string Exchange, List<PriceLevel> Bids, List<PriceLevel> Asks, long Timestamp, ulong? Sequence, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 9);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 17);
            Wire.WriteString(buf, Symbol);
            Wire.WriteString(buf, Exchange);
            Wire.WriteU32BE(buf, (uint)Bids.Count);
            foreach (var item in Bids)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU32BE(buf, (uint)Asks.Count);
            foreach (var item in Asks)
            {
                var itemBuf = new List<byte>();
                PriceLevelEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            Wire.WriteU64BE(buf, Sequence ?? (ulong)0);
            buf.Add((byte)(Sequence.HasValue ? 1 : 0));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderBookSnapshot: validates the header and reads the body.</summary>
    public static class OrderBookSnapshotDecoder
    {
        public static OrderBookSnapshot Decode(byte[] buf)
        {
            var value = new OrderBookSnapshot();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 9) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            string exchange = Wire.ReadString(buf, ref pos);
            uint bidsCount = Wire.ReadU32BE(buf, ref pos);
            var bids = new List<PriceLevel>((int)bidsCount);
            for (int i = 0; i < (int)bidsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                bids.Add(item);
            }
            uint asksCount = Wire.ReadU32BE(buf, ref pos);
            var asks = new List<PriceLevel>((int)asksCount);
            for (int i = 0; i < (int)asksCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PriceLevelDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                asks.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Exchange = exchange;
            value.Bids = bids;
            value.Asks = asks;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message OrderBookDelta</summary>
    public class OrderBookDelta
    {
        public string Symbol;
        public List<MarketDataUpdate> Updates;
        public long Timestamp;
        public ulong? Sequence;
    }

    /// <summary>SBE body encoder for OrderBookDelta (fields only, no header).</summary>
    public static class OrderBookDeltaBodyEncoder
    {
        public static void Encode(OrderBookDelta value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU32BE(buf, (uint)value.Updates.Count);
            foreach (var item in value.Updates)
            {
                var itemBuf = new List<byte>();
                MarketDataUpdateEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            Wire.WriteU64BE(buf, value.Sequence ?? (ulong)0);
            buf.Add((byte)(value.Sequence.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for OrderBookDelta: reads fields at pos.</summary>
    public static class OrderBookDeltaBodyDecoder
    {
        public static (OrderBookDelta Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OrderBookDelta();
            string symbol = Wire.ReadString(buf, ref pos);
            uint updatesCount = Wire.ReadU32BE(buf, ref pos);
            var updates = new List<MarketDataUpdate>((int)updatesCount);
            for (int i = 0; i < (int)updatesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = MarketDataUpdateDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                updates.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            value.Symbol = symbol;
            value.Updates = updates;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OrderBookDelta</summary>
    public static class OrderBookDeltaEncoder
    {
        public static byte[] Encode(string Symbol, List<MarketDataUpdate> Updates, long Timestamp, ulong? Sequence)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 10);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 16);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU32BE(buf, (uint)Updates.Count);
            foreach (var item in Updates)
            {
                var itemBuf = new List<byte>();
                MarketDataUpdateEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            Wire.WriteU64BE(buf, Sequence ?? (ulong)0);
            buf.Add((byte)(Sequence.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderBookDelta: validates the header and reads the body.</summary>
    public static class OrderBookDeltaDecoder
    {
        public static OrderBookDelta Decode(byte[] buf)
        {
            var value = new OrderBookDelta();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 10) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            uint updatesCount = Wire.ReadU32BE(buf, ref pos);
            var updates = new List<MarketDataUpdate>((int)updatesCount);
            for (int i = 0; i < (int)updatesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = MarketDataUpdateDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                updates.Add(item);
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            ulong sequence_v = Wire.ReadU64BE(buf, ref pos);
            ulong? sequence = buf[pos++] == 1 ? sequence_v : null;
            value.Symbol = symbol;
            value.Updates = updates;
            value.Timestamp = timestamp;
            value.Sequence = sequence;
            return value;
        }
    }

    /// <summary>SBE message AggregateTradeEvent</summary>
    public class AggregateTradeEvent
    {
        public AggregateTrade Trade;
    }

    /// <summary>SBE body encoder for AggregateTradeEvent (fields only, no header).</summary>
    public static class AggregateTradeEventBodyEncoder
    {
        public static void Encode(AggregateTradeEvent value, List<byte> buf)
        {
            AggregateTradeEncoder.Encode(value.Trade, buf);
        }
    }

    /// <summary>SBE body decoder for AggregateTradeEvent: reads fields at pos.</summary>
    public static class AggregateTradeEventBodyDecoder
    {
        public static (AggregateTradeEvent Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AggregateTradeEvent();
            var (trade, tradePos) = AggregateTradeDecoder.Decode(buf, pos);
            pos = tradePos;
            value.Trade = trade;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for AggregateTradeEvent</summary>
    public static class AggregateTradeEventEncoder
    {
        public static byte[] Encode(AggregateTrade Trade)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 11);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            AggregateTradeEncoder.Encode(Trade, buf);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AggregateTradeEvent: validates the header and reads the body.</summary>
    public static class AggregateTradeEventDecoder
    {
        public static AggregateTradeEvent Decode(byte[] buf)
        {
            var value = new AggregateTradeEvent();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 11) throw new InvalidOperationException("invalid template_id");
            var (trade, tradePos) = AggregateTradeDecoder.Decode(buf, pos);
            pos = tradePos;
            value.Trade = trade;
            return value;
        }
    }

    /// <summary>SBE message AggregateTradeRequest</summary>
    public class AggregateTradeRequest
    {
        public string Symbol;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for AggregateTradeRequest (fields only, no header).</summary>
    public static class AggregateTradeRequestBodyEncoder
    {
        public static void Encode(AggregateTradeRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for AggregateTradeRequest: reads fields at pos.</summary>
    public static class AggregateTradeRequestBodyDecoder
    {
        public static (AggregateTradeRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AggregateTradeRequest();
            string? cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for AggregateTradeRequest</summary>
    public static class AggregateTradeRequestEncoder
    {
        public static byte[] Encode(string Symbol, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 12);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AggregateTradeRequest: validates the header and reads the body.</summary>
    public static class AggregateTradeRequestDecoder
    {
        public static AggregateTradeRequest Decode(byte[] buf)
        {
            var value = new AggregateTradeRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 12) throw new InvalidOperationException("invalid template_id");
            string? cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message AggregateTradeBatch</summary>
    public class AggregateTradeBatch
    {
        public string Symbol;
        public List<AggregateTrade> Trades;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for AggregateTradeBatch (fields only, no header).</summary>
    public static class AggregateTradeBatchBodyEncoder
    {
        public static void Encode(AggregateTradeBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU32BE(buf, (uint)value.Trades.Count);
            foreach (var item in value.Trades)
            {
                var itemBuf = new List<byte>();
                AggregateTradeEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for AggregateTradeBatch: reads fields at pos.</summary>
    public static class AggregateTradeBatchBodyDecoder
    {
        public static (AggregateTradeBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AggregateTradeBatch();
            string? next_cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            uint tradesCount = Wire.ReadU32BE(buf, ref pos);
            var trades = new List<AggregateTrade>((int)tradesCount);
            for (int i = 0; i < (int)tradesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = AggregateTradeDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                trades.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Trades = trades;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for AggregateTradeBatch</summary>
    public static class AggregateTradeBatchEncoder
    {
        public static byte[] Encode(string Symbol, List<AggregateTrade> Trades, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 13);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU32BE(buf, (uint)Trades.Count);
            foreach (var item in Trades)
            {
                var itemBuf = new List<byte>();
                AggregateTradeEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AggregateTradeBatch: validates the header and reads the body.</summary>
    public static class AggregateTradeBatchDecoder
    {
        public static AggregateTradeBatch Decode(byte[] buf)
        {
            var value = new AggregateTradeBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 13) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            uint tradesCount = Wire.ReadU32BE(buf, ref pos);
            var trades = new List<AggregateTrade>((int)tradesCount);
            for (int i = 0; i < (int)tradesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = AggregateTradeDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                trades.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Trades = trades;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message MiniTicker</summary>
    public class MiniTicker
    {
        public string Symbol;
        public double LastPrice;
        public double Volume;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for MiniTicker (fields only, no header).</summary>
    public static class MiniTickerBodyEncoder
    {
        public static void Encode(MiniTicker value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.LastPrice);
            Wire.WriteF64BE(buf, value.Volume);
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for MiniTicker: reads fields at pos.</summary>
    public static class MiniTickerBodyDecoder
    {
        public static (MiniTicker Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MiniTicker();
            string symbol = Wire.ReadString(buf, ref pos);
            double last_price = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.LastPrice = last_price;
            value.Volume = volume;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MiniTicker</summary>
    public static class MiniTickerEncoder
    {
        public static byte[] Encode(string Symbol, double LastPrice, double Volume, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 14);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 25);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, LastPrice);
            Wire.WriteF64BE(buf, Volume);
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MiniTicker: validates the header and reads the body.</summary>
    public static class MiniTickerDecoder
    {
        public static MiniTicker Decode(byte[] buf)
        {
            var value = new MiniTicker();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 14) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double last_price = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.LastPrice = last_price;
            value.Volume = volume;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message AllMidsRequest</summary>
    public class AllMidsRequest
    {
    }

    /// <summary>SBE body encoder for AllMidsRequest (fields only, no header).</summary>
    public static class AllMidsRequestBodyEncoder
    {
        public static void Encode(AllMidsRequest value, List<byte> buf)
        {
        }
    }

    /// <summary>SBE body decoder for AllMidsRequest: reads fields at pos.</summary>
    public static class AllMidsRequestBodyDecoder
    {
        public static (AllMidsRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AllMidsRequest();
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for AllMidsRequest</summary>
    public static class AllMidsRequestEncoder
    {
        public static byte[] Encode()
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 15);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AllMidsRequest: validates the header and reads the body.</summary>
    public static class AllMidsRequestDecoder
    {
        public static AllMidsRequest Decode(byte[] buf)
        {
            var value = new AllMidsRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 15) throw new InvalidOperationException("invalid template_id");
            return value;
        }
    }

    /// <summary>SBE message AllMidsBatch</summary>
    public class AllMidsBatch
    {
        public List<MiniTicker> Tickers;
    }

    /// <summary>SBE body encoder for AllMidsBatch (fields only, no header).</summary>
    public static class AllMidsBatchBodyEncoder
    {
        public static void Encode(AllMidsBatch value, List<byte> buf)
        {
            Wire.WriteU32BE(buf, (uint)value.Tickers.Count);
            foreach (var item in value.Tickers)
            {
                var itemBuf = new List<byte>();
                MiniTickerBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
        }
    }

    /// <summary>SBE body decoder for AllMidsBatch: reads fields at pos.</summary>
    public static class AllMidsBatchBodyDecoder
    {
        public static (AllMidsBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AllMidsBatch();
            uint tickersCount = Wire.ReadU32BE(buf, ref pos);
            var tickers = new List<MiniTicker>((int)tickersCount);
            for (int i = 0; i < (int)tickersCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = MiniTickerBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                tickers.Add(item);
            }
            value.Tickers = tickers;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for AllMidsBatch</summary>
    public static class AllMidsBatchEncoder
    {
        public static byte[] Encode(List<MiniTicker> Tickers)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 16);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU32BE(buf, (uint)Tickers.Count);
            foreach (var item in Tickers)
            {
                var itemBuf = new List<byte>();
                MiniTickerBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AllMidsBatch: validates the header and reads the body.</summary>
    public static class AllMidsBatchDecoder
    {
        public static AllMidsBatch Decode(byte[] buf)
        {
            var value = new AllMidsBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 16) throw new InvalidOperationException("invalid template_id");
            uint tickersCount = Wire.ReadU32BE(buf, ref pos);
            var tickers = new List<MiniTicker>((int)tickersCount);
            for (int i = 0; i < (int)tickersCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = MiniTickerBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                tickers.Add(item);
            }
            value.Tickers = tickers;
            return value;
        }
    }

    /// <summary>SBE message MarkPriceUpdate</summary>
    public class MarkPriceUpdate
    {
        public string Symbol;
        public double MarkPrice;
        public double? IndexPrice;
        public double? FundingRate;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for MarkPriceUpdate (fields only, no header).</summary>
    public static class MarkPriceUpdateBodyEncoder
    {
        public static void Encode(MarkPriceUpdate value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.MarkPrice);
            Wire.WriteF64BE(buf, value.IndexPrice ?? 0.0);
            buf.Add((byte)(value.IndexPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.FundingRate ?? 0.0);
            buf.Add((byte)(value.FundingRate.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for MarkPriceUpdate: reads fields at pos.</summary>
    public static class MarkPriceUpdateBodyDecoder
    {
        public static (MarkPriceUpdate Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarkPriceUpdate();
            string symbol = Wire.ReadString(buf, ref pos);
            double mark_price = Wire.ReadF64BE(buf, ref pos);
            double index_price_v = Wire.ReadF64BE(buf, ref pos);
            double? index_price = buf[pos++] == 1 ? index_price_v : null;
            double funding_rate_v = Wire.ReadF64BE(buf, ref pos);
            double? funding_rate = buf[pos++] == 1 ? funding_rate_v : null;
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.MarkPrice = mark_price;
            value.IndexPrice = index_price;
            value.FundingRate = funding_rate;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MarkPriceUpdate</summary>
    public static class MarkPriceUpdateEncoder
    {
        public static byte[] Encode(string Symbol, double MarkPrice, double? IndexPrice, double? FundingRate, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 17);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 33);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, MarkPrice);
            Wire.WriteF64BE(buf, IndexPrice ?? 0.0);
            buf.Add((byte)(IndexPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, FundingRate ?? 0.0);
            buf.Add((byte)(FundingRate.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarkPriceUpdate: validates the header and reads the body.</summary>
    public static class MarkPriceUpdateDecoder
    {
        public static MarkPriceUpdate Decode(byte[] buf)
        {
            var value = new MarkPriceUpdate();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 17) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double mark_price = Wire.ReadF64BE(buf, ref pos);
            double index_price_v = Wire.ReadF64BE(buf, ref pos);
            double? index_price = buf[pos++] == 1 ? index_price_v : null;
            double funding_rate_v = Wire.ReadF64BE(buf, ref pos);
            double? funding_rate = buf[pos++] == 1 ? funding_rate_v : null;
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.MarkPrice = mark_price;
            value.IndexPrice = index_price;
            value.FundingRate = funding_rate;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message MarkPriceRequest</summary>
    public class MarkPriceRequest
    {
        public string Symbol;
    }

    /// <summary>SBE body encoder for MarkPriceRequest (fields only, no header).</summary>
    public static class MarkPriceRequestBodyEncoder
    {
        public static void Encode(MarkPriceRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
        }
    }

    /// <summary>SBE body decoder for MarkPriceRequest: reads fields at pos.</summary>
    public static class MarkPriceRequestBodyDecoder
    {
        public static (MarkPriceRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarkPriceRequest();
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MarkPriceRequest</summary>
    public static class MarkPriceRequestEncoder
    {
        public static byte[] Encode(string Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 18);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarkPriceRequest: validates the header and reads the body.</summary>
    public static class MarkPriceRequestDecoder
    {
        public static MarkPriceRequest Decode(byte[] buf)
        {
            var value = new MarkPriceRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 18) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message LiquidationTradeEvent</summary>
    public class LiquidationTradeEvent
    {
        public LiquidationTrade Trade;
    }

    /// <summary>SBE body encoder for LiquidationTradeEvent (fields only, no header).</summary>
    public static class LiquidationTradeEventBodyEncoder
    {
        public static void Encode(LiquidationTradeEvent value, List<byte> buf)
        {
            LiquidationTradeEncoder.Encode(value.Trade, buf);
        }
    }

    /// <summary>SBE body decoder for LiquidationTradeEvent: reads fields at pos.</summary>
    public static class LiquidationTradeEventBodyDecoder
    {
        public static (LiquidationTradeEvent Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new LiquidationTradeEvent();
            var (trade, tradePos) = LiquidationTradeDecoder.Decode(buf, pos);
            pos = tradePos;
            value.Trade = trade;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for LiquidationTradeEvent</summary>
    public static class LiquidationTradeEventEncoder
    {
        public static byte[] Encode(LiquidationTrade Trade)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 19);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            LiquidationTradeEncoder.Encode(Trade, buf);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LiquidationTradeEvent: validates the header and reads the body.</summary>
    public static class LiquidationTradeEventDecoder
    {
        public static LiquidationTradeEvent Decode(byte[] buf)
        {
            var value = new LiquidationTradeEvent();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 19) throw new InvalidOperationException("invalid template_id");
            var (trade, tradePos) = LiquidationTradeDecoder.Decode(buf, pos);
            pos = tradePos;
            value.Trade = trade;
            return value;
        }
    }

    /// <summary>SBE message CandleBarEvent</summary>
    public class CandleBarEvent
    {
        public CandleBar Bar;
    }

    /// <summary>SBE body encoder for CandleBarEvent (fields only, no header).</summary>
    public static class CandleBarEventBodyEncoder
    {
        public static void Encode(CandleBarEvent value, List<byte> buf)
        {
            CandleBarEncoder.Encode(value.Bar, buf);
        }
    }

    /// <summary>SBE body decoder for CandleBarEvent: reads fields at pos.</summary>
    public static class CandleBarEventBodyDecoder
    {
        public static (CandleBarEvent Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CandleBarEvent();
            var (bar, barPos) = CandleBarDecoder.Decode(buf, pos);
            pos = barPos;
            value.Bar = bar;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CandleBarEvent</summary>
    public static class CandleBarEventEncoder
    {
        public static byte[] Encode(CandleBar Bar)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            CandleBarEncoder.Encode(Bar, buf);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CandleBarEvent: validates the header and reads the body.</summary>
    public static class CandleBarEventDecoder
    {
        public static CandleBarEvent Decode(byte[] buf)
        {
            var value = new CandleBarEvent();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 20) throw new InvalidOperationException("invalid template_id");
            var (bar, barPos) = CandleBarDecoder.Decode(buf, pos);
            pos = barPos;
            value.Bar = bar;
            return value;
        }
    }

    /// <summary>SBE message CandleBarRequest</summary>
    public class CandleBarRequest
    {
        public string Symbol;
        public string Interval;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for CandleBarRequest (fields only, no header).</summary>
    public static class CandleBarRequestBodyEncoder
    {
        public static void Encode(CandleBarRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.Interval);
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for CandleBarRequest: reads fields at pos.</summary>
    public static class CandleBarRequestBodyDecoder
    {
        public static (CandleBarRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CandleBarRequest();
            string? cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Interval = interval;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CandleBarRequest</summary>
    public static class CandleBarRequestEncoder
    {
        public static byte[] Encode(string Symbol, string Interval, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 21);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Symbol);
            Wire.WriteString(buf, Interval);
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CandleBarRequest: validates the header and reads the body.</summary>
    public static class CandleBarRequestDecoder
    {
        public static CandleBarRequest Decode(byte[] buf)
        {
            var value = new CandleBarRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 21) throw new InvalidOperationException("invalid template_id");
            string? cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Interval = interval;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message CandleBarBatch</summary>
    public class CandleBarBatch
    {
        public string Symbol;
        public string Interval;
        public List<CandleBar> Bars;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for CandleBarBatch (fields only, no header).</summary>
    public static class CandleBarBatchBodyEncoder
    {
        public static void Encode(CandleBarBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteString(buf, value.Interval);
            Wire.WriteU32BE(buf, (uint)value.Bars.Count);
            foreach (var item in value.Bars)
            {
                var itemBuf = new List<byte>();
                CandleBarEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for CandleBarBatch: reads fields at pos.</summary>
    public static class CandleBarBatchBodyDecoder
    {
        public static (CandleBarBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CandleBarBatch();
            string? next_cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            uint barsCount = Wire.ReadU32BE(buf, ref pos);
            var bars = new List<CandleBar>((int)barsCount);
            for (int i = 0; i < (int)barsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = CandleBarDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                bars.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Interval = interval;
            value.Bars = bars;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CandleBarBatch</summary>
    public static class CandleBarBatchEncoder
    {
        public static byte[] Encode(string Symbol, string Interval, List<CandleBar> Bars, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 22);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Symbol);
            Wire.WriteString(buf, Interval);
            Wire.WriteU32BE(buf, (uint)Bars.Count);
            foreach (var item in Bars)
            {
                var itemBuf = new List<byte>();
                CandleBarEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CandleBarBatch: validates the header and reads the body.</summary>
    public static class CandleBarBatchDecoder
    {
        public static CandleBarBatch Decode(byte[] buf)
        {
            var value = new CandleBarBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 22) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            uint barsCount = Wire.ReadU32BE(buf, ref pos);
            var bars = new List<CandleBar>((int)barsCount);
            for (int i = 0; i < (int)barsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = CandleBarDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                bars.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Interval = interval;
            value.Bars = bars;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message PublicTradeEvent</summary>
    public class PublicTradeEvent
    {
        public PublicTrade Trade;
    }

    /// <summary>SBE body encoder for PublicTradeEvent (fields only, no header).</summary>
    public static class PublicTradeEventBodyEncoder
    {
        public static void Encode(PublicTradeEvent value, List<byte> buf)
        {
            PublicTradeEncoder.Encode(value.Trade, buf);
        }
    }

    /// <summary>SBE body decoder for PublicTradeEvent: reads fields at pos.</summary>
    public static class PublicTradeEventBodyDecoder
    {
        public static (PublicTradeEvent Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PublicTradeEvent();
            var (trade, tradePos) = PublicTradeDecoder.Decode(buf, pos);
            pos = tradePos;
            value.Trade = trade;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PublicTradeEvent</summary>
    public static class PublicTradeEventEncoder
    {
        public static byte[] Encode(PublicTrade Trade)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 23);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            PublicTradeEncoder.Encode(Trade, buf);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PublicTradeEvent: validates the header and reads the body.</summary>
    public static class PublicTradeEventDecoder
    {
        public static PublicTradeEvent Decode(byte[] buf)
        {
            var value = new PublicTradeEvent();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 23) throw new InvalidOperationException("invalid template_id");
            var (trade, tradePos) = PublicTradeDecoder.Decode(buf, pos);
            pos = tradePos;
            value.Trade = trade;
            return value;
        }
    }

    /// <summary>SBE message TradeHistoryRequest</summary>
    public class TradeHistoryRequest
    {
        public string Symbol;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for TradeHistoryRequest (fields only, no header).</summary>
    public static class TradeHistoryRequestBodyEncoder
    {
        public static void Encode(TradeHistoryRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for TradeHistoryRequest: reads fields at pos.</summary>
    public static class TradeHistoryRequestBodyDecoder
    {
        public static (TradeHistoryRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new TradeHistoryRequest();
            string? cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for TradeHistoryRequest</summary>
    public static class TradeHistoryRequestEncoder
    {
        public static byte[] Encode(string Symbol, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 24);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for TradeHistoryRequest: validates the header and reads the body.</summary>
    public static class TradeHistoryRequestDecoder
    {
        public static TradeHistoryRequest Decode(byte[] buf)
        {
            var value = new TradeHistoryRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 24) throw new InvalidOperationException("invalid template_id");
            string? cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message PublicTradeBatch</summary>
    public class PublicTradeBatch
    {
        public string Symbol;
        public List<PublicTrade> Trades;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for PublicTradeBatch (fields only, no header).</summary>
    public static class PublicTradeBatchBodyEncoder
    {
        public static void Encode(PublicTradeBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteU32BE(buf, (uint)value.Trades.Count);
            foreach (var item in value.Trades)
            {
                var itemBuf = new List<byte>();
                PublicTradeEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for PublicTradeBatch: reads fields at pos.</summary>
    public static class PublicTradeBatchBodyDecoder
    {
        public static (PublicTradeBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PublicTradeBatch();
            string? next_cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            uint tradesCount = Wire.ReadU32BE(buf, ref pos);
            var trades = new List<PublicTrade>((int)tradesCount);
            for (int i = 0; i < (int)tradesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PublicTradeDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                trades.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Trades = trades;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PublicTradeBatch</summary>
    public static class PublicTradeBatchEncoder
    {
        public static byte[] Encode(string Symbol, List<PublicTrade> Trades, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 25);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Symbol);
            Wire.WriteU32BE(buf, (uint)Trades.Count);
            foreach (var item in Trades)
            {
                var itemBuf = new List<byte>();
                PublicTradeEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PublicTradeBatch: validates the header and reads the body.</summary>
    public static class PublicTradeBatchDecoder
    {
        public static PublicTradeBatch Decode(byte[] buf)
        {
            var value = new PublicTradeBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 25) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string symbol = Wire.ReadString(buf, ref pos);
            uint tradesCount = Wire.ReadU32BE(buf, ref pos);
            var trades = new List<PublicTrade>((int)tradesCount);
            for (int i = 0; i < (int)tradesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PublicTradeDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                trades.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            value.Trades = trades;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message BestBidOffer</summary>
    public class BestBidOffer
    {
        public string Symbol;
        public double? BidPrice;
        public double? BidQty;
        public double? AskPrice;
        public double? AskQty;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for BestBidOffer (fields only, no header).</summary>
    public static class BestBidOfferBodyEncoder
    {
        public static void Encode(BestBidOffer value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.BidPrice ?? 0.0);
            buf.Add((byte)(value.BidPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.BidQty ?? 0.0);
            buf.Add((byte)(value.BidQty.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.AskPrice ?? 0.0);
            buf.Add((byte)(value.AskPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.AskQty ?? 0.0);
            buf.Add((byte)(value.AskQty.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for BestBidOffer: reads fields at pos.</summary>
    public static class BestBidOfferBodyDecoder
    {
        public static (BestBidOffer Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new BestBidOffer();
            string symbol = Wire.ReadString(buf, ref pos);
            double bid_price_v = Wire.ReadF64BE(buf, ref pos);
            double? bid_price = buf[pos++] == 1 ? bid_price_v : null;
            double bid_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? bid_qty = buf[pos++] == 1 ? bid_qty_v : null;
            double ask_price_v = Wire.ReadF64BE(buf, ref pos);
            double? ask_price = buf[pos++] == 1 ? ask_price_v : null;
            double ask_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? ask_qty = buf[pos++] == 1 ? ask_qty_v : null;
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.BidPrice = bid_price;
            value.BidQty = bid_qty;
            value.AskPrice = ask_price;
            value.AskQty = ask_qty;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for BestBidOffer</summary>
    public static class BestBidOfferEncoder
    {
        public static byte[] Encode(string Symbol, double? BidPrice, double? BidQty, double? AskPrice, double? AskQty, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 26);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 41);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, BidPrice ?? 0.0);
            buf.Add((byte)(BidPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, BidQty ?? 0.0);
            buf.Add((byte)(BidQty.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, AskPrice ?? 0.0);
            buf.Add((byte)(AskPrice.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, AskQty ?? 0.0);
            buf.Add((byte)(AskQty.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for BestBidOffer: validates the header and reads the body.</summary>
    public static class BestBidOfferDecoder
    {
        public static BestBidOffer Decode(byte[] buf)
        {
            var value = new BestBidOffer();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 26) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double bid_price_v = Wire.ReadF64BE(buf, ref pos);
            double? bid_price = buf[pos++] == 1 ? bid_price_v : null;
            double bid_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? bid_qty = buf[pos++] == 1 ? bid_qty_v : null;
            double ask_price_v = Wire.ReadF64BE(buf, ref pos);
            double? ask_price = buf[pos++] == 1 ? ask_price_v : null;
            double ask_qty_v = Wire.ReadF64BE(buf, ref pos);
            double? ask_qty = buf[pos++] == 1 ? ask_qty_v : null;
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.BidPrice = bid_price;
            value.BidQty = bid_qty;
            value.AskPrice = ask_price;
            value.AskQty = ask_qty;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message SymbolTicker</summary>
    public class SymbolTicker
    {
        public string Symbol;
        public double LastPrice;
        public double PriceChange;
        public double PriceChangePct;
        public double Volume;
        public double High;
        public double Low;
        public double Open;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for SymbolTicker (fields only, no header).</summary>
    public static class SymbolTickerBodyEncoder
    {
        public static void Encode(SymbolTicker value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.LastPrice);
            Wire.WriteF64BE(buf, value.PriceChange);
            Wire.WriteF64BE(buf, value.PriceChangePct);
            Wire.WriteF64BE(buf, value.Volume);
            Wire.WriteF64BE(buf, value.High);
            Wire.WriteF64BE(buf, value.Low);
            Wire.WriteF64BE(buf, value.Open);
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for SymbolTicker: reads fields at pos.</summary>
    public static class SymbolTickerBodyDecoder
    {
        public static (SymbolTicker Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new SymbolTicker();
            string symbol = Wire.ReadString(buf, ref pos);
            double last_price = Wire.ReadF64BE(buf, ref pos);
            double price_change = Wire.ReadF64BE(buf, ref pos);
            double price_change_pct = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            double high = Wire.ReadF64BE(buf, ref pos);
            double low = Wire.ReadF64BE(buf, ref pos);
            double open = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.LastPrice = last_price;
            value.PriceChange = price_change;
            value.PriceChangePct = price_change_pct;
            value.Volume = volume;
            value.High = high;
            value.Low = low;
            value.Open = open;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for SymbolTicker</summary>
    public static class SymbolTickerEncoder
    {
        public static byte[] Encode(string Symbol, double LastPrice, double PriceChange, double PriceChangePct, double Volume, double High, double Low, double Open, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 27);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 65);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, LastPrice);
            Wire.WriteF64BE(buf, PriceChange);
            Wire.WriteF64BE(buf, PriceChangePct);
            Wire.WriteF64BE(buf, Volume);
            Wire.WriteF64BE(buf, High);
            Wire.WriteF64BE(buf, Low);
            Wire.WriteF64BE(buf, Open);
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for SymbolTicker: validates the header and reads the body.</summary>
    public static class SymbolTickerDecoder
    {
        public static SymbolTicker Decode(byte[] buf)
        {
            var value = new SymbolTicker();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 27) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double last_price = Wire.ReadF64BE(buf, ref pos);
            double price_change = Wire.ReadF64BE(buf, ref pos);
            double price_change_pct = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            double high = Wire.ReadF64BE(buf, ref pos);
            double low = Wire.ReadF64BE(buf, ref pos);
            double open = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.LastPrice = last_price;
            value.PriceChange = price_change;
            value.PriceChangePct = price_change_pct;
            value.Volume = volume;
            value.High = high;
            value.Low = low;
            value.Open = open;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message TickerRequest</summary>
    public class TickerRequest
    {
        public string Symbol;
    }

    /// <summary>SBE body encoder for TickerRequest (fields only, no header).</summary>
    public static class TickerRequestBodyEncoder
    {
        public static void Encode(TickerRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
        }
    }

    /// <summary>SBE body decoder for TickerRequest: reads fields at pos.</summary>
    public static class TickerRequestBodyDecoder
    {
        public static (TickerRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new TickerRequest();
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for TickerRequest</summary>
    public static class TickerRequestEncoder
    {
        public static byte[] Encode(string Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 28);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for TickerRequest: validates the header and reads the body.</summary>
    public static class TickerRequestDecoder
    {
        public static TickerRequest Decode(byte[] buf)
        {
            var value = new TickerRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 28) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message InstrumentCatalogRequest</summary>
    public class InstrumentCatalogRequest
    {
    }

    /// <summary>SBE body encoder for InstrumentCatalogRequest (fields only, no header).</summary>
    public static class InstrumentCatalogRequestBodyEncoder
    {
        public static void Encode(InstrumentCatalogRequest value, List<byte> buf)
        {
        }
    }

    /// <summary>SBE body decoder for InstrumentCatalogRequest: reads fields at pos.</summary>
    public static class InstrumentCatalogRequestBodyDecoder
    {
        public static (InstrumentCatalogRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new InstrumentCatalogRequest();
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for InstrumentCatalogRequest</summary>
    public static class InstrumentCatalogRequestEncoder
    {
        public static byte[] Encode()
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 29);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for InstrumentCatalogRequest: validates the header and reads the body.</summary>
    public static class InstrumentCatalogRequestDecoder
    {
        public static InstrumentCatalogRequest Decode(byte[] buf)
        {
            var value = new InstrumentCatalogRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 29) throw new InvalidOperationException("invalid template_id");
            return value;
        }
    }

    /// <summary>SBE message InstrumentCatalogResponse</summary>
    public class InstrumentCatalogResponse
    {
        public List<InstrumentMetadata> Instruments;
    }

    /// <summary>SBE body encoder for InstrumentCatalogResponse (fields only, no header).</summary>
    public static class InstrumentCatalogResponseBodyEncoder
    {
        public static void Encode(InstrumentCatalogResponse value, List<byte> buf)
        {
            Wire.WriteU32BE(buf, (uint)value.Instruments.Count);
            foreach (var item in value.Instruments)
            {
                var itemBuf = new List<byte>();
                InstrumentMetadataEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
        }
    }

    /// <summary>SBE body decoder for InstrumentCatalogResponse: reads fields at pos.</summary>
    public static class InstrumentCatalogResponseBodyDecoder
    {
        public static (InstrumentCatalogResponse Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new InstrumentCatalogResponse();
            uint instrumentsCount = Wire.ReadU32BE(buf, ref pos);
            var instruments = new List<InstrumentMetadata>((int)instrumentsCount);
            for (int i = 0; i < (int)instrumentsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = InstrumentMetadataDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                instruments.Add(item);
            }
            value.Instruments = instruments;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for InstrumentCatalogResponse</summary>
    public static class InstrumentCatalogResponseEncoder
    {
        public static byte[] Encode(List<InstrumentMetadata> Instruments)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 30);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU32BE(buf, (uint)Instruments.Count);
            foreach (var item in Instruments)
            {
                var itemBuf = new List<byte>();
                InstrumentMetadataEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for InstrumentCatalogResponse: validates the header and reads the body.</summary>
    public static class InstrumentCatalogResponseDecoder
    {
        public static InstrumentCatalogResponse Decode(byte[] buf)
        {
            var value = new InstrumentCatalogResponse();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 30) throw new InvalidOperationException("invalid template_id");
            uint instrumentsCount = Wire.ReadU32BE(buf, ref pos);
            var instruments = new List<InstrumentMetadata>((int)instrumentsCount);
            for (int i = 0; i < (int)instrumentsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = InstrumentMetadataDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                instruments.Add(item);
            }
            value.Instruments = instruments;
            return value;
        }
    }

    /// <summary>SBE message AccountSummary</summary>
    public class AccountSummary
    {
        public string Account;
        public double Balance;
        public double BuyingPower;
        public string Currency;
    }

    /// <summary>SBE body encoder for AccountSummary (fields only, no header).</summary>
    public static class AccountSummaryBodyEncoder
    {
        public static void Encode(AccountSummary value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteF64BE(buf, value.Balance);
            Wire.WriteF64BE(buf, value.BuyingPower);
            Wire.WriteString(buf, value.Currency);
        }
    }

    /// <summary>SBE body decoder for AccountSummary: reads fields at pos.</summary>
    public static class AccountSummaryBodyDecoder
    {
        public static (AccountSummary Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new AccountSummary();
            string account = Wire.ReadString(buf, ref pos);
            double balance = Wire.ReadF64BE(buf, ref pos);
            double buying_power = Wire.ReadF64BE(buf, ref pos);
            string currency = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Balance = balance;
            value.BuyingPower = buying_power;
            value.Currency = currency;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for AccountSummary</summary>
    public static class AccountSummaryEncoder
    {
        public static byte[] Encode(string Account, double Balance, double BuyingPower, string Currency)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 31);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 16);
            Wire.WriteString(buf, Account);
            Wire.WriteF64BE(buf, Balance);
            Wire.WriteF64BE(buf, BuyingPower);
            Wire.WriteString(buf, Currency);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AccountSummary: validates the header and reads the body.</summary>
    public static class AccountSummaryDecoder
    {
        public static AccountSummary Decode(byte[] buf)
        {
            var value = new AccountSummary();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 31) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            double balance = Wire.ReadF64BE(buf, ref pos);
            double buying_power = Wire.ReadF64BE(buf, ref pos);
            string currency = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Balance = balance;
            value.BuyingPower = buying_power;
            value.Currency = currency;
            return value;
        }
    }

    /// <summary>SBE message MarginSummary</summary>
    public class MarginSummary
    {
        public string Account;
        public double Balance;
        public double BuyingPower;
        public double Equity;
        public double MarginUsed;
        public double Available;
        public string Currency;
    }

    /// <summary>SBE body encoder for MarginSummary (fields only, no header).</summary>
    public static class MarginSummaryBodyEncoder
    {
        public static void Encode(MarginSummary value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteF64BE(buf, value.Balance);
            Wire.WriteF64BE(buf, value.BuyingPower);
            Wire.WriteF64BE(buf, value.Equity);
            Wire.WriteF64BE(buf, value.MarginUsed);
            Wire.WriteF64BE(buf, value.Available);
            Wire.WriteString(buf, value.Currency);
        }
    }

    /// <summary>SBE body decoder for MarginSummary: reads fields at pos.</summary>
    public static class MarginSummaryBodyDecoder
    {
        public static (MarginSummary Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarginSummary();
            string account = Wire.ReadString(buf, ref pos);
            double balance = Wire.ReadF64BE(buf, ref pos);
            double buying_power = Wire.ReadF64BE(buf, ref pos);
            double equity = Wire.ReadF64BE(buf, ref pos);
            double margin_used = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            string currency = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Balance = balance;
            value.BuyingPower = buying_power;
            value.Equity = equity;
            value.MarginUsed = margin_used;
            value.Available = available;
            value.Currency = currency;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MarginSummary</summary>
    public static class MarginSummaryEncoder
    {
        public static byte[] Encode(string Account, double Balance, double BuyingPower, double Equity, double MarginUsed, double Available, string Currency)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 32);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 40);
            Wire.WriteString(buf, Account);
            Wire.WriteF64BE(buf, Balance);
            Wire.WriteF64BE(buf, BuyingPower);
            Wire.WriteF64BE(buf, Equity);
            Wire.WriteF64BE(buf, MarginUsed);
            Wire.WriteF64BE(buf, Available);
            Wire.WriteString(buf, Currency);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarginSummary: validates the header and reads the body.</summary>
    public static class MarginSummaryDecoder
    {
        public static MarginSummary Decode(byte[] buf)
        {
            var value = new MarginSummary();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 32) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            double balance = Wire.ReadF64BE(buf, ref pos);
            double buying_power = Wire.ReadF64BE(buf, ref pos);
            double equity = Wire.ReadF64BE(buf, ref pos);
            double margin_used = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            string currency = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Balance = balance;
            value.BuyingPower = buying_power;
            value.Equity = equity;
            value.MarginUsed = margin_used;
            value.Available = available;
            value.Currency = currency;
            return value;
        }
    }

    /// <summary>SBE message BalanceSnapshot</summary>
    public class BalanceSnapshot
    {
        public string Account;
        public List<BalanceEntry> Balances;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for BalanceSnapshot (fields only, no header).</summary>
    public static class BalanceSnapshotBodyEncoder
    {
        public static void Encode(BalanceSnapshot value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Balances.Count);
            foreach (var item in value.Balances)
            {
                var itemBuf = new List<byte>();
                BalanceEntryEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for BalanceSnapshot: reads fields at pos.</summary>
    public static class BalanceSnapshotBodyDecoder
    {
        public static (BalanceSnapshot Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new BalanceSnapshot();
            string account = Wire.ReadString(buf, ref pos);
            uint balancesCount = Wire.ReadU32BE(buf, ref pos);
            var balances = new List<BalanceEntry>((int)balancesCount);
            for (int i = 0; i < (int)balancesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = BalanceEntryDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                balances.Add(item);
            }
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Balances = balances;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for BalanceSnapshot</summary>
    public static class BalanceSnapshotEncoder
    {
        public static byte[] Encode(string Account, List<BalanceEntry> Balances, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 33);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Balances.Count);
            foreach (var item in Balances)
            {
                var itemBuf = new List<byte>();
                BalanceEntryEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for BalanceSnapshot: validates the header and reads the body.</summary>
    public static class BalanceSnapshotDecoder
    {
        public static BalanceSnapshot Decode(byte[] buf)
        {
            var value = new BalanceSnapshot();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 33) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            uint balancesCount = Wire.ReadU32BE(buf, ref pos);
            var balances = new List<BalanceEntry>((int)balancesCount);
            for (int i = 0; i < (int)balancesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = BalanceEntryDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                balances.Add(item);
            }
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Balances = balances;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message BalanceUpdate</summary>
    public class BalanceUpdate
    {
        public string Account;
        public string Asset;
        public double Delta;
        public double Total;
        public double Available;
        public BalanceUpdateReason Reason;
    }

    /// <summary>SBE body encoder for BalanceUpdate (fields only, no header).</summary>
    public static class BalanceUpdateBodyEncoder
    {
        public static void Encode(BalanceUpdate value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteString(buf, value.Asset);
            Wire.WriteF64BE(buf, value.Delta);
            Wire.WriteF64BE(buf, value.Total);
            Wire.WriteF64BE(buf, value.Available);
            buf.Add(BalanceUpdateReasonHelpers.ToValue(value.Reason));
        }
    }

    /// <summary>SBE body decoder for BalanceUpdate: reads fields at pos.</summary>
    public static class BalanceUpdateBodyDecoder
    {
        public static (BalanceUpdate Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new BalanceUpdate();
            string account = Wire.ReadString(buf, ref pos);
            string asset = Wire.ReadString(buf, ref pos);
            double delta = Wire.ReadF64BE(buf, ref pos);
            double total = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            var reason = BalanceUpdateReasonHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid BalanceUpdateReason");
            value.Account = account;
            value.Asset = asset;
            value.Delta = delta;
            value.Total = total;
            value.Available = available;
            value.Reason = reason;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for BalanceUpdate</summary>
    public static class BalanceUpdateEncoder
    {
        public static byte[] Encode(string Account, string Asset, double Delta, double Total, double Available, BalanceUpdateReason Reason)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 34);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 25);
            Wire.WriteString(buf, Account);
            Wire.WriteString(buf, Asset);
            Wire.WriteF64BE(buf, Delta);
            Wire.WriteF64BE(buf, Total);
            Wire.WriteF64BE(buf, Available);
            buf.Add(BalanceUpdateReasonHelpers.ToValue(Reason));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for BalanceUpdate: validates the header and reads the body.</summary>
    public static class BalanceUpdateDecoder
    {
        public static BalanceUpdate Decode(byte[] buf)
        {
            var value = new BalanceUpdate();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 34) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string asset = Wire.ReadString(buf, ref pos);
            double delta = Wire.ReadF64BE(buf, ref pos);
            double total = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            var reason = BalanceUpdateReasonHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid BalanceUpdateReason");
            value.Account = account;
            value.Asset = asset;
            value.Delta = delta;
            value.Total = total;
            value.Available = available;
            value.Reason = reason;
            return value;
        }
    }

    /// <summary>SBE message PositionSnapshot</summary>
    public class PositionSnapshot
    {
        public string Account;
        public List<PositionEntry> Positions;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for PositionSnapshot (fields only, no header).</summary>
    public static class PositionSnapshotBodyEncoder
    {
        public static void Encode(PositionSnapshot value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Positions.Count);
            foreach (var item in value.Positions)
            {
                var itemBuf = new List<byte>();
                PositionEntryEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for PositionSnapshot: reads fields at pos.</summary>
    public static class PositionSnapshotBodyDecoder
    {
        public static (PositionSnapshot Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PositionSnapshot();
            string account = Wire.ReadString(buf, ref pos);
            uint positionsCount = Wire.ReadU32BE(buf, ref pos);
            var positions = new List<PositionEntry>((int)positionsCount);
            for (int i = 0; i < (int)positionsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PositionEntryDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                positions.Add(item);
            }
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Positions = positions;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PositionSnapshot</summary>
    public static class PositionSnapshotEncoder
    {
        public static byte[] Encode(string Account, List<PositionEntry> Positions, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 35);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Positions.Count);
            foreach (var item in Positions)
            {
                var itemBuf = new List<byte>();
                PositionEntryEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PositionSnapshot: validates the header and reads the body.</summary>
    public static class PositionSnapshotDecoder
    {
        public static PositionSnapshot Decode(byte[] buf)
        {
            var value = new PositionSnapshot();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 35) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            uint positionsCount = Wire.ReadU32BE(buf, ref pos);
            var positions = new List<PositionEntry>((int)positionsCount);
            for (int i = 0; i < (int)positionsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = PositionEntryDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                positions.Add(item);
            }
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Positions = positions;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message PositionUpdate</summary>
    public class PositionUpdate
    {
        public string Account;
        public string Symbol;
        public double Qty;
        public double EntryPrice;
        public double UnrealizedPnl;
    }

    /// <summary>SBE body encoder for PositionUpdate (fields only, no header).</summary>
    public static class PositionUpdateBodyEncoder
    {
        public static void Encode(PositionUpdate value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.Qty);
            Wire.WriteF64BE(buf, value.EntryPrice);
            Wire.WriteF64BE(buf, value.UnrealizedPnl);
        }
    }

    /// <summary>SBE body decoder for PositionUpdate: reads fields at pos.</summary>
    public static class PositionUpdateBodyDecoder
    {
        public static (PositionUpdate Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PositionUpdate();
            string account = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double entry_price = Wire.ReadF64BE(buf, ref pos);
            double unrealized_pnl = Wire.ReadF64BE(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.Qty = qty;
            value.EntryPrice = entry_price;
            value.UnrealizedPnl = unrealized_pnl;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PositionUpdate</summary>
    public static class PositionUpdateEncoder
    {
        public static byte[] Encode(string Account, string Symbol, double Qty, double EntryPrice, double UnrealizedPnl)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 36);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 24);
            Wire.WriteString(buf, Account);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, Qty);
            Wire.WriteF64BE(buf, EntryPrice);
            Wire.WriteF64BE(buf, UnrealizedPnl);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PositionUpdate: validates the header and reads the body.</summary>
    public static class PositionUpdateDecoder
    {
        public static PositionUpdate Decode(byte[] buf)
        {
            var value = new PositionUpdate();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 36) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double entry_price = Wire.ReadF64BE(buf, ref pos);
            double unrealized_pnl = Wire.ReadF64BE(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.Qty = qty;
            value.EntryPrice = entry_price;
            value.UnrealizedPnl = unrealized_pnl;
            return value;
        }
    }

    /// <summary>SBE message MarginUpdate</summary>
    public class MarginUpdate
    {
        public string Account;
        public MarginSummary Summary;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for MarginUpdate (fields only, no header).</summary>
    public static class MarginUpdateBodyEncoder
    {
        public static void Encode(MarginUpdate value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            MarginSummaryBodyEncoder.Encode(value.Summary, buf);
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for MarginUpdate: reads fields at pos.</summary>
    public static class MarginUpdateBodyDecoder
    {
        public static (MarginUpdate Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MarginUpdate();
            string account = Wire.ReadString(buf, ref pos);
            var (summary, summaryPos) = MarginSummaryBodyDecoder.Decode(buf, pos);
            pos = summaryPos;
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Summary = summary;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MarginUpdate</summary>
    public static class MarginUpdateEncoder
    {
        public static byte[] Encode(string Account, MarginSummary Summary, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 37);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            MarginSummaryBodyEncoder.Encode(Summary, buf);
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarginUpdate: validates the header and reads the body.</summary>
    public static class MarginUpdateDecoder
    {
        public static MarginUpdate Decode(byte[] buf)
        {
            var value = new MarginUpdate();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 37) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            var (summary, summaryPos) = MarginSummaryBodyDecoder.Decode(buf, pos);
            pos = summaryPos;
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Summary = summary;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message UserLiquidation</summary>
    public class UserLiquidation
    {
        public string Account;
        public string Symbol;
        public double Qty;
        public double Price;
        public long Timestamp;
    }

    /// <summary>SBE body encoder for UserLiquidation (fields only, no header).</summary>
    public static class UserLiquidationBodyEncoder
    {
        public static void Encode(UserLiquidation value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.Qty);
            Wire.WriteF64BE(buf, value.Price);
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
        }
    }

    /// <summary>SBE body decoder for UserLiquidation: reads fields at pos.</summary>
    public static class UserLiquidationBodyDecoder
    {
        public static (UserLiquidation Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new UserLiquidation();
            string account = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double price = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.Qty = qty;
            value.Price = price;
            value.Timestamp = timestamp;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for UserLiquidation</summary>
    public static class UserLiquidationEncoder
    {
        public static byte[] Encode(string Account, string Symbol, double Qty, double Price, long Timestamp)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 38);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 24);
            Wire.WriteString(buf, Account);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, Qty);
            Wire.WriteF64BE(buf, Price);
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for UserLiquidation: validates the header and reads the body.</summary>
    public static class UserLiquidationDecoder
    {
        public static UserLiquidation Decode(byte[] buf)
        {
            var value = new UserLiquidation();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 38) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double price = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.Qty = qty;
            value.Price = price;
            value.Timestamp = timestamp;
            return value;
        }
    }

    /// <summary>SBE message OrderListStatus</summary>
    public class OrderListStatus
    {
        public string Account;
        public string ListId;
        public OrderListStatusStatus Status;
        public string? Symbol;
    }

    /// <summary>SBE body encoder for OrderListStatus (fields only, no header).</summary>
    public static class OrderListStatusBodyEncoder
    {
        public static void Encode(OrderListStatus value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteString(buf, value.ListId);
            buf.Add(OrderListStatusStatusHelpers.ToValue(value.Status));
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for OrderListStatus: reads fields at pos.</summary>
    public static class OrderListStatusBodyDecoder
    {
        public static (OrderListStatus Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OrderListStatus();
            string? symbol = null;
            string account = Wire.ReadString(buf, ref pos);
            string list_id = Wire.ReadString(buf, ref pos);
            var status = OrderListStatusStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid OrderListStatusStatus");
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.ListId = list_id;
            value.Status = status;
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OrderListStatus</summary>
    public static class OrderListStatusEncoder
    {
        public static byte[] Encode(string Account, string ListId, OrderListStatusStatus Status, string? Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 39);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteString(buf, ListId);
            buf.Add(OrderListStatusStatusHelpers.ToValue(Status));
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderListStatus: validates the header and reads the body.</summary>
    public static class OrderListStatusDecoder
    {
        public static OrderListStatus Decode(byte[] buf)
        {
            var value = new OrderListStatus();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 39) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string account = Wire.ReadString(buf, ref pos);
            string list_id = Wire.ReadString(buf, ref pos);
            var status = OrderListStatusStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid OrderListStatusStatus");
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.ListId = list_id;
            value.Status = status;
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message FillHistoryRequest</summary>
    public class FillHistoryRequest
    {
        public string Account;
        public string? Symbol;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for FillHistoryRequest (fields only, no header).</summary>
    public static class FillHistoryRequestBodyEncoder
    {
        public static void Encode(FillHistoryRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for FillHistoryRequest: reads fields at pos.</summary>
    public static class FillHistoryRequestBodyDecoder
    {
        public static (FillHistoryRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new FillHistoryRequest();
            string? symbol = null;
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for FillHistoryRequest</summary>
    public static class FillHistoryRequestEncoder
    {
        public static byte[] Encode(string Account, string? Symbol, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 40);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Account);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FillHistoryRequest: validates the header and reads the body.</summary>
    public static class FillHistoryRequestDecoder
    {
        public static FillHistoryRequest Decode(byte[] buf)
        {
            var value = new FillHistoryRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 40) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message FillHistoryBatch</summary>
    public class FillHistoryBatch
    {
        public string Account;
        public List<ExecutionReport> Fills;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for FillHistoryBatch (fields only, no header).</summary>
    public static class FillHistoryBatchBodyEncoder
    {
        public static void Encode(FillHistoryBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Fills.Count);
            foreach (var item in value.Fills)
            {
                var itemBuf = new List<byte>();
                ExecutionReportBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for FillHistoryBatch: reads fields at pos.</summary>
    public static class FillHistoryBatchBodyDecoder
    {
        public static (FillHistoryBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new FillHistoryBatch();
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint fillsCount = Wire.ReadU32BE(buf, ref pos);
            var fills = new List<ExecutionReport>((int)fillsCount);
            for (int i = 0; i < (int)fillsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = ExecutionReportBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                fills.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Fills = fills;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for FillHistoryBatch</summary>
    public static class FillHistoryBatchEncoder
    {
        public static byte[] Encode(string Account, List<ExecutionReport> Fills, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 41);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Fills.Count);
            foreach (var item in Fills)
            {
                var itemBuf = new List<byte>();
                ExecutionReportBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FillHistoryBatch: validates the header and reads the body.</summary>
    public static class FillHistoryBatchDecoder
    {
        public static FillHistoryBatch Decode(byte[] buf)
        {
            var value = new FillHistoryBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 41) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint fillsCount = Wire.ReadU32BE(buf, ref pos);
            var fills = new List<ExecutionReport>((int)fillsCount);
            for (int i = 0; i < (int)fillsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = ExecutionReportBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                fills.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Fills = fills;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message FundingPayment</summary>
    public class FundingPayment
    {
        public string Account;
        public string? Symbol;
        public double Amount;
        public double Rate;
        public long Timestamp;
    }

    /// <summary>SBE body encoder for FundingPayment (fields only, no header).</summary>
    public static class FundingPaymentBodyEncoder
    {
        public static void Encode(FundingPayment value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
            Wire.WriteF64BE(buf, value.Amount);
            Wire.WriteF64BE(buf, value.Rate);
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
        }
    }

    /// <summary>SBE body decoder for FundingPayment: reads fields at pos.</summary>
    public static class FundingPaymentBodyDecoder
    {
        public static (FundingPayment Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new FundingPayment();
            string? symbol = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            double amount = Wire.ReadF64BE(buf, ref pos);
            double rate = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.Amount = amount;
            value.Rate = rate;
            value.Timestamp = timestamp;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for FundingPayment</summary>
    public static class FundingPaymentEncoder
    {
        public static byte[] Encode(string Account, string? Symbol, double Amount, double Rate, long Timestamp)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 42);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 24);
            Wire.WriteString(buf, Account);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            Wire.WriteF64BE(buf, Amount);
            Wire.WriteF64BE(buf, Rate);
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FundingPayment: validates the header and reads the body.</summary>
    public static class FundingPaymentDecoder
    {
        public static FundingPayment Decode(byte[] buf)
        {
            var value = new FundingPayment();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 42) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            double amount = Wire.ReadF64BE(buf, ref pos);
            double rate = Wire.ReadF64BE(buf, ref pos);
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.Amount = amount;
            value.Rate = rate;
            value.Timestamp = timestamp;
            return value;
        }
    }

    /// <summary>SBE message FundingHistoryRequest</summary>
    public class FundingHistoryRequest
    {
        public string Account;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for FundingHistoryRequest (fields only, no header).</summary>
    public static class FundingHistoryRequestBodyEncoder
    {
        public static void Encode(FundingHistoryRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for FundingHistoryRequest: reads fields at pos.</summary>
    public static class FundingHistoryRequestBodyDecoder
    {
        public static (FundingHistoryRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new FundingHistoryRequest();
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for FundingHistoryRequest</summary>
    public static class FundingHistoryRequestEncoder
    {
        public static byte[] Encode(string Account, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 43);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Account);
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FundingHistoryRequest: validates the header and reads the body.</summary>
    public static class FundingHistoryRequestDecoder
    {
        public static FundingHistoryRequest Decode(byte[] buf)
        {
            var value = new FundingHistoryRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 43) throw new InvalidOperationException("invalid template_id");
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message FundingHistoryBatch</summary>
    public class FundingHistoryBatch
    {
        public string Account;
        public List<FundingPayment> Payments;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for FundingHistoryBatch (fields only, no header).</summary>
    public static class FundingHistoryBatchBodyEncoder
    {
        public static void Encode(FundingHistoryBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Payments.Count);
            foreach (var item in value.Payments)
            {
                var itemBuf = new List<byte>();
                FundingPaymentBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for FundingHistoryBatch: reads fields at pos.</summary>
    public static class FundingHistoryBatchBodyDecoder
    {
        public static (FundingHistoryBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new FundingHistoryBatch();
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint paymentsCount = Wire.ReadU32BE(buf, ref pos);
            var payments = new List<FundingPayment>((int)paymentsCount);
            for (int i = 0; i < (int)paymentsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = FundingPaymentBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                payments.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Payments = payments;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for FundingHistoryBatch</summary>
    public static class FundingHistoryBatchEncoder
    {
        public static byte[] Encode(string Account, List<FundingPayment> Payments, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 44);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Payments.Count);
            foreach (var item in Payments)
            {
                var itemBuf = new List<byte>();
                FundingPaymentBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FundingHistoryBatch: validates the header and reads the body.</summary>
    public static class FundingHistoryBatchDecoder
    {
        public static FundingHistoryBatch Decode(byte[] buf)
        {
            var value = new FundingHistoryBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 44) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint paymentsCount = Wire.ReadU32BE(buf, ref pos);
            var payments = new List<FundingPayment>((int)paymentsCount);
            for (int i = 0; i < (int)paymentsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = FundingPaymentBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                payments.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Payments = payments;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message LedgerUpdate</summary>
    public class LedgerUpdate
    {
        public string Account;
        public string Asset;
        public double Delta;
        public LedgerUpdateKind Kind;
        public long Timestamp;
        public string? ReferenceId;
    }

    /// <summary>SBE body encoder for LedgerUpdate (fields only, no header).</summary>
    public static class LedgerUpdateBodyEncoder
    {
        public static void Encode(LedgerUpdate value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteString(buf, value.Asset);
            Wire.WriteF64BE(buf, value.Delta);
            buf.Add(LedgerUpdateKindHelpers.ToValue(value.Kind));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            if (value.ReferenceId != null) { buf.Add(1); Wire.WriteString(buf, value.ReferenceId); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for LedgerUpdate: reads fields at pos.</summary>
    public static class LedgerUpdateBodyDecoder
    {
        public static (LedgerUpdate Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new LedgerUpdate();
            string? reference_id = null;
            string account = Wire.ReadString(buf, ref pos);
            string asset = Wire.ReadString(buf, ref pos);
            double delta = Wire.ReadF64BE(buf, ref pos);
            var kind = LedgerUpdateKindHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid LedgerUpdateKind");
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) reference_id = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Asset = asset;
            value.Delta = delta;
            value.Kind = kind;
            value.Timestamp = timestamp;
            value.ReferenceId = reference_id;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for LedgerUpdate</summary>
    public static class LedgerUpdateEncoder
    {
        public static byte[] Encode(string Account, string Asset, double Delta, LedgerUpdateKind Kind, long Timestamp, string? ReferenceId)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 45);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 17);
            Wire.WriteString(buf, Account);
            Wire.WriteString(buf, Asset);
            Wire.WriteF64BE(buf, Delta);
            buf.Add(LedgerUpdateKindHelpers.ToValue(Kind));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            if (ReferenceId != null) { buf.Add(1); Wire.WriteString(buf, ReferenceId); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LedgerUpdate: validates the header and reads the body.</summary>
    public static class LedgerUpdateDecoder
    {
        public static LedgerUpdate Decode(byte[] buf)
        {
            var value = new LedgerUpdate();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 45) throw new InvalidOperationException("invalid template_id");
            string? reference_id = null;
            string account = Wire.ReadString(buf, ref pos);
            string asset = Wire.ReadString(buf, ref pos);
            double delta = Wire.ReadF64BE(buf, ref pos);
            var kind = LedgerUpdateKindHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid LedgerUpdateKind");
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) reference_id = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Asset = asset;
            value.Delta = delta;
            value.Kind = kind;
            value.Timestamp = timestamp;
            value.ReferenceId = reference_id;
            return value;
        }
    }

    /// <summary>SBE message LedgerHistoryRequest</summary>
    public class LedgerHistoryRequest
    {
        public string Account;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for LedgerHistoryRequest (fields only, no header).</summary>
    public static class LedgerHistoryRequestBodyEncoder
    {
        public static void Encode(LedgerHistoryRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for LedgerHistoryRequest: reads fields at pos.</summary>
    public static class LedgerHistoryRequestBodyDecoder
    {
        public static (LedgerHistoryRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new LedgerHistoryRequest();
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for LedgerHistoryRequest</summary>
    public static class LedgerHistoryRequestEncoder
    {
        public static byte[] Encode(string Account, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 46);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Account);
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LedgerHistoryRequest: validates the header and reads the body.</summary>
    public static class LedgerHistoryRequestDecoder
    {
        public static LedgerHistoryRequest Decode(byte[] buf)
        {
            var value = new LedgerHistoryRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 46) throw new InvalidOperationException("invalid template_id");
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message LedgerHistoryBatch</summary>
    public class LedgerHistoryBatch
    {
        public string Account;
        public List<LedgerUpdate> Entries;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for LedgerHistoryBatch (fields only, no header).</summary>
    public static class LedgerHistoryBatchBodyEncoder
    {
        public static void Encode(LedgerHistoryBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Entries.Count);
            foreach (var item in value.Entries)
            {
                var itemBuf = new List<byte>();
                LedgerUpdateBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for LedgerHistoryBatch: reads fields at pos.</summary>
    public static class LedgerHistoryBatchBodyDecoder
    {
        public static (LedgerHistoryBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new LedgerHistoryBatch();
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint entriesCount = Wire.ReadU32BE(buf, ref pos);
            var entries = new List<LedgerUpdate>((int)entriesCount);
            for (int i = 0; i < (int)entriesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = LedgerUpdateBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                entries.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Entries = entries;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for LedgerHistoryBatch</summary>
    public static class LedgerHistoryBatchEncoder
    {
        public static byte[] Encode(string Account, List<LedgerUpdate> Entries, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 47);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Entries.Count);
            foreach (var item in Entries)
            {
                var itemBuf = new List<byte>();
                LedgerUpdateBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LedgerHistoryBatch: validates the header and reads the body.</summary>
    public static class LedgerHistoryBatchDecoder
    {
        public static LedgerHistoryBatch Decode(byte[] buf)
        {
            var value = new LedgerHistoryBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 47) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint entriesCount = Wire.ReadU32BE(buf, ref pos);
            var entries = new List<LedgerUpdate>((int)entriesCount);
            for (int i = 0; i < (int)entriesCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = LedgerUpdateBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                entries.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Entries = entries;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message OpenOrdersRequest</summary>
    public class OpenOrdersRequest
    {
        public string Account;
        public string? Symbol;
    }

    /// <summary>SBE body encoder for OpenOrdersRequest (fields only, no header).</summary>
    public static class OpenOrdersRequestBodyEncoder
    {
        public static void Encode(OpenOrdersRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for OpenOrdersRequest: reads fields at pos.</summary>
    public static class OpenOrdersRequestBodyDecoder
    {
        public static (OpenOrdersRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OpenOrdersRequest();
            string? symbol = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OpenOrdersRequest</summary>
    public static class OpenOrdersRequestEncoder
    {
        public static byte[] Encode(string Account, string? Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 48);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteString(buf, Account);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OpenOrdersRequest: validates the header and reads the body.</summary>
    public static class OpenOrdersRequestDecoder
    {
        public static OpenOrdersRequest Decode(byte[] buf)
        {
            var value = new OpenOrdersRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 48) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message OpenOrdersSnapshot</summary>
    public class OpenOrdersSnapshot
    {
        public string Account;
        public List<ExecutionReport> Orders;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for OpenOrdersSnapshot (fields only, no header).</summary>
    public static class OpenOrdersSnapshotBodyEncoder
    {
        public static void Encode(OpenOrdersSnapshot value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Orders.Count);
            foreach (var item in value.Orders)
            {
                var itemBuf = new List<byte>();
                ExecutionReportBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for OpenOrdersSnapshot: reads fields at pos.</summary>
    public static class OpenOrdersSnapshotBodyDecoder
    {
        public static (OpenOrdersSnapshot Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OpenOrdersSnapshot();
            string account = Wire.ReadString(buf, ref pos);
            uint ordersCount = Wire.ReadU32BE(buf, ref pos);
            var orders = new List<ExecutionReport>((int)ordersCount);
            for (int i = 0; i < (int)ordersCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = ExecutionReportBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                orders.Add(item);
            }
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Orders = orders;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OpenOrdersSnapshot</summary>
    public static class OpenOrdersSnapshotEncoder
    {
        public static byte[] Encode(string Account, List<ExecutionReport> Orders, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 49);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Orders.Count);
            foreach (var item in Orders)
            {
                var itemBuf = new List<byte>();
                ExecutionReportBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OpenOrdersSnapshot: validates the header and reads the body.</summary>
    public static class OpenOrdersSnapshotDecoder
    {
        public static OpenOrdersSnapshot Decode(byte[] buf)
        {
            var value = new OpenOrdersSnapshot();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 49) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            uint ordersCount = Wire.ReadU32BE(buf, ref pos);
            var orders = new List<ExecutionReport>((int)ordersCount);
            for (int i = 0; i < (int)ordersCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = ExecutionReportBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                orders.Add(item);
            }
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Account = account;
            value.Orders = orders;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message OrderHistoryRequest</summary>
    public class OrderHistoryRequest
    {
        public string Account;
        public string? Symbol;
        public long? StartTime;
        public long? EndTime;
        public uint? Limit;
        public string? Cursor;
    }

    /// <summary>SBE body encoder for OrderHistoryRequest (fields only, no header).</summary>
    public static class OrderHistoryRequestBodyEncoder
    {
        public static void Encode(OrderHistoryRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
            Wire.WriteU64BE(buf, (ulong)(value.StartTime ?? (long)0));
            buf.Add((byte)(value.StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.EndTime ?? (long)0));
            buf.Add((byte)(value.EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, value.Limit ?? (uint)0);
            buf.Add((byte)(value.Limit.HasValue ? 1 : 0));
            if (value.Cursor != null) { buf.Add(1); Wire.WriteString(buf, value.Cursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for OrderHistoryRequest: reads fields at pos.</summary>
    public static class OrderHistoryRequestBodyDecoder
    {
        public static (OrderHistoryRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OrderHistoryRequest();
            string? symbol = null;
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OrderHistoryRequest</summary>
    public static class OrderHistoryRequestEncoder
    {
        public static byte[] Encode(string Account, string? Symbol, long? StartTime, long? EndTime, uint? Limit, string? Cursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 50);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 20);
            Wire.WriteString(buf, Account);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            Wire.WriteU64BE(buf, (ulong)(StartTime ?? (long)0));
            buf.Add((byte)(StartTime.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(EndTime ?? (long)0));
            buf.Add((byte)(EndTime.HasValue ? 1 : 0));
            Wire.WriteU32BE(buf, Limit ?? (uint)0);
            buf.Add((byte)(Limit.HasValue ? 1 : 0));
            if (Cursor != null) { buf.Add(1); Wire.WriteString(buf, Cursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderHistoryRequest: validates the header and reads the body.</summary>
    public static class OrderHistoryRequestDecoder
    {
        public static OrderHistoryRequest Decode(byte[] buf)
        {
            var value = new OrderHistoryRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 50) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string? cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            long start_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? start_time = buf[pos++] == 1 ? start_time_v : null;
            long end_time_v = (long)Wire.ReadU64BE(buf, ref pos);
            long? end_time = buf[pos++] == 1 ? end_time_v : null;
            uint limit_v = Wire.ReadU32BE(buf, ref pos);
            uint? limit = buf[pos++] == 1 ? limit_v : null;
            if (Wire.TakeU8(buf, ref pos) == 1) cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Symbol = symbol;
            value.StartTime = start_time;
            value.EndTime = end_time;
            value.Limit = limit;
            value.Cursor = cursor;
            return value;
        }
    }

    /// <summary>SBE message OrderHistoryBatch</summary>
    public class OrderHistoryBatch
    {
        public string Account;
        public List<ExecutionReport> Orders;
        public byte HasMore;
        public string? NextCursor;
    }

    /// <summary>SBE body encoder for OrderHistoryBatch (fields only, no header).</summary>
    public static class OrderHistoryBatchBodyEncoder
    {
        public static void Encode(OrderHistoryBatch value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
            Wire.WriteU32BE(buf, (uint)value.Orders.Count);
            foreach (var item in value.Orders)
            {
                var itemBuf = new List<byte>();
                ExecutionReportBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(value.HasMore);
            if (value.NextCursor != null) { buf.Add(1); Wire.WriteString(buf, value.NextCursor); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for OrderHistoryBatch: reads fields at pos.</summary>
    public static class OrderHistoryBatchBodyDecoder
    {
        public static (OrderHistoryBatch Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new OrderHistoryBatch();
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint ordersCount = Wire.ReadU32BE(buf, ref pos);
            var orders = new List<ExecutionReport>((int)ordersCount);
            for (int i = 0; i < (int)ordersCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = ExecutionReportBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                orders.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Orders = orders;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for OrderHistoryBatch</summary>
    public static class OrderHistoryBatchEncoder
    {
        public static byte[] Encode(string Account, List<ExecutionReport> Orders, byte HasMore, string? NextCursor)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 51);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, Account);
            Wire.WriteU32BE(buf, (uint)Orders.Count);
            foreach (var item in Orders)
            {
                var itemBuf = new List<byte>();
                ExecutionReportBodyEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            buf.Add(HasMore);
            if (NextCursor != null) { buf.Add(1); Wire.WriteString(buf, NextCursor); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderHistoryBatch: validates the header and reads the body.</summary>
    public static class OrderHistoryBatchDecoder
    {
        public static OrderHistoryBatch Decode(byte[] buf)
        {
            var value = new OrderHistoryBatch();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 51) throw new InvalidOperationException("invalid template_id");
            string? next_cursor = null;
            string account = Wire.ReadString(buf, ref pos);
            uint ordersCount = Wire.ReadU32BE(buf, ref pos);
            var orders = new List<ExecutionReport>((int)ordersCount);
            for (int i = 0; i < (int)ordersCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = ExecutionReportBodyDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                orders.Add(item);
            }
            byte has_more = Wire.TakeU8(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) next_cursor = Wire.ReadString(buf, ref pos);
            value.Account = account;
            value.Orders = orders;
            value.HasMore = has_more;
            value.NextCursor = next_cursor;
            return value;
        }
    }

    /// <summary>SBE message CapabilitiesRequest</summary>
    public class CapabilitiesRequest
    {
    }

    /// <summary>SBE body encoder for CapabilitiesRequest (fields only, no header).</summary>
    public static class CapabilitiesRequestBodyEncoder
    {
        public static void Encode(CapabilitiesRequest value, List<byte> buf)
        {
        }
    }

    /// <summary>SBE body decoder for CapabilitiesRequest: reads fields at pos.</summary>
    public static class CapabilitiesRequestBodyDecoder
    {
        public static (CapabilitiesRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CapabilitiesRequest();
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CapabilitiesRequest</summary>
    public static class CapabilitiesRequestEncoder
    {
        public static byte[] Encode()
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 52);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CapabilitiesRequest: validates the header and reads the body.</summary>
    public static class CapabilitiesRequestDecoder
    {
        public static CapabilitiesRequest Decode(byte[] buf)
        {
            var value = new CapabilitiesRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 52) throw new InvalidOperationException("invalid template_id");
            return value;
        }
    }

    /// <summary>SBE message CapabilitiesResponse</summary>
    public class CapabilitiesResponse
    {
        public List<byte> SchemaIds;
        public List<CapabilityPath> Paths;
        public List<string> Symbols;
        public List<string> Intervals;
    }

    /// <summary>SBE body encoder for CapabilitiesResponse (fields only, no header).</summary>
    public static class CapabilitiesResponseBodyEncoder
    {
        public static void Encode(CapabilitiesResponse value, List<byte> buf)
        {
            Wire.WriteU32BE(buf, (uint)value.SchemaIds.Count);
            foreach (var item in value.SchemaIds)
            {
            buf.Add(item);
            }
            Wire.WriteU32BE(buf, (uint)value.Paths.Count);
            foreach (var item in value.Paths)
            {
                var itemBuf = new List<byte>();
                CapabilityPathEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU32BE(buf, (uint)value.Symbols.Count);
            foreach (var item in value.Symbols)
            {
                Wire.WriteStringU32(buf, item);
            }
            Wire.WriteU32BE(buf, (uint)value.Intervals.Count);
            foreach (var item in value.Intervals)
            {
                Wire.WriteStringU32(buf, item);
            }
        }
    }

    /// <summary>SBE body decoder for CapabilitiesResponse: reads fields at pos.</summary>
    public static class CapabilitiesResponseBodyDecoder
    {
        public static (CapabilitiesResponse Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new CapabilitiesResponse();
            uint schema_idsCount = Wire.ReadU32BE(buf, ref pos);
            var schema_ids = new List<byte>((int)schema_idsCount);
            for (int i = 0; i < (int)schema_idsCount; i++)
            {
                schema_ids.Add(Wire.TakeU8(buf, ref pos));
            }
            uint pathsCount = Wire.ReadU32BE(buf, ref pos);
            var paths = new List<CapabilityPath>((int)pathsCount);
            for (int i = 0; i < (int)pathsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = CapabilityPathDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                paths.Add(item);
            }
            uint symbolsCount = Wire.ReadU32BE(buf, ref pos);
            var symbols = new List<string>((int)symbolsCount);
            for (int i = 0; i < (int)symbolsCount; i++)
            {
                symbols.Add(Wire.ReadStringU32(buf, ref pos));
            }
            uint intervalsCount = Wire.ReadU32BE(buf, ref pos);
            var intervals = new List<string>((int)intervalsCount);
            for (int i = 0; i < (int)intervalsCount; i++)
            {
                intervals.Add(Wire.ReadStringU32(buf, ref pos));
            }
            value.SchemaIds = schema_ids;
            value.Paths = paths;
            value.Symbols = symbols;
            value.Intervals = intervals;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for CapabilitiesResponse</summary>
    public static class CapabilitiesResponseEncoder
    {
        public static byte[] Encode(List<byte> SchemaIds, List<CapabilityPath> Paths, List<string> Symbols, List<string> Intervals)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 53);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU32BE(buf, (uint)SchemaIds.Count);
            foreach (var item in SchemaIds)
            {
            buf.Add(item);
            }
            Wire.WriteU32BE(buf, (uint)Paths.Count);
            foreach (var item in Paths)
            {
                var itemBuf = new List<byte>();
                CapabilityPathEncoder.Encode(item, itemBuf);
                Wire.WriteU32BE(buf, (uint)itemBuf.Count);
                buf.AddRange(itemBuf);
            }
            Wire.WriteU32BE(buf, (uint)Symbols.Count);
            foreach (var item in Symbols)
            {
                Wire.WriteStringU32(buf, item);
            }
            Wire.WriteU32BE(buf, (uint)Intervals.Count);
            foreach (var item in Intervals)
            {
                Wire.WriteStringU32(buf, item);
            }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CapabilitiesResponse: validates the header and reads the body.</summary>
    public static class CapabilitiesResponseDecoder
    {
        public static CapabilitiesResponse Decode(byte[] buf)
        {
            var value = new CapabilitiesResponse();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 53) throw new InvalidOperationException("invalid template_id");
            uint schema_idsCount = Wire.ReadU32BE(buf, ref pos);
            var schema_ids = new List<byte>((int)schema_idsCount);
            for (int i = 0; i < (int)schema_idsCount; i++)
            {
                schema_ids.Add(Wire.TakeU8(buf, ref pos));
            }
            uint pathsCount = Wire.ReadU32BE(buf, ref pos);
            var paths = new List<CapabilityPath>((int)pathsCount);
            for (int i = 0; i < (int)pathsCount; i++)
            {
                uint itemLen = Wire.ReadU32BE(buf, ref pos);
                int itemEnd = pos + (int)itemLen;
                var item = CapabilityPathDecoder.Decode(buf, pos).Value;
                pos = itemEnd;
                paths.Add(item);
            }
            uint symbolsCount = Wire.ReadU32BE(buf, ref pos);
            var symbols = new List<string>((int)symbolsCount);
            for (int i = 0; i < (int)symbolsCount; i++)
            {
                symbols.Add(Wire.ReadStringU32(buf, ref pos));
            }
            uint intervalsCount = Wire.ReadU32BE(buf, ref pos);
            var intervals = new List<string>((int)intervalsCount);
            for (int i = 0; i < (int)intervalsCount; i++)
            {
                intervals.Add(Wire.ReadStringU32(buf, ref pos));
            }
            value.SchemaIds = schema_ids;
            value.Paths = paths;
            value.Symbols = symbols;
            value.Intervals = intervals;
            return value;
        }
    }

    /// <summary>SBE message PositionRequest</summary>
    public class PositionRequest
    {
        public string Account;
    }

    /// <summary>SBE body encoder for PositionRequest (fields only, no header).</summary>
    public static class PositionRequestBodyEncoder
    {
        public static void Encode(PositionRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Account);
        }
    }

    /// <summary>SBE body decoder for PositionRequest: reads fields at pos.</summary>
    public static class PositionRequestBodyDecoder
    {
        public static (PositionRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PositionRequest();
            string account = Wire.ReadString(buf, ref pos);
            value.Account = account;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PositionRequest</summary>
    public static class PositionRequestEncoder
    {
        public static byte[] Encode(string Account)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 54);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteString(buf, Account);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PositionRequest: validates the header and reads the body.</summary>
    public static class PositionRequestDecoder
    {
        public static PositionRequest Decode(byte[] buf)
        {
            var value = new PositionRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 54) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            value.Account = account;
            return value;
        }
    }

    /// <summary>SBE message TradingSessionStatusRequest</summary>
    public class TradingSessionStatusRequest
    {
        public string? Symbol;
    }

    /// <summary>SBE body encoder for TradingSessionStatusRequest (fields only, no header).</summary>
    public static class TradingSessionStatusRequestBodyEncoder
    {
        public static void Encode(TradingSessionStatusRequest value, List<byte> buf)
        {
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
        }
    }

    /// <summary>SBE body decoder for TradingSessionStatusRequest: reads fields at pos.</summary>
    public static class TradingSessionStatusRequestBodyDecoder
    {
        public static (TradingSessionStatusRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new TradingSessionStatusRequest();
            string? symbol = null;
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for TradingSessionStatusRequest</summary>
    public static class TradingSessionStatusRequestEncoder
    {
        public static byte[] Encode(string? Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 55);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for TradingSessionStatusRequest: validates the header and reads the body.</summary>
    public static class TradingSessionStatusRequestDecoder
    {
        public static TradingSessionStatusRequest Decode(byte[] buf)
        {
            var value = new TradingSessionStatusRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 55) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message TradingSessionStatus</summary>
    public class TradingSessionStatus
    {
        public string? Symbol;
        public string TradingSessionId;
        public TradingSessionStatusStatus Status;
        public TradingSessionStatusHaltReason? HaltReason;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for TradingSessionStatus (fields only, no header).</summary>
    public static class TradingSessionStatusBodyEncoder
    {
        public static void Encode(TradingSessionStatus value, List<byte> buf)
        {
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
            Wire.WriteString(buf, value.TradingSessionId);
            buf.Add(TradingSessionStatusStatusHelpers.ToValue(value.Status));
            buf.Add(value.HaltReason.HasValue ? TradingSessionStatusHaltReasonHelpers.ToValue(value.HaltReason.Value) : (byte)0);
            buf.Add((byte)(value.HaltReason.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for TradingSessionStatus: reads fields at pos.</summary>
    public static class TradingSessionStatusBodyDecoder
    {
        public static (TradingSessionStatus Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new TradingSessionStatus();
            string? symbol = null;
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            string trading_session_id = Wire.ReadString(buf, ref pos);
            var status = TradingSessionStatusStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid TradingSessionStatusStatus");
            var halt_reasonRaw = Wire.TakeU8(buf, ref pos);
            TradingSessionStatusHaltReason? halt_reason = null;
            if (buf[pos++] == 1) {
                halt_reason = TradingSessionStatusHaltReasonHelpers.FromValue(halt_reasonRaw) ?? throw new InvalidOperationException("invalid TradingSessionStatusHaltReason");
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.TradingSessionId = trading_session_id;
            value.Status = status;
            value.HaltReason = halt_reason;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for TradingSessionStatus</summary>
    public static class TradingSessionStatusEncoder
    {
        public static byte[] Encode(string? Symbol, string TradingSessionId, TradingSessionStatusStatus Status, TradingSessionStatusHaltReason? HaltReason, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 56);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 11);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            Wire.WriteString(buf, TradingSessionId);
            buf.Add(TradingSessionStatusStatusHelpers.ToValue(Status));
            buf.Add(HaltReason.HasValue ? TradingSessionStatusHaltReasonHelpers.ToValue(HaltReason.Value) : (byte)0);
            buf.Add((byte)(HaltReason.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for TradingSessionStatus: validates the header and reads the body.</summary>
    public static class TradingSessionStatusDecoder
    {
        public static TradingSessionStatus Decode(byte[] buf)
        {
            var value = new TradingSessionStatus();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 56) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            string trading_session_id = Wire.ReadString(buf, ref pos);
            var status = TradingSessionStatusStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid TradingSessionStatusStatus");
            var halt_reasonRaw = Wire.TakeU8(buf, ref pos);
            TradingSessionStatusHaltReason? halt_reason = null;
            if (buf[pos++] == 1) {
                halt_reason = TradingSessionStatusHaltReasonHelpers.FromValue(halt_reasonRaw) ?? throw new InvalidOperationException("invalid TradingSessionStatusHaltReason");
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.TradingSessionId = trading_session_id;
            value.Status = status;
            value.HaltReason = halt_reason;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message SecurityStatusRequest</summary>
    public class SecurityStatusRequest
    {
        public string Symbol;
    }

    /// <summary>SBE body encoder for SecurityStatusRequest (fields only, no header).</summary>
    public static class SecurityStatusRequestBodyEncoder
    {
        public static void Encode(SecurityStatusRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
        }
    }

    /// <summary>SBE body decoder for SecurityStatusRequest: reads fields at pos.</summary>
    public static class SecurityStatusRequestBodyDecoder
    {
        public static (SecurityStatusRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new SecurityStatusRequest();
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for SecurityStatusRequest</summary>
    public static class SecurityStatusRequestEncoder
    {
        public static byte[] Encode(string Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 57);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for SecurityStatusRequest: validates the header and reads the body.</summary>
    public static class SecurityStatusRequestDecoder
    {
        public static SecurityStatusRequest Decode(byte[] buf)
        {
            var value = new SecurityStatusRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 57) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message SecurityStatus</summary>
    public class SecurityStatus
    {
        public string Symbol;
        public SecurityStatusStatus Status;
        public SecurityStatusHaltReason? HaltReason;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for SecurityStatus (fields only, no header).</summary>
    public static class SecurityStatusBodyEncoder
    {
        public static void Encode(SecurityStatus value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            buf.Add(SecurityStatusStatusHelpers.ToValue(value.Status));
            buf.Add(value.HaltReason.HasValue ? SecurityStatusHaltReasonHelpers.ToValue(value.HaltReason.Value) : (byte)0);
            buf.Add((byte)(value.HaltReason.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for SecurityStatus: reads fields at pos.</summary>
    public static class SecurityStatusBodyDecoder
    {
        public static (SecurityStatus Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new SecurityStatus();
            string symbol = Wire.ReadString(buf, ref pos);
            var status = SecurityStatusStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid SecurityStatusStatus");
            var halt_reasonRaw = Wire.TakeU8(buf, ref pos);
            SecurityStatusHaltReason? halt_reason = null;
            if (buf[pos++] == 1) {
                halt_reason = SecurityStatusHaltReasonHelpers.FromValue(halt_reasonRaw) ?? throw new InvalidOperationException("invalid SecurityStatusHaltReason");
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Status = status;
            value.HaltReason = halt_reason;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for SecurityStatus</summary>
    public static class SecurityStatusEncoder
    {
        public static byte[] Encode(string Symbol, SecurityStatusStatus Status, SecurityStatusHaltReason? HaltReason, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 58);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 11);
            Wire.WriteString(buf, Symbol);
            buf.Add(SecurityStatusStatusHelpers.ToValue(Status));
            buf.Add(HaltReason.HasValue ? SecurityStatusHaltReasonHelpers.ToValue(HaltReason.Value) : (byte)0);
            buf.Add((byte)(HaltReason.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for SecurityStatus: validates the header and reads the body.</summary>
    public static class SecurityStatusDecoder
    {
        public static SecurityStatus Decode(byte[] buf)
        {
            var value = new SecurityStatus();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 58) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            var status = SecurityStatusStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid SecurityStatusStatus");
            var halt_reasonRaw = Wire.TakeU8(buf, ref pos);
            SecurityStatusHaltReason? halt_reason = null;
            if (buf[pos++] == 1) {
                halt_reason = SecurityStatusHaltReasonHelpers.FromValue(halt_reasonRaw) ?? throw new InvalidOperationException("invalid SecurityStatusHaltReason");
            }
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.Status = status;
            value.HaltReason = halt_reason;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

    /// <summary>SBE message MassCancelRequest</summary>
    public class MassCancelRequest
    {
        public string ClOrdId;
        public string Account;
        public string? Symbol;
        public MassCancelRequestSide? Side;
    }

    /// <summary>SBE body encoder for MassCancelRequest (fields only, no header).</summary>
    public static class MassCancelRequestBodyEncoder
    {
        public static void Encode(MassCancelRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            Wire.WriteString(buf, value.Account);
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
            buf.Add(value.Side.HasValue ? MassCancelRequestSideHelpers.ToValue(value.Side.Value) : (byte)0);
            buf.Add((byte)(value.Side.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for MassCancelRequest: reads fields at pos.</summary>
    public static class MassCancelRequestBodyDecoder
    {
        public static (MassCancelRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MassCancelRequest();
            string? symbol = null;
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            var sideRaw = Wire.TakeU8(buf, ref pos);
            MassCancelRequestSide? side = null;
            if (buf[pos++] == 1) {
                side = MassCancelRequestSideHelpers.FromValue(sideRaw) ?? throw new InvalidOperationException("invalid MassCancelRequestSide");
            }
            value.ClOrdId = cl_ord_id;
            value.Account = account;
            value.Symbol = symbol;
            value.Side = side;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MassCancelRequest</summary>
    public static class MassCancelRequestEncoder
    {
        public static byte[] Encode(string ClOrdId, string Account, string? Symbol, MassCancelRequestSide? Side)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 59);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 1);
            Wire.WriteString(buf, ClOrdId);
            Wire.WriteString(buf, Account);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            buf.Add(Side.HasValue ? MassCancelRequestSideHelpers.ToValue(Side.Value) : (byte)0);
            buf.Add((byte)(Side.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MassCancelRequest: validates the header and reads the body.</summary>
    public static class MassCancelRequestDecoder
    {
        public static MassCancelRequest Decode(byte[] buf)
        {
            var value = new MassCancelRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 59) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            var sideRaw = Wire.TakeU8(buf, ref pos);
            MassCancelRequestSide? side = null;
            if (buf[pos++] == 1) {
                side = MassCancelRequestSideHelpers.FromValue(sideRaw) ?? throw new InvalidOperationException("invalid MassCancelRequestSide");
            }
            value.ClOrdId = cl_ord_id;
            value.Account = account;
            value.Symbol = symbol;
            value.Side = side;
            return value;
        }
    }

    /// <summary>SBE message MassCancelReport</summary>
    public class MassCancelReport
    {
        public string ClOrdId;
        public string Account;
        public string? Symbol;
        public uint TotalAffected;
        public MassCancelReportStatus Status;
        public long Timestamp;
    }

    /// <summary>SBE body encoder for MassCancelReport (fields only, no header).</summary>
    public static class MassCancelReportBodyEncoder
    {
        public static void Encode(MassCancelReport value, List<byte> buf)
        {
            Wire.WriteString(buf, value.ClOrdId);
            Wire.WriteString(buf, value.Account);
            if (value.Symbol != null) { buf.Add(1); Wire.WriteString(buf, value.Symbol); } else { buf.Add(0); }
            Wire.WriteU32BE(buf, value.TotalAffected);
            buf.Add(MassCancelReportStatusHelpers.ToValue(value.Status));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
        }
    }

    /// <summary>SBE body decoder for MassCancelReport: reads fields at pos.</summary>
    public static class MassCancelReportBodyDecoder
    {
        public static (MassCancelReport Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new MassCancelReport();
            string? symbol = null;
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            uint total_affected = Wire.ReadU32BE(buf, ref pos);
            var status = MassCancelReportStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid MassCancelReportStatus");
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.ClOrdId = cl_ord_id;
            value.Account = account;
            value.Symbol = symbol;
            value.TotalAffected = total_affected;
            value.Status = status;
            value.Timestamp = timestamp;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for MassCancelReport</summary>
    public static class MassCancelReportEncoder
    {
        public static byte[] Encode(string ClOrdId, string Account, string? Symbol, uint TotalAffected, MassCancelReportStatus Status, long Timestamp)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 60);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 13);
            Wire.WriteString(buf, ClOrdId);
            Wire.WriteString(buf, Account);
            if (Symbol != null) { buf.Add(1); Wire.WriteString(buf, Symbol); } else { buf.Add(0); }
            Wire.WriteU32BE(buf, TotalAffected);
            buf.Add(MassCancelReportStatusHelpers.ToValue(Status));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MassCancelReport: validates the header and reads the body.</summary>
    public static class MassCancelReportDecoder
    {
        public static MassCancelReport Decode(byte[] buf)
        {
            var value = new MassCancelReport();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 60) throw new InvalidOperationException("invalid template_id");
            string? symbol = null;
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string account = Wire.ReadString(buf, ref pos);
            if (Wire.TakeU8(buf, ref pos) == 1) symbol = Wire.ReadString(buf, ref pos);
            uint total_affected = Wire.ReadU32BE(buf, ref pos);
            var status = MassCancelReportStatusHelpers.FromValue(Wire.TakeU8(buf, ref pos)) ?? throw new InvalidOperationException("invalid MassCancelReportStatus");
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            value.ClOrdId = cl_ord_id;
            value.Account = account;
            value.Symbol = symbol;
            value.TotalAffected = total_affected;
            value.Status = status;
            value.Timestamp = timestamp;
            return value;
        }
    }

    /// <summary>SBE message PriceLimitsRequest</summary>
    public class PriceLimitsRequest
    {
        public string Symbol;
    }

    /// <summary>SBE body encoder for PriceLimitsRequest (fields only, no header).</summary>
    public static class PriceLimitsRequestBodyEncoder
    {
        public static void Encode(PriceLimitsRequest value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
        }
    }

    /// <summary>SBE body decoder for PriceLimitsRequest: reads fields at pos.</summary>
    public static class PriceLimitsRequestBodyDecoder
    {
        public static (PriceLimitsRequest Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PriceLimitsRequest();
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PriceLimitsRequest</summary>
    public static class PriceLimitsRequestEncoder
    {
        public static byte[] Encode(string Symbol)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 61);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PriceLimitsRequest: validates the header and reads the body.</summary>
    public static class PriceLimitsRequestDecoder
    {
        public static PriceLimitsRequest Decode(byte[] buf)
        {
            var value = new PriceLimitsRequest();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 61) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            value.Symbol = symbol;
            return value;
        }
    }

    /// <summary>SBE message PriceLimits</summary>
    public class PriceLimits
    {
        public string Symbol;
        public double? LowerLimit;
        public double? UpperLimit;
        public double? ReferencePrice;
        public long Timestamp;
        public byte? IsSnapshot;
    }

    /// <summary>SBE body encoder for PriceLimits (fields only, no header).</summary>
    public static class PriceLimitsBodyEncoder
    {
        public static void Encode(PriceLimits value, List<byte> buf)
        {
            Wire.WriteString(buf, value.Symbol);
            Wire.WriteF64BE(buf, value.LowerLimit ?? 0.0);
            buf.Add((byte)(value.LowerLimit.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.UpperLimit ?? 0.0);
            buf.Add((byte)(value.UpperLimit.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, value.ReferencePrice ?? 0.0);
            buf.Add((byte)(value.ReferencePrice.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(value.Timestamp));
            buf.Add(value.IsSnapshot ?? (byte)0);
            buf.Add((byte)(value.IsSnapshot.HasValue ? 1 : 0));
        }
    }

    /// <summary>SBE body decoder for PriceLimits: reads fields at pos.</summary>
    public static class PriceLimitsBodyDecoder
    {
        public static (PriceLimits Value, int Pos) Decode(byte[] buf, int pos)
        {
            var value = new PriceLimits();
            string symbol = Wire.ReadString(buf, ref pos);
            double lower_limit_v = Wire.ReadF64BE(buf, ref pos);
            double? lower_limit = buf[pos++] == 1 ? lower_limit_v : null;
            double upper_limit_v = Wire.ReadF64BE(buf, ref pos);
            double? upper_limit = buf[pos++] == 1 ? upper_limit_v : null;
            double reference_price_v = Wire.ReadF64BE(buf, ref pos);
            double? reference_price = buf[pos++] == 1 ? reference_price_v : null;
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.LowerLimit = lower_limit;
            value.UpperLimit = upper_limit;
            value.ReferencePrice = reference_price;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return (value, pos);
        }
    }

    /// <summary>SBE encoder for PriceLimits</summary>
    public static class PriceLimitsEncoder
    {
        public static byte[] Encode(string Symbol, double? LowerLimit, double? UpperLimit, double? ReferencePrice, long Timestamp, byte? IsSnapshot)
        {
            var buf = new List<byte>();
            Wire.WriteU16BE(buf, Schema.SCHEMA_ID);
            Wire.WriteU16BE(buf, 62);
            Wire.WriteU16BE(buf, 0);
            Wire.WriteU16BE(buf, 33);
            Wire.WriteString(buf, Symbol);
            Wire.WriteF64BE(buf, LowerLimit ?? 0.0);
            buf.Add((byte)(LowerLimit.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, UpperLimit ?? 0.0);
            buf.Add((byte)(UpperLimit.HasValue ? 1 : 0));
            Wire.WriteF64BE(buf, ReferencePrice ?? 0.0);
            buf.Add((byte)(ReferencePrice.HasValue ? 1 : 0));
            Wire.WriteU64BE(buf, (ulong)(Timestamp));
            buf.Add(IsSnapshot ?? (byte)0);
            buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PriceLimits: validates the header and reads the body.</summary>
    public static class PriceLimitsDecoder
    {
        public static PriceLimits Decode(byte[] buf)
        {
            var value = new PriceLimits();
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            _ = Wire.ReadU16BE(buf, ref pos);
            if (schemaId != Schema.SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 62) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double lower_limit_v = Wire.ReadF64BE(buf, ref pos);
            double? lower_limit = buf[pos++] == 1 ? lower_limit_v : null;
            double upper_limit_v = Wire.ReadF64BE(buf, ref pos);
            double? upper_limit = buf[pos++] == 1 ? upper_limit_v : null;
            double reference_price_v = Wire.ReadF64BE(buf, ref pos);
            double? reference_price = buf[pos++] == 1 ? reference_price_v : null;
            long timestamp = (long)Wire.ReadU64BE(buf, ref pos);
            byte is_snapshot_v = Wire.TakeU8(buf, ref pos);
            byte? is_snapshot = buf[pos++] == 1 ? is_snapshot_v : null;
            value.Symbol = symbol;
            value.LowerLimit = lower_limit;
            value.UpperLimit = upper_limit;
            value.ReferencePrice = reference_price;
            value.Timestamp = timestamp;
            value.IsSnapshot = is_snapshot;
            return value;
        }
    }

}
