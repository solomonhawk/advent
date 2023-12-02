// https://adventofcode.com/2023/day/1
include Import;

open Containers;

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

module M = {
  [@deriving (compare, sexp, show)]
  type t = list(list(char));

  let title = "Day 1: Trebuchet?!";
  let input_filename = "day1.txt";

  let parse = (input: string): t => {
    input |> String.trim |> String.lines |> List.map(String.to_list);
  };

  /**
   * NOTE: the recursive parsing needs to continue with letters at the end of
   * number words that might be the start of another number word included (e.g.
   * "eighthree" needs to be parsed into 8, 3).
   */
  let rec parse_ns = (l: list(char)): list(int) => {
    switch (l) {
    | [n, ...tl] when Utils.is_digit(n) => [
        Char.code(n) - Char.code('0'),
        ...parse_ns(tl),
      ]
    | ['o', 'n', 'e', ...tl] => [1, ...parse_ns(['e', ...tl])]
    | ['t', 'w', 'o', ...tl] => [2, ...parse_ns(['o', ...tl])]
    | ['t', 'h', 'r', 'e', 'e', ...tl] => [3, ...parse_ns(['e', ...tl])]
    | ['f', 'o', 'u', 'r', ...tl] => [4, ...parse_ns(tl)]
    | ['f', 'i', 'v', 'e', ...tl] => [5, ...parse_ns(['e', ...tl])]
    | ['s', 'i', 'x', ...tl] => [6, ...parse_ns(tl)]
    | ['s', 'e', 'v', 'e', 'n', ...tl] => [7, ...parse_ns(['n', ...tl])]
    | ['e', 'i', 'g', 'h', 't', ...tl] => [8, ...parse_ns(['t', ...tl])]
    | ['n', 'i', 'n', 'e', ...tl] => [9, ...parse_ns(['e', ...tl])]
    | [_, ...tl] => parse_ns(tl)
    | [] => []
    };
  };

  let digits = (c: char): option(char) => {
    switch (c) {
    | n when Utils.is_digit(n) => Some(n)
    | _ => None
    };
  };

  let int_from_bookends = (nums: list(int)): int => {
    let first = List.hd(nums);
    let last = List.rev(nums) |> List.hd;

    int_of_string(string_of_int(first) ++ string_of_int(last));
  };

  let part1 = (parsed_input: t) => {
    parsed_input
    |> List.map(chars => {
         chars
         |> List.filter_map(digits)
         |> List.map(c => Char.code(c) - Char.code('0'))
         |> int_from_bookends
       })
    |> List.fold_left((+), 0)
    |> string_of_int
    |> print_endline;
  };

  let part2 = (parsed_input: t) => {
    parsed_input
    |> List.map(parse_ns >> int_from_bookends)
    |> List.fold_left((+), 0)
    |> string_of_int
    |> print_endline;

    ();
  };
};

include M;
include Day.Make(M);

let example1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

let example2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

let%test_unit "parse" =
  [%test_eq: t](
    parse("1abc2\ni4b1d"),
    [['1', 'a', 'b', 'c', '2'], ['i', '4', 'b', '1', 'd']],
  );

let%test_unit "parse_ns 1" =
  [%test_eq: list(int)](
    parse_ns(['x', '7', 'o', 'n', 'e', '5', 'b']),
    [7, 1, 5],
  );

let%test_unit "parse_ns 2" =
  [%test_eq: list(int)](parse_ns(['t', 'w', 'o']), [2]);

let%test_unit "parse_ns 3" =
  [%test_eq: list(int)](parse_ns(['t', 'h', 'r', 'e', 'e']), [3]);

let%test_unit "parse_ns 4" =
  [%test_eq: list(int)](parse_ns(['f', 'o', 'u', 'r']), [4]);

let%test_unit "parse_ns 5" =
  [%test_eq: list(int)](parse_ns(['f', 'i', 'v', 'e']), [5]);

let%test_unit "parse_ns 6" =
  [%test_eq: list(int)](parse_ns(['s', 'i', 'x']), [6]);

let%test_unit "parse_ns 7" =
  [%test_eq: list(int)](parse_ns(['s', 'e', 'v', 'e', 'n']), [7]);

let%test_unit "parse_ns 8" =
  [%test_eq: list(int)](parse_ns(['e', 'i', 'g', 'h', 't']), [8]);

let%test_unit "parse_ns 9" =
  [%test_eq: list(int)](parse_ns(['n', 'i', 'n', 'e']), [9]);

let%test_unit "parse_ns overlap" =
  [%test_eq: list(int)](
    parse_ns(['n', 'i', 'n', 'e', 'i', 'g', 'h', 't']),
    [9, 8],
  );

let%expect_test "part1" = {
  part1(parse(example1));
  [%expect {| 142 |}];
};

let%expect_test "part2" = {
  part2(parse(example2));
  [%expect {| 281 |}];
};
