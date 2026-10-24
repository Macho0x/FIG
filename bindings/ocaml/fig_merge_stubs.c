#include <caml/alloc.h>
#include <caml/custom.h>
#include <caml/fail.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>

#include "fig.h"

#define Client_val(v) (*((struct FigClientHandle **)Data_custom_val(v)))

struct fig_merge {
    void *ptr;
    void (*free_fn)(void *);
};

static void fig_merge_finalize(value v) {
    struct fig_merge *h = (struct fig_merge *)Data_custom_val(v);
    if (h->ptr != NULL) {
        h->free_fn(h->ptr);
        h->ptr = NULL;
    }
}

static struct custom_operations fig_merge_ops = {
    "fig.FigMerge",
    fig_merge_finalize,
    custom_compare_default,
    custom_hash_default,
    custom_serialize_default,
    custom_deserialize_default,
    custom_compare_ext_default,
    NULL,
};

static value alloc_merge(void *p, void (*free_fn)(void *)) {
    value v = caml_alloc_custom(&fig_merge_ops, sizeof(struct fig_merge), 0, 1);
    struct fig_merge *h = (struct fig_merge *)Data_custom_val(v);
    h->ptr = p;
    h->free_fn = free_fn;
    return v;
}

#define Merge_ptr(v) (((struct fig_merge *)Data_custom_val(v))->ptr)

#define DEFINE_FREE(name, fn, ctype) \
    static void name(void *p) { fn((ctype *)p); }

#define DEFINE_NEW0(caml_name, cnew, freefn)                    \
    CAMLprim value caml_name(value unit) {                     \
        CAMLparam1(unit);                                      \
        CAMLreturn(alloc_merge(cnew(), freefn));               \
    }

#define DEFINE_NEW1(caml_name, cnew, freefn)                    \
    CAMLprim value caml_name(value capacity) {                 \
        CAMLparam1(capacity);                                  \
        CAMLreturn(alloc_merge(cnew((uintptr_t)Int_val(capacity)), freefn)); \
    }

#define DEFINE_APPLY(caml_name, fn, ctype, fail_msg)                                   \
    CAMLprim value caml_name(value h, value payload) {                                \
        CAMLparam2(h, payload);                                                       \
        int32_t rc = fn(                                                              \
            (ctype *)Merge_ptr(h),                                                   \
            (const uint8_t *)String_val(payload),                                    \
            caml_string_length(payload));                                            \
        if (rc != 0) {                                                                \
            caml_failwith(fail_msg);                                                  \
        }                                                                             \
        CAMLreturn(Val_unit);                                                         \
    }

#define DEFINE_F64(caml_name, fn, ctype)                              \
    CAMLprim value caml_name(value h) {                               \
        CAMLparam1(h);                                                \
        CAMLreturn(caml_copy_double(fn((const ctype *)Merge_ptr(h)))); \
    }

#define DEFINE_COUNT(caml_name, fn, ctype)                            \
    CAMLprim value caml_name(value h) {                               \
        CAMLparam1(h);                                                \
        CAMLreturn(Val_int((int)fn((const ctype *)Merge_ptr(h))));    \
    }

DEFINE_FREE(free_order_book, fig_order_book_free, struct FigOrderBookHandle)
DEFINE_FREE(free_mids, fig_mids_free, struct FigMidsHandle)
DEFINE_FREE(free_bbo, fig_bbo_free, struct FigBboHandle)
DEFINE_FREE(free_trade_tape, fig_trade_tape_free, struct FigTradeTapeHandle)
DEFINE_FREE(free_mark_price, fig_mark_price_free, struct FigMarkPriceHandle)
DEFINE_FREE(free_orders, fig_orders_free, struct FigOrdersHandle)
DEFINE_FREE(free_funding, fig_funding_free, struct FigFundingHandle)
DEFINE_FREE(free_agg_trades, fig_agg_trades_free, struct FigAggTradesHandle)
DEFINE_FREE(free_ledger, fig_ledger_free, struct FigLedgerHandle)
DEFINE_FREE(free_liquidation, fig_liquidation_free, struct FigLiquidationHandle)
DEFINE_FREE(free_candle, fig_candle_free, struct FigCandleHandle)

