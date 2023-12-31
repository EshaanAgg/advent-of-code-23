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

let get_ball s =
  let space_index = String.index s ' ' in
  let ball_type = String.sub s (space_index + 1) (String.length s - space_index - 1) in
  let ball_value = int_of_string (String.sub s 0 space_index) in
  match ball_type with
    | "red" -> Red ball_value
    | "blue" -> Blue ball_value
    | "green" -> Green ball_value
    | _ -> failwith "Invalid ball type"

let get_set s = 
  let rec aux s init_set = 
    let len = String.length s in
    let comma_index = match (String.index s ',' ) with
      | exception Not_found -> len 
      | ind -> ind
    in
    let ball = get_ball (String.sub s 0 comma_index) in
    let upd_ball = match ball with
      | Red i -> {init_set with red = max init_set.red i}
      | Blue i -> {init_set with blue = max init_set.blue i}
      | Green i -> {init_set with green = max init_set.green i}
    in 
    let next_ind = comma_index + 2 in
    if next_ind >= len 
      then upd_ball
      else aux (String.sub s next_ind (len - next_ind)) upd_ball 
    in
  aux s {red = 0; green = 0; blue = 0}

let get_min_set s = 
  let rec aux s curr_set =
    let len = String.length s in
    let split_index = match (String.index s ';') with 
      | exception Not_found -> len
      | ind -> ind
    in 
    let next_set = get_set (String.sub s 0 split_index) in
    let upd_set = {
      red = max curr_set.red next_set.red;
      green = max curr_set.green next_set.green;
      blue = max curr_set.blue next_set.blue;
    } in
    let next_ind = split_index + 2 in
    if next_ind >= len 
      then upd_set
      else aux (String.sub s next_ind (len - next_ind)) upd_set
    in 
  aux s {red = 0; green = 0; blue = 0}

let max_allowed_set = {
    red = 12;
    green = 13;
    blue = 14;
}

let is_allowed_set set = 
    set.red <= max_allowed_set.red &&
    set.green <= max_allowed_set.green &&
    set.blue <= max_allowed_set.blue
let get_game_id line = 
  let game_id_str = String.sub line 5 ((String.index line ':') - 5) in
  int_of_string game_id_str
    
let rec solve_part_1 channel curr_ans = 
  match input_line channel with
  | line -> 
    let game_id = get_game_id line in
    let ind = (String.index line ':') + 2 in
    let encompassing_set = get_min_set (String.sub line ind (String.length line - ind)) in
    let new_ans = if is_allowed_set encompassing_set then curr_ans + game_id else curr_ans in
    solve_part_1 channel new_ans
  | exception End_of_file -> curr_ans
        
let () =
  let channel = open_in testFile in
  let ans = solve_part_1 channel 0 in
  print_endline (string_of_int ans);
  close_in channel