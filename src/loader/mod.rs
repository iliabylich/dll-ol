use std::ffi::CString;

use libc::{dlclose, dlopen, dlsym, RTLD_GLOBAL, RTLD_LAZY};
use std::ffi::c_void;

mod error;
pub(crate) use error::{DlopenError, DlsymError};

#[derive(Debug)]
pub(crate) struct Loader {
    handle: *mut c_void,
}

pub(crate) type TestFn = extern "C" fn() -> ();

#[cfg(unix)]
impl Loader {
    pub(crate) fn new(path: &str) -> Result<Self, DlopenError> {
        let path = CString::new(path).unwrap();
        let handle = unsafe { dlopen(path.as_ptr(), RTLD_LAZY | RTLD_GLOBAL) };
        if handle.is_null() {
            return Err(DlopenError::new());
        }
        Ok(Self { handle })
    }

    pub(crate) fn get_symbol(&self, symbol: &str) -> Result<TestFn, DlsymError> {
        let symbol = CString::new(symbol).unwrap();
        let ptr = unsafe { dlsym(self.handle, symbol.as_ptr() as *const i8) };
        if ptr.is_null() {
            return Err(DlsymError::new());
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
