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
  let bytes = Fig.sbe_encode_new_order_single "CONF-001" "AAPL" true 100.0 50.25 in
  if bytes = "" then failwith "empty SBE";
  let root = Sys.getenv "FIG_REPO_ROOT" in
  let expected = expected_hex (Filename.concat root "tests/conformance/vectors/v1.json") in
  let got = hex bytes in
  if got <> expected then
    failwith ("SBE hex mismatch expected=" ^ expected ^ " got=" ^ got);
  print_endline ("fig-ocaml smoke OK " ^ v)
