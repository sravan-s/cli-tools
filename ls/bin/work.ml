type info_type = string * Unix.stats

let print_result (args: Args.args_obj) (li:info_type list) =
  let show_size = List.mem Args.Add_Size args.format in
  let show_inode = List.mem Args.Inode args.format in
  let show_long_listing = List.mem Args.LongListing args.format in
  
  let inode_header = if (show_inode || show_long_listing) then Printf.sprintf "%8s " "inode" else "" in
  let st_mtime_header = if show_long_listing then Printf.sprintf "%32s " "st_mtime" else "" in
  let size_header = if (show_size || show_long_listing) then Printf.sprintf "%8s " "size" else "" in
  let name_header = if (show_inode || show_size || show_long_listing) then Printf.sprintf "%s" "name" else "" in
  
  Printf.printf "%s%s%s%s\n" inode_header st_mtime_header size_header name_header;
  
  let inode_val inode = if (show_inode || show_long_listing) then Printf.sprintf "%8i " inode else "" in
  let st_mtime_val modified = if show_long_listing then Printf.sprintf "%32f " modified else "" in
  let size_val size = if (show_size || show_long_listing) then Printf.sprintf "%8i " size else "" in
  let name_val name = Printf.sprintf "%s" name in
  List.iter (fun (name, stat) ->
    Printf.printf "%s%s%s%s\n" (inode_val stat.Unix.st_ino) (st_mtime_val stat.st_mtime) (size_val stat.st_size) (name_val name)
  ) li

let list_directories (args: Args.args_obj) =
  let dir_handle = Unix.opendir args.directory in
  let dir_list: info_type list ref = ref [] in
  let show_all = args.filter == Args.All_Filter_Args in
  let rec loop_dir () =
    try
      let entry_name = Unix.readdir dir_handle in
      (* fix bug here *)
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

  print_result args !dir_list;
  Unix.closedir dir_handle
