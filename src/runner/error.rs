#[derive(Debug, PartialEq, Eq)]
pub enum NewRunnerError {
    NoDylib,
    FailedToLoadDyLib(String),
    MalformedDylib(String),
}

impl std::fmt::Display for NewRunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for NewRunnerError {}

impl From<std::io::Error> for NewRunnerError {
    fn from(_: std::io::Error) -> Self {
        Self::NoDylib
    }
}

use crate::loader::{DlopenError, DlsymError};

impl From<DlopenError> for NewRunnerError {
    fn from(error: DlopenError) -> Self {
        Self::FailedToLoadDyLib(error.0)
    }
}

impl From<DlsymError> for NewRunnerError {
    fn from(error: DlsymError) -> Self {
        Self::MalformedDylib(error.0)
    }
}
