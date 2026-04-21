use std::borrow::Borrow;
use std::ffi::CStr;

use crate::CHOCO_BACKEND;
use crate::CHOCO_LIB;
use crate::Sealed;
use crate::SolverError;
use crate::model::Model;
use crate::utils::{Handle, HandleT, ModelObject, make_constraint_array};
use crate::variables::{BoolVar, IntVar};

/// Comparison operators for arithmetic constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqualityOperator {
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
}

impl EqualityOperator {
    #[must_use]
    pub fn to_cstr(&self) -> &CStr {
        match self {
            EqualityOperator::Eq => c"=",
            EqualityOperator::Neq => c"!=",
            EqualityOperator::Lt => c"<",
            EqualityOperator::Leq => c"<=",
            EqualityOperator::Gt => c">",
            EqualityOperator::Geq => c">=",
        }
    }
}

/// Arithmetic operators used in constraints (addition, subtraction, multiplication, division).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOperator {
    Sum,
    Sub,
    Mul,
    Div,
}

impl ArithmeticOperator {
    #[must_use]
    pub fn to_cstr(&self) -> &CStr {
        match self {
            ArithmeticOperator::Sum => c"+",
            ArithmeticOperator::Sub => c"-",
            ArithmeticOperator::Mul => c"*",
            ArithmeticOperator::Div => c"/",
        }
    }
}

/// The operational status of a constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintStatus {
    /// Constraint is not yet posted or reified.
    FREE,
    /// Constraint has been posted to the solver.
    POSTED,
    /// Constraint has been converted to a boolean variable.
    REIFIED,
}

impl TryFrom<i32> for ConstraintStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConstraintStatus::FREE),
            1 => Ok(ConstraintStatus::POSTED),
            2 => Ok(ConstraintStatus::REIFIED),
            _ => Err(()),
        }
    }
}

/// Represents the satisfaction state of a constraint.
///
/// This enum indicates whether a constraint is satisfied, not satisfied,
/// or if it is not yet possible to determine its satisfaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ESat {
    /// The constraint is satisfied.
    True,
    /// The constraint is not satisfied.
    False,
    /// The satisfaction state cannot be determined yet.
    Undefined,
}

/// Trait for creating equality constraints between variables or constants.
///
/// Provides methods to create equality (`eq`) and inequality (`ne`) constraints.
pub trait ConstraintEquality<'model, Rhs> {
    /// Creates an equality constraint (=).
    fn eq(self, other: Rhs) -> Constraint<'model>;
    /// Creates an inequality constraint (!=).
    fn ne(self, other: Rhs) -> Constraint<'model>;
}

/// Trait for creating modulo/remainder constraints.
pub trait ArithmConstraint<'model, MOD, RES> {
    /// Creates a modulo constraint: `self` % `modulo` = `res`.
    fn modulo(&'model self, modulo: MOD, res: RES) -> Constraint<'model>;
}

/// A constraint attached to a [`Model`].
///
/// Constraints can be posted, reified, or combined to express relations
/// between variables in the model.
pub struct Constraint<'model> {
    handle: Handle,
    model: &'model Model,
}

impl HandleT for Constraint<'_> {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.handle.get_raw_handle()
    }
}
impl<'model> ModelObject<'model> for Constraint<'model> {
    fn get_model(&self) -> &'model Model {
        self.model
    }
}

impl<'model> Constraint<'model> {
    /// # Safety
    /// - The caller must ensure that the handle is valid constraint pointer and the backend is initialized.
    pub(crate) unsafe fn new(handle: *mut std::os::raw::c_void, model: &'model Model) -> Self {
        Constraint {
            handle: Handle::new(handle),
            model,
        }
    }

    pub fn post(&self) -> Result<(), SolverError> {
        if self.status() != ConstraintStatus::FREE {
            Err(SolverError::NotFreeConstraint)
        } else {
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            CHOCO_BACKEND.with(|backend| unsafe {
                CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_post(
                    backend.thread,
                    self.get_raw_handle(),
                )
            });
            Ok(())
        }
    }

