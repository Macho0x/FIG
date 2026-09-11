#ifndef FIG_FFI_H
#define FIG_FFI_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Opaque aggregate trade tape for C bindings.
 */
typedef struct FigAggTradesHandle FigAggTradesHandle;

/**
 * Opaque BBO state for C bindings.
 */
typedef struct FigBboHandle FigBboHandle;

/**
 * Opaque connected FIG client (one TREE connection).
 */
typedef struct FigClientHandle FigClientHandle;

/**
 * Opaque funding payment ring for C bindings.
 */
typedef struct FigFundingHandle FigFundingHandle;

/**
 * Opaque ledger update ring for C bindings.
 */
typedef struct FigLedgerHandle FigLedgerHandle;

/**
 * Opaque liquidation merge state for C bindings.
 */
typedef struct FigLiquidationHandle FigLiquidationHandle;

/**
 * Opaque mark price cache for C bindings.
 */
typedef struct FigMarkPriceHandle FigMarkPriceHandle;

/**
 * Opaque mids cache for C bindings.
 */
typedef struct FigMidsHandle FigMidsHandle;

/**
 * Opaque order book state for C bindings.
 */
typedef struct FigOrderBookHandle FigOrderBookHandle;

/**
 * Opaque orders + executions merge state for C bindings.
 */
typedef struct FigOrdersHandle FigOrdersHandle;

/**
 * Opaque public trade tape for C bindings.
 */
typedef struct FigTradeTapeHandle FigTradeTapeHandle;

/**
 * Opaque owned byte buffer returned to callers.
 */
typedef struct FigBuffer {
  uint8_t *data;
  uintptr_t len;
} FigBuffer;

/**
 * List of encoded frames returned from request/subscribe calls.
 */
typedef struct FigFrameList {
  struct FigBuffer *frames;
  uintptr_t count;
} FigFrameList;

/**
 * Free a buffer previously returned by fig_* functions.
 */
void fig_buffer_free(struct FigBuffer buf);

/**
 * Returns the workspace crate version (`CARGO_PKG_VERSION`) as a C string.
 */
const char *fig_version(void);

/**
 * Encode a minimal REQUEST frame. Returns 0 on success.
 */
int32_t fig_frame_encode_request(uint16_t channel_id,
                                 uint32_t stream_seq,
                                 uint8_t schema_id,
                                 const uint8_t *payload,
                                 uintptr_t payload_len,
                                 struct FigBuffer *out);

/**
 * Encode REQUEST with optional ChannelPath / Method / ContentType extensions.
 */
int32_t fig_frame_encode_request_ex(uint16_t channel_id,
                                    uint32_t stream_seq,
                                    uint8_t schema_id,
                                    const char *channel_path,
                                    const char *method,
                                    const char *content_type,
                                    const uint8_t *payload,
                                    uintptr_t payload_len,
                                    struct FigBuffer *out);

/**
 * Encode SUBSCRIBE frame with routing key and channel path.
 */
int32_t fig_frame_encode_subscribe(uint16_t channel_id,
                                   uint32_t stream_seq,
                                   const char *routing_key,
                                   const char *channel_path,
                                   struct FigBuffer *out);

/**
 * Encode REQUEST with optional ChannelPath, Method, ContentType, AuthToken.
 */
int32_t fig_frame_encode_request_auth(uint16_t channel_id,
                                      uint32_t stream_seq,
                                      uint8_t schema_id,
                                      const char *channel_path,
                                      const char *method,
                                      const char *content_type,
                                      const char *auth_token,
                                      const uint8_t *payload,
                                      uintptr_t payload_len,
                                      struct FigBuffer *out);

/**
 * Encode SUBSCRIBE with optional auth token (CorrelationId = stream_seq).
 */
int32_t fig_frame_encode_subscribe_auth(uint16_t channel_id,
                                        uint32_t stream_seq,
                                        const char *routing_key,
                                        const char *channel_path,
                                        const char *auth_token,
                                        struct FigBuffer *out);

/**
 * Encode PING control frame.
 */
int32_t fig_frame_encode_ping(struct FigBuffer *out);

