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

## Test

    $ npm run test
    $ npm run test:watch

**Note**: Only one dune process can be run at a time (it locks the build directory), so you must stop the watch process before running tests.

## Notes/Questions

- [ ] Can `esy` be used with `asdf-ocaml` in a sensible way?
  - [ ] `esy` has it's own project sandboxing (like `opam` switches, but project-specific by default)

## References

- https://www.chrisarmstrong.dev/posts/setting-up-a-new-reason-project
- https://mukulrathi.com/ocaml-tooling-dune/
- https://reasonml.github.io/en/try.html
- https://github.com/fangyi-zhou/advent-of-code-ocaml-starter/tree/main
- https://github.com/fangyi-zhou/advent-of-code/tree/main/2022
- https://github.com/DrearyLisper/aoc-2022
- https://ocaml.org/docs
- https://reasonml.github.io/docs/en/overview
- https://dune.readthedocs.io/en/stable/index.html
- https://esy.sh/docs/getting-started/
- https://github.com/janestreet/ppx_inline_test/tree/master
- https://discuss.ocaml.org/t/how-to-set-up-unit-testing-in-2023/12682/26
- https://discuss.ocaml.org/t/ocaml-stdlib-and-death-by-a-thousand-papercuts/9180/9
- https://c-cube.github.io/ocaml-containers/last/containers/index.html