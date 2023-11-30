// https://adventofcode.com/2023/day/n

module M = {
  type t = unit;

  let title = "Day N: <title>";
  let input_filename = "dayn.txt";

  let parse = (_input: string): t => {
    ();
  };

  let part1 = (_parsed_input: t) => {
    ();
  };

  let part2 = (_parsed_input: t) => {
    ();
  };
};

include M;
include Day.Make(M);

// without these 2 lines, I get:
//   Error: Unbound value sexp_of_list OR
//   Error: Unbound value compare_list
open Ppx_compare_lib.Builtin;
open Sexplib.Std;

let%test_unit [@tags "disabled"] "parse" = [%test_eq: unit](parse(""), ());

let%expect_test [@tags "disabled"] "part1" = {
  part1();
  [%expect {| |}];
};

let%expect_test [@tags "disabled"] "part2" = {
  part2();
  [%expect {| |}];
};
