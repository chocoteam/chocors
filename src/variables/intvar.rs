pub(crate) mod boolvar;
use super::{Display, HandleT, Variable};
use crate::constraint::{ArithmeticOperator, Constraint, EqualityOperator};
use crate::model::Model;
use crate::utils::{Handle, make_intvar_array};
use crate::utils::{ModelObject, get_int_array, make_int_array};
use crate::{
    ArithmConstraint, ArrayEqualityConstraints, ConstraintArithmT, ConstraintEquality, IntOrIntVar,
    Sealed,
};
use crate::{CHOCO_BACKEND, CHOCO_LIB};
pub use boolvar::*;

use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// An integer decision variable belonging to a [`Model`].
///
/// `IntVar` represents a domain of possible integer values and provides
/// accessors, views, arithmetic constraints, and reification helpers.
/// Instances are tied to the lifetime of their owning model.
pub struct IntVar<'model> {
    handle: Handle,
    model: &'model Model,
}

impl HandleT for IntVar<'_> {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.handle.get_raw_handle()
    }
}

impl<'model> ModelObject<'model> for IntVar<'model> {
    fn get_model(&self) -> &'model Model {
        self.model
    }
}

/// Safety:
/// - Safe because IntVar is created from Model and therefore the backend is initialized
///   and model is initialized.
unsafe impl<'model> Variable<'model> for IntVar<'model> {}

pub(crate) trait NewIntVarT {
    /// Creates an IntVar.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to which this variable belongs
    /// * `lb` - Lower bound  
    /// * `ub` - Upper bound
    /// * `name` - The name of the intvar (automatically given if None)
    /// * `bounded_domain` - Force bounded (true) or enumerated domain (false). If None, Choco will automatically choose the best option
    ///
    /// # Returns
    ///
    /// An IntVar instance
    fn create_int_var<'model>(&self, model: &'model Model) -> IntVar<'model>;
}

impl NewIntVarT for (i32, Option<&str>) {
    fn create_int_var<'model>(&self, model: &'model Model) -> IntVar<'model> {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        let handle = CHOCO_BACKEND.with(|backend| unsafe {
            match self.1 {
                Some(name) => {
                    let c_name =
                        std::ffi::CString::new(name).expect("Failed to convert name to CString");
                    CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_si(
                        backend.thread,
                        model.get_raw_handle(),
                        c_name.as_ptr().cast_mut(),
                        self.0,
                    )
                }
                None => CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_i(
                    backend.thread,
                    model.get_raw_handle(),
                    self.0,
                ),
            }
        });
        IntVar {
            handle: Handle::new(handle),
            model,
        }
    }
}

impl NewIntVarT for (&[i32], Option<&str>) {
    fn create_int_var<'model>(&self, model: &'model Model) -> IntVar<'model> {
        let vals = make_int_array(self.0);
        let handle = CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            match self.1 {
                Some(name) => {
                    let c_name =
                        std::ffi::CString::new(name).expect("Failed to convert name to CString");
                    CHOCO_LIB
                        .Java_org_chocosolver_capi_IntVarApi_intVar_s_arr(
                            backend.thread,
                            model.get_raw_handle(),
                            c_name.as_ptr().cast_mut(),
                            vals,
                        )
                }
                None => CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_arr(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                ),
            }
        });
        IntVar {
            handle: Handle::new(handle),
            model,
        }
    }
}

impl NewIntVarT for (i32, i32, Option<&str>) {
    fn create_int_var<'model>(&self, model: &'model Model) -> IntVar<'model> {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        let handle = CHOCO_BACKEND.with(|backend| unsafe {
            match self.2 {
                Some(name) => {
                    let c_name =
                        std::ffi::CString::new(name).expect("Failed to convert name to CString");
                    CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_sii(
                        backend.thread,
                        model.get_raw_handle(),
                        c_name.as_ptr().cast_mut(),
                        self.0,
                        self.1,
                    )
                }
                None => CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_ii(
                    backend.thread,
                    model.get_raw_handle(),
                    self.0,
                    self.1,
                ),
            }
        });
        IntVar {
            handle: Handle::new(handle),
            model,
        }
    }
}

