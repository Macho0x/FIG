#ifndef FIG_FFI_H
#define FIG_FFI_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Opaque owned byte buffer returned to callers.
 */
typedef struct FigBuffer {
  uint8_t *data;
  uintptr_t len;
} FigBuffer;

/**
 * Free a buffer previously returned by fig_* functions.
 */
void fig_buffer_free(struct FigBuffer buf);

/**
 * Returns static version string.
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
                                         struct FigBuffer *out);

int32_t fig_cbor_decode_new_order_single_cl_ord_id(const uint8_t *data,
                                                   uintptr_t data_len,
                                                   char **out_cl_ord_id);

int32_t fig_cbor_encode_capabilities(struct FigBuffer *out);

int32_t fig_cbor_encode_open_orders_snapshot(struct FigBuffer *out);

int32_t fig_cbor_encode_market_data_snapshot(struct FigBuffer *out);

int32_t fig_cbor_encode_order_history_request(struct FigBuffer *out);

void fig_string_free(char *s);

uint64_t fig_client_channel_stream_id(uint16_t channel_id, uint8_t is_server);

int32_t fig_cbor_encode_open_orders_request(struct FigBuffer *out);

#endif  /* FIG_FFI_H */
