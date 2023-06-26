mod gen;
use gen::trigger_inclusion as gen_trigger_inclusion;

#[no_mangle]
pub extern "C" fn assert_true(value: bool) {
    AssertEq::run(value, true);
}
#[no_mangle]
pub extern "C" fn assert_false(value: bool) {
    AssertEq::run(value, true);
}

pub(crate) fn trigger_inclusion() -> usize {
    [
        assert_true as *mut std::ffi::c_void as usize,
        assert_false as *mut std::ffi::c_void as usize,
    ]
    .into_iter()
    .fold(0, |acc: usize, e| acc.wrapping_add(e))
        + gen_trigger_inclusion()
}

mod assert_eq;
pub(crate) use assert_eq::AssertEq;

mod assert_ne;
pub(crate) use assert_ne::AssertNe;
