use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn po6_rt_errno_ptr() -> *mut c_int {
    thread_local! {
        static ERRNO: c_int = 0;
    }

    ERRNO.with(|e| e as *const c_int as *mut c_int)
}
