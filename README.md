# Chocors

This is wrapper of Choco5 solver.
The API are similar to python wrapper [pychoco](https://github.com/chocoteam/pychoco) but with some differences:

- Idiomatic Rust as much as possible
- Prefer compile time check instead of runtime check --> avoid as much possible panics.
- No support for set variables

> [!Warning] Don't mix variables from different model
> Presently Rust wrapper doesn't protect from mixing variables from different model in same constraints. Wrapper relays on Java implementation.

> [!Note] Thread Safety
> This library create one separate GraalVM isolate (independent execution environment) for each thread.
> For this reason all types are not Send or Sync.

## Building and testing

[BUILDING.md](./BUILDING.md)
