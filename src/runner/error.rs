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
