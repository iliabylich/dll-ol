use crate::assertions::{AssertEq, AssertNe};
use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort, c_void,
};

#[no_mangle]
pub extern "C" fn assert_eq_char(lhs: c_char, rhs: c_char) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_char(lhs: c_char, rhs: c_char) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_char(lhs: c_uchar, rhs: c_uchar) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_char(lhs: c_uchar, rhs: c_uchar) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_short(lhs: c_short, rhs: c_short) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_short(lhs: c_short, rhs: c_short) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_short(lhs: c_ushort, rhs: c_ushort) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_short(lhs: c_ushort, rhs: c_ushort) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_int(lhs: c_int, rhs: c_int) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_int(lhs: c_int, rhs: c_int) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_int(lhs: c_uint, rhs: c_uint) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_int(lhs: c_uint, rhs: c_uint) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_long(lhs: c_long, rhs: c_long) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_long(lhs: c_long, rhs: c_long) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_long(lhs: c_ulong, rhs: c_ulong) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_long(lhs: c_ulong, rhs: c_ulong) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_long_long(lhs: c_longlong, rhs: c_longlong) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_long_long(lhs: c_longlong, rhs: c_longlong) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_unsigned_long_long(lhs: c_ulonglong, rhs: c_ulonglong) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_unsigned_long_long(lhs: c_ulonglong, rhs: c_ulonglong) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_int8_t(lhs: i8, rhs: i8) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_int8_t(lhs: i8, rhs: i8) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_uint8_t(lhs: u8, rhs: u8) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_uint8_t(lhs: u8, rhs: u8) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_int16_t(lhs: u16, rhs: u16) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_int16_t(lhs: u16, rhs: u16) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_uint16_t(lhs: i16, rhs: i16) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_uint16_t(lhs: i16, rhs: i16) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_int32_t(lhs: u32, rhs: u32) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_int32_t(lhs: u32, rhs: u32) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_uint32_t(lhs: i32, rhs: i32) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_uint32_t(lhs: i32, rhs: i32) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_int64_t(lhs: u64, rhs: u64) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_int64_t(lhs: u64, rhs: u64) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_uint64_t(lhs: i64, rhs: i64) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_uint64_t(lhs: i64, rhs: i64) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_bool(lhs: bool, rhs: bool) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_bool(lhs: bool, rhs: bool) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_size_t(lhs: usize, rhs: usize) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_size_t(lhs: usize, rhs: usize) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_float(lhs: c_float, rhs: c_float) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_float(lhs: c_float, rhs: c_float) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_double(lhs: c_double, rhs: c_double) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_double(lhs: c_double, rhs: c_double) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_char_ptr(lhs: *const c_char, rhs: *const c_char) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_char_ptr(lhs: *const c_char, rhs: *const c_char) {
    AssertNe::run(lhs, rhs);
}

#[no_mangle]
pub extern "C" fn assert_eq_void_ptr(lhs: *const c_void, rhs: *const c_void) {
    AssertEq::run(lhs, rhs);
}
#[no_mangle]
pub extern "C" fn assert_ne_void_ptr(lhs: *const c_void, rhs: *const c_void) {
    AssertNe::run(lhs, rhs);
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