impl NewIntVarT for (i32, i32, Option<&str>, bool) {
    fn create_int_var<'model>(&self, model: &'model Model) -> IntVar<'model> {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        let handle = CHOCO_BACKEND.with(|backend| unsafe {
            match self.2 {
                Some(name) => {
                    let c_name =
                        std::ffi::CString::new(name).expect("Failed to convert name to CString");
                    CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_siib(
                        backend.thread,
                        model.get_raw_handle(),
                        c_name.as_ptr().cast_mut(),
                        self.0,
                        self.1,
                        self.3.into(),
                    )
                }
                None => CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_iib(
                    backend.thread,
                    model.get_raw_handle(),
                    self.0,
                    self.1,
                    self.3.into(),
                ),
            }
        });
        IntVar {
            handle: Handle::new(handle),
            model,
        }
    }
}

impl<'model> IntVar<'model> {
    #[must_use]
    #[allow(private_bounds)]
    pub(crate) fn new<T: NewIntVarT>(model: &'model Model, args: T) -> Self {
        args.create_int_var(model)
    }
    #[must_use]
    pub fn lb(&self) -> i32 {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB
                .Java_org_chocosolver_capi_IntVarApi_getLB(backend.thread, self.get_raw_handle())
        })
    }
    #[must_use]
    pub fn ub(&self) -> i32 {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB
                .Java_org_chocosolver_capi_IntVarApi_getUB(backend.thread, self.get_raw_handle())
        })
    }

    #[must_use]
    pub fn value(&self) -> Option<i32> {
        if self.is_instantiated() {
            Some(CHOCO_BACKEND.with(|backend|
                // Safety:
                // Safe because IntVar is created from Model and therefore the backend is initialized, model handle is valid
                // and get_intvar_value can be called only if the variable is instantiated.
                unsafe {
                CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_getValue(
                    backend.thread,
                    self.get_raw_handle(),
                )
            }))
        } else {
            None
        }
    }
    #[must_use]
    pub fn has_enumerated_domain(&self) -> bool {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_hasEnumeratedDomain(
                backend.thread,
                self.get_raw_handle(),
            ) != 0
        })
    }

    #[must_use]
    pub fn get_domain_values(&self) -> Option<Vec<i32>> {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized, model handle is valid and
        // `get_domain_values` can be called only if the variable has enumerated domain.
        unsafe {
            if self.has_enumerated_domain() {
                let value_handle = CHOCO_BACKEND.with(|backend| {
                    CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_getDomainValues(
                        backend.thread,
                        self.get_raw_handle(),
                    )
                });
                let values = get_int_array(value_handle);
                Some(values)
            } else {
                None
            }
        }
    }

    pub(crate) fn int_offset_view(&self, offset: i32) -> IntVar<'model> {
        // Safety:
        // Safe because view is created from IntVar handle and therefore the backend is initialized.
        let view_handle = CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_offset_view(
                backend.thread,
                self.get_raw_handle(),
                offset,
            )
        });
        IntVar {
            handle: Handle::new(view_handle),
            model: self.model,
        }
    }
    pub(crate) fn int_scale_view(&self, scale: i32) -> IntVar<'model> {
        // Safety:
        // Safe because view is created from IntVar handle and therefore the backend is initialized.
        let view_handle = CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_scale_view(
                backend.thread,
                self.get_raw_handle(),
                scale,
            )
        });
        IntVar {
            handle: Handle::new(view_handle),
            model: self.model,
        }
    }
    pub(crate) fn int_minus_view(&self) -> IntVar<'model> {
        // Safety:
        // Safe because view is created from IntVar handle and therefore the backend is initialized.
        let view_handle = CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_minus_view(
                backend.thread,
                self.get_raw_handle(),
            )
        });
        IntVar {
            handle: Handle::new(view_handle),
            model: self.model,
        }
    }
    /// Creates a boolean view representing the equality of the integer variable to a given value.
    #[must_use]
    pub fn eq_view(&self, value: i32) -> BoolVar<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because view is created from IntVar handle and therefore the backend is initialized.
            unsafe {
            let view_handle = CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_eq_view(
                backend.thread,
                self.get_raw_handle(),
                value,
            );
            BoolVar::from_raw_handle(view_handle, self.model)
        })
    }
    /// Creates a boolean view representing the "greater than or equal to" relation of the integer variable to a given value.
    #[must_use]
    pub fn ge_view(&self, value: i32) -> BoolVar<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because view is created from IntVar handle and therefore the backend is initialized.
            unsafe {
            let view_handle = CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_ge_view(
                backend.thread,
                self.get_raw_handle(),
                value,
            );
            BoolVar::from_raw_handle(view_handle, self.model)})
    }
    /// Creates a boolean view representing the "less than or equal to" relation of the integer variable to a given value.
    #[must_use]
    pub fn le_view(&self, value: i32) -> BoolVar<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because view is created from IntVar handle and therefore the backend is initialized.
            unsafe {
            let view_handle =CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_le_view(
                backend.thread,
                self.get_raw_handle(),
                value,
            );
            BoolVar::from_raw_handle(view_handle, self.model)
        })
    }
    /// Creates a boolean view representing the "not equal to" relation of the integer variable to a given value.
    #[must_use]
    pub fn ne_view(&self, value: i32) -> BoolVar<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because view is created from IntVar handle and therefore the backend is initialized.
            unsafe {
            let view_handle =CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_int_ne_view(
                backend.thread,
                self.get_raw_handle(),
                value,
            );
            BoolVar::from_raw_handle(view_handle, self.model)
        })
    }

    #[must_use]
    #[allow(private_bounds)]
    pub fn arithm<'a, 'b, OP, Y>(&'b self, op: OP, y: Y) -> Constraint<'model>
    where
        (&'a IntVar<'model>, OP, Y): ConstraintArithmT<'model>,
        'model: 'a,
        'model: 'b,
        'b: 'a,
        Y: 'b,
    {
        (self, op, y).create()
    }

    /// Creates a two-operator arithmetic constraint associated with this model.
    #[must_use]
    #[allow(private_bounds)]
    pub fn arithm2<OP, Y, OP2, Z>(&self, op: OP, y: Y, op2: OP2, z: Z) -> Constraint<'model>
    where
        for<'a> (&'a IntVar<'model>, OP, Y, OP2, Z): ConstraintArithmT<'model>,
    {
        (self, op, y, op2, z).create()
    }

    /// Posts a constraint that expresses: (self = y) <=> (b = true)
    /// This bypass reification system.
    #[allow(private_bounds)]
    pub fn reify_eq_y<'a>(&self, y: impl Into<IntOrIntVar<'a, 'model>>, b: &BoolVar<'model>)
    where
        'model: 'a,
    {
        let y_coverted: IntOrIntVar<'_, 'model> = y.into();
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle
        CHOCO_BACKEND.with(|backend| unsafe {
            match y_coverted {
                IntOrIntVar::IntVar(y_var) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_eq_y(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_var.get_raw_handle(),
                        b.get_raw_handle(),
                    ),
                IntOrIntVar::Int(y_int) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_eq_c(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_int,
                        b.get_raw_handle(),
                    ),
            }
        });
    }

    /// Posts a constraint that expresses: (self != y) <=> (b = true)
    /// This bypass reification system.
    #[allow(private_bounds)]
    pub fn reify_ne_y<'a>(&self, y: impl Into<IntOrIntVar<'a, 'model>>, b: &BoolVar<'model>)
    where
        'model: 'a,
    {
        let y_coverted: IntOrIntVar<'_, 'model> = y.into();
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            match y_coverted {
                IntOrIntVar::IntVar(y_var) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_ne_y(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_var.get_raw_handle(),
                        b.get_raw_handle(),
                    ),
                IntOrIntVar::Int(y_int) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_ne_c(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_int,
                        b.get_raw_handle(),
                    ),
            }
        });
    }

    /// Posts a constraint that expresses: (self < y) <=> (b = true)
    /// This bypass reification system.
    #[allow(private_bounds)]
    pub fn reify_lt_y<'a>(&self, y: impl Into<IntOrIntVar<'a, 'model>>, b: &BoolVar<'model>)
    where
        'model: 'a,
    {
        let y_coverted: IntOrIntVar<'_, 'model> = y.into();
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            match y_coverted {
                IntOrIntVar::IntVar(y_var) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_lt_y(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_var.get_raw_handle(),
                        b.get_raw_handle(),
                    ),
                IntOrIntVar::Int(y_int) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_lt_c(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_int,
                        b.get_raw_handle(),
                    ),
            }
        });
    }

    /// Posts a constraint that expresses: (self > y) <=> (b = true)
    /// This bypass reification system.
    #[allow(private_bounds)]
    pub fn reify_gt_y<'a>(&self, y: impl Into<IntOrIntVar<'a, 'model>>, b: &BoolVar<'model>)
    where
        'model: 'a,
    {
        let y_coverted: IntOrIntVar<'_, 'model> = y.into();
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            match y_coverted {
                IntOrIntVar::IntVar(y_var) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_gt_y(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_var.get_raw_handle(),
                        b.get_raw_handle(),
                    ),
                IntOrIntVar::Int(y_int) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_gt_c(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_int,
                        b.get_raw_handle(),
                    ),
            }
        });
    }

    /// Posts a constraint that expresses: (self <= y) <=> (b = true)
    /// This bypass reification system.
    #[allow(private_bounds)]
    pub fn reify_le_y<'a>(&self, y: impl Into<IntOrIntVar<'a, 'model>>, b: &BoolVar<'model>)
    where
        'model: 'a,
    {
        let y_coverted: IntOrIntVar<'_, 'model> = y.into();
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            match y_coverted {
                IntOrIntVar::IntVar(y_var) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_le_y(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_var.get_raw_handle(),
                        b.get_raw_handle(),
                    ),
                IntOrIntVar::Int(y_int) => {
                    // For integer constants, x <= y is equivalent to x < y + 1
                    CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reify_x_lt_c(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_int + 1,
                        b.get_raw_handle(),
                    )
                }
            }
        });
    }

    /// Posts a constraint that expresses: (self >= y) <=> (b = true)
    /// This bypass reification system.
    #[allow(private_bounds)]
    pub fn reify_ge_y<'a>(&self, y: impl Into<IntOrIntVar<'a, 'model>>, b: &BoolVar<'model>)
    where
        'model: 'a,
    {
        let y_coverted: IntOrIntVar<'_, 'model> = y.into();
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            match y_coverted {
                IntOrIntVar::IntVar(y_var) => CHOCO_LIB
                    .Java_org_chocosolver_capi_ReificationApi_reify_x_ge_y(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_var.get_raw_handle(),
                        b.get_raw_handle(),
                    ),
                IntOrIntVar::Int(y_int) => {
                    // For integer constants, x >= y is equivalent to x > y - 1
                    CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reify_x_gt_c(
                        backend.thread,
                        self.get_model().get_raw_handle(),
                        self.get_raw_handle(),
                        y_int - 1,
                        b.get_raw_handle(),
                    )
                }
            }
        });
    }

    ///  Posts a constraint that expresses : (self = y + c) <=> (b is true).
    /// This bypass reification system.
    pub fn reify_eq_yc(&self, y: &IntVar<'model>, c: i32, b: &BoolVar<'model>) {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reify_x_eq_yc(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                y.get_raw_handle(),
                c,
                b.get_raw_handle(),
            )
        });
    }

    ///  Posts a constraint that expresses : (self != y + c) <=> (b is true).
    /// This bypass reification system.
    pub fn reify_ne_yc(&self, y: &IntVar<'model>, c: i32, b: &BoolVar<'model>) {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reify_x_ne_yc(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                y.get_raw_handle(),
                c,
                b.get_raw_handle(),
            )
        });
    }

    ///  Posts a constraint that expresses : (self < y + c) <=> b.
    /// This bypass reification system.
    pub fn reify_lt_yc(&self, y: &IntVar<'model>, c: i32, b: &BoolVar<'model>) {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reify_x_lt_yc(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                y.get_raw_handle(),
                c,
                b.get_raw_handle(),
            )
        });
    }

    ///  Posts a constraint that expresses : (self > y + c) <=> b.
    /// This bypass reification system.
    pub fn reify_gt_yc(&self, y: &IntVar<'model>, c: i32, b: &BoolVar<'model>) {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reify_x_gt_yc(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                y.get_raw_handle(),
                c,
                b.get_raw_handle(),
            )
        });
    }

    ///  Creates a member constraint. Ensures `self` takes its values in `table`.
    #[must_use]
    pub fn member_table(&self, table: &[i32]) -> Constraint<'model> {
        let vals = make_int_array(table);
        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_member_iv_iarray(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    vals,
                );
            assert!(!ptr.is_null(), "Invalid parameters for member constraint");
            Constraint::new(ptr, self.get_model())})
    }
    ///  Creates a member constraint. Ensures `self` takes its values in [`lb`, `ub`].
    #[must_use]
    pub fn member_bounds(&self, lb: i32, ub: i32) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_member_iv_i_i(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    lb,
                    ub,
                );
            assert!(!ptr.is_null(), "Invalid parameters for member constraint");
            Constraint::new(ptr, self.get_model())})
    }
    /// Create not a member constraint. Ensures `self` does not take its values in `table`.
    #[must_use]
    pub fn not_member_table(&self, table: &[i32]) -> Constraint<'model> {
        let vals = make_int_array(table);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_not_member_iv_iarray(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    vals,
                );
                 assert!(
            !ptr.is_null(),
            "Invalid parameters for not member constraint"
        );
         Constraint::new(ptr, self.get_model())})
    }
    /// Create not a member constraint. Ensures `self` does not take its values in [`lb`, `ub`].
    #[must_use]
    pub fn not_member_bounds(&self, lb: i32, ub: i32) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_not_member_iv_i_i(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    lb,
                    ub,
                );
                assert!(
            !ptr.is_null(),
            "Invalid parameters for not member constraint"
        );
         Constraint::new(ptr, self.get_model())})
    }
    /// Creates an absolute value constraint: `self` = | y |
    #[must_use]
    pub fn abs(&self, y: &IntVar<'model>) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
             let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_absolute(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    y.get_raw_handle(),
                );
                assert!(!ptr.is_null(), "Invalid parameters for abs constraint");
        Constraint::new(ptr, self.get_model())})
    }
    ///  Creates a square constraint: `self` = y^2.
    #[must_use]
    pub fn square(&self, y: &IntVar<'model>) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr =CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_square(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                y.get_raw_handle(),
            );
            assert!(!ptr.is_null(), "Invalid parameters for square constraint");
        Constraint::new(ptr, self.get_model())})
    }
    /// Creates a power constraint: `self`^c = y
    #[must_use]
    pub fn pow(&self, c: i32, y: &IntVar<'model>) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_pow(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                c,
                y.get_raw_handle(),
            );
            assert!(!ptr.is_null(), "Invalid parameters for power constraint");
            Constraint::new(ptr, self.get_model())
        })
    }
    ///  Creates a maximum constraint, `self` is the maximum value among IntVars in intvars.
    #[must_use]
    pub fn max(&self, intvars: &[&IntVar<'model>]) -> Constraint<'model> {
        let vals = make_intvar_array(intvars);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_max_iv_ivarray(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    vals,
                );
                assert!(!ptr.is_null(), "Invalid parameters for max constraint");
                Constraint::new(ptr, self.get_model())
        })
    }
    ///  Creates a minimum constraint, `self` is the minimum value among IntVars in intvars.
    #[must_use]
    pub fn min(&self, intvars: &[&IntVar<'model>]) -> Constraint<'model> {
        let vals = make_intvar_array(intvars);
        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
             let ptr =CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_min_iv_ivarray(
                    backend.thread,
                    self.get_model().get_raw_handle(),
                    self.get_raw_handle(),
                    vals,
                );
            assert!(!ptr.is_null(), "Invalid parameters for min constraint");
            Constraint::new(ptr, self.get_model())})
    }
    /// Creates an among constraint.
    /// `self` is the number of variables of the collection `intvars` that take their value in `values`.
    ///
    ///   Propagator :
    ///
    ///    C. Bessiere, E. Hebrard, B. Hnich, Z. Kiziltan, T. Walsh, Among, common and disjoint Constraints CP-2005
    #[must_use]
    pub fn among(&self, intvars: &[&IntVar<'model>], values: &[i32]) -> Constraint<'model> {
        let intvar_vals = make_intvar_array(intvars);
        let vals = make_int_array(values);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_among(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                intvar_vals,
                vals,
            );
                assert!(!ptr.is_null(), "Invalid parameters for among constraint");
                Constraint::new(ptr, self.get_model())
        })
    }
}

