use std::borrow::Borrow;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Not;

use super::{Handle, HandleT};
use crate::CHOCO_LIB;
use crate::Sealed;
use crate::SolverError;

use crate::CHOCO_BACKEND;
use crate::constraint::Constraint;
use crate::model::Model;
use crate::utils;
use crate::utils::ModelObject;
use crate::variables::IntVar;
use crate::variables::Variable;

/// A boolean decision variable (domain 0/1) belonging to a [`Model`].
///
/// `BoolVar` is a thin wrapper over an underlying [`IntVar`] whose domain
/// is restricted to `[0, 1]` and supports boolean and logical operations.
pub struct BoolVar<'model> {
    int_var: IntVar<'model>,
}

impl HandleT for BoolVar<'_> {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.int_var.get_raw_handle()
    }
}

impl<'model> ModelObject<'model> for BoolVar<'model> {
    fn get_model(&self) -> &'model Model {
        self.int_var.get_model()
    }
}

impl<'model> From<BoolVar<'model>> for IntVar<'model> {
    fn from(bool_var: BoolVar<'model>) -> Self {
        bool_var.int_var
    }
}

impl<'model> TryFrom<IntVar<'model>> for BoolVar<'model> {
    type Error = SolverError;

    fn try_from(int_var: IntVar<'model>) -> Result<Self, Self::Error> {
        let lb = int_var.lb();
        let ub = int_var.ub();
        if lb == 0 && ub == 1 {
            Ok(BoolVar { int_var })
        } else {
            Err(SolverError::BoolVarConversionError)
        }
    }
}

impl<'model> Borrow<IntVar<'model>> for BoolVar<'model> {
    fn borrow(&self) -> &IntVar<'model> {
        &self.int_var
    }
}

/// Safety:
/// - Safe because BoolVar is created from Model and therefore the backend is initialized
///   and model is initialized.
unsafe impl<'model> Variable<'model> for BoolVar<'model> {}

impl<'model> BoolVar<'model> {
    /// Creates a new boolean variable.
    ///
    /// # Panics
    ///
    /// Panics if the name contains null bytes or if the backend returns a null handle.
    #[must_use]
    pub(crate) fn new(model: &'model Model, value: Option<bool>, name: Option<&str>) -> Self {
        // Safety:
        // Safe because Model instances are created from valid backend handles.
        let raw_handle = CHOCO_BACKEND.with(|backend| unsafe {
            match name {
                Some(name_str) => {
                    let c_name = std::ffi::CString::new(name_str)
                        .expect("Failed to convert name to CString");
                    match value {
                        Some(x) => CHOCO_LIB.Java_org_chocosolver_capi_BoolVarApi_boolVar_sb(
                            backend.thread,
                            model.get_raw_handle(),
                            c_name.as_ptr().cast_mut(),
                            i32::from(x),
                        ),
                        None => CHOCO_LIB.Java_org_chocosolver_capi_BoolVarApi_boolVar_s(
                            backend.thread,
                            model.get_raw_handle(),
                            c_name.as_ptr().cast_mut(),
                        ),
                    }
                }
                None => match value {
                    Some(x) => CHOCO_LIB.Java_org_chocosolver_capi_BoolVarApi_boolVar_b(
                        backend.thread,
                        model.get_raw_handle(),
                        i32::from(x),
                    ),
                    None => CHOCO_LIB.Java_org_chocosolver_capi_BoolVarApi_boolVar(
                        backend.thread,
                        model.get_raw_handle(),
                    ),
                },
            }
        });
        assert!(
            !raw_handle.is_null(),
            "Failed to create BoolVar: received null handle"
        );

        BoolVar {
            int_var: IntVar {
                handle: Handle::new(raw_handle),
                model,
            },
        }
    }

    #[must_use]
    pub(crate) fn not_view(bool_var: &BoolVar<'model>) -> BoolVar<'model> {
        // Safety:
        // Safe because view is created from BoolVar handle and therefore the backend is initialized.
        let view_handle = CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ViewApi_bool_not_view(
                backend.thread,
                bool_var.get_raw_handle(),
            )
        });
        BoolVar {
            int_var: IntVar {
                handle: Handle::new(view_handle),
                model: bool_var.int_var.model,
            },
        }
    }

    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The handle is a valid pointer to a backend boolean variable
    /// - The backend is initialized
    /// - The model is valid
    ///
    /// # Panics
    ///
    /// Panics if the handle is null.
    pub(crate) unsafe fn from_raw_handle(
        handle: *mut std::os::raw::c_void,
        model: &'model Model,
    ) -> BoolVar<'model> {
        BoolVar {
            int_var: IntVar {
                handle: Handle::new(handle),
                model,
            },
        }
    }

    /// Posts a constraint ensuring that if self BoolVar is true, then `then_constraint`
    /// must be satisfied as well. Otherwise, `else_constraint` must be satisfied.
    pub fn if_then_else(
        &self,
        then_constraint: &Constraint<'model>,
        else_constraint: &Constraint<'model>,
    ) {
        // Safety:
        // Safe because BoolVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_if_then_else_bool(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                then_constraint.get_raw_handle(),
                else_constraint.get_raw_handle(),
            )
        });
    }

    /// Creates an if-then constraint: self BoolVar -> then_constraint.
    pub fn if_then(&self, then_constraint: &Constraint<'model>) {
        // Safety:
        // Safe because BoolVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_if_then_bool(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                then_constraint.get_raw_handle(),
            )
        });
    }

    /// Posts an equivalence constraint stating that:
    /// `self` BoolVar is true <=> `constraint` is satisfied.
    pub fn if_only_if(&self, constraint: &Constraint<'model>) {
        // Safety:
        // Safe because BoolVar is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_reification(
                backend.thread,
                self.get_model().get_raw_handle(),
                self.get_raw_handle(),
                constraint.get_raw_handle(),
            )
        });
    }
}

