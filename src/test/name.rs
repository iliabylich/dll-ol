#[derive(Debug, Clone, Default)]
pub(crate) struct TestName {
    symbol: String,
}

const PREFIX: &str = "__ol_test_";

impl TestName {
    pub(crate) fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
        }
    }

    pub(crate) fn pretty(&self) -> &str {
        if self.symbol.starts_with(PREFIX) {
            self.symbol.strip_prefix(PREFIX).unwrap()
        } else {
            &self.symbol
        }
    }

    #[cfg(test)]
    pub(crate) fn raw(&self) -> &str {
        &self.symbol
    }

    pub(crate) fn safe_to_run(&self) -> bool {
        !self.symbol.contains("__ol_test_crash")
    }
}