impl Debug for IntVar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.has_enumerated_domain() {
            let values = self.get_domain_values().unwrap();
            write!(
                f,
                "IntVar {{ name: {:?}, lb: {}, ub: {}, is_instantiated: {}, is_view: {}, domain_values: {:?} }}",
                self.name(),
                self.lb(),
                self.ub(),
                self.is_instantiated(),
                self.is_view(),
                values
            )
        } else {
            write!(
                f,
                "IntVar {{ name: {:?}, lb: {}, ub: {}, is_instantiated: {}, is_view: {} }}",
                self.name(),
                self.lb(),
                self.ub(),
                self.is_instantiated(),
                self.is_view()
            )
        }
    }
}

impl<'model> Add<&IntVar<'model>> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn add(self, rhs: &IntVar<'model>) -> IntVar<'model> {
        let var = IntVar::new(
            self.model,
            (self.lb() + rhs.lb(), self.ub() + rhs.ub(), None),
        );

        debug_assert_eq!(
            self.arithm2(ArithmeticOperator::Sum, rhs, EqualityOperator::Eq, &var,)
                .post(),
            Ok(())
        );
        var
    }
}

impl<'model> Add<i32> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn add(self, rhs: i32) -> IntVar<'model> {
        self.int_offset_view(rhs)
    }
}

