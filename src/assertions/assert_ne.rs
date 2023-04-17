use crate::{assertions::Assertion, reporter::Reporter};

pub(crate) struct AssertNe<T> {
    pub(crate) expected: T,
    pub(crate) actual: T,
}

impl<T> AssertNe<T> {
    pub(crate) fn new(expected: T, actual: T) -> Self {
        Self { expected, actual }
    }
}

impl<T: PartialEq + std::fmt::Debug> Assertion for AssertNe<T> {
    fn run(&self) {
        if self.expected == self.actual {
            Reporter::report_test_failure(format!(
                "Expected {:?} but got {:?}",
                self.expected, self.actual
            ))
        }
    }
}
