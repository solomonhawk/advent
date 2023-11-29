/**
 * Day modules `include` this module, which provides a `run` function that runs
 * the day's solutions.
 */
module type DayRunner = {
  let run: unit => unit;
};

/**
 * Each day module implements this interface.
 */
module type DayImpl = {
  type t;

  let title: string;
  let input_filename: string;
  let parse: string => t;
  let part1: t => unit;
  let part2: t => unit;
};

module Make = (Impl: DayImpl) : DayRunner => {
  let run = () => {
    let filepath = Printf.sprintf("inputs/%s", Impl.input_filename);

    if (!Sys.file_exists(filepath)) {
      print_endline("Input file not found: " ++ filepath);
      exit(1);
    };

    let parsed = filepath |> Utils.read_file_str |> Impl.parse;

    print_endline(Impl.title);

    print_endline("Part 1:");
    Impl.part1(parsed);

    print_endline("Part 2:");
    Impl.part2(parsed);

    print_endline("---------");

    ();
  };
};
