# rsdd-ocaml

OCaml bindings for RSDD.

This is *very* much in progress. Things on the radar:

- [ ] publish to opam (blocked by: some user testing)
- [ ] add user-facing documentation (blocked by: time)
- [ ] add developer documentation (blocked by: time)
- [ ] add significantly more parts of the RSDD API, including:
  - [ ] SDD support (builder + manager)
  - [ ] allowing different options for generics
- [ ] potentially better support Rust <-> OCaml interop (view types, better/more natural conversions)

## For Users

Currently, this package is not available on opam (but that'll change soon!). In the meantime, you can pin a local version of the package.

Before installing, make sure you have `rust` and `cargo` on your machine. They're available through most package managers of your choice.

First, clone the repo:

```
$ git clone https://github.com/neuppl/rsdd-ocaml.git
```

Then, *in your project directory*, pin a local package:

```
$ opam pin rsdd ../path/to/rsdd-ocaml
```

Then, add `rsdd` to:

- your project's `.opam` file, if relevant (in `depends`)
- your library/executable's `dune` file, if relevant (in `libraries`)

Then, run

```
$ dune build
```

You can now use `Rsdd`!

```ocaml
open Rsdd

let () =
  let robdd_builder = mk_bdd_builder_default_order 6L in
    let bdd = bdd_builder_compile_cnf robdd_builder (cnf_from_dimacs "
    p cnf 6 3\n\
    1 2 3 4 0\n\
    -2 -3 4 5 0\n\
    -4 -5 6 6 0\n\
    ") in
    let model_count = bdd_model_count robdd_builder bdd in
    print_endline (Int64.to_string model_count)

```

## For Developers

To build new bindings:

```
$ dune build
```

TODO
