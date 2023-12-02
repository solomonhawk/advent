open Containers;

let read_file_seq = (path: string) => {
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

let read_file_str = (path: string) => {
  path
  |> read_file_seq
  |> Seq.fold_left((acc, line) => acc ++ line ++ "\n", "");
};

let split_once = (sep: string, s: string): (string, string) => {
  switch (String.split(~by=sep, s)) {
  | [first, second] => (first, second)
  | _ => failwith("Could not split string into two")
  };
};

let sum_int_list = (list: list(int)): int => {
  List.fold_left((+), 0, list);
};

let is_digit = (c: char): bool => {
  switch (c) {
  | '0' .. '9' => true
  | _ => false
  };
};
