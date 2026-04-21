use crate::utils::{Handle, HandleT};
use crate::variables::{IntVar, NewIntVarT};
use crate::{BoolVar, CHOCO_BACKEND, CHOCO_LIB};

/// A constraint programming model that owns variables and constraints.
///
/// Models are the root of object lifetimes and create variables, constraints,
/// and solvers associated with the same backend instance.
pub struct Model {
    handle: Handle,
}

impl Model {
    /// Creates a new Model instance.
    /// # Panics
    ///
    ///  if the string name contains a null byte.
    #[must_use]
    pub fn new(name: Option<&str>) -> Self {
        // Safety:
        // create model function are guarantee to be called after chocosolver_init
        // by the ChocoBackend lazy initialization.
        let model_handle = unsafe {
            match name {
                Some(n) => {
                    let c_name =
                        std::ffi::CString::new(n).expect("Failed to convert name to CString");
                    let model_handle = CHOCO_BACKEND.with(|backend| {
                        CHOCO_LIB.Java_org_chocosolver_capi_ModelApi_createModel_s(
                            backend.thread,
                            c_name.as_ptr().cast_mut(),
                        )
                    });

                    assert!(
                        !model_handle.is_null(),
                        "Failed to create model with name: received null handle"
                    );

                    model_handle
                }
                None => {
                    let model_handle = CHOCO_BACKEND.with(|backend| {
                        CHOCO_LIB.Java_org_chocosolver_capi_ModelApi_createModel(backend.thread)
                    });
                    assert!(
                        !model_handle.is_null(),
                        "Failed to create model: received null handle"
                    );
                    model_handle
                }
            }
        };
        Model {
            handle: Handle::new(model_handle),
        }
    }

    /// Returns the name of the model, if set.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        // Safety:
        // Safe because this can be called only after the model is created and therefore the backend is initialized.
        unsafe {
            let name_ptr = CHOCO_BACKEND.with(|backend| {
                CHOCO_LIB.Java_org_chocosolver_capi_ModelApi_getName(
                    backend.thread,
                    self.handle.get_raw_handle(),
                )
            });
            if name_ptr.is_null() {
                None
            } else {
                let c_str = std::ffi::CStr::from_ptr(name_ptr);
                Some(c_str.to_string_lossy().into_owned())
            }
        }
    }

    /// New integer variable associated with this model.
    /// x can be:
    /// - a single integer value (constant variable)
    /// - a slice of possible integer values
    #[must_use]
    #[allow(private_bounds)]
    pub fn int_var<'b, X>(&'b self, x: X, name: Option<&str>) -> IntVar<'b>
    where
        for<'a> (X, Option<&'a str>): NewIntVarT,
    {
        (x, name).create_int_var(self)
    }

    /// New bounded integer variable associated with this model.
    /// x and y shall be integer values
    /// bounded: Force bounded (True) or enumerated domain (False). If None, Choco will automatically choose the best option.
    #[must_use]
    #[allow(private_bounds)]
    pub fn int_var_bounded<'b, X, Y>(
        &'b self,
        x: X,
        y: Y,
        name: Option<&str>,
        bounded: Option<bool>,
    ) -> IntVar<'b>
    where
        for<'a> (X, Y, Option<&'a str>, bool): NewIntVarT,
        for<'a> (X, Y, Option<&'a str>): NewIntVarT,
    {
        match bounded {
            Some(v) => (x, y, name, v).create_int_var(self),
            None => (x, y, name).create_int_var(self),
        }
    }
    #[must_use]
    pub fn bool_var<'model>(
        &'model self,
        value: Option<bool>,
        name: Option<&str>,
    ) -> BoolVar<'model> {
        BoolVar::new(self, value, name)
    }

    #[must_use]
    pub fn solver(&self) -> crate::solver::Solver<'_> {
        crate::solver::Solver::new(self)
    }
}

impl HandleT for Model {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.handle.get_raw_handle()
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint::{ArithmeticOperator, EqualityOperator};

    use super::Model;

    #[test]
    fn test_model_basic() {
        let model = Model::new(Some("TestModel"));
        assert_eq!(model.name().as_deref(), Some("TestModel"));

        let unnamed_model = Model::new(None);

        let a = unnamed_model.int_var_bounded(-100, 100, Some("var1"), None);
        let b = unnamed_model.int_var_bounded(-100, 100, Some("var2"), None);
        let c = unnamed_model.int_var_bounded(0, 50, Some("var3"), None);
        assert!(a.arithm(EqualityOperator::Eq, 32i32).post().is_ok());
        // unnamed_model
        //     .arithm(&a, EqualityOperator::Eq, &b, ArithmeticOperator::Sum, 10)
        //     .post();

        // Constraint::arithm(&a, EqualityOperator::Eq, &b).post();

        assert!(
            a.arithm2(EqualityOperator::Eq, &b, ArithmeticOperator::Sum, &c)
                .post()
                .is_ok()
        );
        let solver = unnamed_model.solver();
        let solution = solver
            .find_solution(&Default::default())
            .expect("Expected to find a solution");
        println!(
            "Solution: var1 = {}, var2 = {}, var3 = {}",
            solution.get_int_var(&a).unwrap(),
            solution.get_int_var(&b).unwrap(),
            solution.get_int_var(&c).unwrap()
        );
    }
}
