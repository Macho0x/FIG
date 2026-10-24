using System;
using System.Runtime.InteropServices;

namespace Fig;

public sealed class FundingState : IDisposable
{
    private IntPtr _handle;

    public FundingState(int capacity = 128)
    {
        _handle = FigNative.fig_funding_new((UIntPtr)capacity);
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_funding_new failed");
        }
    }

    public void Apply(ReadOnlySpan<byte> payload)
    {
        unsafe
        {
            fixed (byte* p = payload)
            {
                if (FigNative.fig_funding_apply(_handle, (IntPtr)p, (UIntPtr)payload.Length) != 0)
                {
                    throw new InvalidOperationException("fig_funding_apply failed");
                }
            }
        }
    }

    public int Len => (int)FigNative.fig_funding_len(_handle);

    public double LatestAmount => FigNative.fig_funding_latest_amount(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_funding_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class AggTradesState : IDisposable
{
    private IntPtr _handle;

    public AggTradesState(int capacity = 256)
    {
        _handle = FigNative.fig_agg_trades_new((UIntPtr)capacity);
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_agg_trades_new failed");
        }
    }

    public void Apply(ReadOnlySpan<byte> payload)
    {
        unsafe
        {
            fixed (byte* p = payload)
            {
                if (FigNative.fig_agg_trades_apply(_handle, (IntPtr)p, (UIntPtr)payload.Length) != 0)
                {
                    throw new InvalidOperationException("fig_agg_trades_apply failed");
                }
            }
        }
    }

    public int Len => (int)FigNative.fig_agg_trades_len(_handle);

    public double LatestPrice => FigNative.fig_agg_trades_latest_price(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_agg_trades_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class LedgerState : IDisposable
{
    private IntPtr _handle;

    public LedgerState(int capacity = 128)
    {
        _handle = FigNative.fig_ledger_new((UIntPtr)capacity);
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_ledger_new failed");
        }
    }

    public void Apply(ReadOnlySpan<byte> payload)
    {
        unsafe
        {
            fixed (byte* p = payload)
            {
                if (FigNative.fig_ledger_apply(_handle, (IntPtr)p, (UIntPtr)payload.Length) != 0)
                {
                    throw new InvalidOperationException("fig_ledger_apply failed");
                }
            }
        }
    }

    public int Len => (int)FigNative.fig_ledger_len(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_ledger_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class LiquidationState : IDisposable
{
    private IntPtr _handle;

    public LiquidationState(int capacity = 64)
    {
        _handle = FigNative.fig_liquidation_new((UIntPtr)capacity);
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_liquidation_new failed");
        }
    }

    public void ApplyUser(ReadOnlySpan<byte> payload)
    {
        unsafe
        {
            fixed (byte* p = payload)
            {
                if (FigNative.fig_liquidation_apply_user(_handle, (IntPtr)p, (UIntPtr)payload.Length) != 0)
                {
                    throw new InvalidOperationException("fig_liquidation_apply_user failed");
                }
            }
        }
    }

    public void ApplyPublic(ReadOnlySpan<byte> payload)
    {
        unsafe
        {
            fixed (byte* p = payload)
            {
                if (FigNative.fig_liquidation_apply_public(_handle, (IntPtr)p, (UIntPtr)payload.Length) != 0)
                {
                    throw new InvalidOperationException("fig_liquidation_apply_public failed");
                }
            }
        }
    }

    public int UserCount => (int)FigNative.fig_liquidation_user_count(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_liquidation_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

static class MergePayload
{
    public static void Apply(IntPtr handle, ReadOnlySpan<byte> payload, string name, Func<IntPtr, IntPtr, UIntPtr, int> call)
    {
        unsafe
        {
            fixed (byte* p = payload)
            {
                if (call(handle, (IntPtr)p, (UIntPtr)payload.Length) != 0)
                {
                    throw new InvalidOperationException(name + " failed");
                }
            }
        }
    }
}

public sealed class OrderBookState : IDisposable
{
    private IntPtr _handle = FigNative.fig_order_book_new();

    public OrderBookState()
    {
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_order_book_new failed");
        }
    }

    public void ApplySnapshot(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_order_book_apply_snapshot", FigNative.fig_order_book_apply_snapshot);

    public void ApplyDelta(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_order_book_apply_delta", FigNative.fig_order_book_apply_delta);

    public double BestBid => FigNative.fig_order_book_best_bid(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_order_book_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class MidsState : IDisposable
{
    private IntPtr _handle = FigNative.fig_mids_new();

    public MidsState()
    {
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_mids_new failed");
        }
    }

    public void ApplyTicker(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_mids_apply_ticker", FigNative.fig_mids_apply_ticker);

    public void ApplyBatch(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_mids_apply_batch", FigNative.fig_mids_apply_batch);

    public double Mid(string symbol) => FigNative.fig_mids_mid(_handle, symbol);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_mids_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class BboState : IDisposable
{
    private IntPtr _handle = FigNative.fig_bbo_new();

    public BboState()
    {
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_bbo_new failed");
        }
    }

    public void Apply(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_bbo_apply", FigNative.fig_bbo_apply);

    public double ImpliedMid => FigNative.fig_bbo_implied_mid(_handle);
    public double BestBid => FigNative.fig_bbo_best_bid(_handle);
    public double BestAsk => FigNative.fig_bbo_best_ask(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_bbo_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class TradeTape : IDisposable
{
    private IntPtr _handle;

    public TradeTape(int capacity = 256)
    {
        _handle = FigNative.fig_trade_tape_new((UIntPtr)capacity);
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_trade_tape_new failed");
        }
    }

    public void Push(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_trade_tape_push", FigNative.fig_trade_tape_push);

    public int Len => (int)FigNative.fig_trade_tape_len(_handle);
    public double LatestPrice => FigNative.fig_trade_tape_latest_price(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_trade_tape_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class MarkPriceState : IDisposable
{
    private IntPtr _handle = FigNative.fig_mark_price_new();

    public MarkPriceState()
    {
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_mark_price_new failed");
        }
    }

    public void Apply(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_mark_price_apply", FigNative.fig_mark_price_apply);

    public double Get(string symbol) => FigNative.fig_mark_price_get(_handle, symbol);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_mark_price_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class OrdersState : IDisposable
{
    private IntPtr _handle;

    public OrdersState(int execCapacity = 256)
    {
        _handle = FigNative.fig_orders_new((UIntPtr)execCapacity);
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_orders_new failed");
        }
    }

    public void ApplySnapshot(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_orders_apply_snapshot", FigNative.fig_orders_apply_snapshot);

    public void ApplyExecution(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_orders_apply_execution", FigNative.fig_orders_apply_execution);

    public int OpenCount => (int)FigNative.fig_orders_open_count(_handle);
    public int ExecutionCount => (int)FigNative.fig_orders_execution_count(_handle);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_orders_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

public sealed class CandleState : IDisposable
{
    private IntPtr _handle = FigNative.fig_candle_new();

    public CandleState()
    {
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("fig_candle_new failed");
        }
    }

    public void Apply(ReadOnlySpan<byte> payload) =>
        MergePayload.Apply(_handle, payload, "fig_candle_apply", FigNative.fig_candle_apply);

    public double Close => FigNative.fig_candle_close(_handle);
    public int IsFinal => FigNative.fig_candle_is_final(_handle);

    public string Symbol
    {
        get
        {
            if (FigNative.fig_candle_symbol(_handle, out var text) != 0)
            {
                return "";
            }
            return FigNative.TakeString(text);
        }
    }

    public string Interval
    {
        get
        {
            if (FigNative.fig_candle_interval(_handle, out var text) != 0)
            {
                return "";
            }
            return FigNative.TakeString(text);
        }
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_candle_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}
