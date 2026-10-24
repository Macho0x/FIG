let find_sub s pat =
  let n = String.length s and m = String.length pat in
  let rec loop i =
    if i + m > n then raise Not_found
    else if String.sub s i m = pat then i
    else loop (i + 1)
  in
  loop 0

let hex s =
  let b = Buffer.create (String.length s * 2) in
  String.iter (fun c -> Printf.bprintf b "%02x" (Char.code c)) s;
  Buffer.contents b

let expected_hex path =
  let ic = open_in_bin path in
  let json = really_input_string ic (in_channel_length ic) in
  close_in ic;
  let i = find_sub json "\"id\": \"sbe.new_order_single.limit_buy\"" in
  let rest = String.sub json i (String.length json - i) in
  let k = find_sub rest "\"expected_hex\": \"" in
  let start = k + String.length "\"expected_hex\": \"" in
  let stop = String.index_from rest start '"' in
  String.sub rest start (stop - start)

let () =
  let v = Fig.version () in
  if v = "" then failwith "empty fig_version";
  let token = Fig.jwt_encode "alice" 4000000000L "fig-smoke-secret" in
  let sub = Fig.jwt_decode_sub token "fig-smoke-secret" in
  if sub <> "alice" then failwith "jwt sub";
  Fig.jwt_verify_bearer token "fig-smoke-secret";
  let check_candle () =
    let candle = Fig.candle_new () in
    if Fig.candle_is_final candle <> -1 then failwith "empty candle"
  in
  check_candle ();
  Gc.full_major ();
  let bytes = Fig.sbe_encode_new_order_single "CONF-001" "AAPL" true 100.0 50.25 in
  if bytes = "" then failwith "empty SBE";
  let root = Sys.getenv "FIG_REPO_ROOT" in
  let expected = expected_hex (Filename.concat root "tests/conformance/vectors/v1.json") in
  let got = hex bytes in
  if got <> expected then
    failwith ("SBE hex mismatch expected=" ^ expected ^ " got=" ^ got);
  let body = Fig.cbor_encode_mass_cancel_request "MC-001" "DEMO" "AAPL" 1 in
  let (cl, account, symbol, side) = Fig.cbor_decode_mass_cancel_request body in
  if cl <> "MC-001" || account <> "DEMO" || symbol <> Some "AAPL" || side <> 1 then
    failwith "mass-cancel CBOR round trip failed";
  print_endline ("fig-ocaml smoke OK " ^ v)
