#[no_mangle]
pub extern "C" fn assert_eq_floats(
    _lhs_type: *const u8,
    _rhs_type: *const u8,
    _lhs: f64,
    _rhs: f64,
) {
}

#[no_mangle]
pub extern "C" fn assert_eq_signed_ints(
    _lhs_type: *const u8,
    _rhs_type: *const u8,
    _lhs: i64,
    _rhs: i64,
) {
}

pub fn trigger_inclusion() -> usize {
    [
        assert_eq_floats as *mut std::ffi::c_void as usize,
        assert_eq_signed_ints as *mut std::ffi::c_void as usize,
    ]
    .into_iter()
    .fold(0, |acc, e| acc.wrapping_add(e))
}
