use std::ffi::{c_char, c_int, CStr};

#[no_mangle]
pub unsafe extern "C" fn po6_rt_assert_fail(expr: *const c_char, file: *const c_char, line: c_int) {
    let expr = CStr::from_ptr(expr).to_str().unwrap_or("<utf-8 error>");
    let file = CStr::from_ptr(file).to_str().unwrap_or("<utf-8 error>");

    panic!("Assertion failed: {expr}, file {file}, line {line}")
}
