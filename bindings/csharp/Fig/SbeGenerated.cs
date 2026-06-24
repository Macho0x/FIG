// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

using System;
using System.Collections.Generic;
using System.Text;

namespace Fig.Sbe
{
    public const ushort SCHEMA_ID = 0x01;

    internal static class Wire
    {
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

    public static NewOrderSingleSide? NewOrderSingleSideFromValue(byte v) => v switch {
        1 => NewOrderSingleSide.Buy,
        2 => NewOrderSingleSide.Sell,
        3 => NewOrderSingleSide.SellShort,
        4 => NewOrderSingleSide.SellShortExempt,
        _ => null,
    };

    public static byte NewOrderSingleSideToValue(NewOrderSingleSide e) => (byte)e;

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

    public static NewOrderSingleOrderType? NewOrderSingleOrderTypeFromValue(byte v) => v switch {
        1 => NewOrderSingleOrderType.Market,
        2 => NewOrderSingleOrderType.Limit,
        3 => NewOrderSingleOrderType.Stop,
        4 => NewOrderSingleOrderType.StopLimit,
        5 => NewOrderSingleOrderType.MarketOnClose,
        6 => NewOrderSingleOrderType.LimitOnClose,
        7 => NewOrderSingleOrderType.Pegged,
        _ => null,
    };

    public static byte NewOrderSingleOrderTypeToValue(NewOrderSingleOrderType e) => (byte)e;

    public enum NewOrderSingleTimeInForce : byte
    {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
        Gtd = 5,
    }

    public static NewOrderSingleTimeInForce? NewOrderSingleTimeInForceFromValue(byte v) => v switch {
        1 => NewOrderSingleTimeInForce.Day,
        2 => NewOrderSingleTimeInForce.Gtc,
        3 => NewOrderSingleTimeInForce.Ioc,
        4 => NewOrderSingleTimeInForce.Fok,
        5 => NewOrderSingleTimeInForce.Gtd,
        _ => null,
    };

    public static byte NewOrderSingleTimeInForceToValue(NewOrderSingleTimeInForce e) => (byte)e;

    public enum NewOrderSingleIdSource : byte
    {
        Cusip = 1,
        Sedol = 2,
        Isin = 3,
        Ric = 4,
        ExchangeSymbol = 5,
    }

    public static NewOrderSingleIdSource? NewOrderSingleIdSourceFromValue(byte v) => v switch {
        1 => NewOrderSingleIdSource.Cusip,
        2 => NewOrderSingleIdSource.Sedol,
        3 => NewOrderSingleIdSource.Isin,
        4 => NewOrderSingleIdSource.Ric,
        5 => NewOrderSingleIdSource.ExchangeSymbol,
        _ => null,
    };

    public static byte NewOrderSingleIdSourceToValue(NewOrderSingleIdSource e) => (byte)e;

    public enum CancelRequestSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
    }

    public static CancelRequestSide? CancelRequestSideFromValue(byte v) => v switch {
        1 => CancelRequestSide.Buy,
        2 => CancelRequestSide.Sell,
        3 => CancelRequestSide.SellShort,
        _ => null,
    };

    public static byte CancelRequestSideToValue(CancelRequestSide e) => (byte)e;

