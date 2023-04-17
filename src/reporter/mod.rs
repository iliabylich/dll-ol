pub(crate) struct Reporter {
    failures: Vec<String>,
}

impl Reporter {
    pub(crate) fn instance() -> Self {
        Self { failures: vec![] }
    }

    pub(crate) fn report_success(&mut self) {}

    pub(crate) fn report_failure(&mut self, message: String) {
        self.failures.push(message)
    }
}
