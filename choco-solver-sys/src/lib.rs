//! Choco Solver Rust FFI bindings
//! This crate provides low-level FFI bindings to the Choco Solver library, allowing Rust code to interact with Choco's constraint programming capabilities.
//! It is designed to be used as a backend for higher-level Rust libraries that provide a more ergonomic API for constraint programming.
mod bindings;
pub use bindings::*;
pub use libloading::library_filename;
