#![allow(clippy::indexing_slicing)]
mod criterion;
use crate::{
    CHOCO_BACKEND, Model, Solution, SolverError,
    utils::{Handle, HandleT, ModelObject},
};

pub use criterion::Criterion;
/// A solver instance associated with a [`Model`].
///
/// Provides solution search and propagation utilities for the model.
pub struct Solver<'model> {
    handle: Handle,
    model: &'model Model,
}

impl<'model> Solver<'model> {
    pub(crate) fn new(model: &'model Model) -> Self {
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // create solver function are guarantee to be called after chocosolver_init
        // because Model can be created only after that by the ChocoBackend lazy initialization.
        unsafe {
            let handle = backend.lib.Java_org_chocosolver_capi_ModelApi_getSolver(backend.thread, model.get_raw_handle());
            assert!(
                !handle.is_null(),
                "Failed to create solver: received null handle"
            );
            Solver {
                handle: Handle::new(handle),
                model,
            }
        })
    }

    /// Sets the time limit for the solver in milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if `time_limit_ms` is negative.
    pub fn set_time_limit(&self, time_limit_ms: i64) {
        assert!(
            !time_limit_ms.is_negative(),
            "Time limit must be non-negative"
        );
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // Safe because Solver instances are created from valid backend handles.
        unsafe {
            backend.lib.Java_org_chocosolver_capi_SolverApi_limit_time_ms(backend.thread, self.get_raw_handle(), time_limit_ms);
        })
    }

    #[must_use]
    pub fn find_solution(&self, criterions: &Criterion) -> Option<Solution> {
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // Safe because Solver instances are created from valid backend handles.
        unsafe {
            let criterion_array = criterion::make_criterion_var_array(criterions, self.model);
            let solution_handle =
                backend.lib.Java_org_chocosolver_capi_SolverApi_find_solution(backend.thread, self.get_raw_handle(), criterion_array);
            if solution_handle.is_null() {
                None
            } else {
                Some(Solution::new(solution_handle))
            }
        })
    }

    ///
    ///    Propagates constraints and related events through the constraint network until a fix point is find,
    ///    or a contradiction is detected.
    /// # Returns
    /// SolverError::FoundContradiction if a contradiction is detected during propagation.
    pub fn propagate(&self) -> Result<(), SolverError> {
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // Safe because Solver instances are created from valid backend handles.
        unsafe {
            if backend.lib.Java_org_chocosolver_capi_SolverApi_propagate(backend.thread, self.get_raw_handle()) != 0 {
                Ok(())
            } else {
                Err(SolverError::FoundContradiction)
            }
        })
    }
}

impl HandleT for Solver<'_> {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.handle.get_raw_handle()
    }
}

impl<'model> ModelObject<'model> for Solver<'model> {
    fn get_model(&self) -> &'model Model {
        self.model
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ModelObject;
    use crate::*;
    use crate::{EqualityOperator, Model, SolverError};
    #[test]
    fn test_solver_contradiction_propagate() {
        let model = Model::new(None);
        let solver = model.solver();
        let var1 = model.int_var_bounded(0, 10, Some("var1"), None);

        let constraint = var1.arithm(EqualityOperator::Eq, 25);
        constraint.post().unwrap();
        let result = solver.propagate();
        assert!(matches!(result, Err(SolverError::FoundContradiction)));
    }

    #[test]
    fn test_solver_model_object() {
        let model = Model::new(Some("TestModel"));
        let solver = model.solver();

        // Verify that solver.get_model() returns a reference to the same model
        let retrieved_model = solver.get_model();
        assert_eq!(retrieved_model.name().as_deref(), Some("TestModel"));
    }

    #[test]
    fn test_solver_time_limit() {
        let model = Model::new(Some("TimeLimitTest"));
        let vec_vars: Vec<_> = (0..=99)
            .map(|_| model.int_var_bounded(0, 100, None, None))
            .collect();
        vec_vars.all_different().post().unwrap();

        // Post constraints that require search

        assert!(
            vec_vars[0]
                .arithm(EqualityOperator::Gt, 50i32)
                .post()
                .is_ok()
        );
        assert!(
            vec_vars[1]
                .arithm(EqualityOperator::Lt, 50i32)
                .post()
                .is_ok()
        );
        assert!(
            vec_vars[2]
                .arithm(EqualityOperator::Eq, 25i32)
                .post()
                .is_ok()
        );

        let solver = model.solver();

        // Set time limit to 1 millisecond
        solver.set_time_limit(1);

        let criterion = Criterion::new();
        let solution = solver.find_solution(&criterion);

        // The solver should not find a solution due to the 1ms time limit
        assert!(solution.is_none());
    }
}
