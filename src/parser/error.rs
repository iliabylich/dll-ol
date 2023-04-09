#[derive(Debug)]
pub(crate) enum ParseError {
    ParseError(goblin::error::Error),
    UnsupportedFormat(u64),
}

impl From<goblin::error::Error> for ParseError {
    fn from(error: goblin::error::Error) -> Self {
        Self::ParseError(error)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
