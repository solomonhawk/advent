// https://adventofcode.com/2022/day/1
open Containers;

module M = {
  type t = list(list(int));

  let title = "Day 1: Calorie Counting";
  let input_filename = "day1.txt";

  let parse = (input: string): t => {
    input
    |> String.trim
    |> String.split(~by="\n\n")
    |> List.map(line =>
         line |> String.split_on_char('\n') |> List.map(int_of_string)
       );
  };

  let part1 = (parsed_input: t) => {
    let _ =
      parsed_input
      |> List.map(List.fold_left((+), 0))
      |> List.sort(compare)
      |> List.rev
      |> List.hd
      |> string_of_int
      |> print_endline;

    ();
  };

  let part2 = (parsed_input: t) => {
    let _ =
      parsed_input
      |> List.map(List.fold_left((+), 0))
      |> List.sort(compare)
      |> List.rev
      |> List.take(3)
      |> List.fold_left((+), 0)
      |> string_of_int
      |> print_endline;

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

let%test_unit "parse" =
  [%test_eq: list(list(int))](
    parse("1\n2\n3\n\n4\n5\n6\n"),
    [[1, 2, 3], [4, 5, 6]],
  );

let%expect_test "part1" = {
  part1([[1, 2, 3], [4, 5, 6]]);
  [%expect {| 15 |}]; // 6 + 5 + 4
};

let%expect_test "part2" = {
  part2([[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]]);
  [%expect {| 72 |}]; // [6 + 5 + 4] + [7 + 8 + 9] + [10 + 11 + 12]
};
