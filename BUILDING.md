# Build

## Build requirements for building the DLL and C header files

- Graalvm 25 jdk
- `JAVA_HOME` environment variable pointing to GraalVM JDK
- Maven installed

## Build requirements to generate `binding.rs` in `choco-solver-sys package`

- [see `bindgen` requirements](https://rust-lang.github.io/rust-bindgen/requirements.html)

# Build

- Run `cargo xtask build-dll` for building the DLL and generating header files
  - `libchoco_capi.dll` is located in `choco-solver-sys/target`
- Run `cargo xtask generate-bindings` to re-generate the `bindings.rs` in `choco-solver-sys`
-
- execute usual `cargo` command to build and test
