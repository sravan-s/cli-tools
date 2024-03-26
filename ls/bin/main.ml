let parse_args args =
  let without_exe_path = List.tl @@ Array.to_list @@ args in
  let arg_record = Args.create_params without_exe_path in
  (* uncomment to see arg_record *)
  (* ignore(print_endline (Args.show_args_obj arg_record)); *)
  match without_exe_path with
  | ["--help"] -> print_string Meta.help
  | ["--version"] -> print_string Meta.version
  | _ -> Work.list_directories arg_record

let _ = Sys.argv
  |> parse_args
