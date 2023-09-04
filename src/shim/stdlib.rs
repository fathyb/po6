use std::{
    ffi::{c_char, c_double, c_int, c_long, c_longlong, CStr},
    process,
    str::FromStr,
};

#[no_mangle]
pub extern "C" fn po6_rt_abort() {
    process::abort()
}
#[no_mangle]
pub extern "C" fn po6_rt_exit(status: c_int) {
    process::exit(status)
}

#[no_mangle]
pub extern "C" fn po6_rt_sysconf(_name: c_int) -> c_long {
    0
}

#[no_mangle]
pub extern "C" fn po6_rt_abs(j: c_int) -> c_int {
    j.abs()
}

#[no_mangle]
pub extern "C" fn po6_rt_llabs(j: c_longlong) -> c_longlong {
    j.abs()
}

#[no_mangle]
pub unsafe extern "C" fn po6_rt_atof(str: *const c_char) -> c_double {
    parse_str(str)
}
#[no_mangle]
pub unsafe extern "C" fn po6_rt_atoi(str: *const c_char) -> c_int {
    parse_str(str)
}
#[no_mangle]
pub unsafe extern "C" fn po6_rt_atol(str: *const c_char) -> c_long {
    parse_str(str)
}
#[no_mangle]
pub unsafe extern "C" fn po6_rt_atoll(str: *const c_char) -> c_longlong {
    parse_str(str)
}

unsafe fn parse_str<T: Default + FromStr>(str: *const c_char) -> T {
    CStr::from_ptr(str)
        .to_str()
        .ok()
        .and_then(|str| str.parse().ok())
        .unwrap_or_default()
}
