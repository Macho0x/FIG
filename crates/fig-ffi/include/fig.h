#ifndef FIG_FFI_H
#define FIG_FFI_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Opaque connected FIG client (one TREE connection).
 */
typedef struct FigClientHandle FigClientHandle;

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

#endif  /* FIG_FFI_H */
