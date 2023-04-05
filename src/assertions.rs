#[no_mangle]
pub extern "C" fn assert_eq_floats(lhs_type: *const u8, rhs_type: *const u8, lhs: f64, rhs: f64) {}

#[no_mangle]
pub extern "C" fn assert_eq_signed_ints(
    lhs_type: *const u8,
    rhs_type: *const u8,
    lhs: i64,
    rhs: i64,
) {
}

#[cfg(test)]
pub(crate) fn trigger_inclusion() -> usize {
    [
        assert_eq_floats as *mut std::ffi::c_void as usize,
        assert_eq_signed_ints as *mut std::ffi::c_void as usize,
    ]
    .into_iter()
    .fold(0, |acc, e| acc.wrapping_add(e))
}