    public enum CancelReplaceRequestSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
    }

    public static CancelReplaceRequestSide? CancelReplaceRequestSideFromValue(byte v) => v switch {
        1 => CancelReplaceRequestSide.Buy,
        2 => CancelReplaceRequestSide.Sell,
        3 => CancelReplaceRequestSide.SellShort,
        _ => null,
    };

    public static byte CancelReplaceRequestSideToValue(CancelReplaceRequestSide e) => (byte)e;

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

    public static ExecutionReportExecType? ExecutionReportExecTypeFromValue(byte v) => v switch {
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

    public static byte ExecutionReportExecTypeToValue(ExecutionReportExecType e) => (byte)e;

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

    public static ExecutionReportOrdStatus? ExecutionReportOrdStatusFromValue(byte v) => v switch {
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

    public static byte ExecutionReportOrdStatusToValue(ExecutionReportOrdStatus e) => (byte)e;

    public enum ExecutionReportSide : byte
    {
        Buy = 1,
        Sell = 2,
        SellShort = 3,
    }

    public static ExecutionReportSide? ExecutionReportSideFromValue(byte v) => v switch {
        1 => ExecutionReportSide.Buy,
        2 => ExecutionReportSide.Sell,
        3 => ExecutionReportSide.SellShort,
        _ => null,
    };

    public static byte ExecutionReportSideToValue(ExecutionReportSide e) => (byte)e;

    public enum CancelRejectRejectReason : byte
    {
        OrderNotFound = 1,
        AlreadyCanceled = 2,
        AlreadyFilled = 3,
        TooLateToCancel = 4,
    }

    public static CancelRejectRejectReason? CancelRejectRejectReasonFromValue(byte v) => v switch {
        1 => CancelRejectRejectReason.OrderNotFound,
        2 => CancelRejectRejectReason.AlreadyCanceled,
        3 => CancelRejectRejectReason.AlreadyFilled,
        4 => CancelRejectRejectReason.TooLateToCancel,
        _ => null,
    };

    public static byte CancelRejectRejectReasonToValue(CancelRejectRejectReason e) => (byte)e;

    public enum BalanceUpdateReason : byte
    {
        Trade = 1,
        Deposit = 2,
        Withdrawal = 3,
        Transfer = 4,
        Fee = 5,
    }

    public static BalanceUpdateReason? BalanceUpdateReasonFromValue(byte v) => v switch {
        1 => BalanceUpdateReason.Trade,
        2 => BalanceUpdateReason.Deposit,
        3 => BalanceUpdateReason.Withdrawal,
        4 => BalanceUpdateReason.Transfer,
        5 => BalanceUpdateReason.Fee,
        _ => null,
    };

    public static byte BalanceUpdateReasonToValue(BalanceUpdateReason e) => (byte)e;

    public enum OrderListStatusStatus : byte
    {
        Executing = 1,
        AllDone = 2,
        Reject = 3,
    }

    public static OrderListStatusStatus? OrderListStatusStatusFromValue(byte v) => v switch {
        1 => OrderListStatusStatus.Executing,
        2 => OrderListStatusStatus.AllDone,
        3 => OrderListStatusStatus.Reject,
        _ => null,
    };

    public static byte OrderListStatusStatusToValue(OrderListStatusStatus e) => (byte)e;

    public enum LedgerUpdateKind : byte
    {
        Deposit = 1,
        Withdrawal = 2,
        Transfer = 3,
        Fee = 4,
        Funding = 5,
    }

    public static LedgerUpdateKind? LedgerUpdateKindFromValue(byte v) => v switch {
        1 => LedgerUpdateKind.Deposit,
        2 => LedgerUpdateKind.Withdrawal,
        3 => LedgerUpdateKind.Transfer,
        4 => LedgerUpdateKind.Fee,
        5 => LedgerUpdateKind.Funding,
        _ => null,
    };

    public static byte LedgerUpdateKindToValue(LedgerUpdateKind e) => (byte)e;

    public enum AggregateTradeSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static AggregateTradeSide? AggregateTradeSideFromValue(byte v) => v switch {
        1 => AggregateTradeSide.Buy,
        2 => AggregateTradeSide.Sell,
        _ => null,
    };

    public static byte AggregateTradeSideToValue(AggregateTradeSide e) => (byte)e;

    public enum LiquidationTradeSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static LiquidationTradeSide? LiquidationTradeSideFromValue(byte v) => v switch {
        1 => LiquidationTradeSide.Buy,
        2 => LiquidationTradeSide.Sell,
        _ => null,
    };

    public static byte LiquidationTradeSideToValue(LiquidationTradeSide e) => (byte)e;

    public enum MarketDataUpdateSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static MarketDataUpdateSide? MarketDataUpdateSideFromValue(byte v) => v switch {
        1 => MarketDataUpdateSide.Buy,
        2 => MarketDataUpdateSide.Sell,
        _ => null,
    };

    public static byte MarketDataUpdateSideToValue(MarketDataUpdateSide e) => (byte)e;

    public enum MarketDataUpdateAction : byte
    {
        New = 1,
        Change = 2,
        Delete = 3,
    }

    public static MarketDataUpdateAction? MarketDataUpdateActionFromValue(byte v) => v switch {
        1 => MarketDataUpdateAction.New,
        2 => MarketDataUpdateAction.Change,
        3 => MarketDataUpdateAction.Delete,
        _ => null,
    };

    public static byte MarketDataUpdateActionToValue(MarketDataUpdateAction e) => (byte)e;

    public enum PublicTradeSide : byte
    {
        Buy = 1,
        Sell = 2,
    }

    public static PublicTradeSide? PublicTradeSideFromValue(byte v) => v switch {
        1 => PublicTradeSide.Buy,
        2 => PublicTradeSide.Sell,
        _ => null,
    };

    public static byte PublicTradeSideToValue(PublicTradeSide e) => (byte)e;

    public enum CapabilityPathPattern : byte
    {
        PubSub = 1,
        RequestResponse = 2,
        RequestStream = 3,
    }

    public static CapabilityPathPattern? CapabilityPathPatternFromValue(byte v) => v switch {
        1 => CapabilityPathPattern.PubSub,
        2 => CapabilityPathPattern.RequestResponse,
        3 => CapabilityPathPattern.RequestStream,
        _ => null,
    };

    public static byte CapabilityPathPatternToValue(CapabilityPathPattern e) => (byte)e;

    /// <summary>SBE encoder for NewOrderSingle</summary>
    public static class NewOrderSingleEncoder
    {
        public static byte[] Encode(ClOrdId string, Side NewOrderSingleSide, OrderQty double, Price double?, StopPrice double?, Symbol string, OrderType NewOrderSingleOrderType, TimeInForce NewOrderSingleTimeInForce, ExpireTime long?, Account string?, StrategyId string?, SecurityId string?, IdSource NewOrderSingleIdSource?, SecurityExchange string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 1);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 36);

            // Fixed fields
                Wire.WriteString(buf, ClOrdId);
                buf.Add(NewOrderSingleSideToValue(Side));
                Wire.WriteF64BE(buf, OrderQty);
                Wire.WriteF64BE(buf, Price ?? 0.0);
                buf.Add((byte)(Price.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, StopPrice ?? 0.0);
                buf.Add((byte)(StopPrice.HasValue ? 1 : 0));
                Wire.WriteString(buf, Symbol);
                buf.Add(NewOrderSingleOrderTypeToValue(OrderType));
                buf.Add(NewOrderSingleTimeInForceToValue(TimeInForce));
                if (ExpireTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, ExpireTime.Value); } else { buf.Add(0); }
                if (Account.HasValue) { buf.Add(1); Wire.WriteString(buf, Account.Value); } else { buf.Add(0); }
                if (StrategyId.HasValue) { buf.Add(1); Wire.WriteString(buf, StrategyId.Value); } else { buf.Add(0); }
                if (SecurityId.HasValue) { buf.Add(1); Wire.WriteString(buf, SecurityId.Value); } else { buf.Add(0); }
                buf.Add(IdSource.HasValue ? NewOrderSingleIdSourceToValue(IdSource.Value) : (byte)0);
                if (SecurityExchange.HasValue) { buf.Add(1); Wire.WriteString(buf, SecurityExchange.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for NewOrderSingle</summary>
    public class NewOrderSingleDecoder
    {
        public string ClOrdId { get; set; }
        public NewOrderSingleSide Side { get; set; }
        public double OrderQty { get; set; }
        public double? Price { get; set; }
        public double? StopPrice { get; set; }
        public string Symbol { get; set; }
        public NewOrderSingleOrderType OrderType { get; set; }
        public NewOrderSingleTimeInForce TimeInForce { get; set; }
        public long? ExpireTime { get; set; }
        public string? Account { get; set; }
        public string? StrategyId { get; set; }
        public string? SecurityId { get; set; }
        public NewOrderSingleIdSource? IdSource { get; set; }
        public string? SecurityExchange { get; set; }
        public int EncodedLen() => 0;

        public static NewOrderSingleDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 1) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            byte sideRaw = buf[pos++];
            var side = NewOrderSingleSideFromValue(sideRaw) ?? throw new InvalidOperationException("invalid NewOrderSingleSide");
            double order_qty = Wire.ReadF64BE(buf, ref pos);
            double priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? price = buf[pos++] == 1 ? priceRaw : null;
            double stop_priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? stop_price = buf[pos++] == 1 ? stop_priceRaw : null;
            string symbol = Wire.ReadString(buf, ref pos);
            byte order_typeRaw = buf[pos++];
            var order_type = NewOrderSingleOrderTypeFromValue(order_typeRaw) ?? throw new InvalidOperationException("invalid NewOrderSingleOrderType");
            byte time_in_forceRaw = buf[pos++];
            var time_in_force = NewOrderSingleTimeInForceFromValue(time_in_forceRaw) ?? throw new InvalidOperationException("invalid NewOrderSingleTimeInForce");
            long? expire_time;
            if (buf[pos++] == 1) expire_time = Wire.ReadI64BE(buf, ref pos);
            string? account;
            if (buf[pos++] == 1) account = Wire.ReadString(buf, ref pos);
            string? strategy_id;
            if (buf[pos++] == 1) strategy_id = Wire.ReadString(buf, ref pos);
            string? security_id;
            if (buf[pos++] == 1) security_id = Wire.ReadString(buf, ref pos);
            byte id_sourceRaw = buf[pos++];
            var id_source = NewOrderSingleIdSourceFromValue(id_sourceRaw) ?? throw new InvalidOperationException("invalid NewOrderSingleIdSource");
            string? security_exchange;
            if (buf[pos++] == 1) security_exchange = Wire.ReadString(buf, ref pos);
            return new NewOrderSingleDecoder
            {
                ClOrdId = cl_ord_id,
                Side = side,
                OrderQty = order_qty,
                Price = price,
                StopPrice = stop_price,
                Symbol = symbol,
                OrderType = order_type,
                TimeInForce = time_in_force,
                ExpireTime = expire_time,
                Account = account,
                StrategyId = strategy_id,
                SecurityId = security_id,
                IdSource = id_source,
                SecurityExchange = security_exchange,
            };
        }
    }

    /// <summary>SBE encoder for CancelRequest</summary>
    public static class CancelRequestEncoder
    {
        public static byte[] Encode(ClOrdId string, OrigClOrdId string, Symbol string, Side CancelRequestSide, OrderQty double?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 2);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 9);

            // Fixed fields
                Wire.WriteString(buf, ClOrdId);
                Wire.WriteString(buf, OrigClOrdId);
                Wire.WriteString(buf, Symbol);
                buf.Add(CancelRequestSideToValue(Side));
                Wire.WriteF64BE(buf, OrderQty ?? 0.0);
                buf.Add((byte)(OrderQty.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CancelRequest</summary>
    public class CancelRequestDecoder
    {
        public string ClOrdId { get; set; }
        public string OrigClOrdId { get; set; }
        public string Symbol { get; set; }
        public CancelRequestSide Side { get; set; }
        public double? OrderQty { get; set; }
        public int EncodedLen() => 0;

        public static CancelRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 2) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            byte sideRaw = buf[pos++];
            var side = CancelRequestSideFromValue(sideRaw) ?? throw new InvalidOperationException("invalid CancelRequestSide");
            double order_qtyRaw = Wire.ReadF64BE(buf, ref pos);
            double? order_qty = buf[pos++] == 1 ? order_qtyRaw : null;
            return new CancelRequestDecoder
            {
                ClOrdId = cl_ord_id,
                OrigClOrdId = orig_cl_ord_id,
                Symbol = symbol,
                Side = side,
                OrderQty = order_qty,
            };
        }
    }

    /// <summary>SBE encoder for CancelReplaceRequest</summary>
    public static class CancelReplaceRequestEncoder
    {
        public static byte[] Encode(ClOrdId string, OrigClOrdId string, Symbol string, Side CancelReplaceRequestSide, OrderQty double, Price double?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 3);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 17);

            // Fixed fields
                Wire.WriteString(buf, ClOrdId);
                Wire.WriteString(buf, OrigClOrdId);
                Wire.WriteString(buf, Symbol);
                buf.Add(CancelReplaceRequestSideToValue(Side));
                Wire.WriteF64BE(buf, OrderQty);
                Wire.WriteF64BE(buf, Price ?? 0.0);
                buf.Add((byte)(Price.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CancelReplaceRequest</summary>
    public class CancelReplaceRequestDecoder
    {
        public string ClOrdId { get; set; }
        public string OrigClOrdId { get; set; }
        public string Symbol { get; set; }
        public CancelReplaceRequestSide Side { get; set; }
        public double OrderQty { get; set; }
        public double? Price { get; set; }
        public int EncodedLen() => 0;

        public static CancelReplaceRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 3) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            byte sideRaw = buf[pos++];
            var side = CancelReplaceRequestSideFromValue(sideRaw) ?? throw new InvalidOperationException("invalid CancelReplaceRequestSide");
            double order_qty = Wire.ReadF64BE(buf, ref pos);
            double priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? price = buf[pos++] == 1 ? priceRaw : null;
            return new CancelReplaceRequestDecoder
            {
                ClOrdId = cl_ord_id,
                OrigClOrdId = orig_cl_ord_id,
                Symbol = symbol,
                Side = side,
                OrderQty = order_qty,
                Price = price,
            };
        }
    }

    /// <summary>SBE encoder for ExecutionReport</summary>
    public static class ExecutionReportEncoder
    {
        public static byte[] Encode(ClOrdId string, OrderId string, ExecId string, ExecType ExecutionReportExecType, OrdStatus ExecutionReportOrdStatus, Side ExecutionReportSide, LastQty double?, LastPrice double?, LeavesQty double, CumQty double, AvgPrice double, Symbol string, TransactTime long)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 4);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 51);

            // Fixed fields
                Wire.WriteString(buf, ClOrdId);
                Wire.WriteString(buf, OrderId);
                Wire.WriteString(buf, ExecId);
                buf.Add(ExecutionReportExecTypeToValue(ExecType));
                buf.Add(ExecutionReportOrdStatusToValue(OrdStatus));
                buf.Add(ExecutionReportSideToValue(Side));
                Wire.WriteF64BE(buf, LastQty ?? 0.0);
                buf.Add((byte)(LastQty.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, LastPrice ?? 0.0);
                buf.Add((byte)(LastPrice.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, LeavesQty);
                Wire.WriteF64BE(buf, CumQty);
                Wire.WriteF64BE(buf, AvgPrice);
                Wire.WriteString(buf, Symbol);
                Wire.WriteI64BE(buf, TransactTime);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for ExecutionReport</summary>
    public class ExecutionReportDecoder
    {
        public string ClOrdId { get; set; }
        public string OrderId { get; set; }
        public string ExecId { get; set; }
        public ExecutionReportExecType ExecType { get; set; }
        public ExecutionReportOrdStatus OrdStatus { get; set; }
        public ExecutionReportSide Side { get; set; }
        public double? LastQty { get; set; }
        public double? LastPrice { get; set; }
        public double LeavesQty { get; set; }
        public double CumQty { get; set; }
        public double AvgPrice { get; set; }
        public string Symbol { get; set; }
        public long TransactTime { get; set; }
        public int EncodedLen() => 0;

        public static ExecutionReportDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 4) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string order_id = Wire.ReadString(buf, ref pos);
            string exec_id = Wire.ReadString(buf, ref pos);
            byte exec_typeRaw = buf[pos++];
            var exec_type = ExecutionReportExecTypeFromValue(exec_typeRaw) ?? throw new InvalidOperationException("invalid ExecutionReportExecType");
            byte ord_statusRaw = buf[pos++];
            var ord_status = ExecutionReportOrdStatusFromValue(ord_statusRaw) ?? throw new InvalidOperationException("invalid ExecutionReportOrdStatus");
            byte sideRaw = buf[pos++];
            var side = ExecutionReportSideFromValue(sideRaw) ?? throw new InvalidOperationException("invalid ExecutionReportSide");
            double last_qtyRaw = Wire.ReadF64BE(buf, ref pos);
            double? last_qty = buf[pos++] == 1 ? last_qtyRaw : null;
            double last_priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? last_price = buf[pos++] == 1 ? last_priceRaw : null;
            double leaves_qty = Wire.ReadF64BE(buf, ref pos);
            double cum_qty = Wire.ReadF64BE(buf, ref pos);
            double avg_price = Wire.ReadF64BE(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            long transact_time = Wire.ReadI64BE(buf, ref pos);
            return new ExecutionReportDecoder
            {
                ClOrdId = cl_ord_id,
                OrderId = order_id,
                ExecId = exec_id,
                ExecType = exec_type,
                OrdStatus = ord_status,
                Side = side,
                LastQty = last_qty,
                LastPrice = last_price,
                LeavesQty = leaves_qty,
                CumQty = cum_qty,
                AvgPrice = avg_price,
                Symbol = symbol,
                TransactTime = transact_time,
            };
        }
    }

    /// <summary>SBE encoder for CancelReject</summary>
    public static class CancelRejectEncoder
    {
        public static byte[] Encode(ClOrdId string, OrigClOrdId string, RejectReason CancelRejectRejectReason, Symbol string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 5);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, ClOrdId);
                Wire.WriteString(buf, OrigClOrdId);
                buf.Add(CancelRejectRejectReasonToValue(RejectReason));
                Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CancelReject</summary>
    public class CancelRejectDecoder
    {
        public string ClOrdId { get; set; }
        public string OrigClOrdId { get; set; }
        public CancelRejectRejectReason RejectReason { get; set; }
        public string Symbol { get; set; }
        public int EncodedLen() => 0;

        public static CancelRejectDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 5) throw new InvalidOperationException("invalid template_id");
            string cl_ord_id = Wire.ReadString(buf, ref pos);
            string orig_cl_ord_id = Wire.ReadString(buf, ref pos);
            byte reject_reasonRaw = buf[pos++];
            var reject_reason = CancelRejectRejectReasonFromValue(reject_reasonRaw) ?? throw new InvalidOperationException("invalid CancelRejectRejectReason");
            string symbol = Wire.ReadString(buf, ref pos);
            return new CancelRejectDecoder
            {
                ClOrdId = cl_ord_id,
                OrigClOrdId = orig_cl_ord_id,
                RejectReason = reject_reason,
                Symbol = symbol,
            };
        }
    }

    /// <summary>SBE encoder for MarketDataSnapshot</summary>
    public static class MarketDataSnapshotEncoder
    {
        public static byte[] Encode(Symbol string, Exchange string, Bids List<string>, Asks List<string>, Timestamp long, Sequence long?, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 6);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 17);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteString(buf, Exchange);
                Wire.WriteU32BE(buf, (uint)Bids.Count);
                foreach (var item in Bids) {
                        PriceLevelEncoder.Encode(item, buf);
                }
                Wire.WriteU32BE(buf, (uint)Asks.Count);
                foreach (var item in Asks) {
                        PriceLevelEncoder.Encode(item, buf);
                }
                Wire.WriteI64BE(buf, Timestamp);
                if (Sequence.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, Sequence.Value); } else { buf.Add(0); }
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarketDataSnapshot</summary>
    public class MarketDataSnapshotDecoder
    {
        public string Symbol { get; set; }
        public string Exchange { get; set; }
        public List<string> Bids { get; set; }
        public List<string> Asks { get; set; }
        public long Timestamp { get; set; }
        public long? Sequence { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static MarketDataSnapshotDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 6) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            string exchange = Wire.ReadString(buf, ref pos);
            int bidsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var bids = new List<PriceLevel>(bidsCount);
            for (int i = 0; i < bidsCount; i++) {
                var item = PriceLevelDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                bids.Add(item);
            }
            int asksCount = (int)Wire.ReadU32BE(buf, ref pos);
            var asks = new List<PriceLevel>(asksCount);
            for (int i = 0; i < asksCount; i++) {
                var item = PriceLevelDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                asks.Add(item);
            }
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            long? sequence;
            if (buf[pos++] == 1) sequence = Wire.ReadI64BE(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new MarketDataSnapshotDecoder
            {
                Symbol = symbol,
                Exchange = exchange,
                Bids = bids,
                Asks = asks,
                Timestamp = timestamp,
                Sequence = sequence,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for MarketDataIncrementalRefresh</summary>
    public static class MarketDataIncrementalRefreshEncoder
    {
        public static byte[] Encode(Symbol string, Updates List<string>, Timestamp long, Sequence long?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 7);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 16);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteU32BE(buf, (uint)Updates.Count);
                foreach (var item in Updates) {
                        MarketDataUpdateEncoder.Encode(item, buf);
                }
                Wire.WriteI64BE(buf, Timestamp);
                if (Sequence.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, Sequence.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarketDataIncrementalRefresh</summary>
    public class MarketDataIncrementalRefreshDecoder
    {
        public string Symbol { get; set; }
        public List<string> Updates { get; set; }
        public long Timestamp { get; set; }
        public long? Sequence { get; set; }
        public int EncodedLen() => 0;

        public static MarketDataIncrementalRefreshDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 7) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            int updatesCount = (int)Wire.ReadU32BE(buf, ref pos);
            var updates = new List<MarketDataUpdate>(updatesCount);
            for (int i = 0; i < updatesCount; i++) {
                var item = MarketDataUpdateDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                updates.Add(item);
            }
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            long? sequence;
            if (buf[pos++] == 1) sequence = Wire.ReadI64BE(buf, ref pos);
            return new MarketDataIncrementalRefreshDecoder
            {
                Symbol = symbol,
                Updates = updates,
                Timestamp = timestamp,
                Sequence = sequence,
            };
        }
    }

    /// <summary>SBE encoder for OrderBookRequest</summary>
    public static class OrderBookRequestEncoder
    {
        public static byte[] Encode(Symbol string, Depth u32?, AtTime long?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 8);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 12);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
            // TODO: encode depth as u32
                if (AtTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, AtTime.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderBookRequest</summary>
    public class OrderBookRequestDecoder
    {
        public string Symbol { get; set; }
        public u32? Depth { get; set; }
        public long? AtTime { get; set; }
        public int EncodedLen() => 0;

        public static OrderBookRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 8) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
        // TODO: decode depth
            long? at_time;
            if (buf[pos++] == 1) at_time = Wire.ReadI64BE(buf, ref pos);
            return new OrderBookRequestDecoder
            {
                Symbol = symbol,
                Depth = depth,
                AtTime = at_time,
            };
        }
    }

    /// <summary>SBE encoder for OrderBookSnapshot</summary>
    public static class OrderBookSnapshotEncoder
    {
        public static byte[] Encode(Symbol string, Exchange string, Bids List<string>, Asks List<string>, Timestamp long, Sequence long?, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 9);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 17);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteString(buf, Exchange);
                Wire.WriteU32BE(buf, (uint)Bids.Count);
                foreach (var item in Bids) {
                        PriceLevelEncoder.Encode(item, buf);
                }
                Wire.WriteU32BE(buf, (uint)Asks.Count);
                foreach (var item in Asks) {
                        PriceLevelEncoder.Encode(item, buf);
                }
                Wire.WriteI64BE(buf, Timestamp);
                if (Sequence.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, Sequence.Value); } else { buf.Add(0); }
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderBookSnapshot</summary>
    public class OrderBookSnapshotDecoder
    {
        public string Symbol { get; set; }
        public string Exchange { get; set; }
        public List<string> Bids { get; set; }
        public List<string> Asks { get; set; }
        public long Timestamp { get; set; }
        public long? Sequence { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static OrderBookSnapshotDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 9) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            string exchange = Wire.ReadString(buf, ref pos);
            int bidsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var bids = new List<PriceLevel>(bidsCount);
            for (int i = 0; i < bidsCount; i++) {
                var item = PriceLevelDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                bids.Add(item);
            }
            int asksCount = (int)Wire.ReadU32BE(buf, ref pos);
            var asks = new List<PriceLevel>(asksCount);
            for (int i = 0; i < asksCount; i++) {
                var item = PriceLevelDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                asks.Add(item);
            }
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            long? sequence;
            if (buf[pos++] == 1) sequence = Wire.ReadI64BE(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new OrderBookSnapshotDecoder
            {
                Symbol = symbol,
                Exchange = exchange,
                Bids = bids,
                Asks = asks,
                Timestamp = timestamp,
                Sequence = sequence,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for OrderBookDelta</summary>
    public static class OrderBookDeltaEncoder
    {
        public static byte[] Encode(Symbol string, Updates List<string>, Timestamp long, Sequence long?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 10);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 16);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteU32BE(buf, (uint)Updates.Count);
                foreach (var item in Updates) {
                        MarketDataUpdateEncoder.Encode(item, buf);
                }
                Wire.WriteI64BE(buf, Timestamp);
                if (Sequence.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, Sequence.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderBookDelta</summary>
    public class OrderBookDeltaDecoder
    {
        public string Symbol { get; set; }
        public List<string> Updates { get; set; }
        public long Timestamp { get; set; }
        public long? Sequence { get; set; }
        public int EncodedLen() => 0;

        public static OrderBookDeltaDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 10) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            int updatesCount = (int)Wire.ReadU32BE(buf, ref pos);
            var updates = new List<MarketDataUpdate>(updatesCount);
            for (int i = 0; i < updatesCount; i++) {
                var item = MarketDataUpdateDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                updates.Add(item);
            }
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            long? sequence;
            if (buf[pos++] == 1) sequence = Wire.ReadI64BE(buf, ref pos);
            return new OrderBookDeltaDecoder
            {
                Symbol = symbol,
                Updates = updates,
                Timestamp = timestamp,
                Sequence = sequence,
            };
        }
    }

    /// <summary>SBE encoder for AggregateTradeEvent</summary>
    public static class AggregateTradeEventEncoder
    {
        public static byte[] Encode(Trade string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 11);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Trade);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AggregateTradeEvent</summary>
    public class AggregateTradeEventDecoder
    {
        public string Trade { get; set; }
        public int EncodedLen() => 0;

        public static AggregateTradeEventDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 11) throw new InvalidOperationException("invalid template_id");
            string trade = Wire.ReadString(buf, ref pos);
            return new AggregateTradeEventDecoder
            {
                Trade = trade,
            };
        }
    }

    /// <summary>SBE encoder for AggregateTradeRequest</summary>
    public static class AggregateTradeRequestEncoder
    {
        public static byte[] Encode(Symbol string, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 12);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AggregateTradeRequest</summary>
    public class AggregateTradeRequestDecoder
    {
        public string Symbol { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static AggregateTradeRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 12) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new AggregateTradeRequestDecoder
            {
                Symbol = symbol,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for AggregateTradeBatch</summary>
    public static class AggregateTradeBatchEncoder
    {
        public static byte[] Encode(Symbol string, Trades List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 13);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteU32BE(buf, (uint)Trades.Count);
                foreach (var item in Trades) {
                        AggregateTradeEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AggregateTradeBatch</summary>
    public class AggregateTradeBatchDecoder
    {
        public string Symbol { get; set; }
        public List<string> Trades { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static AggregateTradeBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 13) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            int tradesCount = (int)Wire.ReadU32BE(buf, ref pos);
            var trades = new List<AggregateTrade>(tradesCount);
            for (int i = 0; i < tradesCount; i++) {
                var item = AggregateTradeDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                trades.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new AggregateTradeBatchDecoder
            {
                Symbol = symbol,
                Trades = trades,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for MiniTicker</summary>
    public static class MiniTickerEncoder
    {
        public static byte[] Encode(Symbol string, LastPrice double, Volume double, Timestamp long, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 14);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 25);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteF64BE(buf, LastPrice);
                Wire.WriteF64BE(buf, Volume);
                Wire.WriteI64BE(buf, Timestamp);
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MiniTicker</summary>
    public class MiniTickerDecoder
    {
        public string Symbol { get; set; }
        public double LastPrice { get; set; }
        public double Volume { get; set; }
        public long Timestamp { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static MiniTickerDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 14) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double last_price = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new MiniTickerDecoder
            {
                Symbol = symbol,
                LastPrice = last_price,
                Volume = volume,
                Timestamp = timestamp,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for AllMidsRequest</summary>
    public static class AllMidsRequestEncoder
    {
        public static byte[] Encode()
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 15);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AllMidsRequest</summary>
    public class AllMidsRequestDecoder
    {
        public int EncodedLen() => 0;

        public static AllMidsRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 15) throw new InvalidOperationException("invalid template_id");
            return new AllMidsRequestDecoder
            {

            };
        }
    }

    /// <summary>SBE encoder for AllMidsBatch</summary>
    public static class AllMidsBatchEncoder
    {
        public static byte[] Encode(Tickers List<string>)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 16);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteU32BE(buf, (uint)Tickers.Count);
                foreach (var item in Tickers) {
                        MiniTickerEncoder.Encode(item, buf);
                }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AllMidsBatch</summary>
    public class AllMidsBatchDecoder
    {
        public List<string> Tickers { get; set; }
        public int EncodedLen() => 0;

        public static AllMidsBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 16) throw new InvalidOperationException("invalid template_id");
            int tickersCount = (int)Wire.ReadU32BE(buf, ref pos);
            var tickers = new List<MiniTicker>(tickersCount);
            for (int i = 0; i < tickersCount; i++) {
                var item = MiniTickerDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                tickers.Add(item);
            }
            return new AllMidsBatchDecoder
            {
                Tickers = tickers,
            };
        }
    }

    /// <summary>SBE encoder for MarkPriceUpdate</summary>
    public static class MarkPriceUpdateEncoder
    {
        public static byte[] Encode(Symbol string, MarkPrice double, IndexPrice double?, FundingRate double?, Timestamp long, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 17);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 33);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteF64BE(buf, MarkPrice);
                Wire.WriteF64BE(buf, IndexPrice ?? 0.0);
                buf.Add((byte)(IndexPrice.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, FundingRate ?? 0.0);
                buf.Add((byte)(FundingRate.HasValue ? 1 : 0));
                Wire.WriteI64BE(buf, Timestamp);
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarkPriceUpdate</summary>
    public class MarkPriceUpdateDecoder
    {
        public string Symbol { get; set; }
        public double MarkPrice { get; set; }
        public double? IndexPrice { get; set; }
        public double? FundingRate { get; set; }
        public long Timestamp { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static MarkPriceUpdateDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 17) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double mark_price = Wire.ReadF64BE(buf, ref pos);
            double index_priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? index_price = buf[pos++] == 1 ? index_priceRaw : null;
            double funding_rateRaw = Wire.ReadF64BE(buf, ref pos);
            double? funding_rate = buf[pos++] == 1 ? funding_rateRaw : null;
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new MarkPriceUpdateDecoder
            {
                Symbol = symbol,
                MarkPrice = mark_price,
                IndexPrice = index_price,
                FundingRate = funding_rate,
                Timestamp = timestamp,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for MarkPriceRequest</summary>
    public static class MarkPriceRequestEncoder
    {
        public static byte[] Encode(Symbol string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 18);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarkPriceRequest</summary>
    public class MarkPriceRequestDecoder
    {
        public string Symbol { get; set; }
        public int EncodedLen() => 0;

        public static MarkPriceRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 18) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            return new MarkPriceRequestDecoder
            {
                Symbol = symbol,
            };
        }
    }

    /// <summary>SBE encoder for LiquidationTradeEvent</summary>
    public static class LiquidationTradeEventEncoder
    {
        public static byte[] Encode(Trade string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 19);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Trade);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LiquidationTradeEvent</summary>
    public class LiquidationTradeEventDecoder
    {
        public string Trade { get; set; }
        public int EncodedLen() => 0;

        public static LiquidationTradeEventDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 19) throw new InvalidOperationException("invalid template_id");
            string trade = Wire.ReadString(buf, ref pos);
            return new LiquidationTradeEventDecoder
            {
                Trade = trade,
            };
        }
    }

    /// <summary>SBE encoder for CandleBarEvent</summary>
    public static class CandleBarEventEncoder
    {
        public static byte[] Encode(Bar string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 20);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Bar);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CandleBarEvent</summary>
    public class CandleBarEventDecoder
    {
        public string Bar { get; set; }
        public int EncodedLen() => 0;

        public static CandleBarEventDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 20) throw new InvalidOperationException("invalid template_id");
            string bar = Wire.ReadString(buf, ref pos);
            return new CandleBarEventDecoder
            {
                Bar = bar,
            };
        }
    }

    /// <summary>SBE encoder for CandleBarRequest</summary>
    public static class CandleBarRequestEncoder
    {
        public static byte[] Encode(Symbol string, Interval string, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 21);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteString(buf, Interval);
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CandleBarRequest</summary>
    public class CandleBarRequestDecoder
    {
        public string Symbol { get; set; }
        public string Interval { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static CandleBarRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 21) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new CandleBarRequestDecoder
            {
                Symbol = symbol,
                Interval = interval,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for CandleBarBatch</summary>
    public static class CandleBarBatchEncoder
    {
        public static byte[] Encode(Symbol string, Interval string, Bars List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 22);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteString(buf, Interval);
                Wire.WriteU32BE(buf, (uint)Bars.Count);
                foreach (var item in Bars) {
                        CandleBarEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CandleBarBatch</summary>
    public class CandleBarBatchDecoder
    {
        public string Symbol { get; set; }
        public string Interval { get; set; }
        public List<string> Bars { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static CandleBarBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 22) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            string interval = Wire.ReadString(buf, ref pos);
            int barsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var bars = new List<CandleBar>(barsCount);
            for (int i = 0; i < barsCount; i++) {
                var item = CandleBarDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                bars.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new CandleBarBatchDecoder
            {
                Symbol = symbol,
                Interval = interval,
                Bars = bars,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for PublicTradeEvent</summary>
    public static class PublicTradeEventEncoder
    {
        public static byte[] Encode(Trade string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 23);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Trade);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PublicTradeEvent</summary>
    public class PublicTradeEventDecoder
    {
        public string Trade { get; set; }
        public int EncodedLen() => 0;

        public static PublicTradeEventDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 23) throw new InvalidOperationException("invalid template_id");
            string trade = Wire.ReadString(buf, ref pos);
            return new PublicTradeEventDecoder
            {
                Trade = trade,
            };
        }
    }

    /// <summary>SBE encoder for TradeHistoryRequest</summary>
    public static class TradeHistoryRequestEncoder
    {
        public static byte[] Encode(Symbol string, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 24);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for TradeHistoryRequest</summary>
    public class TradeHistoryRequestDecoder
    {
        public string Symbol { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static TradeHistoryRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 24) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new TradeHistoryRequestDecoder
            {
                Symbol = symbol,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for PublicTradeBatch</summary>
    public static class PublicTradeBatchEncoder
    {
        public static byte[] Encode(Symbol string, Trades List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 25);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteU32BE(buf, (uint)Trades.Count);
                foreach (var item in Trades) {
                        PublicTradeEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PublicTradeBatch</summary>
    public class PublicTradeBatchDecoder
    {
        public string Symbol { get; set; }
        public List<string> Trades { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static PublicTradeBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 25) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            int tradesCount = (int)Wire.ReadU32BE(buf, ref pos);
            var trades = new List<PublicTrade>(tradesCount);
            for (int i = 0; i < tradesCount; i++) {
                var item = PublicTradeDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                trades.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new PublicTradeBatchDecoder
            {
                Symbol = symbol,
                Trades = trades,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for BestBidOffer</summary>
    public static class BestBidOfferEncoder
    {
        public static byte[] Encode(Symbol string, BidPrice double?, BidQty double?, AskPrice double?, AskQty double?, Timestamp long, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 26);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 41);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteF64BE(buf, BidPrice ?? 0.0);
                buf.Add((byte)(BidPrice.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, BidQty ?? 0.0);
                buf.Add((byte)(BidQty.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, AskPrice ?? 0.0);
                buf.Add((byte)(AskPrice.HasValue ? 1 : 0));
                Wire.WriteF64BE(buf, AskQty ?? 0.0);
                buf.Add((byte)(AskQty.HasValue ? 1 : 0));
                Wire.WriteI64BE(buf, Timestamp);
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for BestBidOffer</summary>
    public class BestBidOfferDecoder
    {
        public string Symbol { get; set; }
        public double? BidPrice { get; set; }
        public double? BidQty { get; set; }
        public double? AskPrice { get; set; }
        public double? AskQty { get; set; }
        public long Timestamp { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static BestBidOfferDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 26) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double bid_priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? bid_price = buf[pos++] == 1 ? bid_priceRaw : null;
            double bid_qtyRaw = Wire.ReadF64BE(buf, ref pos);
            double? bid_qty = buf[pos++] == 1 ? bid_qtyRaw : null;
            double ask_priceRaw = Wire.ReadF64BE(buf, ref pos);
            double? ask_price = buf[pos++] == 1 ? ask_priceRaw : null;
            double ask_qtyRaw = Wire.ReadF64BE(buf, ref pos);
            double? ask_qty = buf[pos++] == 1 ? ask_qtyRaw : null;
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new BestBidOfferDecoder
            {
                Symbol = symbol,
                BidPrice = bid_price,
                BidQty = bid_qty,
                AskPrice = ask_price,
                AskQty = ask_qty,
                Timestamp = timestamp,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for SymbolTicker</summary>
    public static class SymbolTickerEncoder
    {
        public static byte[] Encode(Symbol string, LastPrice double, PriceChange double, PriceChangePct double, Volume double, High double, Low double, Open double, Timestamp long, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 27);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 65);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
                Wire.WriteF64BE(buf, LastPrice);
                Wire.WriteF64BE(buf, PriceChange);
                Wire.WriteF64BE(buf, PriceChangePct);
                Wire.WriteF64BE(buf, Volume);
                Wire.WriteF64BE(buf, High);
                Wire.WriteF64BE(buf, Low);
                Wire.WriteF64BE(buf, Open);
                Wire.WriteI64BE(buf, Timestamp);
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for SymbolTicker</summary>
    public class SymbolTickerDecoder
    {
        public string Symbol { get; set; }
        public double LastPrice { get; set; }
        public double PriceChange { get; set; }
        public double PriceChangePct { get; set; }
        public double Volume { get; set; }
        public double High { get; set; }
        public double Low { get; set; }
        public double Open { get; set; }
        public long Timestamp { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static SymbolTickerDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 27) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            double last_price = Wire.ReadF64BE(buf, ref pos);
            double price_change = Wire.ReadF64BE(buf, ref pos);
            double price_change_pct = Wire.ReadF64BE(buf, ref pos);
            double volume = Wire.ReadF64BE(buf, ref pos);
            double high = Wire.ReadF64BE(buf, ref pos);
            double low = Wire.ReadF64BE(buf, ref pos);
            double open = Wire.ReadF64BE(buf, ref pos);
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new SymbolTickerDecoder
            {
                Symbol = symbol,
                LastPrice = last_price,
                PriceChange = price_change,
                PriceChangePct = price_change_pct,
                Volume = volume,
                High = high,
                Low = low,
                Open = open,
                Timestamp = timestamp,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for TickerRequest</summary>
    public static class TickerRequestEncoder
    {
        public static byte[] Encode(Symbol string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 28);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Symbol);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for TickerRequest</summary>
    public class TickerRequestDecoder
    {
        public string Symbol { get; set; }
        public int EncodedLen() => 0;

        public static TickerRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 28) throw new InvalidOperationException("invalid template_id");
            string symbol = Wire.ReadString(buf, ref pos);
            return new TickerRequestDecoder
            {
                Symbol = symbol,
            };
        }
    }

    /// <summary>SBE encoder for AccountSummary</summary>
    public static class AccountSummaryEncoder
    {
        public static byte[] Encode(Account string, Balance double, BuyingPower double, Currency string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 29);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 16);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteF64BE(buf, Balance);
                Wire.WriteF64BE(buf, BuyingPower);
                Wire.WriteString(buf, Currency);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for AccountSummary</summary>
    public class AccountSummaryDecoder
    {
        public string Account { get; set; }
        public double Balance { get; set; }
        public double BuyingPower { get; set; }
        public string Currency { get; set; }
        public int EncodedLen() => 0;

        public static AccountSummaryDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 29) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            double balance = Wire.ReadF64BE(buf, ref pos);
            double buying_power = Wire.ReadF64BE(buf, ref pos);
            string currency = Wire.ReadString(buf, ref pos);
            return new AccountSummaryDecoder
            {
                Account = account,
                Balance = balance,
                BuyingPower = buying_power,
                Currency = currency,
            };
        }
    }

    /// <summary>SBE encoder for MarginSummary</summary>
    public static class MarginSummaryEncoder
    {
        public static byte[] Encode(Account string, Balance double, BuyingPower double, Equity double, MarginUsed double, Available double, Currency string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 30);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 40);

            // Fixed fields
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

    /// <summary>SBE decoder for MarginSummary</summary>
    public class MarginSummaryDecoder
    {
        public string Account { get; set; }
        public double Balance { get; set; }
        public double BuyingPower { get; set; }
        public double Equity { get; set; }
        public double MarginUsed { get; set; }
        public double Available { get; set; }
        public string Currency { get; set; }
        public int EncodedLen() => 0;

        public static MarginSummaryDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 30) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            double balance = Wire.ReadF64BE(buf, ref pos);
            double buying_power = Wire.ReadF64BE(buf, ref pos);
            double equity = Wire.ReadF64BE(buf, ref pos);
            double margin_used = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            string currency = Wire.ReadString(buf, ref pos);
            return new MarginSummaryDecoder
            {
                Account = account,
                Balance = balance,
                BuyingPower = buying_power,
                Equity = equity,
                MarginUsed = margin_used,
                Available = available,
                Currency = currency,
            };
        }
    }

    /// <summary>SBE encoder for BalanceSnapshot</summary>
    public static class BalanceSnapshotEncoder
    {
        public static byte[] Encode(Account string, Balances List<string>, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 31);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Balances.Count);
                foreach (var item in Balances) {
                        BalanceEntryEncoder.Encode(item, buf);
                }
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for BalanceSnapshot</summary>
    public class BalanceSnapshotDecoder
    {
        public string Account { get; set; }
        public List<string> Balances { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static BalanceSnapshotDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 31) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int balancesCount = (int)Wire.ReadU32BE(buf, ref pos);
            var balances = new List<BalanceEntry>(balancesCount);
            for (int i = 0; i < balancesCount; i++) {
                var item = BalanceEntryDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                balances.Add(item);
            }
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new BalanceSnapshotDecoder
            {
                Account = account,
                Balances = balances,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for BalanceUpdate</summary>
    public static class BalanceUpdateEncoder
    {
        public static byte[] Encode(Account string, Asset string, Delta double, Total double, Available double, Reason BalanceUpdateReason)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 32);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 25);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteString(buf, Asset);
                Wire.WriteF64BE(buf, Delta);
                Wire.WriteF64BE(buf, Total);
                Wire.WriteF64BE(buf, Available);
                buf.Add(BalanceUpdateReasonToValue(Reason));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for BalanceUpdate</summary>
    public class BalanceUpdateDecoder
    {
        public string Account { get; set; }
        public string Asset { get; set; }
        public double Delta { get; set; }
        public double Total { get; set; }
        public double Available { get; set; }
        public BalanceUpdateReason Reason { get; set; }
        public int EncodedLen() => 0;

        public static BalanceUpdateDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 32) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string asset = Wire.ReadString(buf, ref pos);
            double delta = Wire.ReadF64BE(buf, ref pos);
            double total = Wire.ReadF64BE(buf, ref pos);
            double available = Wire.ReadF64BE(buf, ref pos);
            byte reasonRaw = buf[pos++];
            var reason = BalanceUpdateReasonFromValue(reasonRaw) ?? throw new InvalidOperationException("invalid BalanceUpdateReason");
            return new BalanceUpdateDecoder
            {
                Account = account,
                Asset = asset,
                Delta = delta,
                Total = total,
                Available = available,
                Reason = reason,
            };
        }
    }

    /// <summary>SBE encoder for PositionSnapshot</summary>
    public static class PositionSnapshotEncoder
    {
        public static byte[] Encode(Account string, Positions List<string>, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 33);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Positions.Count);
                foreach (var item in Positions) {
                        PositionEntryEncoder.Encode(item, buf);
                }
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PositionSnapshot</summary>
    public class PositionSnapshotDecoder
    {
        public string Account { get; set; }
        public List<string> Positions { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static PositionSnapshotDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 33) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int positionsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var positions = new List<PositionEntry>(positionsCount);
            for (int i = 0; i < positionsCount; i++) {
                var item = PositionEntryDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                positions.Add(item);
            }
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new PositionSnapshotDecoder
            {
                Account = account,
                Positions = positions,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for PositionUpdate</summary>
    public static class PositionUpdateEncoder
    {
        public static byte[] Encode(Account string, Symbol string, Qty double, EntryPrice double, UnrealizedPnl double)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 34);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 24);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteString(buf, Symbol);
                Wire.WriteF64BE(buf, Qty);
                Wire.WriteF64BE(buf, EntryPrice);
                Wire.WriteF64BE(buf, UnrealizedPnl);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PositionUpdate</summary>
    public class PositionUpdateDecoder
    {
        public string Account { get; set; }
        public string Symbol { get; set; }
        public double Qty { get; set; }
        public double EntryPrice { get; set; }
        public double UnrealizedPnl { get; set; }
        public int EncodedLen() => 0;

        public static PositionUpdateDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 34) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double entry_price = Wire.ReadF64BE(buf, ref pos);
            double unrealized_pnl = Wire.ReadF64BE(buf, ref pos);
            return new PositionUpdateDecoder
            {
                Account = account,
                Symbol = symbol,
                Qty = qty,
                EntryPrice = entry_price,
                UnrealizedPnl = unrealized_pnl,
            };
        }
    }

    /// <summary>SBE encoder for MarginUpdate</summary>
    public static class MarginUpdateEncoder
    {
        public static byte[] Encode(Account string, Summary string, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 35);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteString(buf, Summary);
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for MarginUpdate</summary>
    public class MarginUpdateDecoder
    {
        public string Account { get; set; }
        public string Summary { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static MarginUpdateDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 35) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string summary = Wire.ReadString(buf, ref pos);
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new MarginUpdateDecoder
            {
                Account = account,
                Summary = summary,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for UserLiquidation</summary>
    public static class UserLiquidationEncoder
    {
        public static byte[] Encode(Account string, Symbol string, Qty double, Price double, Timestamp long)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 36);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 24);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteString(buf, Symbol);
                Wire.WriteF64BE(buf, Qty);
                Wire.WriteF64BE(buf, Price);
                Wire.WriteI64BE(buf, Timestamp);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for UserLiquidation</summary>
    public class UserLiquidationDecoder
    {
        public string Account { get; set; }
        public string Symbol { get; set; }
        public double Qty { get; set; }
        public double Price { get; set; }
        public long Timestamp { get; set; }
        public int EncodedLen() => 0;

        public static UserLiquidationDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 36) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string symbol = Wire.ReadString(buf, ref pos);
            double qty = Wire.ReadF64BE(buf, ref pos);
            double price = Wire.ReadF64BE(buf, ref pos);
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            return new UserLiquidationDecoder
            {
                Account = account,
                Symbol = symbol,
                Qty = qty,
                Price = price,
                Timestamp = timestamp,
            };
        }
    }

    /// <summary>SBE encoder for OrderListStatus</summary>
    public static class OrderListStatusEncoder
    {
        public static byte[] Encode(Account string, ListId string, Status OrderListStatusStatus, Symbol string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 37);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteString(buf, ListId);
                buf.Add(OrderListStatusStatusToValue(Status));
                if (Symbol.HasValue) { buf.Add(1); Wire.WriteString(buf, Symbol.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderListStatus</summary>
    public class OrderListStatusDecoder
    {
        public string Account { get; set; }
        public string ListId { get; set; }
        public OrderListStatusStatus Status { get; set; }
        public string? Symbol { get; set; }
        public int EncodedLen() => 0;

        public static OrderListStatusDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 37) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string list_id = Wire.ReadString(buf, ref pos);
            byte statusRaw = buf[pos++];
            var status = OrderListStatusStatusFromValue(statusRaw) ?? throw new InvalidOperationException("invalid OrderListStatusStatus");
            string? symbol;
            if (buf[pos++] == 1) symbol = Wire.ReadString(buf, ref pos);
            return new OrderListStatusDecoder
            {
                Account = account,
                ListId = list_id,
                Status = status,
                Symbol = symbol,
            };
        }
    }

    /// <summary>SBE encoder for FillHistoryRequest</summary>
    public static class FillHistoryRequestEncoder
    {
        public static byte[] Encode(Account string, Symbol string?, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 38);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Account);
                if (Symbol.HasValue) { buf.Add(1); Wire.WriteString(buf, Symbol.Value); } else { buf.Add(0); }
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FillHistoryRequest</summary>
    public class FillHistoryRequestDecoder
    {
        public string Account { get; set; }
        public string? Symbol { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static FillHistoryRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 38) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string? symbol;
            if (buf[pos++] == 1) symbol = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new FillHistoryRequestDecoder
            {
                Account = account,
                Symbol = symbol,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for FillHistoryBatch</summary>
    public static class FillHistoryBatchEncoder
    {
        public static byte[] Encode(Account string, Fills List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 39);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Fills.Count);
                foreach (var item in Fills) {
                        ExecutionReportEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FillHistoryBatch</summary>
    public class FillHistoryBatchDecoder
    {
        public string Account { get; set; }
        public List<string> Fills { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static FillHistoryBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 39) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int fillsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var fills = new List<ExecutionReport>(fillsCount);
            for (int i = 0; i < fillsCount; i++) {
                var item = ExecutionReportDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                fills.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new FillHistoryBatchDecoder
            {
                Account = account,
                Fills = fills,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for FundingPayment</summary>
    public static class FundingPaymentEncoder
    {
        public static byte[] Encode(Account string, Symbol string?, Amount double, Rate double, Timestamp long)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 40);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 24);

            // Fixed fields
                Wire.WriteString(buf, Account);
                if (Symbol.HasValue) { buf.Add(1); Wire.WriteString(buf, Symbol.Value); } else { buf.Add(0); }
                Wire.WriteF64BE(buf, Amount);
                Wire.WriteF64BE(buf, Rate);
                Wire.WriteI64BE(buf, Timestamp);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FundingPayment</summary>
    public class FundingPaymentDecoder
    {
        public string Account { get; set; }
        public string? Symbol { get; set; }
        public double Amount { get; set; }
        public double Rate { get; set; }
        public long Timestamp { get; set; }
        public int EncodedLen() => 0;

        public static FundingPaymentDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 40) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string? symbol;
            if (buf[pos++] == 1) symbol = Wire.ReadString(buf, ref pos);
            double amount = Wire.ReadF64BE(buf, ref pos);
            double rate = Wire.ReadF64BE(buf, ref pos);
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            return new FundingPaymentDecoder
            {
                Account = account,
                Symbol = symbol,
                Amount = amount,
                Rate = rate,
                Timestamp = timestamp,
            };
        }
    }

    /// <summary>SBE encoder for FundingHistoryRequest</summary>
    public static class FundingHistoryRequestEncoder
    {
        public static byte[] Encode(Account string, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 41);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Account);
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FundingHistoryRequest</summary>
    public class FundingHistoryRequestDecoder
    {
        public string Account { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static FundingHistoryRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 41) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new FundingHistoryRequestDecoder
            {
                Account = account,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for FundingHistoryBatch</summary>
    public static class FundingHistoryBatchEncoder
    {
        public static byte[] Encode(Account string, Payments List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 42);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Payments.Count);
                foreach (var item in Payments) {
                        FundingPaymentEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for FundingHistoryBatch</summary>
    public class FundingHistoryBatchDecoder
    {
        public string Account { get; set; }
        public List<string> Payments { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static FundingHistoryBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 42) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int paymentsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var payments = new List<FundingPayment>(paymentsCount);
            for (int i = 0; i < paymentsCount; i++) {
                var item = FundingPaymentDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                payments.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new FundingHistoryBatchDecoder
            {
                Account = account,
                Payments = payments,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for LedgerUpdate</summary>
    public static class LedgerUpdateEncoder
    {
        public static byte[] Encode(Account string, Asset string, Delta double, Kind LedgerUpdateKind, Timestamp long, ReferenceId string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 43);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 17);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteString(buf, Asset);
                Wire.WriteF64BE(buf, Delta);
                buf.Add(LedgerUpdateKindToValue(Kind));
                Wire.WriteI64BE(buf, Timestamp);
                if (ReferenceId.HasValue) { buf.Add(1); Wire.WriteString(buf, ReferenceId.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LedgerUpdate</summary>
    public class LedgerUpdateDecoder
    {
        public string Account { get; set; }
        public string Asset { get; set; }
        public double Delta { get; set; }
        public LedgerUpdateKind Kind { get; set; }
        public long Timestamp { get; set; }
        public string? ReferenceId { get; set; }
        public int EncodedLen() => 0;

        public static LedgerUpdateDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 43) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string asset = Wire.ReadString(buf, ref pos);
            double delta = Wire.ReadF64BE(buf, ref pos);
            byte kindRaw = buf[pos++];
            var kind = LedgerUpdateKindFromValue(kindRaw) ?? throw new InvalidOperationException("invalid LedgerUpdateKind");
            long timestamp = Wire.ReadI64BE(buf, ref pos);
            string? reference_id;
            if (buf[pos++] == 1) reference_id = Wire.ReadString(buf, ref pos);
            return new LedgerUpdateDecoder
            {
                Account = account,
                Asset = asset,
                Delta = delta,
                Kind = kind,
                Timestamp = timestamp,
                ReferenceId = reference_id,
            };
        }
    }

    /// <summary>SBE encoder for LedgerHistoryRequest</summary>
    public static class LedgerHistoryRequestEncoder
    {
        public static byte[] Encode(Account string, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 44);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Account);
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LedgerHistoryRequest</summary>
    public class LedgerHistoryRequestDecoder
    {
        public string Account { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static LedgerHistoryRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 44) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new LedgerHistoryRequestDecoder
            {
                Account = account,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for LedgerHistoryBatch</summary>
    public static class LedgerHistoryBatchEncoder
    {
        public static byte[] Encode(Account string, Entries List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 45);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Entries.Count);
                foreach (var item in Entries) {
                        LedgerUpdateEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for LedgerHistoryBatch</summary>
    public class LedgerHistoryBatchDecoder
    {
        public string Account { get; set; }
        public List<string> Entries { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static LedgerHistoryBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 45) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int entriesCount = (int)Wire.ReadU32BE(buf, ref pos);
            var entries = new List<LedgerUpdate>(entriesCount);
            for (int i = 0; i < entriesCount; i++) {
                var item = LedgerUpdateDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                entries.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new LedgerHistoryBatchDecoder
            {
                Account = account,
                Entries = entries,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for OpenOrdersRequest</summary>
    public static class OpenOrdersRequestEncoder
    {
        public static byte[] Encode(Account string, Symbol string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 46);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Account);
                if (Symbol.HasValue) { buf.Add(1); Wire.WriteString(buf, Symbol.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OpenOrdersRequest</summary>
    public class OpenOrdersRequestDecoder
    {
        public string Account { get; set; }
        public string? Symbol { get; set; }
        public int EncodedLen() => 0;

        public static OpenOrdersRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 46) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string? symbol;
            if (buf[pos++] == 1) symbol = Wire.ReadString(buf, ref pos);
            return new OpenOrdersRequestDecoder
            {
                Account = account,
                Symbol = symbol,
            };
        }
    }

    /// <summary>SBE encoder for OpenOrdersSnapshot</summary>
    public static class OpenOrdersSnapshotEncoder
    {
        public static byte[] Encode(Account string, Orders List<string>, IsSnapshot byte?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 47);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Orders.Count);
                foreach (var item in Orders) {
                        ExecutionReportEncoder.Encode(item, buf);
                }
                buf.Add(IsSnapshot ?? 0);
                buf.Add((byte)(IsSnapshot.HasValue ? 1 : 0));
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OpenOrdersSnapshot</summary>
    public class OpenOrdersSnapshotDecoder
    {
        public string Account { get; set; }
        public List<string> Orders { get; set; }
        public byte? IsSnapshot { get; set; }
        public int EncodedLen() => 0;

        public static OpenOrdersSnapshotDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 47) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int ordersCount = (int)Wire.ReadU32BE(buf, ref pos);
            var orders = new List<ExecutionReport>(ordersCount);
            for (int i = 0; i < ordersCount; i++) {
                var item = ExecutionReportDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                orders.Add(item);
            }
            byte v = buf[pos++];
            byte? is_snapshot = buf[pos++] == 1 ? v : null;
            return new OpenOrdersSnapshotDecoder
            {
                Account = account,
                Orders = orders,
                IsSnapshot = is_snapshot,
            };
        }
    }

    /// <summary>SBE encoder for OrderHistoryRequest</summary>
    public static class OrderHistoryRequestEncoder
    {
        public static byte[] Encode(Account string, Symbol string?, StartTime long?, EndTime long?, Limit u32?, Cursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 48);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 20);

            // Fixed fields
                Wire.WriteString(buf, Account);
                if (Symbol.HasValue) { buf.Add(1); Wire.WriteString(buf, Symbol.Value); } else { buf.Add(0); }
                if (StartTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, StartTime.Value); } else { buf.Add(0); }
                if (EndTime.HasValue) { buf.Add(1); Wire.WriteI64BE(buf, EndTime.Value); } else { buf.Add(0); }
            // TODO: encode limit as u32
                if (Cursor.HasValue) { buf.Add(1); Wire.WriteString(buf, Cursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderHistoryRequest</summary>
    public class OrderHistoryRequestDecoder
    {
        public string Account { get; set; }
        public string? Symbol { get; set; }
        public long? StartTime { get; set; }
        public long? EndTime { get; set; }
        public u32? Limit { get; set; }
        public string? Cursor { get; set; }
        public int EncodedLen() => 0;

        public static OrderHistoryRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 48) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            string? symbol;
            if (buf[pos++] == 1) symbol = Wire.ReadString(buf, ref pos);
            long? start_time;
            if (buf[pos++] == 1) start_time = Wire.ReadI64BE(buf, ref pos);
            long? end_time;
            if (buf[pos++] == 1) end_time = Wire.ReadI64BE(buf, ref pos);
        // TODO: decode limit
            string? cursor;
            if (buf[pos++] == 1) cursor = Wire.ReadString(buf, ref pos);
            return new OrderHistoryRequestDecoder
            {
                Account = account,
                Symbol = symbol,
                StartTime = start_time,
                EndTime = end_time,
                Limit = limit,
                Cursor = cursor,
            };
        }
    }

    /// <summary>SBE encoder for OrderHistoryBatch</summary>
    public static class OrderHistoryBatchEncoder
    {
        public static byte[] Encode(Account string, Orders List<string>, HasMore byte, NextCursor string?)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 49);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 1);

            // Fixed fields
                Wire.WriteString(buf, Account);
                Wire.WriteU32BE(buf, (uint)Orders.Count);
                foreach (var item in Orders) {
                        ExecutionReportEncoder.Encode(item, buf);
                }
                buf.Add(HasMore);
                if (NextCursor.HasValue) { buf.Add(1); Wire.WriteString(buf, NextCursor.Value); } else { buf.Add(0); }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for OrderHistoryBatch</summary>
    public class OrderHistoryBatchDecoder
    {
        public string Account { get; set; }
        public List<string> Orders { get; set; }
        public byte HasMore { get; set; }
        public string? NextCursor { get; set; }
        public int EncodedLen() => 0;

        public static OrderHistoryBatchDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 49) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            int ordersCount = (int)Wire.ReadU32BE(buf, ref pos);
            var orders = new List<ExecutionReport>(ordersCount);
            for (int i = 0; i < ordersCount; i++) {
                var item = ExecutionReportDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                orders.Add(item);
            }
            byte has_more = buf[pos++];
            string? next_cursor;
            if (buf[pos++] == 1) next_cursor = Wire.ReadString(buf, ref pos);
            return new OrderHistoryBatchDecoder
            {
                Account = account,
                Orders = orders,
                HasMore = has_more,
                NextCursor = next_cursor,
            };
        }
    }

    /// <summary>SBE encoder for CapabilitiesRequest</summary>
    public static class CapabilitiesRequestEncoder
    {
        public static byte[] Encode()
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 50);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CapabilitiesRequest</summary>
    public class CapabilitiesRequestDecoder
    {
        public int EncodedLen() => 0;

        public static CapabilitiesRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 50) throw new InvalidOperationException("invalid template_id");
            return new CapabilitiesRequestDecoder
            {

            };
        }
    }

    /// <summary>SBE encoder for CapabilitiesResponse</summary>
    public static class CapabilitiesResponseEncoder
    {
        public static byte[] Encode(SchemaIds List<string>, Paths List<string>, Symbols List<string>, Intervals List<string>)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 51);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteU32BE(buf, (uint)SchemaIds.Count);
                foreach (var item in SchemaIds) {
                        u8Encoder.Encode(item, buf);
                }
                Wire.WriteU32BE(buf, (uint)Paths.Count);
                foreach (var item in Paths) {
                        CapabilityPathEncoder.Encode(item, buf);
                }
                Wire.WriteU32BE(buf, (uint)Symbols.Count);
                foreach (var item in Symbols) {
                        SymbolEncoder.Encode(item, buf);
                }
                Wire.WriteU32BE(buf, (uint)Intervals.Count);
                foreach (var item in Intervals) {
                        CandleIntervalEncoder.Encode(item, buf);
                }
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for CapabilitiesResponse</summary>
    public class CapabilitiesResponseDecoder
    {
        public List<string> SchemaIds { get; set; }
        public List<string> Paths { get; set; }
        public List<string> Symbols { get; set; }
        public List<string> Intervals { get; set; }
        public int EncodedLen() => 0;

        public static CapabilitiesResponseDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 51) throw new InvalidOperationException("invalid template_id");
            int schema_idsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var schema_ids = new List<u8>(schema_idsCount);
            for (int i = 0; i < schema_idsCount; i++) {
                var item = u8Decoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                schema_ids.Add(item);
            }
            int pathsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var paths = new List<CapabilityPath>(pathsCount);
            for (int i = 0; i < pathsCount; i++) {
                var item = CapabilityPathDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                paths.Add(item);
            }
            int symbolsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var symbols = new List<Symbol>(symbolsCount);
            for (int i = 0; i < symbolsCount; i++) {
                var item = SymbolDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                symbols.Add(item);
            }
            int intervalsCount = (int)Wire.ReadU32BE(buf, ref pos);
            var intervals = new List<CandleInterval>(intervalsCount);
            for (int i = 0; i < intervalsCount; i++) {
                var item = CandleIntervalDecoder.Decode(buf[pos..]);
                pos += item.EncodedLen();
                intervals.Add(item);
            }
            return new CapabilitiesResponseDecoder
            {
                SchemaIds = schema_ids,
                Paths = paths,
                Symbols = symbols,
                Intervals = intervals,
            };
        }
    }

    /// <summary>SBE encoder for PositionRequest</summary>
    public static class PositionRequestEncoder
    {
        public static byte[] Encode(Account string)
        {
            var buf = new List<byte>();
            // SBE Message Header (8 bytes)
                Wire.WriteU16BE(buf, SCHEMA_ID);
                Wire.WriteU16BE(buf, 52);
                Wire.WriteU16BE(buf, 0);
                Wire.WriteU16BE(buf, 0);

            // Fixed fields
                Wire.WriteString(buf, Account);
            return buf.ToArray();
        }
    }

    /// <summary>SBE decoder for PositionRequest</summary>
    public class PositionRequestDecoder
    {
        public string Account { get; set; }
        public int EncodedLen() => 0;

        public static PositionRequestDecoder Decode(byte[] buf)
        {
            if (buf.Length < 8) throw new InvalidOperationException("buffer too short for SBE header");
            int pos = 0;
            ushort schemaId = Wire.ReadU16BE(buf, ref pos);
            ushort tmplId = Wire.ReadU16BE(buf, ref pos);
            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);
            if (schemaId != SCHEMA_ID) throw new InvalidOperationException("invalid schema_id");
            if (tmplId != 52) throw new InvalidOperationException("invalid template_id");
            string account = Wire.ReadString(buf, ref pos);
            return new PositionRequestDecoder
            {
                Account = account,
            };
        }
    }

}
