use crate::CHOCO_BACKEND;

use std::borrow::Borrow;

#[derive(Debug)]
pub(crate) struct Handle {
    handle: *mut std::os::raw::c_void,
}
impl Handle {
    pub(crate) fn new(handle: *mut std::os::raw::c_void) -> Self {
        CHOCO_BACKEND.with(|_backend| {
            if handle.is_null() {
                panic!("Attempted to create Handle with null pointer");
            }
            Handle { handle }
        })
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        // Safety:
        // Guaranteed by CHOCO_BACKEND that the backend is initialized when Handle is destroyed.
        unsafe {
            assert!(!self.handle.is_null());
            CHOCO_BACKEND.with(|backend| {
                backend
                    .lib
                    .Java_org_chocosolver_capi_HandlesApi_destroy(backend.thread, self.handle);
                self.handle = std::ptr::null_mut();
            });
        }
    }
}
impl HandleT for Handle {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void {
        self.handle
    }
}

pub(crate) trait HandleT {
    fn get_raw_handle(&self) -> *mut std::os::raw::c_void;
}

pub(crate) trait ModelObject<'model>: HandleT {
    fn get_model(&self) -> &'model crate::model::Model;
}

/// Safety:
/// - This function assumes that the provided handle is valid and points to an integer array in the backend.
/// - It must be called in a context where the Choco backend is initialized.
pub(crate) unsafe fn get_int_array(handle: *const std::os::raw::c_void) -> Vec<i32> {
    // Safety:
    // Caller must ensure that the handle is valid and the backend is initialized.
    CHOCO_BACKEND.with(|backend| unsafe {
        let length = backend
            .lib
            .Java_org_chocosolver_capi_ArrayApi_int_length(backend.thread, handle.cast_mut());
        let vec_size: usize = length
            .try_into()
            .expect("Array length exceeds usize  or is negative");
        let mut array = Vec::with_capacity(vec_size);
        for i in 0..length {
            array.push(backend.lib.Java_org_chocosolver_capi_ArrayApi_int_get(
                backend.thread,
                handle.cast_mut(),
                i,
            ));
        }
        array
    })
}

/// Safety:
/// - Must be called in a context where the Choco backend is initialized.
pub(crate) fn make_int_array(ints: &[i32]) -> *mut std::os::raw::c_void {
    // Safety:
    // Guaranteed by assertion
    CHOCO_BACKEND.with(|backend| unsafe {
        let len_i32: i32 = ints
            .len()
            .try_into()
            .expect("Slice length exceeds i32 limits");
        let ints_array = backend
            .lib
            .Java_org_chocosolver_capi_ArrayApi_int_create(backend.thread, len_i32);
        for (i, &value) in ints.iter().enumerate() {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "Length checked to fit in i32"
            )]
            #[allow(clippy::cast_possible_wrap, reason = "Length checked to fit in i32")]
            backend.lib.Java_org_chocosolver_capi_ArrayApi_int_set(
                backend.thread,
                ints_array,
                value,
                i as i32,
            );
        }
        ints_array
    })
}

pub(crate) fn make_boolvar_array<'a, 'model: 'a, Q: Borrow<crate::variables::BoolVar<'model>>>(
    boolvars: &[Q],
) -> *mut std::os::raw::c_void {
    // Safety:
    // Guaranteed by BoolVar creation that implies backend is initialized
    CHOCO_BACKEND.with(|backend| unsafe {
        let len_i32: i32 = boolvars
            .len()
            .try_into()
            .expect("Slice length exceeds i32 limits");
        let boolvar_array = backend
            .lib
            .Java_org_chocosolver_capi_ArrayApi_boolVar_create(backend.thread, len_i32);
        for (i, boolvar) in boolvars.iter().enumerate() {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "Length checked to fit in i32"
            )]
            #[allow(clippy::cast_possible_wrap, reason = "Length checked to fit in i32")]
            backend.lib.Java_org_chocosolver_capi_ArrayApi_boolVar_set(
                backend.thread,
                boolvar_array,
                boolvar.borrow().get_raw_handle(),
                i as i32,
            );
        }
        boolvar_array
    })
}

