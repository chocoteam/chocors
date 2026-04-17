use crate::{CHOCO_BACKEND, CHOCO_LIB, Model, utils::HandleT};

/// Search limits used when finding solutions (e.g., node, fail, restart, backtrack).
#[derive(Debug, Clone, Default)]
pub struct Criterion {
    node_limit: Option<i64>,
    fail_limit: Option<i64>,
    restart_limit: Option<i64>,
    backtrack_limit: Option<i64>,
}

impl Criterion {
    /// Creates a new empty Criterion.
    #[must_use]
    pub fn new() -> Self {
        Criterion::default()
    }

    /// Sets a node limit for the criterion.
    #[must_use]
    pub fn with_node_limit(mut self, limit: i64) -> Self {
        self.node_limit = Some(limit);
        self
    }

    /// Sets a fail limit for the criterion.
    #[must_use]
    pub fn with_fail_limit(mut self, limit: i64) -> Self {
        self.fail_limit = Some(limit);
        self
    }

    /// Sets a restart limit for the criterion.
    #[must_use]
    pub fn with_restart_limit(mut self, limit: i64) -> Self {
        self.restart_limit = Some(limit);
        self
    }

    /// Sets a backtrack limit for the criterion.
    #[must_use]
    pub fn with_backtrack_limit(mut self, limit: i64) -> Self {
        self.backtrack_limit = Some(limit);
        self
    }
}

pub(super) fn make_criterion_var_array(
    criterions: &Criterion,
    model: &Model,
) -> *mut std::os::raw::c_void {
    CHOCO_BACKEND.with(|backend| {
        // Safety:
        // Safe because Criterion instances are created from valid backend handles.
        unsafe {
            let Criterion {
                node_limit,
                fail_limit,
                restart_limit,
                backtrack_limit,
            } = *criterions;

            let mut raw_criterions = vec![];
            if let Some(node_limit) = node_limit {
                assert!(!node_limit.is_negative(), "Node limit must be non-negative");

                raw_criterions.push(
                    CHOCO_LIB.Java_org_chocosolver_capi_CriterionApi_node_counter(
                        backend.thread,
                        model.get_raw_handle(),
                        node_limit,
                    ),
                );
            };
            if let Some(fail_limit) = fail_limit {
                // Safety:
                // Safe because Criterion instances are created from valid backend handles.
                assert!(!fail_limit.is_negative(), "Fail limit must be non-negative");

                raw_criterions.push(
                    CHOCO_LIB.Java_org_chocosolver_capi_CriterionApi_fail_counter(
                        backend.thread,
                        model.get_raw_handle(),
                        fail_limit,
                    ),
                );
            };
            if let Some(restart_limit) = restart_limit {
                // Safety:
                // Safe because Criterion instances are created from valid backend handles.
                assert!(
                    !restart_limit.is_negative(),
                    "Restart limit must be non-negative"
                );

                raw_criterions.push(
                    CHOCO_LIB.Java_org_chocosolver_capi_CriterionApi_restart_counter(
                        backend.thread,
                        model.get_raw_handle(),
                        restart_limit,
                    ),
                );
            };
            if let Some(backtrack_limit) = backtrack_limit {
                // Safety:
                // Safe because Criterion instances are created from valid backend handles.
                assert!(
                    !backtrack_limit.is_negative(),
                    "Backtrack limit must be non-negative"
                );

                raw_criterions.push(
                    CHOCO_LIB.Java_org_chocosolver_capi_CriterionApi_backtrack_counter(
                        backend.thread,
                        model.get_raw_handle(),
                        backtrack_limit,
                    ),
                );
            };
            let len_i32: i32 = raw_criterions
                .len()
                .try_into()
                .expect("Criterion array length exceeds i32");
            let criterion_array = CHOCO_LIB
                .Java_org_chocosolver_capi_ArrayApi_criterion_create(backend.thread, len_i32);
            for (i, criterion) in raw_criterions.iter().enumerate() {
                #[allow(
                    clippy::cast_possible_truncation,
                    reason = "Length checked to fit in i32"
                )]
                #[allow(clippy::cast_possible_wrap, reason = "Length checked to fit in i32")]
                CHOCO_LIB.Java_org_chocosolver_capi_ArrayApi_criterion_set(
                    backend.thread,
                    criterion_array,
                    *criterion,
                    i as i32,
                );
            }
            criterion_array
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criterion_all_methods() {
        let criterion = Criterion::new()
            .with_node_limit(1000)
            .with_fail_limit(500)
            .with_restart_limit(100)
            .with_backtrack_limit(200);

        assert_eq!(criterion.node_limit, Some(1000));
        assert_eq!(criterion.fail_limit, Some(500));
        assert_eq!(criterion.restart_limit, Some(100));
        assert_eq!(criterion.backtrack_limit, Some(200));
    }

    #[test]
    fn test_criterion_restrictive_limits() {
        use crate::{Model, constraint::EqualityOperator};

        // Create a model with 3 variables and constraints
        let model = Model::new(Some("RestrictiveTest"));
        let x = model.int_var_bounded(0, 10, Some("x"), None);
        let y = model.int_var_bounded(0, 10, Some("y"), None);
        let z = model.int_var_bounded(0, 10, Some("z"), None);

        // Post constraints that require search to solve
        assert!(x.arithm(EqualityOperator::Lt, 5i32).post().is_ok());
        assert!(y.arithm(EqualityOperator::Gt, 3i32).post().is_ok());
        assert!(z.arithm(EqualityOperator::Neq, 7i32).post().is_ok());

        // Create a solver with very restrictive criteria
        let solver = model.solver();
        let criterion = Criterion::new()
            .with_node_limit(1)
            .with_fail_limit(0)
            .with_restart_limit(0)
            .with_backtrack_limit(0);

        // Try to solve with restrictive limits
        let solution = solver.find_solution(&criterion);

        // The solver should not find a solution due to the restrictive limits
        assert!(solution.is_none());
    }
}
