module M = {
  type t = list(int);

  let title = "Day 1: Something or Other";
  let input_filename = "day1.txt";

  let parse = (input: string): t => {
    input
    |> String.trim
    |> String.split_on_char('\n')
    |> List.map(int_of_string);
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

let%test "parse" = M.parse("1\n2\n3\n") == [1, 2, 3];
