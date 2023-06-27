use crate::{backtrace::Backtrace, test::Test};

pub(crate) struct AssertEq<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: PartialEq + std::fmt::Debug> AssertEq<T> {
    pub(crate) fn run(lhs: T, rhs: T) {
        if lhs != rhs {
            Test::fail_current(
                format!("Expected {:?} but got {:?}", lhs, rhs),
                Backtrace::new(),
            )
        }
    }
}
