use crate::{
    CHOCO_BACKEND, CHOCO_LIB,
    utils::{Handle, HandleT},
    variables::{BoolVar, IntVar},
};

/// A solver result holding instantiated variable values.
#[derive(Debug)]
pub struct Solution {
    handle: Handle,
}

impl Solution {
    /// # Safety:
    /// Must be called with a valid raw handle that points to a Solution instance in the backend.
    pub(crate) unsafe fn new(raw_handle: *mut std::os::raw::c_void) -> Self {
        Solution {
            handle: Handle::new(raw_handle),
        }
    }

    #[must_use]
    /// Gets the value of an integer variable in this solution.
    /// :TODO: presently if the var is not instantiated in solution Java raise an exception
    /// Need to be added a new function in backend to check if the var is instantiated
    pub fn get_int_var(&self, int_var: &IntVar<'_>) -> Option<i32> {
        // Safety:
        // Safe because Solution instances are created from valid backend handles.
        CHOCO_BACKEND.with(|backend| unsafe {
            Some(CHOCO_LIB.Java_org_chocosolver_capi_SolutionApi_getIntVal(
                backend.thread,
                self.get_raw_handle(),
                int_var.get_raw_handle(),
            ))
        })
    }

    /// Gets the value of a boolean variable in this solution.
    /// :TODO: presently if the var is not instantiated in solution Java raise an exception
    /// Need to be added a new function in backend to check if the var is instantiated
    #[must_use]
    pub fn get_bool_var(&self, bool_var: &BoolVar<'_>) -> Option<bool> {
        CHOCO_BACKEND.with(|backend|
        // Safety:
        // Safe because Solution instances are created from valid backend handles.
        unsafe {
            Some(
                CHOCO_LIB.Java_org_chocosolver_capi_SolutionApi_getIntVal(backend.thread, self.get_raw_handle(), bool_var.get_raw_handle())
                    != 0,
            )
        })
    }
}

impl HandleT for Solution {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.handle.get_raw_handle()
    }
}
