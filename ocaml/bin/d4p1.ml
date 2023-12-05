let testFile = "../data/4/input.txt"

type card = {
  winners: int list;
  tickets: int list;
}

let get_tickets str =
  let lis = String.split_on_char ' ' str in
  let filtered_list = List.filter (fun x -> (String.trim x) <> "") lis in 
  List.map int_of_string filtered_list

let get_card line = 
  let ind_colon = (String.index line ':') in
  let ind_divider = (String.index line '|') in
  {
    winners = get_tickets (String.sub line (ind_colon + 2) (ind_divider - 3 - ind_colon));
    tickets = get_tickets (String.sub line (ind_divider + 2) ((String.length line) - ind_divider - 2));
  }

let rec get_all_cards channel curr =
  match input_line channel with
  | line -> get_all_cards channel (curr @ [get_card line])
  | exception End_of_file -> curr

let contains list ele =
  List.exists (fun x -> x = ele) list

let power x = 
  let rec aux x acc =
    if x = 0 then acc
    else aux (x-1) (acc*2)
  in
  aux x 1

let get_pow card =
  let count = List.length (List.filter (fun x -> contains card.winners x) card.tickets) in 
  match count with
  | 0 -> 0
  | x -> power (x-1)
  
let rec solve cards sum =
  match cards with 
  | [] -> sum
  | card :: tail ->
      solve tail (sum + (get_pow card))

let () =
  let channel = open_in testFile in
  let cards = get_all_cards channel [] in
  print_endline (string_of_int (solve cards 0));
  close_in channel