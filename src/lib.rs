//! Safe Rust wrapper around the [Choco Solver](https://choco-solver.org/) API.
//!
//! This crate provides an idiomatic Rust interface to model and solve constraint
//! satisfaction/optimization problems while delegating solving to Choco through
//! the native `libchoco_capi` library.
//!
//! For complete solver semantics and advanced API details, refer to the official
//! Choco documentation: <https://choco-solver.org/>.
//!
//! # Getting `libchoco_capi.dll`
//!
//! You can obtain the native DLL in two main ways:
//! - Build/fetch it from the upstream project:
//!   [choco-solver-capi](https://github.com/chocoteam/choco-solver-capi)
//! - Clone this repository and follow [BUILDING](./BUILDING.md)
//!   (for example, `cargo xtask build-dll`)
//!
//! # Where to put the DLL
//!
//! `libchoco_capi.dll` must be discoverable by the dynamic loader. Typical options:
//! - Place it in a folder already present in your system DLL search path.
//! - Set `CHOCO_SOLVER_DLL_FOLDER` to the DLL directory.
//! - Call [`ChocoBackend::set_dll_folder`] before creating/using models.
//!
//! # Scope of this crate
//!
//! This crate is a safe wrapper on top of the Choco solver C API (`choco-solver-sys`
//! is the unsafe FFI boundary). The high-level API is Rust-oriented, but solver
//! behavior is defined by Choco itself.
//!
//! # Warning: Avoid mixing variables from different models
//! <div class="warning">
//! Do not mix variables from different models.
//! The wrapper currently does not prevent cross-model variable mixing in constraints
//! and relies on backend behavior.
//! </div>
//!
//! Thread safety
//!
//! This library creates one separate GraalVM isolate (independent execution
//! environment) per thread. For this reason, all main types are not `Send`/`Sync`.
//!
//!
//! # Example
//!
//! ```no_run
//! use choco_solver::*;
//!
//! fn main() {
//!     // Optional: point to directory containing `libchoco_capi.dll`.
//!     // ChocoBackend::set_dll_folder("C:/path/to/dll/folder".to_string());
//!
//!     let model = Model::new(Some("DemoModel"));
//!     let x = model.int_var_bounded(0, 200, Some("x"), None);
//!     let y = model.int_var_bounded(0, 200, Some("y"), None);
//!     let sum_is_156 = (&x + &y).eq(156).reify();
//!
//!     true.eq(&sum_is_156).post().expect("failed to post constraint");
//!
//!     let solver = model.solver();
//!     let solution = solver
//!         .find_solution(&Criterion::default())
//!         .expect("failed to find solution");
//!
//!     let bx = solution.get_int_var(&x).expect("x not available");
//!     let by = solution.get_int_var(&y).expect("y not available");
//!     let bsum = solution
//!         .get_bool_var(&sum_is_156)
//!         .expect("reified bool not available");
//!
//!     println!("solution: x = {bx}, y = {by}, x + y = {}, reified = {bsum}", bx + by);
//! }
//! ```

pub(crate) mod constraint;
pub(crate) mod model;
pub(crate) mod solution;
pub(crate) mod solver;
pub(crate) mod utils;
pub(crate) mod variables;

// Re-export main modules
pub use constraint::*;
pub use model::*;
pub use solution::*;
pub use solver::*;
pub use variables::*;

use choco_solver_sys::{graal_isolate_t, graal_isolatethread_t, libchoco_capi, library_filename};
use std::{cell::Cell, path::Path, process, ptr};
use thiserror::Error;
#[cfg(target_os = "windows")]
const CHOCO_SOLVER_LIB_NAME: &str = "libchoco_capi";
#[cfg(not(target_os = "windows"))]
const CHOCO_SOLVER_LIB_NAME: &str = "choco_capi";
thread_local! {
    pub(crate) static CHOCO_BACKEND: ChocoBackend = ChocoBackend::new();
    static DLL_FOLDER: Cell<Option<String>> = Cell::new(std::env::var("CHOCO_SOLVER_DLL_FOLDER").ok());
}

/// Errors that can occur during constraint solving.
#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SolverError {
    #[error("Tried to post a not free constraint or reified constraint")]
    NotFreeConstraint,
    #[error("Found contraddiction while propagating")]
    FoundContradiction,
    #[error("Cannot be converted to BoolVar: domain is not [0,1]")]
    BoolVarConversionError,
    #[error("unknown data store error")]
    Unknown,
}

/// Either an integer constant or an integer variable reference.
///
/// Used to allow functions to accept either a constant value or a variable.
pub(crate) enum IntOrIntVar<'a, 'model> {
    /// An integer variable.
    IntVar(&'a IntVar<'model>),
    /// An integer constant.
    Int(i32),
}

impl<'a, 'model: 'a> From<&'a IntVar<'model>> for IntOrIntVar<'a, 'model> {
    fn from(val: &'a IntVar<'model>) -> Self {
        IntOrIntVar::IntVar(val)
    }
}
impl From<i32> for IntOrIntVar<'_, '_> {
    fn from(val: i32) -> Self {
        IntOrIntVar::Int(val)
    }
}

