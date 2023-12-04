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

  [@deriving (compare, sexp, show)]
  type t = list(component);

  let is_sym = (c: char): bool => {
    switch (c) {
    | '.' => false
    | '='
    | '@'
    | '!' .. '/' => true
    | _ => false
    };
  };

  module LineParser = {
    type t = (list(component), int, string);

    let int = int_of_string;

    let new_state = ([], 0, "");

    let append_digit_to_n =
        (els: list(component), c: char, x: int, n: string): t => {
      (els, x + 1, n ++ Char.escaped(c));
    };

    let append_last_n_if_defined =
        (els: list(component), x: int, y: int, n: string) =>
      if (String.is_empty(n)) {
        (els, x + 1, n);
      } else {
        ([Num(int(n), x - String.length(n), y), ...els], x + 1, "");
      };

    let append_sym_and_last_n_if_defined =
        (els: list(component), c: char, x: int, y: int, n: string) =>
      if (String.is_empty(n)) {
        ([Sym(c, x, y), ...els], x + 1, n);
      } else {
        (
          [Num(int(n), x - String.length(n), y), Sym(c, x, y), ...els],
          x + 1,
          "",
        );
      };

    let elements = ((els: 'a, _, _)): 'a => els;

    let parse_line = (y: int, line: string): list(component) => {
      line
      // appending "." makes sure we parse numbers that appear at the end of the line
      ++ "."
      |> String.to_list
      |> List.fold_left(
           ((els, x, n), c) => {
             switch (c) {
             | '0' .. '9' => append_digit_to_n(els, c, x, n)
             | '.' => append_last_n_if_defined(els, x, y, n)
             | c when is_sym(c) =>
               append_sym_and_last_n_if_defined(els, c, x, y, n)
             | _ => (els, x, n)
             }
           },
           new_state,
         )
      |> elements;
    };
  };

  let from_str = (input: string): t => {
    input |> String.lines |> List.flat_map_i(LineParser.parse_line);
  };

  let component_bounds = (w: int, x: int, y: int): (int, int, int, int) => {
    (x - 1, x + w, y - 1, y + 1);
  };

  let is_symbol_adjacent = (components: t, n: int, x: int, y: int): bool => {
    let (l, r, t, b) =
      component_bounds(String.length(string_of_int(n)), x, y);

    components
    |> List.filter(c => {
         switch (c) {
         | Num(_) => false
         | Sym(_, sx, sy) => sx >= l && sx <= r && sy >= t && sy <= b
         }
       })
    |> List.length > 0;
  };

  let is_number_adjacent = (x: int, y: int, c: component): bool => {
    let (l, r, t, b) = component_bounds(1, x, y);

    switch (c) {
    | Num(n, nx, ny) =>
      let w = String.length(string_of_int(n));
      nx + w - 1 >= l && nx <= r && ny >= t && ny <= b;
    | _ => false
    };
  };

  let adjacent_numbers = (components: t, x: int, y: int): t => {
    components |> List.filter(is_number_adjacent(x, y));
  };

  let is_part_number = (components: t, c: component): option(int) => {
    switch (c) {
    | Num(n, x, y) when is_symbol_adjacent(components, n, x, y) => Some(n)
    | _ => None
    };
  };

  let is_gear = (components: t, c: component): option(int) => {
    switch (c) {
    | Sym('*', x, y) =>
      let nums = adjacent_numbers(components, x, y);

      if (List.length(nums) == 2) {
        let product =
          nums
          |> List.filter_map(c => {
               switch (c) {
               | Num(n, _, _) => Some(n)
               | _ => None
               }
             })
          |> List.fold_left(( * ), 1);
        Some(product);
      } else {
        None;
      };
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
    |> List.filter_map(Schematic.is_part_number(components))
    |> Utils.sum_int_list
    |> string_of_int
    |> print_endline;
  };

  let part2 = (components: t) => {
    components
    |> List.filter_map(Schematic.is_gear(components))
    |> Utils.sum_int_list
    |> string_of_int
    |> print_endline;
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

let%test_unit "is_raw_symbol" =
  [%test_eq: bool](
    List.for_all(
      Schematic.is_sym,
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

let%test_unit "component_bounds" =
  [%test_eq: list((int, int, int, int))](
    [
      // width of 1
      Schematic.component_bounds(1, 2, 3),
      // width > 1
      Schematic.component_bounds(3, 2, 3),
    ],
    [(1, 3, 2, 4), (1, 5, 2, 4)],
  );

let%test_unit "is_part_number" =
  [%test_eq: list(option(int))](
    [
      // symbol left
      Schematic.is_part_number(
        [Schematic.Sym('%', 0, 1)],
        Schematic.Num(111, 1, 1),
      ),
      // symbol above
      Schematic.is_part_number(
        [Schematic.Sym('%', 1, 0)],
        Schematic.Num(222, 1, 1),
      ),
      // symbol right
      Schematic.is_part_number(
        [Schematic.Sym('%', 4, 1)],
        Schematic.Num(333, 1, 1),
      ),
      // symbol below
      Schematic.is_part_number(
        [Schematic.Sym('%', 1, 2)],
        Schematic.Num(444, 1, 1),
      ),
      // symbol diagonal
      Schematic.is_part_number(
        [Schematic.Sym('%', 0, 0)],
        Schematic.Num(555, 1, 1),
      ),
      // not adjacent
      Schematic.is_part_number(
        [Schematic.Sym('%', 6, 1)],
        Schematic.Num(111, 1, 1),
      ),
      // nothing to compare to
      Schematic.is_part_number([], Schematic.Num(222, 1, 1)),
    ],
    [Some(111), Some(222), Some(333), Some(444), Some(555), None, None],
  );

let%test_unit "is_gear" =
  [%test_eq: list(option(int))](
    [
      // numbers above and below
      Schematic.is_gear(
        [Schematic.Num(2, 1, 0), Schematic.Num(4, 1, 2)],
        Schematic.Sym('*', 1, 1),
      ),
      // numbers diagonal
      Schematic.is_gear(
        [Schematic.Num(10, 1, 0), Schematic.Num(20, 4, 2)],
        Schematic.Sym('*', 3, 1),
      ),
      // not enough numbers
      Schematic.is_gear(
        [Schematic.Num(10, 1, 0)],
        Schematic.Sym('*', 3, 1),
      ),
      // too many adjacent numbers
      Schematic.is_gear(
        [
          Schematic.Num(10, 1, 0),
          Schematic.Num(20, 4, 2),
          Schematic.Num(420, 0, 1),
        ],
        Schematic.Sym('*', 3, 1),
      ),
      // not a '*' symbol
      Schematic.is_gear(
        [Schematic.Num(10, 1, 0), Schematic.Num(20, 4, 2)],
        Schematic.Sym('%', 3, 1),
      ),
    ],
    [Some(8), Some(200), None, None, None],
  );

let%expect_test "part1" = {
  part1(parse(example));
  [%expect {| 4361 |}];
};

let%expect_test "part2" = {
  part2(parse(example));
  [%expect {| 467835 |}];
};
