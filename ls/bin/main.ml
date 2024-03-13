(* open Meta *)

let parse_args args =
  let without_exe_path = List.tl @@ Array.to_list @@ args in
  match without_exe_path with
  | [] -> print_string "implementing~"
  | ["--help"] -> print_string Meta.help
  | ["--version"] -> print_string Meta.version
  | _ -> print_string "unknown parameter"

let _ = Sys.argv
  |> parse_args
