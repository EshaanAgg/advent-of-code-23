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
  match String.split_on_char ' ' s with
  | count :: typ :: _ -> 
    let ball_value = int_of_string count in
    begin match typ with
      | "red" -> Red ball_value
      | "blue" -> Blue ball_value
      | "green" -> Green ball_value
      | _ -> failwith "Invalid ball type"
    end
  | _ -> failwith "Invalid ball"

let get_set s =
  let rec aux l init_set = 
    match l with
    | [] -> init_set
    | h :: t -> 
      let ball = get_ball (String.trim h) in
      let upd_ball = match ball with
        | Red i -> {init_set with red = max init_set.red i}
        | Blue i -> {init_set with blue = max init_set.blue i}
        | Green i -> {init_set with green = max init_set.green i}
      in
      aux t upd_ball
    in
  aux (String.split_on_char ',' s) {red = 0; green = 0; blue = 0}

let get_min_set s = 
  let rec aux l curr_set = 
    match l with 
    | [] -> curr_set
    | h :: t -> aux t {
      red = max h.red curr_set.red;
      green = max h.green curr_set.green;
      blue = max h.blue curr_set.blue;
    }
  in 
  let l = List.map (fun t -> get_set (String.trim t)) (String.split_on_char ';' s) in 
  aux l {red = 0; green = 0; blue = 0}
    
let rec solve_part_1 channel curr_ans = 
  match input_line channel with
  | line -> 
    let ind = (String.index line ':') + 2 in
    let encompassing_set = get_min_set (String.sub line ind (String.length line - ind)) in
    solve_part_1 channel (curr_ans + (encompassing_set.red * encompassing_set.green * encompassing_set.blue))
  | exception End_of_file -> curr_ans
        
let () =
  let channel = open_in testFile in
  let ans = solve_part_1 channel 0 in
  print_endline (string_of_int ans);
  close_in channel