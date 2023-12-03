// https://adventofcode.com/2023/day/2
include Import;

open Containers;

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

module M = {
  [@deriving (compare, sexp, show)]
  type cubes = {
    red: int,
    green: int,
    blue: int,
  };

  [@deriving (compare, sexp, show)]
  type game = {
    id: int,
    rounds: list(cubes),
  };

  [@deriving (compare, sexp, show)]
  type t = list(game);

  let title = "Day 2: Cube Conundrum";
  let input_filename = "day2.txt";

  let parse_hand = (hand: string): (string, int) => {
    let (count, color) = hand |> String.trim |> Utils.split_once(" ");
    (color, int_of_string(count));
  };

  let parse_round = (round: string): cubes => {
    round
    |> String.split_on_char(',')
    |> List.map(parse_hand)
    |> List.fold_left(
         (acc, (color, count)) => {
           switch (color) {
           | "red" => {...acc, red: count}
           | "blue" => {...acc, blue: count}
           | "green" => {...acc, green: count}
           | _ => acc
           }
         },
         {red: 0, green: 0, blue: 0},
       );
  };

  let parse_game = (input: string): game => {
    let (id, rounds_str) =
      input
      |> String.chop_prefix(~pre="Game ")
      |> Option.get_exn_or("Invalid game, could not strip Game prefix")
      |> Utils.split_once(":");

    let rounds =
      rounds_str
      |> String.trim
      |> String.split_on_char(';')
      |> List.map(parse_round);

    {id: int_of_string(id), rounds};
  };

  let game_is_possible = (constraints: cubes, game: game): bool => {
    List.for_all(
      round => {
        round.red <= constraints.red
        && round.green <= constraints.green
        && round.blue <= constraints.blue
      },
      game.rounds,
    );
  };

  let min_cubes_required = (game: game): cubes => {
    List.fold_left(
      (acc, round) => {
        {
          red: max(acc.red, round.red),
          blue: max(acc.blue, round.blue),
          green: max(acc.green, round.green),
        }
      },
      {red: 0, blue: 0, green: 0},
      game.rounds,
    );
  };

  let cube_power = (cubes: cubes): int => {
    cubes.red * cubes.green * cubes.blue;
  };

  let parse = (input: string): t => {
    input |> String.lines |> List.map(parse_game);
  };

  let part1 = (games: t) => {
    let constraints = {red: 12, green: 13, blue: 14};

    games
    |> List.filter(game_is_possible(constraints))
    |> List.map(game => game.id)
    |> Utils.sum_int_list
    |> string_of_int
    |> print_endline;
  };

  let part2 = (games: t) => {
    games
    |> List.map(min_cubes_required >> cube_power)
    |> Utils.sum_int_list
    |> string_of_int
    |> print_endline;
  };
};

include M;
include Day.Make(M);

let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

let%test_unit "parse" =
  [%test_eq: t](
    parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
    [
      {
        id: 1,
        rounds: [
          {red: 4, green: 0, blue: 3},
          {red: 1, green: 2, blue: 6},
          {red: 0, green: 2, blue: 0},
        ],
      },
    ],
  );

let%test_unit "game_is_possible true" =
  [%test_eq: bool](
    game_is_possible(
      {red: 3, blue: 3, green: 3},
      {
        id: 1,
        rounds: [{red: 2, blue: 3, green: 1}, {red: 1, blue: 0, green: 3}],
      },
    ),
    true,
  );

let%test_unit "game_is_possible false" =
  [%test_eq: bool](
    game_is_possible(
      {red: 3, blue: 3, green: 3},
      {
        id: 1,
        rounds: [{red: 2, blue: 3, green: 4}, {red: 1, blue: 0, green: 3}],
      },
    ),
    false,
  );

let%test_unit "min_cubes_required" =
  [%test_eq: cubes](
    min_cubes_required({
      id: 1,
      rounds: [{red: 2, blue: 3, green: 4}, {red: 1, blue: 0, green: 3}],
    }),
    {red: 2, blue: 3, green: 4},
  );

let%test_unit "cube_power" =
  [%test_eq: int](cube_power({red: 2, blue: 3, green: 4}), 24);

let%expect_test "part1" = {
  part1(parse(example));
  [%expect {| 8 |}];
};

let%expect_test "part2" = {
  part2(parse(example));
  [%expect {| 2286 |}];
};
