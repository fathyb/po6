use std::{
    cmp::Ordering,
    ffi::{c_int, c_void},
};

#[no_mangle]
pub unsafe extern "C" fn po6_rt_memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int {
    let s1 = std::slice::from_raw_parts(s1 as *const u8, n);
    let s2 = std::slice::from_raw_parts(s2 as *const u8, n);

    match s1.cmp(&s2) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn po6_rt_memcpy(dst: *mut c_void, src: *const c_void, n: usize) {
    let dst = std::slice::from_raw_parts_mut(dst as *mut u8, n);
    let src = std::slice::from_raw_parts(src as *const u8, n);

    dst.copy_from_slice(src)
}

#[no_mangle]
pub unsafe extern "C" fn po6_rt_memset(str: *mut c_void, c: c_int, n: usize) {
    let src = std::slice::from_raw_parts_mut(str as *mut u8, n);

    src.fill(c as _)
}
