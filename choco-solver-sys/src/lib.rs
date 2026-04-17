//! Low-level Rust FFI bindings for the Choco Solver native C API.
//!
//! This crate exposes the generated bindings for `libchoco_capi`, the native library
//! produced for Choco Solver. It is the unsafe boundary used by the higher-level Rust
//! API in this repository. Most users should prefer the top-level crate instead of
//! depending on `choco-solver-sys` directly.
//!
//! # What This Crate Contains
//!
//! - Auto-generated `bindgen` output for `libchoco_capi.h`
//! - A generated dynamic symbol loader, exposed as [`libchoco_capi`]
//! - Re-exported raw FFI types, constants, and functions from the generated bindings
//!
//! # Building The Native Library
//!
//! This crate does not build `libchoco_capi` for you. To produce the native library,
//! see `BUILDING.md` in the repository root or the upstream `choco-solver-capi`
//! project.
//!
//! # Safety
//!
//! The API exposed by this crate is intentionally low-level and mostly unsafe:
//!
//! - Function signatures are raw FFI declarations generated from the C header
//! - Dynamically loaded symbols must only be called with valid pointers and values
//! - Lifetime and thread-safety guarantees are determined by the native library, not
//!   by Rust types in this crate
//!
//! # Example
//!
//! ```no_run
//! use choco_solver_sys::{libchoco_capi, library_filename};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let library_path = if cfg!(target_os = "windows") {
//!         library_filename("libchoco_capi")
//!     } else {
//!         library_filename("choco_capi")
//!     };
//!
//!     let api = unsafe { libchoco_capi::new(&library_path)? };
//!
//!     // Generated loaders store each symbol lookup result on the struct.
//!     let create_isolate = api.graal_create_isolate.as_ref()?;
//!     let _ = create_isolate;
//!
//!     Ok(())
//! }
//! ```
mod bindings;
pub use bindings::*;

/// Builds a platform-specific dynamic library filename.
///
/// This is re-exported from `libloading` as a convenience when loading
/// `libchoco_capi` dynamically.
pub use libloading::library_filename;
