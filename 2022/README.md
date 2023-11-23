# Advent of Code

In Rust! Uses `cargo-aoc` crate to download problem inputs and run the days/parts. See below.

## Getting Started

1. Install Rust (https://www.rust-lang.org/tools/install)

    ```console
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Build the project & install dependencies

    ```console
    $ cargo build
    ```

3. While working, use `cargo watch` to continuously re-run the solutions

    To run the `aoc-runner`:
    ```console
    $ cargo watch -qc -x aoc
    ```

    - `-q` = quiet
    - `-c` = clear console before each run
    - `-x <command>` run this command when files change

    To run tests:
    ```console
    $ cargo watch -qc -x test
    ```

## `cargo-aoc`
https://github.com/gobanos/cargo-aoc

### Example Usage
https://github.com/gobanos/advent-of-code-2015

### Downloading new inputs for today:

```console
$ cargo aoc input
```

### Downloading new inputs for a specific year/day:

```console
$ cargo aoc input -d {day} -y {year}
```

