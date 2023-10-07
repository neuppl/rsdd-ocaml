(lang dune 2.7)
(name rsdd)

(generate_opam_files true)

(version 0.1.0)

(package
 (name rsdd)
 (synopsis "Bindings for RSDD")
 (description "OCaml bindings for the Rust Decision Diagram (RSDD) library, generated with ocaml-rs.")
 (authors "Steven Holtzen <s.holtzen@northeastern.edu>" "Matt Wang <mxw@cs.washington.edu>")
 (maintainers "Matt Wang <mxw@cs.washington.edu>" "Minsung Cho <minsung@ccs.neu.edu>")
 (license "MIT")
 (source (github neuppl/rsdd-ocaml))
 (depends
  (ocaml (and (>= "4.08.0") (< "5")))
  dune
  conf-rust-2021))
