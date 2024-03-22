type filter_args =
  | Default_Filter_Args
  | All_Filter_Args

type format =
  | Default_Format
  | Inode
  | LongListing
  | Add_Size

type sort =
  | Word
  | None
  | Time
  | Size
  | Extention

type args_obj = {
  mutable filter: filter_args;
  mutable format: format list;
  mutable sort: sort;
  mutable recursive: bool;
}

let rec exist elem lst =
  match lst with
  | [] -> false
  | hd::tl -> elem = hd || exist elem tl
  
(* To remove duplicates from list *)
let rec dupExist lst =
  match lst with
  | [] -> false
  | hd::tl -> (exist hd tl) || dupExist tl

let unique_strings lst =
  let compare_strings s1 s2 = String.compare s1 s2 in
  List.sort_uniq compare_strings lst  

let folder (accum: args_obj) (i: string): args_obj =
  let new_accum = match i with
  (* filters *)
  | "-a"
  | "--all" -> { accum with filter = All_Filter_Args }
  (* Formatters *)
  | "-i"
  | "-inode" -> (match accum.format with
    | [Default_Format] -> { accum with format = [Inode] }
    | _ -> { accum with format = (accum.format@[Inode]) })
  | "-l" -> (match accum.format with
    | [Default_Format] -> { accum with format = [LongListing] }
    | _ -> { accum with format = (accum.format@[LongListing]) })
  | "-s"
  | "--size" -> (match accum.format with
    | [Default_Format] -> { accum with format = [Add_Size] }
    | _ -> { accum with format = (accum.format@[Add_Size]) })
  (* recursive *)
  | "-R"
  | "--recursive" -> { accum with recursive = true }
  (* Sorting *)
  | "--sort=WORD"
  | "--sort=word"-> { accum with sort = Word }
  | "--sort=NONE"
  | "--sort=none"-> { accum with sort = None }
  | "--sort=SIZE"
  | "--sort=size"-> { accum with sort = Size }
  | "--sort=TIME"
  | "--sort=time"-> { accum with sort = Time }
  | "--sort=EXTENSION"
  | "--sort=extension"-> { accum with sort = Extention }
  | _ -> accum in
  new_accum

let create_params (li: string list): args_obj =
  (* ('acc -> 'a -> 'acc) -> 'acc -> 'a list -> 'acc *)
  let accum: args_obj = {
    filter = Default_Filter_Args;
    format = [Default_Format];
    sort = None;
    recursive = false;
  } in
  List.fold_left folder accum li