impl<'model> Sub<&IntVar<'model>> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn sub(self, rhs: &IntVar<'model>) -> IntVar<'model> {
        let var = IntVar::new(
            self.model,
            (self.lb() - rhs.ub(), self.ub() - rhs.lb(), None),
        );
        debug_assert_eq!(
            self.arithm2(ArithmeticOperator::Sub, rhs, EqualityOperator::Eq, &var,)
                .post(),
            Ok(())
        );
        var
    }
}

impl<'model> Sub<i32> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn sub(self, rhs: i32) -> IntVar<'model> {
        self.int_offset_view(-rhs)
    }
}

impl<'model> Mul<&IntVar<'model>> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn mul(self, rhs: &IntVar<'model>) -> IntVar<'model> {
        let candidates = [
            self.lb() * rhs.lb(),
            self.lb() * rhs.ub(),
            self.ub() * rhs.lb(),
            self.ub() * rhs.ub(),
        ];
        let (min_val, max_val) = candidates
            .iter()
            .fold((i32::MAX, i32::MIN), |acc, &v| (acc.0.min(v), acc.1.max(v)));
        let var = IntVar::new(self.model, (min_val, max_val, None));

        debug_assert_eq!(
            self.arithm2(ArithmeticOperator::Mul, rhs, EqualityOperator::Eq, &var,)
                .post(),
            Ok(())
        );
        var
    }
}

