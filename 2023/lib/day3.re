// https://adventofcode.com/2023/day/3
open Containers;

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

module Schematic = {
  [@deriving (compare, sexp, show)]
  type component =
    | Num(int, int, int)
    | Sym(char, int, int);

  // [@deriving (compare, sexp, show)]
  // type element = (component, int, int); // c, x, y

  [@deriving (compare, sexp, show)]
  type t = list(component);

  let is_raw_symbol = (c: char): bool => {
    switch (c) {
    | '.' => false
    | '='
    | '@'
    | '!' .. '/' => true
    | _ => false
    };
  };

  let parse_line = (y: int, line: string): t => {
    let (els, _, _) =
      line
      // appending "." makes sure parsing handles numbers that appear at the end of the line
      ++ "."
      |> String.to_list
      |> List.fold_left(
           ((els, x, n), c) => {
             switch (c) {
             // when we find a digit, append it to the current number
             // TODO: handle case where number is last in row
             | '0' .. '9' => (els, x + 1, n ++ Char.escaped(c))

             // when we find a '.'
             | '.' =>
               if (String.is_empty(n)) {
                 (
                   // if empty previous number, continue
                   els,
                   x + 1,
                   n,
                 );
               } else {
                 (
                   // if non-empty previous number, add it to els
                   [
                     Num(int_of_string(n), x - String.length(n), y),
                     ...els,
                   ],
                   x + 1,
                   "",
                 );
               }
             // when we find a symbol
             | c when is_raw_symbol(c) =>
               if (String.is_empty(n)) {
                 (
                   // if empty previous number, add symbol element
                   [Sym(c, x, y), ...els],
                   x + 1,
                   n,
                 );
               } else {
                 (
                   // if non-empty previous number, add it then add the symbol
                   [
                     Num(int_of_string(n), x - String.length(n), y),
                     Sym(c, x, y),
                     ...els,
                   ],
                   x + 1,
                   "",
                 );
               }
             | _ => (els, x, n)
             }
           },
           ([], 0, ""),
         );

    els;
  };

  let from_str = (input: string): t => {
    input |> String.lines |> List.flat_map_i(parse_line);
  };

  let is_num = (c: component): bool => {
    switch (c) {
    | Num(_) => true
    | _ => false
    };
  };

  let is_sym = (c: component): bool => {
    switch (c) {
    | Sym(_) => true
    | _ => false
    };
  };

  let component_bounds = (n: int, x: int, y: int): (int, int, int, int) => {
    let len = String.length(string_of_int(n));
    (x - 1, x + len, y - 1, y + 1);
  };

  let is_symbol_adjacent = (components: t, n: int, x: int, y: int): bool => {
    let (l, r, t, b) = component_bounds(n, x, y);

    components
    |> List.filter(c => {
         switch (c) {
         | Num(_) => false
         | Sym(_, sx, sy) => sx >= l && sx <= r && sy >= t && sy <= b
         }
       })
    |> List.length > 0;
  };

  let is_part_number = (components: t, c: component): option(int) => {
    switch (c) {
    | Num(n, x, y) when is_symbol_adjacent(components, n, x, y) => Some(n)
    | _ => None
    };
  };
};

module M = {
  type t = Schematic.t;

  let title = "Day 3: Gear Ratios";
  let input_filename = "day3.txt";

  let parse = (input: string): t => {
    Schematic.from_str(input);
  };

  let part1 = (components: t) => {
    components
    |> List.filter(Schematic.is_num)
    |> List.filter_map(Schematic.is_part_number(components))
    |> Utils.sum_int_list
    |> string_of_int
    |> print_endline;
  };

  let part2 = (_parsed_input: t) => {
    ();
  };
};

include M;
include Day.Make(M);

let example = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

let%test_unit "parse" =
  [%test_eq: Schematic.t](
    parse("617*......\n...*......"),
    [Num(617, 0, 0), Sym('*', 3, 0), Sym('*', 3, 1)],
  );

let%test_unit "parse number at end of line" =
  [%test_eq: Schematic.t](
    parse("617*......\n...*...289"),
    [Num(617, 0, 0), Sym('*', 3, 0), Num(289, 7, 1), Sym('*', 3, 1)],
  );

let%test_unit "parse number at end of line" =
  [%test_eq: bool](
    List.for_all(
      Schematic.is_raw_symbol,
      [
        '@',
        '!',
        '"',
        '#',
        '$',
        '%',
        '&',
        '\'',
        '(',
        ')',
        '*',
        '+',
        ',',
        '-',
        '/',
        '=',
      ],
    ),
    true,
  );

let%expect_test "part1" = {
  part1(parse(example));
  [%expect {| 4361 |}];
};

// let%expect_test [@tags "disabled"] "part2" = {
//   part2();
//   [%expect {| |}];
// };
