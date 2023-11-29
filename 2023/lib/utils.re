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
  let s = read_file_seq(path);

  Seq.fold_left((acc, line) => acc ++ line ++ "\n", "", s);
};
