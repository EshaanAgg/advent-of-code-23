let testFile = "../data/3/input.txt"

type number = {
  start: int;
  length: int;
  value: int;
}

let is_dot c = 
  match c with
  | '.' -> true
  | _ -> false

let is_digit c = 
  match c with
  | '0'..'9' -> true
  | _ -> false

let is_symbol c =  not (is_dot c) && not (is_digit c) 

let has_symbol str = 
  let rec aux str i = 
    if i == (String.length str)
      then false
      else if is_symbol str.[i]
        then true
        else aux str (i+1)
  in aux str 0

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

let get_sum numbers =
  let rec aux numbers sum = 
    match numbers with
    | [] -> sum
    | number :: numbers -> 
      aux numbers (sum + number.value)
  in aux numbers 0
  
let check_num prev curr next num = 
  has_symbol (String.sub prev (num.start - 1) (num.length + 2)) ||
  has_symbol (String.sub next (num.start - 1) (num.length + 2)) ||
  has_symbol (String.sub curr (num.start - 1) (num.length + 2))

let rec solve lines sum = 
  match lines with
  | prev :: curr :: next :: tail -> 
    let numbers = get_numbers curr in
    let filter_func = check_num prev curr next in
    let filtered_numbers = List.filter filter_func numbers in
    solve (curr :: next :: tail) (sum + get_sum filtered_numbers)
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

