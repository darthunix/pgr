//! Dynamic library that can evaluate Rust code inside `PostgreSQL`.

use libc::{c_char, c_void, size_t, strlen};
use std::ffi::CString;
use std::ptr;

/// Error codes to return postgres.
#[allow(dead_code)]
pub enum PgError {
    NoErrors,
    InvalidChunk,
    InvalidPointer,
    InvalidSize,
    OutOfMemory,
}

impl PgError {
    /// Translate rust enum to the error codes.
    fn code(&self) -> usize {
        match self {
            PgError::NoErrors => 0,
            PgError::InvalidChunk => 1,
            PgError::InvalidPointer => 2,
            PgError::InvalidSize => 3,
            PgError::OutOfMemory => 4,
        }
    }
}

// Postgres functions for memory allocation.
#[allow(dead_code)]
extern "C" {
    fn palloc(size: size_t) -> *mut c_void;
    fn repalloc(p: *mut c_void, size: size_t) -> *mut c_void;
    fn pfree(p: *mut c_void);
}

/// Postgres allocator.
///
/// Thread unsafe. Can be used **only** in the main postgres thread.
///
/// # Errors
/// If `palloc()` fails to allocate some memory, return an error.
#[allow(dead_code)]
pub fn pgalloc(size: usize) -> Result<*mut i8, PgError> {
    let ptr = unsafe { palloc(size as libc::size_t).cast::<i8>() };
    if ptr.is_null() {
        return Err(PgError::OutOfMemory);
    }
    Ok(ptr)
}

/// Postgres memory chunk.
///
/// Thread unsafe. A memory chunk allocated with postgres allocator.
#[repr(C)]
pub struct PgMemoryChunk {
    pub ptr: *mut i8,
    pub len: usize,
    pub capacity: usize,
    pub error: usize,
}

impl PgMemoryChunk {
    /// Returns an invalid memory chunk without heap allocations.
    fn invalid() -> Self {
        PgMemoryChunk {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
            error: PgError::InvalidChunk.code(),
        }
    }

    /// Returns an allocated chunk or an error.
    fn new(size: usize) -> Result<Self, PgError> {
        let ptr = pgalloc(size)?;
        Ok(PgMemoryChunk {
            ptr,
            len: 0,
            capacity: size,
            error: PgError::NoErrors.code(),
        })
    }

    /// Copy data to the current memory chunk.
    ///
    /// If fails for some reason, returns an error.
    fn copy_from(&mut self, s_ptr: *mut c_char, s_size: usize) -> Result<(), PgError> {
        if self.error != PgError::NoErrors.code() {
            return Err(PgError::InvalidChunk);
        }

        if s_size > self.capacity {
            self.error = PgError::InvalidSize.code();
            return Err(PgError::InvalidSize);
        }

        if s_ptr.is_null() || self.ptr.is_null() {
            self.error = PgError::InvalidPointer.code();
            return Err(PgError::InvalidPointer);
        }

        unsafe {
            std::ptr::copy(s_ptr, self.ptr, s_size);
        }
        self.len += s_size;
        Ok(())
    }
}

/// Print "hello world!" from Rust using `PostgreSQL` memory allocator.
///
/// # Safety
/// Unsafe. Copies Rust `CString` to the provided buffer by the foreign allocator.
#[no_mangle]
pub extern "C" fn hello_world() -> PgMemoryChunk {
    // Rust safe code here.
    let s = CString::new("hello world!");

    // Copy string to the PG memory chunk.
    if let Ok(res) = s {
        let s_ptr = res.into_raw();
        let s_size = unsafe { strlen(s_ptr) } + 1;

        if let Ok(mut chunk) = PgMemoryChunk::new(s_size) {
            if let Err(e) = chunk.copy_from(s_ptr, s_size) {
                chunk.error = e.code();
            }
            return chunk;
        }
    }
    PgMemoryChunk::invalid()
}
