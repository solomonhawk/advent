let part1 = () => print_endline("Hello Day 1 part 1!");
let part2 = () => print_endline("Hello Day 1 part 2!");

let add = (a, b) => a + b;

let%test "add" = add(1, 2) == 3;
