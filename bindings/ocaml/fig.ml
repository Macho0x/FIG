(* Thin ctypes wrapper over fig.h — link libfig_ffi first. *)

open Ctypes

let lib =
  foreign "fig_version" (void @-> returning string)

let fig_version () = lib ()

let fig_buffer_free =
  foreign "fig_buffer_free"
    (struct FigBuffer [ data = ptr uint8_t; len = size_t ] @-> returning void)

let fig_client_connect =
  foreign "fig_client_connect"
    (string @-> string @-> ptr (ptr void) @-> returning int32_t)

let fig_client_close = foreign "fig_client_close" (ptr void @-> returning void)

let fig_client_ping = foreign "fig_client_ping" (ptr void @-> returning int32_t)

let fig_payload_compress =
  foreign "fig_payload_compress"
    (ptr uint8_t @-> size_t @-> ptr (struct FigBuffer [ data = ptr uint8_t; len = size_t ])
    @-> returning int32_t)

let fig_payload_decompress =
  foreign "fig_payload_decompress"
    (ptr uint8_t @-> size_t @-> ptr (struct FigBuffer [ data = ptr uint8_t; len = size_t ])
    @-> returning int32_t)

let fig_frame_encode_subscribe_auth =
  foreign "fig_frame_encode_subscribe_auth"
    (uint16_t @-> uint32_t @-> string @-> string @-> string
    @-> ptr (struct FigBuffer [ data = ptr uint8_t; len = size_t ])
    @-> returning int32_t)

let fig_jwt_encode =
  foreign "fig_jwt_encode"
    (string @-> uint64_t @-> string
    @-> ptr (struct FigBuffer [ data = ptr uint8_t; len = size_t ])
    @-> returning int32_t)

let fig_jwt_verify_bearer =
  foreign "fig_jwt_verify_bearer" (string @-> string @-> returning int32_t)

let fig_sbe_encode_new_order_single =
  foreign "fig_sbe_encode_new_order_single"
    (string @-> string @-> uint8_t @-> float @-> float
    @-> ptr (struct FigBuffer [ data = ptr uint8_t; len = size_t ])
    @-> returning int32_t)
