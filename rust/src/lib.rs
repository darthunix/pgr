use libc::{c_char, strlen};
use std::ffi::CString;

/// # Safety
///
/// Unsafe. Copies Rust `CString` to the provided buffer by the foreign allocator
#[no_mangle]
pub unsafe extern "C" fn hello_world(buf: *mut c_char, len: usize) -> usize {
    // Rust safe code here
    let s = CString::new("hello world!");

    // copy result to the memory, allocated with some other allocator
    if let Ok(s) = s {
        let s_ptr = s.into_raw();
        let length = strlen(s_ptr) + 1;

        if length > len {
            return 0;
        }
        std::ptr::copy(s_ptr, buf, length);
        return length;
    }
    0
}
