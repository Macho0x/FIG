(* Thin C-stub wrapper over fig.h — link libfig_ffi.a (no ctypes-foreign). *)

type client
type subscription

external version : unit -> string = "caml_fig_version"

external sbe_encode_new_order_single :
  string -> string -> bool -> float -> float -> string
  = "caml_fig_sbe_encode_new_order_single"

external jwt_encode : string -> int64 -> string -> string = "caml_fig_jwt_encode"
external jwt_verify_bearer : string -> string -> unit = "caml_fig_jwt_verify_bearer"

external encode_subscribe_auth :
  int -> int -> string -> string -> string -> string
  = "caml_fig_frame_encode_subscribe_auth"

external connect : string -> string -> client = "caml_fig_client_connect"
external close : client -> unit = "caml_fig_client_close"
external ping : client -> unit = "caml_fig_client_ping"
external request_and_recv : client -> string -> string array
  = "caml_fig_client_request_and_recv"

(* SUBSCRIBE snapshot plus a live handle. Do not use request_and_recv. *)
external subscribe : client -> string -> string array * subscription
  = "caml_fig_client_subscribe"

external sub_next : subscription -> int -> string option = "caml_fig_client_sub_next"
external sub_close : subscription -> unit = "caml_fig_client_sub_close"

(* Empty symbol omits the filter. side 0 omits the side filter.
   Wire values: 1 buy, 2 sell, 3 sell short, 4 sell short exempt. *)
external cbor_encode_mass_cancel_request : string -> string -> string -> int -> string
  = "caml_fig_cbor_encode_mass_cancel_request"

external cbor_decode_mass_cancel_request : string -> string * string * string option * int
  = "caml_fig_cbor_decode_mass_cancel_request"

external cbor_decode_trading_session_status : string -> int * string
  = "caml_fig_cbor_decode_trading_session_status"

external cbor_decode_security_status : string -> string * int
  = "caml_fig_cbor_decode_security_status"

external cbor_decode_mass_cancel_report : string -> string * int * int
  = "caml_fig_cbor_decode_mass_cancel_report"

external request_session_status : client -> int -> int * string
  = "caml_fig_client_request_session_status"

external request_security_status : client -> string -> int -> string * int
  = "caml_fig_client_request_security_status"

external mass_cancel :
  client -> string -> string -> string -> int -> int -> string * int * int
  = "caml_fig_client_mass_cancel_byte" "caml_fig_client_mass_cancel"
