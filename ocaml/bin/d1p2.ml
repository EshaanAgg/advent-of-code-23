let testFile = "../data/1/input.txt"
let is_digit c = 
  match c with
  | '0'..'9' -> true
  | _ -> false

let parse_int_from_char c = int_of_char c - int_of_char '0'

let starts_with_digit str = 
  match str with
    | s when String.starts_with ~prefix:"zero" s-> 0
    | s when String.starts_with ~prefix:"one" s-> 1
    | s when String.starts_with ~prefix:"two" s-> 2
    | s when String.starts_with ~prefix:"three" s-> 3
    | s when String.starts_with ~prefix:"four" s-> 4
    | s when String.starts_with ~prefix:"five" s-> 5
    | s when String.starts_with ~prefix:"six" s-> 6
    | s when String.starts_with ~prefix:"seven" s-> 7
    | s when String.starts_with ~prefix:"eight" s-> 8
    | s when String.starts_with ~prefix:"nine" s-> 9
    | _ -> -1

let find_first_digit str = 
  let rec find_first_digit str curr =  
    if is_digit str.[curr] 
      then parse_int_from_char str.[curr]
      else 
        let len = (String.length str) - curr in
        let found_digit = starts_with_digit (String.sub str curr len) in
        if found_digit != -1 
          then found_digit
      else find_first_digit str (curr+1)
  in find_first_digit str 0

let find_last_digit str = 
  let rec find_last_digit str curr =  
    if is_digit str.[curr] 
      then parse_int_from_char str.[curr] 
      else 
        let len = (String.length str) - curr in
        let found_digit = starts_with_digit (String.sub str curr len) in
        if found_digit != -1 
          then found_digit
      else find_last_digit str (curr - 1)
  in find_last_digit str (String.length str - 1)

(* 
  A recursive function which reads in a file at a time from the channel 
  and then calls itself again to process the next line 
  args:
    channel: used to read the file
    value: the current final result that we have obtained from the already processed lines
*)
let rec solve_part_2 channel curr_ans = 
  match input_line channel with
  | line -> 
    let first_digit = find_first_digit line in
    let last_digit = find_last_digit line in

    print_endline (Format.sprintf "line: %s, first_digit: %d, last_digit: %d" line first_digit last_digit);

    (* Call the function recursively *)
    solve_part_2 channel curr_ans + 10 * first_digit + last_digit;
  | exception End_of_file -> curr_ans

let () =
  let channel = open_in testFile in
  let ans = solve_part_2 channel 0 in
  print_endline (string_of_int ans);
  close_in channel