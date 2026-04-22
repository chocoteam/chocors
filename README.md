# Chocors

This is wrapper of Choco5 solver.
The API are similar to python wrapper [pychoco](https://github.com/chocoteam/pychoco) but with some differences:

- Idiomatic Rust as much as possible
- Prefer compile time check instead of runtime check --> avoid as much possible panics.
- No support for set variables

> [!WARNING]
>**Don't mix variables from different model**
> 
> Presently Rust wrapper doesn't protect from mixing variables from different model in same constraints. Wrapper relays on Java implementation.

> [!NOTE]
> **Thread Safety**
> 
> This library create one GraalVM isolate (independent execution environment) for each process.
> Currently the all types are not Send and Sync until it is clarified the thread safety of Choco Solver API and GraalVM native C API

## Building and testing

[BUILDING.md](./BUILDING.md)
