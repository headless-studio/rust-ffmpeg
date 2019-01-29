use ffi::*;

pub struct Context {
    ptr: *mut AVIOContext,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut AVIOContext) -> Self {
        Context { ptr: ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVIOContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVIOContext {
        self.ptr
    }
}

impl Context {
    pub fn size(&mut self) -> i64 {
        unsafe { avio_size(self.as_mut_ptr()) }
    }
}
