let testFile = "../data/1/input.txt"
let is_digit c = 
  match c with
  | '0'..'9' -> true
  | _ -> false

let parse_int_from_char c = int_of_char c - int_of_char '0'

let find_first_digit str = 
  let rec find_first_digit str curr =  
    if is_digit str.[curr] 
      then str.[curr]
      else find_first_digit str (curr+1)
  in find_first_digit str 0

  let find_last_digit str = 
  let rec find_last_digit str curr =  
    if is_digit str.[curr] 
      then str.[curr] 
      else find_last_digit str (curr - 1)
  in find_last_digit str (String.length str - 1)

(* 
  A recursive function which reads in a file at a time from the channel 
  and then calls itself again to process the next line 
  args:
    channel: used to read the file
    value: the current final result that we have obtained from the already processed lines
*)
let rec solve_part_1 channel curr_ans = 
  match input_line channel with
  | line -> 
    let first_digit = parse_int_from_char (find_first_digit line) in
    let last_digit = parse_int_from_char (find_last_digit line) in

    print_endline (Format.sprintf "line: %s, first_digit: %d, last_digit: %d" line first_digit last_digit);

    (* Call the function recursively *)
    solve_part_1 channel curr_ans + 10 * first_digit + last_digit;
  | exception End_of_file -> curr_ans

let () =
  let channel = open_in testFile in
  let ans = solve_part_1 channel 0 in
  print_endline (string_of_int ans);
  close_in channel