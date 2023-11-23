# AOC Reason

## Install Deps

    $ esy

This is an alias for `esy install` or `esy i`.

## Build

    $ esy b

`b` is an alias for `esy build`.

## Run

    $ esy x aoc

`x` is an alias for `esy exec`.

## Watch

    $ npm run build:watch

    # with alias n="npm run"
    $ n build:watch

## TODO

- [x] Run dune build in watch mode
- [x] Set up unit testing
  - [x] `dune runtest`
- [ ] Can `esy` be used with `asdf-ocaml` in a sensible way?
  - [ ] `esy` has it's own project sandboxing (like `opam` switches, but project-specific by default)

## References

- https://www.chrisarmstrong.dev/posts/setting-up-a-new-reason-project
- https://mukulrathi.com/ocaml-tooling-dune/