use std::ffi::c_void;

pub(crate) struct FFIAlloc<T> {
    ptr: *mut T,
}

impl<T> FFIAlloc<T> {
    pub fn new(sz: usize) -> Self {
        Self { ptr: unsafe { libc::malloc(sz) as *mut T } }
    }

    pub fn realloc(&mut self, sz: usize) -> bool {
        self.ptr = unsafe { libc::realloc(self.ptr as *mut c_void, sz) as *mut T };
        self.valid()
    }

    pub const fn as_ptr(&self) -> *const T {
        self.ptr
    }

    pub const fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn valid(&self) -> bool {
        !self.ptr.is_null()
    }
}

impl<T> Drop for FFIAlloc<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut c_void) }
    }
}