    /// Reifies the constraint, i.e., returns a BoolVar whose instantiation in a solution
    /// corresponds to the satisfaction state of the constraint in this solution.
    ///
    /// # Returns
    ///
    /// A `BoolVar` that encodes the satisfaction state of the constraint.
    ///
    /// # Panics
    ///
    /// Panics if the backend fails to reify the constraint and returns a null handle.
    #[must_use]
    pub fn reify(&self) -> BoolVar<'model> {
        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
           let var_handle = CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_reify(
                backend.thread,
                self.get_raw_handle(),
            );
             assert!(!var_handle.is_null(), "Failed to reify constraint");
             BoolVar::from_raw_handle(var_handle, self.model)})
    }

    /// Reifies the constraint with a given BoolVar whose instantiation in a solution
    /// corresponds to the satisfaction state of the constraint in this solution.
    ///
    /// # Arguments
    ///
    /// * `boolvar` - The BoolVar to reify with.
    pub fn reify_with(&self, boolvar: &BoolVar<'model>) {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_reify_with(
                backend.thread,
                self.get_raw_handle(),
                boolvar.get_raw_handle(),
            )
        });
    }

    /// Encapsulates this constraint in an implication relationship.
    /// The truth value of this constraint implies the truth value of the BoolVar.
    ///
    /// # Arguments
    ///
    /// * `boolvar` - The BoolVar that is implied by this constraint.
    pub fn implies(&self, boolvar: &BoolVar<'model>) {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_implies(
                backend.thread,
                self.get_raw_handle(),
                boolvar.get_raw_handle(),
            )
        });
    }

    /// Encapsulates this constraint in an implication relationship.
    /// Represents half-reification of the constraint. If the BoolVar is true,
    /// then the constraint must be satisfied.
    ///
    /// # Arguments
    ///
    /// * `boolvar` - The BoolVar that implies this constraint.
    pub fn implied_by(&self, boolvar: &BoolVar<'model>) {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_implied_by(
                backend.thread,
                self.get_raw_handle(),
                boolvar.get_raw_handle(),
            )
        });
    }

    /// Check whether the constraint is satisfied.
    ///
    /// Returns `ESat::True` if the constraint is satisfied, `ESat::False` if it is not,
    /// or `ESat::Undefined` if it is not yet possible to determine whether the constraint
    /// is satisfied or not.
    ///
    /// # Note
    ///
    /// This method can be useful to verify whether a constraint is satisfied (or not)
    /// regardless of the variables' instantiation.
    ///
    /// # Returns
    ///
    /// The satisfaction state of the constraint.
    #[must_use]
    pub fn is_satisfied(&self) -> ESat {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        let state = CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_is_satisfied(
                backend.thread,
                self.get_raw_handle(),
            )
        });
        match state {
            0 => ESat::False,
            1 => ESat::True,
            _ => ESat::Undefined,
        }
    }

    #[must_use]
    pub fn status(&self) -> ConstraintStatus {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        let status_code = CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ConstraintApi_getStatus(
                backend.thread,
                self.get_raw_handle(),
            )
        });
        match ConstraintStatus::try_from(status_code) {
            Ok(status) => status,
            Err(_) => panic!("Unknown constraint status code: {}", status_code),
        }
    }
    /// Posts a constraint ensuring that if self constraint is satisfied , then `then_constraint`
    /// must be satisfied as well. Otherwise, `else_constraint` must be satisfied.
    pub fn if_then_else(
        &self,
        then_constraint: &Constraint<'model>,
        else_constraint: &Constraint<'model>,
    ) {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_if_then_else(
                backend.thread,
                self.model.get_raw_handle(),
                self.get_raw_handle(),
                then_constraint.get_raw_handle(),
                else_constraint.get_raw_handle(),
            )
        });
    }
    /// Creates an if-then constraint: self constraint -> then_constraint.
    pub fn if_then(&self, then_constraint: &Constraint<'model>) {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_if_then(
                backend.thread,
                self.model.get_raw_handle(),
                self.get_raw_handle(),
                then_constraint.get_raw_handle(),
            )
        });
    }

    /// Posts an equivalence constraint stating that:
    /// `self` constraint is satisfied (or true) <=> `constraint` is satisfied.
    pub fn if_only_if(&self, constraint: &Constraint<'model>) {
        // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_ReificationApi_if_only_if(
                backend.thread,
                self.model.get_raw_handle(),
                self.get_raw_handle(),
                constraint.get_raw_handle(),
            )
        });
    }
}

