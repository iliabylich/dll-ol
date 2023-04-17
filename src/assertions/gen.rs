use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort, c_void,
};

#[no_mangle]
pub extern "C" fn assert_eq_char(_lhs: c_char, _rhs: c_char) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_char(_lhs: c_char, _rhs: c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_char(_lhs: c_uchar, _rhs: c_uchar) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_char(_lhs: c_uchar, _rhs: c_uchar) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_short(_lhs: c_short, _rhs: c_short) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_short(_lhs: c_short, _rhs: c_short) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_short(_lhs: c_ushort, _rhs: c_ushort) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_short(_lhs: c_ushort, _rhs: c_ushort) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_int(_lhs: c_int, _rhs: c_int) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_int(_lhs: c_int, _rhs: c_int) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_int(_lhs: c_uint, _rhs: c_uint) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_int(_lhs: c_uint, _rhs: c_uint) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_long(_lhs: c_long, _rhs: c_long) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_long(_lhs: c_long, _rhs: c_long) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_long(_lhs: c_ulong, _rhs: c_ulong) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_long(_lhs: c_ulong, _rhs: c_ulong) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_long_long(_lhs: c_longlong, _rhs: c_longlong) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_long_long(_lhs: c_longlong, _rhs: c_longlong) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_long_long(_lhs: c_ulonglong, _rhs: c_ulonglong) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_long_long(_lhs: c_ulonglong, _rhs: c_ulonglong) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_int8_t(_lhs: i8, _rhs: i8) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_int8_t(_lhs: i8, _rhs: i8) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_uint8_t(_lhs: u8, _rhs: u8) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_uint8_t(_lhs: u8, _rhs: u8) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_int16_t(_lhs: u16, _rhs: u16) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_int16_t(_lhs: u16, _rhs: u16) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_uint16_t(_lhs: i16, _rhs: i16) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_uint16_t(_lhs: i16, _rhs: i16) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_int32_t(_lhs: u32, _rhs: u32) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_int32_t(_lhs: u32, _rhs: u32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_uint32_t(_lhs: i32, _rhs: i32) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_uint32_t(_lhs: i32, _rhs: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_int64_t(_lhs: u64, _rhs: u64) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_int64_t(_lhs: u64, _rhs: u64) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_uint64_t(_lhs: i64, _rhs: i64) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_uint64_t(_lhs: i64, _rhs: i64) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_bool(_lhs: bool, _rhs: bool) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_bool(_lhs: bool, _rhs: bool) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_size_t(_lhs: usize, _rhs: usize) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_size_t(_lhs: usize, _rhs: usize) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_float(_lhs: c_float, _rhs: c_float) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_float(_lhs: c_float, _rhs: c_float) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_double(_lhs: c_double, _rhs: c_double) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_double(_lhs: c_double, _rhs: c_double) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_char_ptr(_lhs: *const c_char, _rhs: *const c_char) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_char_ptr(_lhs: *const c_char, _rhs: *const c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn assert_eq_void_ptr(_lhs: *const c_void, _rhs: *const c_void) {
    todo!()
}
#[no_mangle]
pub extern "C" fn assert_ne_void_ptr(_lhs: *const c_void, _rhs: *const c_void) {
    todo!()
}

pub fn trigger_inclusion() -> usize {
    [
        assert_eq_char as *mut std::ffi::c_void as usize,
        assert_ne_char as *mut std::ffi::c_void as usize,
        assert_eq_unsigned_char as *mut std::ffi::c_void as usize,
        assert_ne_unsigned_char as *mut std::ffi::c_void as usize,
        assert_eq_short as *mut std::ffi::c_void as usize,
        assert_ne_short as *mut std::ffi::c_void as usize,
        assert_eq_unsigned_short as *mut std::ffi::c_void as usize,
        assert_ne_unsigned_short as *mut std::ffi::c_void as usize,
        assert_eq_int as *mut std::ffi::c_void as usize,
        assert_ne_int as *mut std::ffi::c_void as usize,
        assert_eq_unsigned_int as *mut std::ffi::c_void as usize,
        assert_ne_unsigned_int as *mut std::ffi::c_void as usize,
        assert_eq_long as *mut std::ffi::c_void as usize,
        assert_ne_long as *mut std::ffi::c_void as usize,
        assert_eq_unsigned_long as *mut std::ffi::c_void as usize,
        assert_ne_unsigned_long as *mut std::ffi::c_void as usize,
        assert_eq_long_long as *mut std::ffi::c_void as usize,
        assert_ne_long_long as *mut std::ffi::c_void as usize,
        assert_eq_unsigned_long_long as *mut std::ffi::c_void as usize,
        assert_ne_unsigned_long_long as *mut std::ffi::c_void as usize,
        assert_eq_int8_t as *mut std::ffi::c_void as usize,
        assert_ne_int8_t as *mut std::ffi::c_void as usize,
        assert_eq_uint8_t as *mut std::ffi::c_void as usize,
        assert_ne_uint8_t as *mut std::ffi::c_void as usize,
        assert_eq_int16_t as *mut std::ffi::c_void as usize,
        assert_ne_int16_t as *mut std::ffi::c_void as usize,
        assert_eq_uint16_t as *mut std::ffi::c_void as usize,
        assert_ne_uint16_t as *mut std::ffi::c_void as usize,
        assert_eq_int32_t as *mut std::ffi::c_void as usize,
        assert_ne_int32_t as *mut std::ffi::c_void as usize,
        assert_eq_uint32_t as *mut std::ffi::c_void as usize,
        assert_ne_uint32_t as *mut std::ffi::c_void as usize,
        assert_eq_int64_t as *mut std::ffi::c_void as usize,
        assert_ne_int64_t as *mut std::ffi::c_void as usize,
        assert_eq_uint64_t as *mut std::ffi::c_void as usize,
        assert_ne_uint64_t as *mut std::ffi::c_void as usize,
        assert_eq_bool as *mut std::ffi::c_void as usize,
        assert_ne_bool as *mut std::ffi::c_void as usize,
        assert_eq_size_t as *mut std::ffi::c_void as usize,
        assert_ne_size_t as *mut std::ffi::c_void as usize,
        assert_eq_float as *mut std::ffi::c_void as usize,
        assert_ne_float as *mut std::ffi::c_void as usize,
        assert_eq_double as *mut std::ffi::c_void as usize,
        assert_ne_double as *mut std::ffi::c_void as usize,
        assert_eq_char_ptr as *mut std::ffi::c_void as usize,
        assert_ne_char_ptr as *mut std::ffi::c_void as usize,
        assert_eq_void_ptr as *mut std::ffi::c_void as usize,
        assert_ne_void_ptr as *mut std::ffi::c_void as usize,
    ]
    .into_iter()
    .fold(0, |acc, e| acc.wrapping_add(e))
}
