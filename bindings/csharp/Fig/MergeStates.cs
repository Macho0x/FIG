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