impl<'model> BitAnd for &BoolVar<'model> {
    type Output = BoolVar<'model>;

    fn bitand(self, rhs: Self) -> Self::Output {
        [self, rhs].and().reify()
    }
}

impl<'model> BitAnd<bool> for &BoolVar<'model> {
    type Output = BoolVar<'model>;

    fn bitand(self, rhs: bool) -> Self::Output {
        let rhs_bool_var = BoolVar::new(self.get_model(), Some(rhs), None);
        [self, &rhs_bool_var].and().reify()
    }
}

impl<'model> BitAnd<&BoolVar<'model>> for bool {
    type Output = BoolVar<'model>;

    fn bitand(self, rhs: &BoolVar<'model>) -> Self::Output {
        let self_bool_var = BoolVar::new(rhs.get_model(), Some(self), None);
        [&self_bool_var, rhs].and().reify()
    }
}

impl<'model> BitOr for &BoolVar<'model> {
    type Output = BoolVar<'model>;

    fn bitor(self, rhs: Self) -> Self::Output {
        [self, rhs].or().reify()
    }
}

impl<'model> BitOr<bool> for &BoolVar<'model> {
    type Output = BoolVar<'model>;

    fn bitor(self, rhs: bool) -> Self::Output {
        let rhs_bool_var = BoolVar::new(self.get_model(), Some(rhs), None);
        [self, &rhs_bool_var].or().reify()
    }
}

impl<'model> BitOr<&BoolVar<'model>> for bool {
    type Output = BoolVar<'model>;

    fn bitor(self, rhs: &BoolVar<'model>) -> Self::Output {
        let self_bool_var = BoolVar::new(rhs.get_model(), Some(self), None);
        [&self_bool_var, rhs].or().reify()
    }
}

impl<'model> Not for &BoolVar<'model> {
    type Output = BoolVar<'model>;

    fn not(self) -> Self::Output {
        BoolVar::not_view(self)
    }
}

#[allow(private_bounds)]
/// Logical operations on slices of Constraints/BoolVars
pub trait BoolVarArrayLogicOps<'model>: Sealed {
    /// AND of Constraints
    /// # Arguments
    /// * `constraints` - slice of Constraints/BoolVars
    /// # Returns
    /// Constraint representing the AND of the Constraints/BoolVars
    /// # Panic:
    /// if slice is empty
    fn and(self) -> Constraint<'model>;
    /// OR of Constraints
    /// # Arguments
    /// * `constraints` - slice of Constraints/BoolVars
    /// # Returns
    /// Constraint representing the OR of the Constraints/BoolVars
    /// # Panic:
    /// if slice is empty
    fn or(self) -> Constraint<'model>;
}

impl<'model, Q: Borrow<BoolVar<'model>> + Sealed> BoolVarArrayLogicOps<'model> for &[Q] {
    fn and(self) -> Constraint<'model> {
        let array_handle = utils::make_boolvar_array(self);
        let model = self
            .first()
            .expect("Slice shall contains at least one element")
            .borrow()
            .get_model();
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_and_bv_bv(
                    backend.thread,
                    model.get_raw_handle(),
                    array_handle,
                );
         assert!(
            !constraint_handle.is_null(),
            "Failed to create AND constraint"
        );
        Constraint::new(constraint_handle, model)})
    }
    fn or(self) -> Constraint<'model> {
        let array_handle = utils::make_boolvar_array(self);
        let model = self
            .first()
            .expect("Slice shall contains at least one element")
            .borrow()
            .get_model();
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_or_bv_bv(
                    backend.thread,
                    model.get_raw_handle(),
                    array_handle,
                );
                    assert!(
            !constraint_handle.is_null(),
            "Failed to create OR constraint"
        );
        Constraint::new(constraint_handle, model)
            })
    }
}

#[cfg(test)]
mod tests;
