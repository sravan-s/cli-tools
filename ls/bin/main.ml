let list_directories directory =
  let dir_handle = Unix.opendir directory in
  let rec loop_dir () =
    try
      let entry_name = Unix.readdir dir_handle in
      if entry_name == "." || entry_name == ".." then
        (* Process the entry here *)
        print_endline entry_name
      else
        ignore (print_endline entry_name);
        loop_dir();
    with
      | End_of_file -> ()
  in
  loop_dir();
  Unix.closedir dir_handle

let parse_args args =
  let without_exe_path = List.tl @@ Array.to_list @@ args in
  let _args = Args.create_params without_exe_path in
  match without_exe_path with
  | [] -> list_directories "./"
  | ["--help"] -> print_string Meta.help
  | ["--version"] -> print_string Meta.version
  | _ -> list_directories "./"

let _ = Sys.argv
  |> parse_args
