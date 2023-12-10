let testFile = "../data/9/input.txt"

let get_diff_arr list =
  let rec aux list col =
    match list with
    | x::y::xs -> aux (y::xs) (col @ [y-x])
    | _ -> col
  in 
  aux list []

let rec get_prev list = 
  if List.for_all (fun x -> x == 0) list then 0
  else 
    let diff_list = get_diff_arr list in
    let head = List.hd list in
    head - (get_prev diff_list)

let process_line l =
  let elements = List.map int_of_string (String.split_on_char ' ' l) in
  get_prev elements

let rec solve channel cur_sum = 
  match input_line channel with
  | line -> solve channel (cur_sum + (process_line line))
  | exception End_of_file -> cur_sum

let () =
  let channel = open_in testFile in
  let ans = solve channel 0 in
  print_endline (string_of_int ans);
  close_in channel