int32_t fig_frame_decode_header(const uint8_t *data,
                                uintptr_t data_len,
                                uint16_t *out_channel_id,
                                uint32_t *out_stream_seq,
                                uint8_t *out_schema_id,
                                uint8_t *out_frame_type);

int32_t fig_cbor_encode_new_order_single(const char *cl_ord_id,
                                         const char *symbol,
                                         uint8_t side_buy,
                                         double order_qty,
                                         double price,
                                         int8_t post_only,
                                         int8_t reduce_only,
                                         struct FigBuffer *out);

int32_t fig_cbor_decode_new_order_single_cl_ord_id(const uint8_t *data,
                                                   uintptr_t data_len,
                                                   char **out_cl_ord_id);

int32_t fig_cbor_encode_candle_bar(struct FigBuffer *out);

int32_t fig_cbor_encode_capabilities(struct FigBuffer *out);

int32_t fig_cbor_encode_open_orders_snapshot(struct FigBuffer *out);

int32_t fig_cbor_encode_market_data_snapshot(struct FigBuffer *out);

int32_t fig_cbor_encode_order_history_request(struct FigBuffer *out);

void fig_string_free(char *s);

uint64_t fig_client_channel_stream_id(uint16_t channel_id, uint8_t is_server);

int32_t fig_cbor_encode_instrument_catalog_response(struct FigBuffer *out);

int32_t fig_cbor_encode_open_orders_request(struct FigBuffer *out);

/**
 * Compress payload bytes with zstd. Returns 0 on success.
 */
int32_t fig_payload_compress(const uint8_t *data, uintptr_t data_len, struct FigBuffer *out);

/**
 * Decompress zstd payload bytes. Returns 0 on success.
 */
int32_t fig_payload_decompress(const uint8_t *data, uintptr_t data_len, struct FigBuffer *out);

/**
 * Split an encoded frame into fragments (max payload size per fragment).
 */
int32_t fig_frame_split(const uint8_t *frame_bytes,
                        uintptr_t frame_len,
                        uintptr_t max_payload,
                        struct FigFrameList *out);

/**
 * Reassemble fragment frames (encoded bytes concatenated in `frames` array).
 */
int32_t fig_frames_reassemble(const uint8_t *const *frame_ptrs,
                              const uintptr_t *frame_lens,
                              uintptr_t count,
                              struct FigBuffer *out);

/**
 * Connect with optional 0-RTT resumption token.
 */
int32_t fig_client_connect_0rtt(const char *addr,
                                const char *server_name,
                                const uint8_t *resumption_token,
                                uintptr_t token_len,
                                struct FigClientHandle **out);

/**
 * Export resumption token for an active session (if present).
 */
int32_t fig_client_export_resumption_token(struct FigClientHandle *handle, struct FigBuffer *out);

/**
 * Prepare migration token CBOR bytes from active session + connection.
 */
int32_t fig_client_migration_prepare(struct FigClientHandle *handle, struct FigBuffer *out);

/**
 * Apply migration token CBOR and reopen channels on the connection.
 */
int32_t fig_client_migration_apply(struct FigClientHandle *handle,
                                   const uint8_t *token_bytes,
                                   uintptr_t token_len);

/**
 * Restore session from resumption token bytes (without connecting).
 */
int32_t fig_session_from_resumption_token(const uint8_t *token_bytes,
                                          uintptr_t token_len,
                                          struct FigBuffer *out);

void fig_frame_list_free(struct FigFrameList list);

/**
 * Connect to a FIG server (`host:port`). Returns 0 on success.
 */
int32_t fig_client_connect(const char *addr, const char *server_name, struct FigClientHandle **out);

void fig_client_close(struct FigClientHandle *handle);

/**
 * Send encoded frame bytes; receive all response frames.
 */
int32_t fig_client_request_and_recv(struct FigClientHandle *handle,
                                    const uint8_t *frame_bytes,
                                    uintptr_t frame_len,
                                    struct FigFrameList *out);

/**
 * Send PING on channel 0.
 */
int32_t fig_client_ping(struct FigClientHandle *handle);

/**
 * Encode HS256 JWT claims (`sub`, `exp`) into `out`. Returns 0 on success.
 */
int32_t fig_jwt_encode(const char *sub, uint64_t exp, const char *secret, struct FigBuffer *out);