/// # Safety
/// - This trait assumes that the implementor use this only on IntVar or i32 types.
unsafe trait IntoRawIntVarHandleT: Copy {
    fn into_raw_ptr(self, model: &Model) -> *mut std::os::raw::c_void;
}

/// # SAFETY:
/// - Implemented on &IntVar and i32 only, as required by the trait safety comment.
unsafe impl<'model, T: Borrow<IntVar<'model>>> IntoRawIntVarHandleT for &T {
    fn into_raw_ptr(self, _model: &Model) -> *mut std::os::raw::c_void {
        self.borrow().get_raw_handle()
    }
}
/// # SAFETY:
/// - Implemented on &IntVar and i32 only, as required by the trait safety comment.
/// - the integer variable is not actually destroyed after use, but this is safe as the backend will manage memory and the temporary IntVar will be garbage collected when the model is disposed.
unsafe impl IntoRawIntVarHandleT for i32 {
    fn into_raw_ptr(self, model: &Model) -> *mut std::os::raw::c_void {
        // Safety:
        // Safe because IntVar is created from Model and therefore the backend is initialized, model handle is valid.
        CHOCO_BACKEND.with(|backend| unsafe {
            CHOCO_LIB.Java_org_chocosolver_capi_IntVarApi_intVar_i(
                backend.thread,
                model.get_raw_handle(),
                self,
            )
        })
    }
}

pub(crate) trait ConstraintArithmT<'model> {
    fn create(&self) -> Constraint<'model>;
}

impl<'model, X: Borrow<IntVar<'model>>> ConstraintArithmT<'model> for (X, EqualityOperator, i32) {
    fn create(&self) -> Constraint<'model> {
        let int_var = self.0.borrow();
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_arithm_iv_cst(
                    backend.thread,
                    int_var.get_model().get_raw_handle(),
                    int_var.get_raw_handle(),
                    self.1.to_cstr().as_ptr().cast_mut(),
                    self.2,
                );
assert!(
            !constraint_handle.is_null(),
            "Invalid parameters combination for arithm constraint. Please refer to the doc"
        );
        Constraint::new(constraint_handle, int_var.get_model())})
    }
}

impl<'model, X: Borrow<IntVar<'model>>, Y: Borrow<IntVar<'model>>> ConstraintArithmT<'model>
    for (X, EqualityOperator, Y)
{
    fn create(&self) -> Constraint<'model> {
        let x_var = self.0.borrow();
        let y_var = self.2.borrow();
        CHOCO_BACKEND.with(|backend|
// Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_arithm_iv_iv(
                    backend.thread,
                    x_var.get_model().get_raw_handle(),
                    x_var.get_raw_handle(),
                    self.1.to_cstr().as_ptr().cast_mut(),
                    y_var.get_raw_handle(),
                );
assert!(
            !constraint_handle.is_null(),
            "Invalid parameters combination for arithm constraint. Please refer to the doc"
        );
        Constraint::new(constraint_handle, x_var.get_model())})
    }
}

impl<'model, X: Borrow<IntVar<'model>>, Y: IntoRawIntVarHandleT> ConstraintArithmT<'model>
    for (X, EqualityOperator, Y, ArithmeticOperator, &IntVar<'model>)
{
    fn create(&self) -> Constraint<'model> {
        let x_var = self.0.borrow();
        let yy = self.2.into_raw_ptr(x_var.get_model());
        assert!(
            !yy.is_null(),
            "Failed to convert parameter to raw pointer for arithm constraint"
        );

        CHOCO_BACKEND.with(|backend|
// Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_arithm_iv_iv_iv(
                    backend.thread,
                    x_var.get_model().get_raw_handle(),
                    x_var.get_raw_handle(),
                    self.1.to_cstr().as_ptr().cast_mut(),
                    yy,
                    self.3.to_cstr().as_ptr().cast_mut(),
                    self.4.get_raw_handle(),
                );
                assert!(
            !constraint_handle.is_null(),
            "Invalid parameters combination for arithm constraint. Please refer to the doc"
        );
        Constraint::new(constraint_handle, x_var.get_model())})
    }
}
impl<'model, X: Borrow<IntVar<'model>>, Y: IntoRawIntVarHandleT> ConstraintArithmT<'model>
    for (X, EqualityOperator, Y, ArithmeticOperator, i32)
{
    fn create(&self) -> Constraint<'model> {
        let x_var = self.0.borrow();
        let yy = self.2.into_raw_ptr(x_var.get_model());

        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_arithm_iv_iv_cst(
                    backend.thread,
                    x_var.get_model().get_raw_handle(),
                    x_var.get_raw_handle(),
                    self.1.to_cstr().as_ptr().cast_mut(),
                    yy,
                    self.3.to_cstr().as_ptr().cast_mut(),
                    self.4,
                );
                assert!(
            !constraint_handle.is_null(),
            "Invalid parameters combination for arithm constraint. Please refer to the doc");
        Constraint::new(constraint_handle, x_var.get_model())
        })
    }
}

