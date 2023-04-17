use crate::{assertions::Assertion, context::Context};

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
            Context::current_test().unwrap().failed(format!(
                "Expected {:?} but got {:?}",
                self.expected, self.actual
            ))
        }
    }
}
