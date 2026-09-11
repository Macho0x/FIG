(* fig_version round-trip — no live FIG server, no ctypes-foreign. *)
external fig_version : unit -> string = "caml_fig_version"

let () =
  let v = fig_version () in
  if String.length v = 0 then failwith "empty fig_version";
  print_endline ("fig-ocaml smoke OK " ^ v)
