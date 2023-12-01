// https://adventofcode.com/2023/day/1
open Containers;

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

module M = {
  [@deriving (compare, sexp, show)]
  type v =
    | N(int)
    | L(char);

  [@deriving (compare, sexp, show)]
  type t = list(list(v));

  let title = "Day 1: Trebuchet?!";
  let input_filename = "day1.txt";

  let parse = (input: string): t => {
    input
    |> String.trim
    |> String.split_on_char('\n')
    |> List.map(line =>
         line
         |> String.to_list
         |> List.map(char =>
              switch (char) {
              | '0' .. '9' => N(Char.to_int(char) - Char.to_int('0'))
              | _ => L(char)
              }
            )
       );
  };

  /**
   * NOTE: the recursive parsing needs to continue with letters at the end of
   * number words that might be the start of another number word included (e.g.
   * "eighthree" needs to be parsed into 8, 3).
   */
  let rec parse_number_words = (l: list(v)): list(v) => {
    switch (l) {
    | [N(n), ...tl] => [N(n), ...parse_number_words(tl)]
    | [L('o'), L('n'), L('e'), ...tl] => [
        N(1),
        ...parse_number_words([L('e'), ...tl]),
      ]
    | [L('t'), L('w'), L('o'), ...tl] => [
        N(2),
        ...parse_number_words([L('o'), ...tl]),
      ]
    | [L('t'), L('h'), L('r'), L('e'), L('e'), ...tl] => [
        N(3),
        ...parse_number_words([L('e'), ...tl]),
      ]
    | [L('f'), L('o'), L('u'), L('r'), ...tl] => [
        N(4),
        ...parse_number_words(tl),
      ]
    | [L('f'), L('i'), L('v'), L('e'), ...tl] => [
        N(5),
        ...parse_number_words([L('e'), ...tl]),
      ]
    | [L('s'), L('i'), L('x'), ...tl] => [N(6), ...parse_number_words(tl)]
    | [L('s'), L('e'), L('v'), L('e'), L('n'), ...tl] => [
        N(7),
        ...parse_number_words([L('n'), ...tl]),
      ]
    | [L('e'), L('i'), L('g'), L('h'), L('t'), ...tl] => [
        N(8),
        ...parse_number_words([L('t'), ...tl]),
      ]
    | [L('n'), L('i'), L('n'), L('e'), ...tl] => [
        N(9),
        ...parse_number_words([L('e'), ...tl]),
      ]
    | [L(_), ...tl] => parse_number_words(tl)
    | [] => []
    };
  };

  let nums_as_str = (vv: v): option(string) => {
    switch (vv) {
    | L(_) => None
    | N(n) => Some(string_of_int(n))
    };
  };

  let int_from_bookends = (nums: list(string)): int => {
    let first = List.hd(nums);
    let last = List.rev(nums) |> List.hd;

    int_of_string(first ++ last);
  };

  let part1 = (parsed_input: t) => {
    parsed_input
    |> List.map(line => {
         line |> List.filter_map(nums_as_str) |> int_from_bookends
       })
    |> List.fold_left((+), 0)
    |> string_of_int
    |> print_endline;
  };

  let part2 = (parsed_input: t) => {
    parsed_input |> List.map(parse_number_words) |> part1;
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
    [
      [N(1), L('a'), L('b'), L('c'), N(2)],
      [L('i'), N(4), L('b'), N(1), L('d')],
    ],
  );

let%test_unit "parse_number_words 1" =
  [%test_eq: list(v)](
    parse_number_words([
      L('x'),
      N(7),
      L('o'),
      L('n'),
      L('e'),
      N(5),
      L('b'),
    ]),
    [N(7), N(1), N(5)],
  );

let%test_unit "parse_number_words 2" =
  [%test_eq: list(v)](
    parse_number_words([L('t'), L('w'), L('o')]),
    [N(2)],
  );

let%test_unit "parse_number_words 3" =
  [%test_eq: list(v)](
    parse_number_words([L('t'), L('h'), L('r'), L('e'), L('e')]),
    [N(3)],
  );

let%test_unit "parse_number_words 4" =
  [%test_eq: list(v)](
    parse_number_words([L('f'), L('o'), L('u'), L('r')]),
    [N(4)],
  );

let%test_unit "parse_number_words 5" =
  [%test_eq: list(v)](
    parse_number_words([L('f'), L('i'), L('v'), L('e')]),
    [N(5)],
  );

let%test_unit "parse_number_words 6" =
  [%test_eq: list(v)](
    parse_number_words([L('s'), L('i'), L('x')]),
    [N(6)],
  );

let%test_unit "parse_number_words 7" =
  [%test_eq: list(v)](
    parse_number_words([L('s'), L('e'), L('v'), L('e'), L('n')]),
    [N(7)],
  );

let%test_unit "parse_number_words 8" =
  [%test_eq: list(v)](
    parse_number_words([L('e'), L('i'), L('g'), L('h'), L('t')]),
    [N(8)],
  );

let%test_unit "parse_number_words 9" =
  [%test_eq: list(v)](
    parse_number_words([L('n'), L('i'), L('n'), L('e')]),
    [N(9)],
  );

let%test_unit "parse_number_words overlap" =
  [%test_eq: list(v)](
    parse_number_words([
      L('n'),
      L('i'),
      L('n'),
      L('e'),
      L('i'),
      L('g'),
      L('h'),
      L('t'),
    ]),
    [N(9), N(8)],
  );

let%expect_test "part1" = {
  part1(parse(example1));
  [%expect {| 142 |}];
};

let%expect_test "part2" = {
  part2(parse(example2));
  [%expect {| 281 |}];
};