impl<'model> Mul<i32> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn mul(self, rhs: i32) -> IntVar<'model> {
        self.int_scale_view(rhs)
    }
}

impl<'model> Div<&IntVar<'model>> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn div(self, rhs: &IntVar<'model>) -> IntVar<'model> {
        let candidates = [
            self.lb() / rhs.lb(),
            self.lb() / rhs.ub(),
            self.ub() / rhs.lb(),
            self.ub() / rhs.ub(),
        ];
        let (min_val, max_val) = candidates
            .iter()
            .fold((i32::MAX, i32::MIN), |acc, &v| (acc.0.min(v), acc.1.max(v)));
        let var = IntVar::new(self.model, (min_val, max_val, None));

        debug_assert_eq!(
            self.arithm2(ArithmeticOperator::Div, rhs, EqualityOperator::Eq, &var,)
                .post(),
            Ok(())
        );
        var
    }
}

impl<'model> Div<i32> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn div(self, rhs: i32) -> IntVar<'model> {
        let candidates = [self.lb() / rhs, self.ub() / rhs];
        let (min_val, max_val) = candidates
            .iter()
            .fold((i32::MAX, i32::MIN), |acc, &v| (acc.0.min(v), acc.1.max(v)));
        let var = IntVar::new(self.model, (min_val, max_val, None));