pub(crate) fn make_constraint_array<
    'a,
    'model: 'a,
    Q: Borrow<crate::constraint::Constraint<'model>>,
>(
    constraints: &[Q],
) -> *mut std::os::raw::c_void {
    CHOCO_BACKEND.with(|backend|
    // Safety:
    // Guaranteed by Constraint creation that implies backend is initialized
    unsafe {
        let len_i32: i32 = constraints
            .len()
            .try_into()
            .expect("Slice length exceeds i32 limits");
        let constraint_array = backend.lib.Java_org_chocosolver_capi_ArrayApi_constraint_create(backend.thread, len_i32);
        for (i, constraint) in constraints.iter().enumerate() {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "Length checked to fit in i32"
            )]
            #[allow(clippy::cast_possible_wrap, reason = "Length checked to fit in i32")]
            backend.lib.Java_org_chocosolver_capi_ArrayApi_constraint_set(
                backend.thread,
                constraint_array,
                constraint.borrow().get_raw_handle(),
                i as i32,
            );
        }
        constraint_array
    })
}

pub(crate) fn make_intvar_array<'a, 'model: 'a, Q: Borrow<crate::variables::IntVar<'model>>>(
    intvars: &[Q],
) -> *mut std::os::raw::c_void {
    CHOCO_BACKEND.with(|backend|
    // Safety:
    // Guaranteed by IntVar creation that implies backend is initialized
    unsafe {
        let len_i32: i32 = intvars
            .len()
            .try_into()
            .expect("Slice length exceeds i32 limits");
        let intvar_array = backend.lib.Java_org_chocosolver_capi_ArrayApi_intVar_create(backend.thread, len_i32);
        for (i, intvar) in intvars.iter().enumerate() {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "Length checked to fit in i32"
            )]
            #[allow(clippy::cast_possible_wrap, reason = "Length checked to fit in i32")]
            backend.lib.Java_org_chocosolver_capi_ArrayApi_intVar_set(backend.thread, intvar_array, intvar.borrow().get_raw_handle(), i as i32);
        }
        intvar_array
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_int_array_single_element() {
        let input = [42];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![42]);
            }
        });
    }

    #[test]
    fn test_make_int_array_multiple_elements() {
        let input = [1, 2, 3, 4, 5];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![1, 2, 3, 4, 5]);
            }
        });
    }

    #[test]
    fn test_make_int_array_negative_values() {
        let input = [-10, -5, 0, 5, 10];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![-10, -5, 0, 5, 10]);
            }
        });
    }

    #[test]
    fn test_make_int_array_all_zeros() {
        let input = [0, 0, 0, 0];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![0, 0, 0, 0]);
            }
        });
    }

    #[test]
    fn test_make_int_array_large_values() {
        let input = [i32::MAX, i32::MIN, 0, 1000000, -1000000];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![i32::MAX, i32::MIN, 0, 1000000, -1000000]);
            }
        });
    }

    #[test]
    fn test_make_int_array_from_vec() {
        let input = vec![10, 20, 30];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![10, 20, 30]);
            }
        });
    }

    #[test]
    fn test_make_int_array_roundtrip() {
        let original = [7, 14, 21, 28, 35, 42];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&original);
                let retrieved = get_int_array(handle);
                assert_eq!(retrieved, original.to_vec());
                assert_eq!(retrieved.len(), original.len());
            }
        });
    }

    #[test]
    fn test_make_int_array_duplicates() {
        let input = [5, 5, 5, 5, 5];
        CHOCO_BACKEND.with(|_| {
            // Safety: CHOCO_BACKEND is initialized by the lazy static, making backend calls safe.
            unsafe {
                let handle = make_int_array(&input);
                let result = get_int_array(handle);
                assert_eq!(result, vec![5, 5, 5, 5, 5]);
            }
        });
    }
}
