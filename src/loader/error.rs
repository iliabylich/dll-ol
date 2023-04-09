use libc::dlerror;
use std::ffi::CStr;

#[derive(Debug)]
pub(crate) struct DlopenError(pub(crate) String);

impl std::fmt::Display for DlopenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DlopenError {}

impl DlopenError {
    pub(crate) fn new() -> Self {
        let error = unsafe { CStr::from_ptr(dlerror()) };
        return Self(error.to_string_lossy().into_owned());
    }
}

#[derive(Debug)]
pub(crate) struct DlsymError(pub(crate) String);

impl std::fmt::Display for DlsymError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DlsymError {}

impl DlsymError {
    pub(crate) fn new() -> Self {
        let error = unsafe { CStr::from_ptr(dlerror()) };
        return Self(error.to_string_lossy().into_owned());
    }
}
