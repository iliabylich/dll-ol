#[derive(Debug, PartialEq, Eq)]
pub enum FileError {
    NoDylib,
    FailedToLoadDyLib(String),
    MalformedDylib(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FileError {}

impl From<std::io::Error> for FileError {
    fn from(_: std::io::Error) -> Self {
        Self::NoDylib
    }
}

use crate::loader::{DlopenError, DlsymError};

impl From<DlopenError> for FileError {
    fn from(error: DlopenError) -> Self {
        Self::FailedToLoadDyLib(error.0)
    }
}

impl From<DlsymError> for FileError {
    fn from(error: DlsymError) -> Self {
        Self::MalformedDylib(error.0)
    }
}
