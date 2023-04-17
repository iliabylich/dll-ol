use std::ffi::CString;

use libc::{dlclose, dlopen, dlsym, RTLD_GLOBAL, RTLD_LAZY};
use std::ffi::c_void;

macro_rules! panic_with_dlerror {
    () => {
        let error = unsafe { std::ffi::CStr::from_ptr(libc::dlerror()) };
        panic!("{}", error.to_string_lossy());
    };
}

#[derive(Debug)]
pub(crate) struct Loader {
    pub(crate) path: String,
    handle: *mut c_void,
}

pub(crate) type TestFn = extern "C" fn() -> ();

#[cfg(unix)]
impl Loader {
    pub(crate) fn new(path: &str) -> Self {
        let c_path = CString::new(path).unwrap();
        let handle = unsafe { dlopen(c_path.as_ptr(), RTLD_LAZY | RTLD_GLOBAL) };
        if handle.is_null() {
            panic_with_dlerror!();
        }
        Self {
            handle,
            path: path.to_string(),
        }
    }

    pub(crate) fn get_symbol(&self, symbol: &str) -> TestFn {
        let symbol = CString::new(symbol).unwrap();
        let ptr = unsafe { dlsym(self.handle, symbol.as_ptr() as *const i8) };
        if ptr.is_null() {
            panic_with_dlerror!();
        }
        let f: TestFn = unsafe { std::mem::transmute(ptr) };
        f
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
