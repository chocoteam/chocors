# Changelog

## Unreleased

### Breaking Changes

- `ArrayEqualityConstraints::all_different` now requires an `AllDifferentConsistency` argument. Choose `Default`, `ArcConsistency`, `BoundConsistency`, or `FullConsistency`.

### Added

- Regenerated FFI bindings for the Choco Solver C API 2.0.2, including callback types and APIs for custom constraints and search.

### Build

- Bumped the `choco-solver` crate version from 0.1.0 to 0.2.0.
- Adjusted bindings generation (xtask) to handle callback typedefs.
