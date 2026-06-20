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
 * Returns an owned version string (caller must free with `fig_buffer_free` on the inner bytes
 * via copying — use `fig_version` pointer only, it is static).
 */
const char *fig_version(void);

/**
 * Encode a minimal REQUEST frame. Returns 0 on success.
 *
 * # Safety
 *
 * `out` must be a valid pointer. `payload`/`payload_len` must describe a valid buffer when non-null.
 */
int32_t fig_frame_encode_request(uint16_t channel_id,
                                 uint32_t stream_seq,
                                 uint8_t schema_id,
                                 const uint8_t *payload,
                                 uintptr_t payload_len,
                                 struct FigBuffer *out);

/**
 * Decode frame header fields from encoded bytes. Returns 0 on success.
 *
 * # Safety
 *
 * Pointers must be valid for reads/writes of the described lengths.
 */
int32_t fig_frame_decode_header(const uint8_t *data,
                                uintptr_t data_len,
                                uint16_t *out_channel_id,
                                uint32_t *out_stream_seq,
                                uint8_t *out_schema_id,
                                uint8_t *out_frame_type);

/**
 * Encode a canonical limit-order NewOrderSingle as CBOR.
 *
 * # Safety
 *
 * String pointers must be valid NUL-terminated UTF-8; `out` must be non-null.
 */
int32_t fig_cbor_encode_new_order_single(const char *cl_ord_id,
                                         const char *symbol,
                                         uint8_t side_buy,
                                         double order_qty,
                                         double price,
                                         struct FigBuffer *out);

/**
 * Decode CBOR bytes into cl_ord_id (caller frees with `fig_string_free`).
 *
 * # Safety
 *
 * `data` must point to `data_len` readable bytes; `out_cl_ord_id` must be non-null.
 */
int32_t fig_cbor_decode_new_order_single_cl_ord_id(const uint8_t *data,
                                                   uintptr_t data_len,
                                                   char **out_cl_ord_id);

/**
 * Free a C string returned by this library.
 *
 * # Safety
 *
 * `s` must be a pointer previously returned by this crate and not yet freed.
 */
void fig_string_free(char *s);

uint64_t fig_client_channel_stream_id(uint16_t channel_id, uint8_t is_server);

#endif  /* FIG_FFI_H */
