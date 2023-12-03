let testFile = "../data/3/input.txt"

type number = {
  start: int;
  length: int;
  value: int;
}

let is_digit c = 
  match c with
  | '0'..'9' -> true
  | _ -> false

let get_gears str =
  let rec aux str i l = 
    if i == (String.length str)
      then l
      else if str.[i] == '*'
        then aux str (i+1) (i::l)
        else aux str (i+1) l
  in aux str 0 []

let get_numbers str =
  let rec aux str i l nums =
    if i == (String.length str)
      then nums
      else if is_digit str.[i]
        then aux str (i+1) (l+1) nums
        else 
          if l == 0 
            then aux str (i+1) 0 nums
            else 
            aux str (i+1) 0 ({
              start = i-l;
              length = l;
              value = int_of_string (String.sub str (i-l) l)
            } :: nums)
    in aux str 0 0 []

let num_adjacent_to_pos_same_level i num = 
  num.start == i + 1 || (num.start + num.length) == i - 1

let num_adjacent_to_pos_diff_level i num = 
  num.start <= i + 1 && (num.start + num.length) >= i - 1

let get_power prev curr next i = 
  let curr_nums = List.filter (num_adjacent_to_pos_same_level i) (get_numbers curr) in
  let prev_nums = List.filter (num_adjacent_to_pos_diff_level i) (get_numbers prev) in
  let next_nums = List.filter (num_adjacent_to_pos_diff_level i) (get_numbers next) in
  let nums = curr_nums @ prev_nums @ next_nums in
  if List.length nums == 2 then 
    match nums with 
      | a :: b :: _ -> a.value * b.value
      | _ -> 0
    else 0

let rec solve lines sum = 
  match lines with
  | prev :: curr :: next :: tail -> 
    let dots = get_gears curr in
    let p_sum = List.fold_left (fun acc ele -> acc + (get_power prev curr next ele)) 0 dots in 
    solve (curr :: next :: tail) (sum + p_sum)
  | _ -> sum

let rec get_file channel l = 
  match input_line channel with
  | line -> get_file channel (("." ^ line ^ ".") :: l)
  | exception End_of_file -> l 

let () =
  let channel = open_in testFile in
  let contents = get_file channel [] in

  (* Update the grid by adding a box of . on all the sides *)
  let x = String.length (List.hd contents) in
  let contents = [String.make x '.'] @ contents @ [String.make x '.'] in
  let result = solve contents 0 in
  print_endline (string_of_int result)

