type filter_args =
  | Default_Filter_Args
  | All_Filter_Args
  [@@deriving show]

type format =
  | Default_Format
  | Inode
  | Add_Size
  | LongListing
  [@@deriving show]

type sort =
  | Sort_None
  | Time
  | Size
  | Extention
  [@@deriving show]

type args_obj = {
  mutable filter: filter_args;
  mutable format: format list;
  mutable sort: sort;
  mutable recursive: bool;
  mutable directory: string;
} [@@deriving show]

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

type accum_type = args_obj * int * int

let folder (a: accum_type) (i: string): accum_type =
  let (accum, idx, li_len) = a in
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
  | "--sort=NONE"
  | "--sort=none"-> { accum with sort = Sort_None }
  | "--sort=SIZE"
  | "--sort=size"-> { accum with sort = Size }
  | "--sort=TIME"
  | "--sort=time"-> { accum with sort = Time }
  | "--sort=EXTENSION"
  | "--sort=extension"-> { accum with sort = Extention }
  | _ -> if idx + 1 == li_len then { accum with directory = i } else accum
  in (new_accum, idx + 1, li_len)

let create_params (li: string list): args_obj =
  (* ('acc -> 'a -> 'acc) -> 'acc -> 'a list -> 'acc *)
  let arg: args_obj = {
    filter = Default_Filter_Args;
    format = [Default_Format];
    sort = Sort_None;
    recursive = false;
    directory = "./";
  } in
  let list_length = List.length li in
  let accum: accum_type = (arg, 0, list_length) in
  let (a, _, _) = List.fold_left folder accum li in
  a
