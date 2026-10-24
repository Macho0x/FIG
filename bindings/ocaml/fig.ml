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

external jwt_decode_sub : string -> string -> string = "caml_fig_jwt_decode_sub"

external post_batch : client -> string -> string -> int -> string = "caml_fig_client_post_batch"

type order_book
type mids
type bbo
type trade_tape
type mark_price
type orders
type funding
type agg_trades
type ledger
type liquidation
type candle

external order_book_new : unit -> order_book = "caml_fig_order_book_new"
external order_book_apply_snapshot : order_book -> string -> unit = "caml_fig_order_book_apply_snapshot"
external order_book_apply_delta : order_book -> string -> unit = "caml_fig_order_book_apply_delta"
external order_book_best_bid : order_book -> float = "caml_fig_order_book_best_bid"

external mids_new : unit -> mids = "caml_fig_mids_new"
external mids_apply_ticker : mids -> string -> unit = "caml_fig_mids_apply_ticker"
external mids_apply_batch : mids -> string -> unit = "caml_fig_mids_apply_batch"
external mids_mid : mids -> string -> float = "caml_fig_mids_mid"

external bbo_new : unit -> bbo = "caml_fig_bbo_new"
external bbo_apply : bbo -> string -> unit = "caml_fig_bbo_apply"
external bbo_implied_mid : bbo -> float = "caml_fig_bbo_implied_mid"
external bbo_best_bid : bbo -> float = "caml_fig_bbo_best_bid"
external bbo_best_ask : bbo -> float = "caml_fig_bbo_best_ask"

external trade_tape_new : int -> trade_tape = "caml_fig_trade_tape_new"
external trade_tape_push : trade_tape -> string -> unit = "caml_fig_trade_tape_push"
external trade_tape_len : trade_tape -> int = "caml_fig_trade_tape_len"
external trade_tape_latest_price : trade_tape -> float = "caml_fig_trade_tape_latest_price"

external mark_price_new : unit -> mark_price = "caml_fig_mark_price_new"
external mark_price_apply : mark_price -> string -> unit = "caml_fig_mark_price_apply"
external mark_price_get : mark_price -> string -> float = "caml_fig_mark_price_get"

external orders_new : int -> orders = "caml_fig_orders_new"
external orders_apply_snapshot : orders -> string -> unit = "caml_fig_orders_apply_snapshot"
external orders_apply_execution : orders -> string -> unit = "caml_fig_orders_apply_execution"
external orders_open_count : orders -> int = "caml_fig_orders_open_count"
external orders_execution_count : orders -> int = "caml_fig_orders_execution_count"

external funding_new : int -> funding = "caml_fig_funding_new"
external funding_apply : funding -> string -> unit = "caml_fig_funding_apply"
external funding_len : funding -> int = "caml_fig_funding_len"
external funding_latest_amount : funding -> float = "caml_fig_funding_latest_amount"

external agg_trades_new : int -> agg_trades = "caml_fig_agg_trades_new"
external agg_trades_apply : agg_trades -> string -> unit = "caml_fig_agg_trades_apply"
external agg_trades_len : agg_trades -> int = "caml_fig_agg_trades_len"
external agg_trades_latest_price : agg_trades -> float = "caml_fig_agg_trades_latest_price"

external ledger_new : int -> ledger = "caml_fig_ledger_new"
external ledger_apply : ledger -> string -> unit = "caml_fig_ledger_apply"
external ledger_len : ledger -> int = "caml_fig_ledger_len"

external liquidation_new : int -> liquidation = "caml_fig_liquidation_new"
external liquidation_apply_user : liquidation -> string -> unit = "caml_fig_liquidation_apply_user"
external liquidation_apply_public : liquidation -> string -> unit = "caml_fig_liquidation_apply_public"
external liquidation_user_count : liquidation -> int = "caml_fig_liquidation_user_count"

external candle_new : unit -> candle = "caml_fig_candle_new"
external candle_apply : candle -> string -> unit = "caml_fig_candle_apply"
external candle_close : candle -> float = "caml_fig_candle_close"
external candle_is_final : candle -> int = "caml_fig_candle_is_final"
external candle_symbol : candle -> string = "caml_fig_candle_symbol"
external candle_interval : candle -> string = "caml_fig_candle_interval"