/**
 * Decode JWT and return the `sub` claim via newly allocated C string.
 */
int32_t fig_jwt_decode_sub(const char *token, const char *secret, char **out_sub);

/**
 * Verify bearer JWT signature and expiry. Returns 0 on success.
 */
int32_t fig_jwt_verify_bearer(const char *token, const char *secret);

/**
 * Create an empty order book state.
 */
struct FigOrderBookHandle *fig_order_book_new(void);

/**
 * Free order book state.
 *
 * # Safety
 * `handle` must be from `fig_order_book_new` and not already freed.
 */
void fig_order_book_free(struct FigOrderBookHandle *handle);

/**
 * Apply CBOR `OrderBookSnapshot` payload. Returns 0 on success, -1 on error.
 *
 * # Safety
 * `payload` must be valid for `len` bytes.
 */
int32_t fig_order_book_apply_snapshot(struct FigOrderBookHandle *handle,
                                      const uint8_t *payload,
                                      uintptr_t len);

/**
 * Apply CBOR `OrderBookDelta` payload. Returns 0 on success, -1 on error.
 *
 * # Safety
 * `payload` must be valid for `len` bytes.
 */
int32_t fig_order_book_apply_delta(struct FigOrderBookHandle *handle,
                                   const uint8_t *payload,
                                   uintptr_t len);

/**
 * Best bid price, or NaN if empty.
 *
 * # Safety
 * `handle` must be valid.
 */
double fig_order_book_best_bid(const struct FigOrderBookHandle *handle);

int32_t fig_sbe_encode_new_order_single(const char *cl_ord_id,
                                        const char *symbol,
                                        uint8_t side_buy,
                                        double qty,
                                        double price,
                                        int8_t post_only,
                                        int8_t reduce_only,
                                        struct FigBuffer *out);

int32_t fig_sbe_decode_new_order_single_cl_ord_id(const uint8_t *data,
                                                  uintptr_t len,
                                                  char **out_cl_ord_id);

int32_t fig_sbe_encode_candle_bar(const char *symbol,
                                  const char *interval,
                                  double open,
                                  double high,
                                  double low,
                                  double close,
                                  double volume,
                                  int64_t bar_start,
                                  int64_t bar_end,
                                  uint8_t is_final,
                                  struct FigBuffer *out);

int32_t fig_sbe_encode_symbol_ticker(const char *symbol,
                                     double last_price,
                                     double price_change,
                                     double price_change_pct,
                                     double volume,
                                     double high,
                                     double low,
                                     double open,
                                     int64_t timestamp,
                                     uint8_t is_snapshot,
                                     struct FigBuffer *out);

struct FigMidsHandle *fig_mids_new(void);

void fig_mids_free(struct FigMidsHandle *handle);

int32_t fig_mids_apply_ticker(struct FigMidsHandle *handle, const uint8_t *payload, uintptr_t len);

int32_t fig_mids_apply_batch(struct FigMidsHandle *handle, const uint8_t *payload, uintptr_t len);

double fig_mids_mid(const struct FigMidsHandle *handle, const char *symbol);

struct FigBboHandle *fig_bbo_new(void);

void fig_bbo_free(struct FigBboHandle *handle);

int32_t fig_bbo_apply(struct FigBboHandle *handle, const uint8_t *payload, uintptr_t len);

double fig_bbo_implied_mid(const struct FigBboHandle *handle);

double fig_bbo_best_bid(const struct FigBboHandle *handle);

double fig_bbo_best_ask(const struct FigBboHandle *handle);

struct FigTradeTapeHandle *fig_trade_tape_new(uintptr_t capacity);

void fig_trade_tape_free(struct FigTradeTapeHandle *handle);

int32_t fig_trade_tape_push(struct FigTradeTapeHandle *handle,
                            const uint8_t *payload,
                            uintptr_t len);

uintptr_t fig_trade_tape_len(const struct FigTradeTapeHandle *handle);

double fig_trade_tape_latest_price(const struct FigTradeTapeHandle *handle);

struct FigMarkPriceHandle *fig_mark_price_new(void);

void fig_mark_price_free(struct FigMarkPriceHandle *handle);

