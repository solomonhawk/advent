open Utils;

type t = list(int);

let parse = (input: string) => {
  input
  |> String.trim
  |> String.split_on_char('\n')
  |> List.map(int_of_string);
};

let part2 = (parsed_input: t) => {
  ();
};

let part1 = (parsed_input: t) => {
  ();
};

let run = () => {
  let input = read_file_str("./inputs/day1.txt") |> parse;

  part1(input);
  part2(input);

  ();
};

let%test_module _ =
  (module
   {
     let fseq =
       read_file_seq(Filename.concat(Sys.getenv("PWD"), "inputs/day1.txt"));

     Seq.iter(print_endline, fseq);
   });
