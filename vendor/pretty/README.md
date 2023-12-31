# pretty.rs

Pretty printing combinators for Rust

## Synopsis

This crate provides functionality for defining pretty printers. It is
particularly useful for printing structured recursive data like trees.

The implementation was originally based on Larsen's SML translation
(https://github.com/kfl/wpp) of Wadler's Haskell pretty printer
(http://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf). It
has since been modified in various ways to better fit Rust's
programming model. In particular, it uses iteration rather than
recursion and provides streaming output.

## Documentation

See the generated API documentation [here](http://freebroccolo.github.io/pretty.rs/doc/pretty/).

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo run         ## run the example (pretty trees)
$ cargo bench       ## run benchmarks
$ cargo test        ## run tests
```
