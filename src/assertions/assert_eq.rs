use crate::{assertions::Assertion, reporter::Reporter};

pub(crate) struct AssertEq<T> {
    pub(crate) expected: T,
    pub(crate) actual: T,
}

impl<T> AssertEq<T> {
    pub(crate) fn new(expected: T, actual: T) -> Self {
        Self { expected, actual }
    }
}

impl<T: PartialEq + std::fmt::Debug> Assertion for AssertEq<T> {
    fn run_with_reporter(&self, reporter: &mut Reporter) {
        if self.expected == self.actual {
            reporter.report_success();
        } else {
            reporter.report_failure(format!(
                "Expected {:?} but got {:?}",
                self.expected, self.actual
            ))
        }
    }
}
