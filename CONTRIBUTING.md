# Contributing to Chocors

Contributions to this repository are welcome.

This project is a Rust wrapper around the Choco solver native API. The workspace contains:

- the safe Rust crate in `src/`
- the low-level FFI crate in `choco-solver-sys/`
- an `xtask` helper for native build and binding generation tasks
- the `choco-solver-capi` native project as a submodule under `choco-solver-sys/`

## Before you start

Use the issue tracker for bug reports, feature requests, and pull requests.

When reporting a bug, include:

1. the version or commit you tested
2. your platform and toolchain
3. whether `libchoco_capi` was built locally or obtained elsewhere
4. a minimal reproduction

If the issue appears to come from the native solver bridge rather than the Rust wrapper, say so explicitly in the report. That helps determine whether the fix belongs in this repository or in the upstream native project.

## Development setup

Clone the repository with submodules:

```bash
git clone https://github.com/chocoteam/chocors.git
git submodule update --init --recursive
```

### Tooling

For regular Rust development, you need a current Rust toolchain and Cargo.

If you need to build the native DLL or regenerate bindings, you also need:

- GraalVM 25 JDK
- `GRAALVM_HOME` set to the GraalVM installation directory
- Maven available on `PATH`
- the system requirements needed by `bindgen`

See `BUILDING.md` for the native build details.

## Typical workflow

1. Fork the repository and create a topic branch.
2. Make focused changes.
3. Add or update tests with the change.
4. Run the relevant checks.
5. Open a pull request with a clear description of the change and why it is needed.

## Running checks

### Rust-only changes

For changes limited to the Rust wrapper, run:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
```

Tests require `libchoco_capi` to be discoverable. The simplest project-local command is:

```bash
cargo xtask test-all
```

That command runs `cargo test` with `CHOCO_SOLVER_DLL_FOLDER` set to the native build output.

If you already have the DLL available through your environment or system library path, running `cargo test` directly is also fine.

### Native or binding changes

If you modify anything in the native layer, the submodule, or code that depends on regenerated bindings, run:

```bash
cargo xtask build-dll
cargo xtask generate-bindings
cargo xtask test-all
```

Do not hand-edit `choco-solver-sys/src/bindings.rs` unless you are deliberately making a temporary diagnostic change. In normal development, regenerate it.

### Dependency changes

If you change dependencies or licensing-sensitive configuration, also run:

```bash
cargo deny check
```

## Code expectations

- Keep changes narrow and directly related to the problem being solved.
- Preserve the existing public API unless the change intentionally modifies it.
- Add tests for bug fixes and new behavior.
- Keep documentation in sync when behavior or setup changes.
- Follow the workspace lint configuration.

This workspace is strict about correctness and safety. In particular:

- unsafe code must be justified and documented well enough to satisfy the lint configuration
- generated code should stay generated
- panics should not be introduced casually in wrapper code

## Testing guidance

Prefer the smallest test that proves the behavior:

- unit tests near the affected module for API behavior
- integration tests in `tests/` for end-to-end wrapper behavior
- compile-fail tests in `tests/compile_fails/` for type-system or trait guarantees

If a change affects model or solver behavior, include a test that exercises the solver rather than only checking local helper logic.

## Pull requests

Pull requests should explain:

1. what problem is being solved
2. how the fix works
3. what checks were run
4. any follow-up work that remains

Small, focused pull requests are much easier to review than broad refactors.

## Documentation changes

Documentation improvements are welcome and do not need accompanying code changes unless the documentation depends on new behavior.

If you update setup or build instructions, make sure `README.md` and `BUILDING.md` remain consistent.
