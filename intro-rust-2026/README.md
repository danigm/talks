# Rust programming language

## About me

* Daniel García Moreno <dani@danigm.net> (@danigm)
* Ex US / Ex SUGUS (2005-2008)
* GNOME developer (evince, fractal, gtranslator)
* SUSE packaging team, Python Engineer
* https://danigm.net

## About Rust

A language empowering everyone
to build reliable and efficient software.

 * https://rust-lang.org/

## Creating a rust project with cargo

 * cargo: Rust's package manager

```
$ cargo new http-server
$ cd http-server
$ cargo run
```

 * What is Cargo.toml
 * Main source file in src/main.rs

### Extra

 * add: Add external dependencies (https://crates.io)
 * test: Run test suite
 * doc: Build doc
 * publish: Send lib to crates.io

Look for more helpful command with:

```
cargo --list
```

## Ownership, Borrowing and Lifetimes

 * Variable definiton with `let`:

```
let n: i32 = 4; // typed, stack
let vector = vec![1, 2, 3]; // auto-detect type, heap
```

 * Let's try to modify these variables

```
n = 5;
vector[0] = 4;
```

 * Variables are inmutable by default, use the `mut` prefix to make a variable
   mutable.

### Ownership rules

 * Each value in Rust has an owner.
 * There can only be one owner at a time.
 * When the owner goes out of scope, the value will be dropped.
