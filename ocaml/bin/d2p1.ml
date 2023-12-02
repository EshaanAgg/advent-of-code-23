let testFile = "../data/2/input.txt"

type set = {
    red: int;
    green: int;
    blue: int;
}

type ball = 
    | Red of int
    | Blue of int
    | Green of int

let max_allowed_set = {
    red = 12;
    green = 13;
    blue = 14;
}

let rec solve_part_1 channel curr_ans = 
  match input_line channel with
  | line -> 
    print_endline line;
    solve_part_1 channel (curr_ans + 1)
  | exception End_of_file -> curr_ans

let () =
  let channel = open_in testFile in
  let ans = solve_part_1 channel 0 in
  print_endline (string_of_int ans);
  close_in channel