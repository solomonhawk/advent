// https://adventofcode.com/2023/day/1
open Containers;

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

module M = {
  [@deriving (compare, sexp)]
  type v =
    | Num(int)
    | Letter(char);

  [@deriving (compare, sexp)]
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
              | '0' .. '9' => Num(Char.to_int(char) - Char.to_int('0'))
              | _ => Letter(char)
              }
            )
       );
  };

  let nums_as_str = (vv: v): option(string) => {
    switch (vv) {
    | Letter(_) => None
    | Num(n) => Some(string_of_int(n))
    };
  };

  let int_from_bookends = (nums: list(string)): int => {
    let first = List.hd(nums);
    let last = List.rev(nums) |> List.hd;

    int_of_string(first ++ last);
  };
  let part1 = (parsed_input: t) => {
    let _ =
      parsed_input
      |> List.map(line => {
           line |> List.filter_map(nums_as_str) |> int_from_bookends
         })
      |> List.fold_left((+), 0)
      |> string_of_int
      |> print_endline;

    ();
  };

  let part2 = (_parsed_input: t) => {
    ();
  };
};

include M;
include Day.Make(M);

let example = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

let%test_unit "parse" =
  [%test_eq: t](
    parse("1abc2\ni4b1d"),
    [
      [Num(1), Letter('a'), Letter('b'), Letter('c'), Num(2)],
      [Letter('i'), Num(4), Letter('b'), Num(1), Letter('d')],
    ],
  );

let%expect_test "part1" = {
  part1(parse(example));
  [%expect {| 142 |}];
};

// let%expect_test "part2" = {
//   part2([[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]]);
//   [%expect {| 72 |}]; // [6 + 5 + 4] + [7 + 8 + 9] + [10 + 11 + 12]
// };
