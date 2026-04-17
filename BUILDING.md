# Build

## Repository checkout

```bash
git clone https://github.com/chocoteam/chocors.git
git submodule update --init --recursive # Initialize choco-solver-capi
```

## Build requirements for building the DLL and C header files

- Install/unzip Graalvm 25 jdk
- `GRAALVM_HOME` environment variable pointing to GraalVM JDK folder
- Maven installed

## Build requirements to generate `binding.rs` in `choco-solver-sys package`

- [see `bindgen` requirements](https://rust-lang.github.io/rust-bindgen/requirements.html)

# Build

- Run `cargo xtask build-dll` for building the DLL and generating header files
  - `libchoco_capi.dll` is located in `choco-solver-sys/target`
- Run `cargo xtask generate-bindings` to re-generate the `bindings.rs` in `choco-solver-sys`
-
- execute usual `cargo` command to build and test