        debug_assert_eq!(
            self.arithm2(ArithmeticOperator::Div, rhs, EqualityOperator::Eq, &var,)
                .post(),
            Ok(())
        );
        var
    }
}

impl<'model> Neg for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn neg(self) -> IntVar<'model> {
        self.int_minus_view()
    }
}

impl<'model> Rem<&IntVar<'model>> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn rem(self, rhs: &IntVar<'model>) -> IntVar<'model> {
        assert!(
            rhs.lb() != 0 && rhs.ub() != 0,
            "Remainder by zero is not allowed"
        );
        let candidates = [
            self.lb() % rhs.lb(),
            self.lb() % rhs.ub(),
            self.ub() % rhs.lb(),
            self.ub() % rhs.ub(),
        ];
        let (min_val, max_val) = candidates
            .iter()
            .fold((i32::MAX, i32::MIN), |acc, &v| (acc.0.min(v), acc.1.max(v)));
        let var = IntVar::new(self.model, (min_val, max_val, None));

        debug_assert_eq!(Ok(()), self.modulo(rhs, &var).post());
        var
    }
}

impl<'model> Rem<i32> for &IntVar<'model> {
    type Output = IntVar<'model>;

    fn rem(self, rhs: i32) -> IntVar<'model> {
        assert!(rhs != 0, "Remainder by zero is not allowed");
        let candidates = [self.lb() % rhs, self.ub() % rhs];
        let (min_val, max_val) = candidates
            .iter()
            .fold((i32::MAX, i32::MIN), |acc, &v| (acc.0.min(v), acc.1.max(v)));
        let var = IntVar::new(self.model, (min_val, max_val, None));
        debug_assert_eq!(Ok(()), self.modulo(rhs, &var).post());
        var
    }
}

