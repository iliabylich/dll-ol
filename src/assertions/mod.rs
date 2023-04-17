mod gen;
use gen::trigger_inclusion as gen_trigger_inclusion;

#[no_mangle]
pub extern "C" fn assert_true(_value: bool) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_false(_value: bool) {
    todo!()
}

pub fn trigger_inclusion() -> usize {
    [
        assert_true as *mut std::ffi::c_void as usize,
        assert_false as *mut std::ffi::c_void as usize,
    ]
    .into_iter()
    .fold(0, |acc: usize, e| acc.wrapping_add(e))
        + gen_trigger_inclusion()
}
