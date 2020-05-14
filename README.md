# Minsky

A toy implementation of Minsky Machines in Rust.

See [1], [2], and especially [3].

## Goals

The goals are:

* [X] write a basic interpreter for Magnificent Minsky Machines (M3)
* [X] write some arithmetic programs using the M3 interpreter
* [ ] write a transpiler from M3 to M3 that produces machines with a single
      machine state (and possibly many more tapes)
* [ ] write a transpiler from M3 to M3 that produces machines with a single
      machine state and a single state.
* [ ] support "Portable Minsky Machine Notation" [4]

## Progress

* Basic M3 interpreter works and has tests and applications (e.g.
  `adder.rs`).

## References

[1]: https://en.wikipedia.org/wiki/Counter_machine
[2]: https://esolangs.org/wiki/Minsky_machine
[3]: http://raganwald.com/2020/05/03/fractran.html
[4]: https://esolangs.org/wiki/Portable_Minsky_Machine_Notation
