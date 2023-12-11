module CharMap = Map.Make(Char)

let testFile = "../data/7/input.txt"

type card = {
  hand: string;
  bid: int;
  score: int;
}

let get_char_value c =
  match c with
  | 'A' -> 14
  | 'K' -> 13
  | 'Q' -> 12
  | 'J' -> 11
  | 'T' -> 10
  | _ -> int_of_char c - int_of_char '0'

let rec compare_hand a b i =
  match i with 
  | 5 -> 0
  | _ ->
    let score = get_char_value a.hand.[i] - get_char_value b.hand.[i] in
    if score != 0 
      then score 
      else compare_hand a b (i+1)

let calculate_score char_map = 
  let list = List.sort (fun (_,a) (_,b) -> compare a b) 
    (List.of_seq (CharMap.to_seq char_map)) in
    
  match list with 
  | [_] -> 7
  | [_; (_,4)] -> 6
  | [_; (_,3)] -> 5
  | [_; _; (_,3)] -> 4
  | [_; _; (_,2)] -> 3
  | [_; _; _; _] -> 2
  | _ -> 1
  
let scorer hand = 
  let map = CharMap.empty in
  let chars = String.to_seq hand in
  let map = Seq.fold_left (fun acc c -> 
      CharMap.add c (
        match CharMap.find_opt c acc with
          | Some v -> v + 1
          | None -> 1
        ) acc
    ) map chars in
  calculate_score map

(* gets a card from a passed in line *)
let get_card l =
  match String.split_on_char ' ' l with 
  | [hand; bid] -> 
    {
      hand = hand;
      bid = int_of_string bid;
      score = scorer hand;
    }
  | _ -> failwith "Invalid card"

(* reads all lines from a channel and returns a list of cards *)
let rec read_lines channel curr = 
  match input_line channel with
  | line -> read_lines channel (get_card line :: curr)
  | exception End_of_file -> curr

(* sorts cards in an ascending order based on the ranking rules *)
let sord_cards a b =
  let score = a.score - b.score in
  if score != 0 
    then score 
    else compare_hand a b 0

(* main function *)
let () =
  let channel = open_in testFile in
  let cards = read_lines channel [] in
  let cards = List.sort sord_cards cards in
  (* sorts the cards and then sums then by multiplying the bids by index *)
  let sum = 
    List.fold_left (fun acc c -> acc + c) 0 
      (List.mapi (fun i c -> (i+1)*c.bid) 
        cards) in
  print_endline (string_of_int sum);
  close_in channel