impl<'model, A> ConstraintEquality<'model, i32> for &A
where
    A: Borrow<IntVar<'model>>,
{
    fn eq(self, other: i32) -> Constraint<'model> {
        self.borrow().arithm(EqualityOperator::Eq, other)
    }

    fn ne(self, other: i32) -> Constraint<'model> {
        self.borrow().arithm(EqualityOperator::Neq, other)
    }
}

impl<'model, A> ConstraintEquality<'model, bool> for &A
where
    A: Borrow<IntVar<'model>>,
{
    fn eq(self, other: bool) -> Constraint<'model> {
        self.borrow().arithm(EqualityOperator::Eq, i32::from(other))
    }

    fn ne(self, other: bool) -> Constraint<'model> {
        self.borrow()
            .arithm(EqualityOperator::Neq, i32::from(other))
    }
}

impl<'model, A, B> ConstraintEquality<'model, &A> for &B
where
    A: Borrow<IntVar<'model>>,
    B: Borrow<IntVar<'model>>,
{
    fn eq(self, other: &A) -> Constraint<'model> {
        self.borrow().arithm(EqualityOperator::Eq, other.borrow())
    }

    fn ne(self, other: &A) -> Constraint<'model> {
        self.borrow().arithm(EqualityOperator::Neq, other.borrow())
    }
}

impl<'model, A> ConstraintEquality<'model, &A> for bool
where
    A: Borrow<IntVar<'model>>,
{
    fn eq(self, other: &A) -> Constraint<'model> {
        other.eq(self)
    }

    fn ne(self, other: &A) -> Constraint<'model> {
        other.ne(self)
    }
}

impl<'model, A> ConstraintEquality<'model, &A> for i32
where
    A: Borrow<IntVar<'model>>,
{
    fn eq(self, other: &A) -> Constraint<'model> {
        other.eq(self)
    }

    fn ne(self, other: &A) -> Constraint<'model> {
        other.ne(self)
    }
}

impl<'model> ArithmConstraint<'model, &IntVar<'model>, &IntVar<'model>> for IntVar<'model> {
    fn modulo(&'model self, modulo: &IntVar<'model>, res: &IntVar<'model>) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            // Also, the handles passed are valid IntVar handles.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_mod_iv_iv_iv(
                    backend.thread,
                    self.model.get_raw_handle(),
                    self.get_raw_handle(),
                    modulo.get_raw_handle(),
                    res.get_raw_handle(),
                );
            assert!(
            !ptr.is_null(),
            "Invalid parameters combination for mod constraint"
        );
         Constraint::new(ptr, self.model)})
    }
}

impl<'model> ArithmConstraint<'model, i32, i32> for IntVar<'model> {
    fn modulo(&'model self, modulo: i32, res: i32) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            // Also, the handles passed are valid IntVar handles.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_mod_iv_i_i(
                    backend.thread,
                    self.model.get_raw_handle(),
                    self.get_raw_handle(),
                    modulo,
                    res,
                );
                assert!(
            !ptr.is_null(),
            "Invalid parameters combination for mod constraint"
        );
        Constraint::new(ptr, self.model)})
    }
}

