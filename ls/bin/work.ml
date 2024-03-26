type info_type = string * Unix.stats

let print_result (_li:info_type list) (_args: Args.args_obj) =
  print_endline "to implement" 

let format_long (i: info_type) =
  let (name, info) = i in
  Printf.printf "%8i %32f %8i %s\n" info.st_ino info.st_mtime info.st_size name
let format_long_header =
  Printf.printf "%8s %32s %8s %s\n" "inode" "st_mtime" "size" "name"

let list_directories (args: Args.args_obj) =
  let dir_handle = Unix.opendir args.directory in
  let dir_list: info_type list ref = ref [] in
  let show_all = args.filter == Args.All_Filter_Args in
  let rec loop_dir () =
    try
      let entry_name = Unix.readdir dir_handle in
      if ((entry_name == "." || entry_name == "..") && not show_all) then
        loop_dir()
      else
        let info = Unix.stat (args.directory ^ "/" ^ entry_name) in
        dir_list := !dir_list @ [(entry_name, info)];
        loop_dir();
    with
      | End_of_file -> ()
  in
  loop_dir();

  format_long_header;
  List.iter format_long !dir_list;
  Unix.closedir dir_handle