int32_t fig_mark_price_apply(struct FigMarkPriceHandle *handle,
                             const uint8_t *payload,
                             uintptr_t len);

double fig_mark_price_get(const struct FigMarkPriceHandle *handle, const char *symbol);

struct FigOrdersHandle *fig_orders_new(uintptr_t exec_capacity);

void fig_orders_free(struct FigOrdersHandle *handle);

int32_t fig_orders_apply_snapshot(struct FigOrdersHandle *handle,
                                  const uint8_t *payload,
                                  uintptr_t len);

int32_t fig_orders_apply_execution(struct FigOrdersHandle *handle,
                                   const uint8_t *payload,
                                   uintptr_t len);

uintptr_t fig_orders_open_count(const struct FigOrdersHandle *handle);

uintptr_t fig_orders_execution_count(const struct FigOrdersHandle *handle);

struct FigAggTradesHandle *fig_agg_trades_new(uintptr_t capacity);

void fig_agg_trades_free(struct FigAggTradesHandle *handle);

int32_t fig_agg_trades_apply(struct FigAggTradesHandle *handle,
                             const uint8_t *payload,
                             uintptr_t len);

uintptr_t fig_agg_trades_len(const struct FigAggTradesHandle *handle);

double fig_agg_trades_latest_price(const struct FigAggTradesHandle *handle);

struct FigFundingHandle *fig_funding_new(uintptr_t capacity);

void fig_funding_free(struct FigFundingHandle *handle);

int32_t fig_funding_apply(struct FigFundingHandle *handle, const uint8_t *payload, uintptr_t len);

uintptr_t fig_funding_len(const struct FigFundingHandle *handle);

double fig_funding_latest_amount(const struct FigFundingHandle *handle);

struct FigLedgerHandle *fig_ledger_new(uintptr_t capacity);

void fig_ledger_free(struct FigLedgerHandle *handle);

int32_t fig_ledger_apply(struct FigLedgerHandle *handle, const uint8_t *payload, uintptr_t len);

uintptr_t fig_ledger_len(const struct FigLedgerHandle *handle);

struct FigLiquidationHandle *fig_liquidation_new(uintptr_t capacity);

void fig_liquidation_free(struct FigLiquidationHandle *handle);

int32_t fig_liquidation_apply_user(struct FigLiquidationHandle *handle,
                                   const uint8_t *payload,
                                   uintptr_t len);

int32_t fig_liquidation_apply_public(struct FigLiquidationHandle *handle,
                                     const uint8_t *payload,
                                     uintptr_t len);

uintptr_t fig_liquidation_user_count(const struct FigLiquidationHandle *handle);

/**
 * Extract payload bytes from an encoded FIG frame.
 */
int32_t fig_frame_payload(const uint8_t *frame_bytes,
                          uintptr_t frame_len,
                          struct FigBuffer *out,
                          uint8_t *out_frame_type,
                          uint8_t *out_schema_id);

/**
 * Return 1 when frame is STREAM_ITEM, else 0. Returns -1 on decode error.
 */
int32_t fig_frame_is_stream_item(const uint8_t *frame_bytes, uintptr_t frame_len);

/**
 * Decode ExecutionReport CBOR payload; write cl_ord_id to newly allocated C string.
 */
int32_t fig_cbor_decode_execution_report_cl_ord_id(const uint8_t *data,
                                                   uintptr_t data_len,
                                                   char **out_cl_ord_id);

/**
 * Decode SymbolTicker CBOR; returns last price in `out_price` (0 on success).
 */
int32_t fig_cbor_decode_symbol_ticker_price(const uint8_t *data,
                                            uintptr_t data_len,
                                            double *out_price);

/**
 * Decode BalanceSnapshot CBOR; returns account id via allocated C string.
 */
int32_t fig_cbor_decode_balance_snapshot_account(const uint8_t *data,
                                                 uintptr_t data_len,
                                                 char **out_account);

/**
 * Decode MarketDataSnapshot CBOR; returns symbol via allocated C string.
 */
int32_t fig_cbor_decode_market_data_snapshot_symbol(const uint8_t *data,
                                                    uintptr_t data_len,
                                                    char **out_symbol);

#endif  /* FIG_FFI_H */
