// https://adventofcode.com/2023/day/4
include Import;

open Containers;

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

module IntSet = Set.Make(Int);
module IntMap = Map.Make(Int);

module Card = {
  [@deriving (compare, sexp, show)]
  type t = {
    id: int,
    winners: list(int),
    picks: list(int),
  };

  let wins = (card: t): list(int) => {
    IntSet.add_list(IntSet.empty, card.winners)
    |> IntSet.inter(IntSet.add_list(IntSet.empty, card.picks))
    |> IntSet.to_list;
  };

  let points = (matches: list(int)): int => {
    Int.pow(2, List.length(matches)) / 2;
  };

  let score = wins >> points;

  // for a winning card, the sequence of card ids to make a "copy" of
  let ids_for_win = (start: int, count: int, max: int): list(int) =>
    if (start >= max) {
      [];
    } else {
      List.range(start + 1, Int.min(start + count, max));
    };

  let all_wins = (cards: list(t)): int => {
    let max_id = List.length(cards);

    let (total, _) =
      cards
      |> List.fold_left(
           ((total, map), card) => {
             let card_wins = wins(card);
             let is_winner = List.length(card_wins) > 0;

             let new_copies_ids =
               if (is_winner) {
                 ids_for_win(card.id, List.length(card_wins), max_id);
               } else {
                 [];
               };

             let copies_of_card =
               map |> IntMap.find_opt(card.id) |> Option.get_or(~default=1);

             let next_map =
               new_copies_ids
               |> List.fold_left(
                    (m, id) => {
                      IntMap.update(
                        id,
                        v =>
                          Some(
                            Option.get_or(~default=1, v) + copies_of_card,
                          ),
                        m,
                      )
                    },
                    map,
                  );

             (total + copies_of_card, next_map);
           },
           (0, IntMap.empty),
         );

    total;
  };
};

module Parser = {
  let parse_card = (line: string): Card.t => {
    let (n, rest) =
      line
      |> String.chop_prefix(~pre="Card ")
      |> Option.get_exn_or("Invalid card")
      |> Utils.split_once(":");

    let (winners, picks) = rest |> String.trim |> Utils.split_once("|");

    let winners =
      winners
      |> String.split_on_char(' ')
      |> List.filter(s => !String.equal(s, ""))
      |> List.map(int_of_string);

    let picks =
      picks
      |> String.split_on_char(' ')
      |> List.filter(s => !String.equal(s, ""))
      |> List.map(int_of_string);

    {id: int_of_string(String.trim(n)), winners, picks};
  };

  let parse_cards = (input: string): list(Card.t) => {
    input |> String.trim |> String.lines |> List.map(parse_card);
  };
};

module M = {
  type t = list(Card.t);

  let title = "Day 4: Scratchcards";
  let input_filename = "day4.txt";

  let parse = (input: string): t => {
    Parser.parse_cards(input);
  };

  let part1 = (cards: t) => {
    cards
    |> List.map(Card.score)
    |> Utils.sum_int_list
    |> string_of_int
    |> print_endline;
  };

  let part2 = (cards: t) => {
    cards |> Card.all_wins |> string_of_int |> print_endline;
  };
};

include M;
include Day.Make(M);

let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

let%test_unit "parse" =
  [%test_eq: list(Card.t)](
    parse(
      "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    ),
    [
      {
        id: 1,
        winners: [41, 48, 83, 86, 17],
        picks: [83, 86, 6, 31, 17, 9, 48, 53],
      },
      {
        id: 2,
        winners: [13, 32, 20, 16, 61],
        picks: [61, 30, 68, 82, 17, 32, 24, 19],
      },
    ],
  );

let%test_unit "points" =
  [%test_eq: list(int)](
    [
      Card.points([]),
      Card.points([1]),
      Card.points([1, 2]),
      Card.points([1, 2, 3]),
      Card.points([1, 2, 3, 4]),
      Card.points([1, 2, 3, 4, 5]),
      Card.points([1, 2, 3, 4, 5, 6]),
      Card.points([1, 2, 3, 4, 5, 6, 7]),
      Card.points([1, 2, 3, 4, 5, 6, 7, 8]),
    ],
    [0, 1, 2, 4, 8, 16, 32, 64, 128],
  );

let%test_unit "score" =
  [%test_eq: list(int)](
    [
      Card.score({
        id: 1,
        winners: [41, 48, 83, 86, 17],
        picks: [83, 86, 6, 31, 17, 9, 48, 53],
      }),
      Card.score({
        id: 2,
        winners: [13, 32, 20, 16, 61],
        picks: [61, 30, 68, 82, 17, 32, 24, 19],
      }),
    ],
    [8, 2],
  );

let%test_unit "ids_for_win" =
  [%test_eq: list(list(int))](
    [
      Card.ids_for_win(10, 0, 10),
      Card.ids_for_win(0, 3, 10),
      Card.ids_for_win(7, 5, 10),
    ],
    [[], [1, 2, 3], [8, 9, 10]],
  );

let%expect_test "part1" = {
  part1(parse(example));
  [%expect {| 13 |}];
};

let%expect_test "part2" = {
  part2(parse(example));
  [%expect {| 30 |}];
};