trait Sealed {}
impl Sealed for BoolVar<'_> {}
impl Sealed for IntVar<'_> {}
impl Sealed for Constraint<'_> {}
impl<Q> Sealed for &Q {}
impl<Q: Sealed> Sealed for &[Q] {}

// :TODO: Unsafe code isolate in backend need to be sincronized
pub struct ChocoBackend {
    #[allow(dead_code)]
    isolate: *mut graal_isolate_t,
    thread: *mut graal_isolatethread_t,
    lib: libchoco_capi,
}

impl ChocoBackend {
    fn new() -> Self {
        let dll_folder = DLL_FOLDER.take();
        // # Safety:
        // Initializes the GraalVM isolate for the current thread. This is required before any interaction with the Choco backend. It is safe to call this function multiple times from the same thread, as it will only initialize once.
        unsafe {
            let lib = match dll_folder {
                Some(folder) => libchoco_capi::new(
                    Path::new(&folder).join(library_filename(CHOCO_SOLVER_LIB_NAME)),
                )
                .expect("Failed to load Choco C API library"),
                None => libchoco_capi::new(library_filename(CHOCO_SOLVER_LIB_NAME))
                    .expect("Failed to load Choco C API library"),
            };
            let mut isolate: *mut graal_isolate_t = ptr::null_mut();
            let mut thread: *mut graal_isolatethread_t = ptr::null_mut();
            if lib.graal_create_isolate(ptr::null_mut(), &mut isolate, &mut thread) != 0 {
                eprintln!("graal_create_isolate error");
                process::exit(1);
            }
            println!("GraalVM isolate created successfully");
            ChocoBackend {
                lib,
                isolate,
                thread,
            }
        }
    }
    /// Sets the folder path where the DLL files are located.
    /// # Arguments
    ///
    /// * `dll_folder_path` - A `String` representing the path to the folder containing the DLL files.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use choco_solver::ChocoBackend;
    /// ChocoBackend::set_dll_folder(String::from("C:/path/to/dll/folder"));
    /// ```
    ///
    /// # Note
    ///
    /// This function should be called before any interaction with the Choco backend,
    /// as it configures the directory from which the `libchoco_capi` library will be loaded.
    /// If not set, the library will be searched in folder specified by environment variable `CHOCO_SOLVER_DLL_FOLDER`.
    /// If the environment variable is not set, the library will be searched in the default system paths.
    pub fn set_dll_folder(dll_folder_path: String) {
        DLL_FOLDER.set(Some(dll_folder_path));
    }
}

#[allow(private_bounds)]
/// Trait for creating global constraints over arrays of variables.
///
/// Provides methods for constraints that operate on collections of variables,
/// such as all-different, all-equal, and cardinality constraints.
pub trait ArrayEqualityConstraints<'model>: Sealed {
    ///  Creates an allDifferent constraint, which ensures that all variables from vars take a different value.    /// # Panic:
    /// if slice is empty
    /// # Panic:
    /// if slice is empty
    fn all_different(self) -> Constraint<'model>;
    ///  Creates an allDifferent constraint for variables that are not equal to 0.
    ///  There can be multiple variables equal to 0.
    /// # Panic:
    /// if slice is empty
    fn all_different_except_0(self) -> Constraint<'model>;
    /// Creates an all_equal constraint.
    /// Ensures that all variables from vars take the same value.
    /// # Panic:
    /// if slice is empty
    fn all_equal(self) -> Constraint<'model>;
    /// Creates a not_all_equal constraint.
    /// Ensures that not all variables from vars take the same value.
    /// # Panic:
    /// if slice is empty
    fn not_all_equal(self) -> Constraint<'model>;
    /// Creates an at_least_n_value constraint.
    /// Let N be the number of distinct values assigned to the variables of the intvars collection.
    /// Enforce condition N >= n_values to hold.
    /// This embeds a light propagator by default.
    /// Additional filtering algorithms can be added.
    /// # Arguments
    /// * `n_values` - IntVar (limit variable).
    /// * `ac` - If True, add additional filtering algorithm, domain filtering algorithm derivated
    ///   from (Soft) AllDifferent.
    /// # Panic:
    /// if slice is empty
    fn at_least_n_value<'a>(self, n_values: &'a IntVar<'model>, ac: bool) -> Constraint<'model>
    where
        'model: 'a;
    ///  Creates an at_mostn_value constraint.
    ///  Let N be the number of distinct values assigned to the variables of the intvars collection.
    ///  Enforce condition N <= n_values to hold.
    ///  This embeds a light propagator by default. Additional filtering algorithms can be added.
    ///  # Arguments
    ///  * `n_values` - IntVar (limit variable).
    ///  * `strong` - "AMNV<Gci | MDRk | R13>" Filters the conjunction of AtMostNValue and inequalities
    ///    (see Fages and Lap&egrave;gue Artificial Intelligence 2014)
    ///    automatically detects inequalities and allDifferent constraints.
    ///    Presumably useful when nValues must be minimized.
    fn at_most_n_value<'a>(self, n_values: &IntVar<'model>, strong: bool) -> Constraint<'model>
    where
        'model: 'a;
}
