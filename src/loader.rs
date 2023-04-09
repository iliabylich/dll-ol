use std::ffi::CString;

use libc::{dlclose, dlerror, dlopen, dlsym, RTLD_GLOBAL, RTLD_LAZY};
use std::ffi::{c_void, CStr};

#[derive(Debug)]
pub(crate) struct Loader {
    handle: *mut c_void,
}

pub(crate) type TestFn = extern "C" fn() -> ();

#[cfg(unix)]
impl Loader {
    pub(crate) fn new(path: &str) -> Result<Self, String> {
        let path = CString::new(path).unwrap();
        let handle = unsafe { dlopen(path.as_ptr(), RTLD_LAZY | RTLD_GLOBAL) };
        if handle.is_null() {
            let error = unsafe { CStr::from_ptr(dlerror()) };
            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Self { handle })
    }

    pub(crate) fn get_symbol(&self, symbol: &str) -> Result<TestFn, String> {
        let symbol = CString::new(symbol).unwrap();
        let ptr = unsafe { dlsym(self.handle, symbol.as_ptr() as *const i8) };
        if ptr.is_null() {
            let error = unsafe { CStr::from_ptr(dlerror()) };
            return Err(error.to_string_lossy().into_owned());
        }
        let f: TestFn = unsafe { std::mem::transmute(ptr) };
        Ok(f)
    }
}

#[cfg(unix)]
impl Drop for Loader {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.handle);
        }
    }
}