DEFINE_NEW0(caml_fig_order_book_new, fig_order_book_new, free_order_book)
DEFINE_APPLY(caml_fig_order_book_apply_snapshot, fig_order_book_apply_snapshot, struct FigOrderBookHandle, "fig_order_book_apply_snapshot failed")
DEFINE_APPLY(caml_fig_order_book_apply_delta, fig_order_book_apply_delta, struct FigOrderBookHandle, "fig_order_book_apply_delta failed")
DEFINE_F64(caml_fig_order_book_best_bid, fig_order_book_best_bid, struct FigOrderBookHandle)

DEFINE_NEW0(caml_fig_mids_new, fig_mids_new, free_mids)
DEFINE_APPLY(caml_fig_mids_apply_ticker, fig_mids_apply_ticker, struct FigMidsHandle, "fig_mids_apply_ticker failed")
DEFINE_APPLY(caml_fig_mids_apply_batch, fig_mids_apply_batch, struct FigMidsHandle, "fig_mids_apply_batch failed")

CAMLprim value caml_fig_mids_mid(value h, value symbol) {
    CAMLparam2(h, symbol);
    CAMLreturn(caml_copy_double(fig_mids_mid((const struct FigMidsHandle *)Merge_ptr(h), String_val(symbol))));
}

DEFINE_NEW0(caml_fig_bbo_new, fig_bbo_new, free_bbo)
DEFINE_APPLY(caml_fig_bbo_apply, fig_bbo_apply, struct FigBboHandle, "fig_bbo_apply failed")
DEFINE_F64(caml_fig_bbo_implied_mid, fig_bbo_implied_mid, struct FigBboHandle)
DEFINE_F64(caml_fig_bbo_best_bid, fig_bbo_best_bid, struct FigBboHandle)
DEFINE_F64(caml_fig_bbo_best_ask, fig_bbo_best_ask, struct FigBboHandle)

DEFINE_NEW1(caml_fig_trade_tape_new, fig_trade_tape_new, free_trade_tape)
DEFINE_APPLY(caml_fig_trade_tape_push, fig_trade_tape_push, struct FigTradeTapeHandle, "fig_trade_tape_push failed")
DEFINE_COUNT(caml_fig_trade_tape_len, fig_trade_tape_len, struct FigTradeTapeHandle)
DEFINE_F64(caml_fig_trade_tape_latest_price, fig_trade_tape_latest_price, struct FigTradeTapeHandle)

DEFINE_NEW0(caml_fig_mark_price_new, fig_mark_price_new, free_mark_price)
DEFINE_APPLY(caml_fig_mark_price_apply, fig_mark_price_apply, struct FigMarkPriceHandle, "fig_mark_price_apply failed")

CAMLprim value caml_fig_mark_price_get(value h, value symbol) {
    CAMLparam2(h, symbol);
    CAMLreturn(caml_copy_double(
        fig_mark_price_get((const struct FigMarkPriceHandle *)Merge_ptr(h), String_val(symbol))));
}

DEFINE_NEW1(caml_fig_orders_new, fig_orders_new, free_orders)
DEFINE_APPLY(caml_fig_orders_apply_snapshot, fig_orders_apply_snapshot, struct FigOrdersHandle, "fig_orders_apply_snapshot failed")
DEFINE_APPLY(caml_fig_orders_apply_execution, fig_orders_apply_execution, struct FigOrdersHandle, "fig_orders_apply_execution failed")
DEFINE_COUNT(caml_fig_orders_open_count, fig_orders_open_count, struct FigOrdersHandle)
DEFINE_COUNT(caml_fig_orders_execution_count, fig_orders_execution_count, struct FigOrdersHandle)

DEFINE_NEW1(caml_fig_funding_new, fig_funding_new, free_funding)
DEFINE_APPLY(caml_fig_funding_apply, fig_funding_apply, struct FigFundingHandle, "fig_funding_apply failed")
DEFINE_COUNT(caml_fig_funding_len, fig_funding_len, struct FigFundingHandle)
DEFINE_F64(caml_fig_funding_latest_amount, fig_funding_latest_amount, struct FigFundingHandle)

