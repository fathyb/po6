use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    ffi::c_void,
};

#[no_mangle]
pub extern "C" fn po6_rt_malloc(size: usize) -> *mut c_void {
    po6_rt_memalign(1, size)
}

#[no_mangle]
pub extern "C" fn po6_rt_memalign(align: usize, size: usize) -> *mut c_void {
    let layout = Layout::from_size_align(PointerStorage::size(size), align).unwrap();

    unsafe {
        let ptr: *mut PointerStorage = alloc(layout).cast();

        ptr.write(PointerStorage { size, align });
        ptr.add(1).cast()
    }
}

#[no_mangle]
pub extern "C" fn po6_rt_calloc(nmemb: usize, size: usize) -> *mut c_void {
    let len = size * nmemb;
    let ptr = po6_rt_malloc(len);

    unsafe { std::slice::from_raw_parts_mut(ptr.cast::<u8>(), len) }.fill(0);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn po6_rt_free(ptr: *mut c_void) {
    let ptr = PointerStorage::take(ptr);

    dealloc(ptr.cast(), ptr.read().into());
}
#[no_mangle]
pub unsafe extern "C" fn po6_rt_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    let ptr = PointerStorage::take(ptr);
    let align = ptr.read().align;
    let ptr =
        realloc(ptr.cast(), ptr.read().into(), PointerStorage::size(size)).cast::<PointerStorage>();

    ptr.write(PointerStorage { size, align });
    ptr.cast()
}

#[repr(C)]
struct PointerStorage {
    size: usize,
    align: usize,
}

impl PointerStorage {
    unsafe fn take(ptr: *mut c_void) -> *mut Self {
        ptr.cast::<Self>().offset(-1)
    }

    fn size(size: usize) -> usize {
        size + std::mem::size_of::<Self>()
    }
}

impl From<PointerStorage> for Layout {
    fn from(value: PointerStorage) -> Self {
        Self::from_size_align(value.size, value.align).unwrap()
    }
}
