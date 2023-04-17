use crate::{assertions::Assertion, reporter::Reporter};

pub(crate) struct AssertNe<T> {
    expected: T,
    actual: T,
}

impl<T> AssertNe<T> {
    pub(crate) fn new(expected: T, actual: T) -> Self {
        Self { expected, actual }
    }
}

impl<T: PartialEq + std::fmt::Debug> Assertion for AssertNe<T> {
    fn run(&self) {
        if self.expected == self.actual {
            Reporter::test_failed(format!(
                "Expected {:?} but got {:?}",
                self.expected, self.actual
            ))
        }
    }
}
