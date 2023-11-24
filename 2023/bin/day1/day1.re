let part1 = () => print_endline("Hello Day 1 part 1!");
let part2 = () => print_endline("Hello Day 1 part 2!");

let add = (a, b) => a + b;

let read_file = (path: string) => {
  let ch = open_in(path);

  Seq.unfold(
    () => {
      switch (input_line(ch)) {
      | line => Some((line, ()))
      | exception End_of_file =>
        close_in(ch);
        None;
      }
    },
    (),
  );
};

let%test_module _ =
  (module
   {
     // This doesn't work:
     // let fseq = read_file("./bin/day1/input.txt");

     // This works, but feels super hacky:
     let fseq =
       read_file(Filename.concat(Sys.getenv("PWD"), "bin/day1/input.txt"));

     Seq.iter(print_endline, fseq);
   });
