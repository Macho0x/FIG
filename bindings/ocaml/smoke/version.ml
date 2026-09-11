(* fig_version round-trip — no live FIG server. *)
open Ctypes
open Foreign

let fig_version = foreign "fig_version" (void @-> returning string)

let () =
  let v = fig_version () in
  if String.length v = 0 then failwith "empty fig_version";
  print_endline ("fig-ocaml smoke OK " ^ v)