DEFINE_NEW1(caml_fig_agg_trades_new, fig_agg_trades_new, free_agg_trades)
DEFINE_APPLY(caml_fig_agg_trades_apply, fig_agg_trades_apply, struct FigAggTradesHandle, "fig_agg_trades_apply failed")
DEFINE_COUNT(caml_fig_agg_trades_len, fig_agg_trades_len, struct FigAggTradesHandle)
DEFINE_F64(caml_fig_agg_trades_latest_price, fig_agg_trades_latest_price, struct FigAggTradesHandle)

DEFINE_NEW1(caml_fig_ledger_new, fig_ledger_new, free_ledger)
DEFINE_APPLY(caml_fig_ledger_apply, fig_ledger_apply, struct FigLedgerHandle, "fig_ledger_apply failed")
DEFINE_COUNT(caml_fig_ledger_len, fig_ledger_len, struct FigLedgerHandle)

DEFINE_NEW1(caml_fig_liquidation_new, fig_liquidation_new, free_liquidation)
DEFINE_APPLY(caml_fig_liquidation_apply_user, fig_liquidation_apply_user, struct FigLiquidationHandle, "fig_liquidation_apply_user failed")
DEFINE_APPLY(caml_fig_liquidation_apply_public, fig_liquidation_apply_public, struct FigLiquidationHandle, "fig_liquidation_apply_public failed")
DEFINE_COUNT(caml_fig_liquidation_user_count, fig_liquidation_user_count, struct FigLiquidationHandle)

DEFINE_NEW0(caml_fig_candle_new, fig_candle_new, free_candle)
DEFINE_APPLY(caml_fig_candle_apply, fig_candle_apply, struct FigCandleHandle, "fig_candle_apply failed")
DEFINE_F64(caml_fig_candle_close, fig_candle_close, struct FigCandleHandle)

CAMLprim value caml_fig_candle_is_final(value h) {
    CAMLparam1(h);
    CAMLreturn(Val_int(fig_candle_is_final((const struct FigCandleHandle *)Merge_ptr(h))));
}

static value candle_text(value h, int32_t (*fn)(const struct FigCandleHandle *, char **)) {
    CAMLparam1(h);
    CAMLlocal1(str);
    char *out = NULL;
    int32_t rc = fn((const struct FigCandleHandle *)Merge_ptr(h), &out);
    if (rc != 0 || out == NULL) {
        str = caml_alloc_string(0);
    } else {
        str = caml_copy_string(out);
        fig_string_free(out);
    }
    CAMLreturn(str);
}

CAMLprim value caml_fig_candle_symbol(value h) {
    return candle_text(h, fig_candle_symbol);
}

CAMLprim value caml_fig_candle_interval(value h) {
    return candle_text(h, fig_candle_interval);
}

CAMLprim value caml_fig_jwt_decode_sub(value token, value secret) {
    CAMLparam2(token, secret);
    CAMLlocal1(str);
    char *sub = NULL;
    int32_t rc = fig_jwt_decode_sub(String_val(token), String_val(secret), &sub);
    if (rc != 0) {
        caml_failwith("fig_jwt_decode_sub failed");
    }
    str = caml_copy_string(sub);
    fig_string_free(sub);
    CAMLreturn(str);
}

static value bytes_of_fig_buffer(struct FigBuffer buf) {
    value v;
    if (buf.data == NULL || buf.len == 0) {
        fig_buffer_free(buf);
        return caml_alloc_string(0);
    }
    v = caml_alloc_initialized_string(buf.len, (const char *)buf.data);
    fig_buffer_free(buf);
    return v;
}

CAMLprim value caml_fig_client_post_batch(value client, value account, value payload, value channel) {
    CAMLparam4(client, account, payload, channel);
    struct FigBuffer out = {0};
    int32_t rc = fig_client_post_batch(
        Client_val(client),
        String_val(account),
        (const uint8_t *)String_val(payload),
        caml_string_length(payload),
        (uint16_t)Int_val(channel),
        &out);
    if (rc != 0) {
        caml_failwith("fig_client_post_batch failed");
    }
    CAMLreturn(bytes_of_fig_buffer(out));
}
