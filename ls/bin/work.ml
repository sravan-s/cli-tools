type entry_name = string
type info_type = entry_name * Unix.stats

let extract_extension filepath =
  match String.rindex_opt filepath '.' with
  | Some dot_index ->
      let extension_start = dot_index + 1 in
      let extension_length = String.length filepath - extension_start in
      if extension_length > 0 then
        String.sub filepath extension_start extension_length
      else
        ""  (* No extension if dot is at the end of the string or no dot found *)
  | None -> ""  (* No dot found in the filepath *)

let sort_by_time (stats1: Unix.stats) (stats2: Unix.stats) =
  int_of_float @@ stats1.st_mtime -. stats2.st_mtime

let sort_by_size (stats1: Unix.stats) (stats2: Unix.stats): int =
  stats1.st_size - stats2.st_size

let sort_by_extention (entry_name1: string) (entry_name2: string): int =
  let extention_1 = extract_extension entry_name1 in
  let extention_2 = extract_extension entry_name2 in
  String.compare extention_1 extention_2

let sort_by (args: Args.args_obj) (li:info_type list) =
  let sort = args.sort in
  ignore(print_endline (Args.show_args_obj args));
  List.sort (fun i j ->
    let (entry_name1, stats1) = i in
    let (entry_name2, stats2) = j in
    if (entry_name1 = ".") then
      -1
    else if (entry_name1 = "..") then begin
      if entry_name2 = "." then 1 else -1
    end else begin
      match sort with
      | Time -> sort_by_time stats1 stats2 
      | Size -> sort_by_size stats1 stats2
      | Extention -> sort_by_extention entry_name1 entry_name2
      | _ -> String.compare entry_name1 entry_name2
    end
  ) li

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
      if ((entry_name = "." || entry_name = "..") && not show_all) then
        loop_dir()
      else begin
        let info = Unix.stat (args.directory ^ "/" ^ entry_name) in
        dir_list := !dir_list @ [(entry_name, info)];
        loop_dir()
      end
    with
      | End_of_file -> ()
  in loop_dir();

  let sorted = sort_by args !dir_list in
  print_result args sorted;
  Unix.closedir dir_handle
