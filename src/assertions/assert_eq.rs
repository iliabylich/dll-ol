use crate::{assertions::Assertion, context::Context};

pub(crate) struct AssertEq<T> {
    expected: T,
    actual: T,
}

impl<T> AssertEq<T> {
    pub(crate) fn new(expected: T, actual: T) -> Self {
        Self { expected, actual }
    }
}

impl<T: PartialEq + std::fmt::Debug> Assertion for AssertEq<T> {
    fn run(&self) {
        if self.expected != self.actual {
            Context::current_test().unwrap().failed(format!(
                "Expected {:?} but got {:?}",
                self.expected, self.actual
            ))
        }
    }
}