impl<'model, X: Borrow<IntVar<'model>>, Y: IntoRawIntVarHandleT> ConstraintArithmT<'model>
    for (X, ArithmeticOperator, Y, EqualityOperator, &IntVar<'model>)
{
    fn create(&self) -> Constraint<'model> {
        let x_var = self.0.borrow();
        let yy = self.2.into_raw_ptr(x_var.get_model());
        assert!(
            !yy.is_null(),
            "Failed to convert parameter to raw pointer for arithm constraint"
        );

        CHOCO_BACKEND.with(|backend|
            // Safety:
        // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
             unsafe {
            let constraint_handle =CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_arithm_iv_iv_iv(
                    backend.thread,
                    x_var.get_model().get_raw_handle(),
                    x_var.get_raw_handle(),
                    self.1.to_cstr().as_ptr().cast_mut(),
                    yy,
                    self.3.to_cstr().as_ptr().cast_mut(),
                    self.4.get_raw_handle(),
                );
                assert!(
            !constraint_handle.is_null(),
            "Invalid parameters combination for arithm constraint. Please refer to the doc");
        Constraint::new(constraint_handle, x_var.get_model())
        })
    }
}
impl<'model, X: Borrow<IntVar<'model>>, Y: IntoRawIntVarHandleT> ConstraintArithmT<'model>
    for (X, ArithmeticOperator, Y, EqualityOperator, i32)
{
    fn create(&self) -> Constraint<'model> {
        let x_var = self.0.borrow();
        let yy = self.2.into_raw_ptr(x_var.get_model());

        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
             let constraint_handle = CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_arithm_iv_iv_cst(
                    backend.thread,
                    x_var.get_model().get_raw_handle(),
                    x_var.get_raw_handle(),
                    self.1.to_cstr().as_ptr().cast_mut(),
                    yy,
                    self.3.to_cstr().as_ptr().cast_mut(),
                    self.4,
                );
        assert!(
            !constraint_handle.is_null(),
            "Invalid parameters combination for arithm constraint. Please refer to the doc");
        Constraint::new(constraint_handle, x_var.get_model())
        })
    }
}
#[allow(private_bounds)]
/// Logical operations on slices of Constraints/BoolVars
pub trait ConstraintArrayLogicOps<'model>: Sealed {
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

impl<'model, Q: Borrow<Constraint<'model>> + Sealed> ConstraintArrayLogicOps<'model> for &[Q] {
    fn and(self) -> Constraint<'model> {
        let array_handle = make_constraint_array(self);
        let model = self
            .first()
            .expect("Slice shall contains at least one element")
            .borrow()
            .get_model();
        CHOCO_BACKEND.with(|backend|
            // Safety:
            // Safe because Constraint is created from Model and therefore the backend is initialized and model handle is valid.
            unsafe {
            let constraint_handle =CHOCO_LIB
                .Java_org_chocosolver_capi_ConstraintApi_and_cs_cs(
                    backend.thread,
                    model.get_raw_handle(),
                    array_handle,
                );
        assert!(
            !constraint_handle.is_null(),
            "Failed to create AND constraint"
        );
        Constraint::new(constraint_handle, model)
    })
    }
    fn or(self) -> Constraint<'model> {
        let array_handle = make_constraint_array(self);
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
                .Java_org_chocosolver_capi_ConstraintApi_or_cs_cs(
                    backend.thread,
                    model.get_raw_handle(),
                    array_handle,
                );
        assert!(!constraint_handle.is_null(),
            "Failed to create OR constraint"
        );
        Constraint::new(constraint_handle, model)
        })
    }
}

#[cfg(test)]
mod tests;