impl<'model> ArithmConstraint<'model, i32, &IntVar<'model>> for IntVar<'model> {
    fn modulo(&'model self, modulo: i32, res: &IntVar<'model>) -> Constraint<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            // Also, the handles passed are valid IntVar handles.
            unsafe {
            let ptr =CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_mod_iv_i_iv(
                    backend.thread,
                    self.model.get_raw_handle(),
                    self.get_raw_handle(),
                    modulo,
                    res.get_raw_handle(),
                );
                assert!(
            !ptr.is_null(),
            "Invalid parameters combination for mod constraint"
        );
        Constraint::new(ptr, self.model)
    })
    }
}

impl<'model, Q: Borrow<IntVar<'model>> + Sealed> ArrayEqualityConstraints<'model> for &[Q] {
    /// Creates an all different constraint over a slice of integer variables.
    fn all_different(self) -> Constraint<'model> {
        assert!(!self.is_empty(), "intvars slice cannot be empty");

        let model = self.first().unwrap().borrow().get_model();
        let vals = make_intvar_array(self);
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        unsafe {
              let ptr =CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_allDifferent(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                );
                debug_assert!(
            !ptr.is_null(),
            "Invalid parameters for all different constraint"
        );
        Constraint::new(ptr, model)
    })
    }
    fn all_different_except_0(self) -> Constraint<'model> {
        assert!(!self.is_empty(), "intvars slice cannot be empty");

        let model = self.first().unwrap().borrow().get_model();
        let vals = make_intvar_array(self);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr =CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_allDifferentExcept0(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                );
                assert!(
            !ptr.is_null(),
            "Invalid parameters for all different except 0 constraint"
        );
        Constraint::new(ptr, model)})
    }
    fn all_equal(self) -> Constraint<'model> {
        assert!(!self.is_empty(), "intvars slice cannot be empty");

        let model = self.first().unwrap().borrow().get_model();
        let vals = make_intvar_array(self);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_all_equal(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                );
                debug_assert!(
            !ptr.is_null(),
            "Invalid parameters for all equal constraint"
        );
        Constraint::new(ptr, model)
    })
    }
    fn not_all_equal(self) -> Constraint<'model> {
        assert!(!self.is_empty(), "intvars slice cannot be empty");

        let model = self.first().unwrap().borrow().get_model();
        let vals = make_intvar_array(self);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_not_all_equal(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                );
                 debug_assert!(
            !ptr.is_null(),
            "Invalid parameters for not all equal constraint"
        );
        Constraint::new(ptr, model)
            })
    }
    fn at_least_n_value<'a>(self, n_values: &'a IntVar<'model>, ac: bool) -> Constraint<'model>
    where
        'model: 'a,
    {
        assert!(!self.is_empty(), "intvars slice cannot be empty");

        let model = self.first().unwrap().borrow().get_model();
        let vals = make_intvar_array(self);
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
        unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_atLeastNValues(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                    n_values.get_raw_handle(),
                    ac.into(),
                );
                debug_assert!(
            !ptr.is_null(),
            "Invalid parameters for at least n value constraint"
        );
        Constraint::new(ptr, model)
    })
    }
    fn at_most_n_value<'a>(self, n_values: &IntVar<'model>, strong: bool) -> Constraint<'model>
    where
        'model: 'a,
    {
        assert!(!self.is_empty(), "intvars slice cannot be empty");

        let model = self.first().unwrap().borrow().get_model();
        let vals = make_intvar_array(self);
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because IntVar is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let ptr = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_atMostNValues(
                    backend.thread,
                    model.get_raw_handle(),
                    vals,
                    n_values.get_raw_handle(),
                    strong.into(),
                );
                 debug_assert!(
            !ptr.is_null(),
            "Invalid parameters for at most n value constraint"
        );
        Constraint::new(ptr, model)
    })
    }
}

impl Display for IntVar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.has_enumerated_domain() {
            let values = self.get_domain_values().unwrap();
            write!(
                f,
                "IntVar {{ name: {:?}, lb: {}, ub: {}, is_instantiated: {}, is_view: {}, domain_values: {:?} }}",
                self.name(),
                self.lb(),
                self.ub(),
                self.is_instantiated(),
                self.is_view(),
                values
            )
        } else {
            write!(
                f,
                "IntVar {{ name: {:?}, lb: {}, ub: {}, is_instantiated: {}, is_view: {} }}",
                self.name(),
                self.lb(),
                self.ub(),
                self.is_instantiated(),
                self.is_view()
            )
        }
    }
}

#[cfg(test)]
mod tests;
