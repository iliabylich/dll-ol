use crate::{backtrace::Backtrace, test::Test};

pub(crate) struct AssertNe<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: PartialEq + std::fmt::Debug> AssertNe<T> {
    pub(crate) fn run(lhs: T, rhs: T) {
        if lhs == rhs {
            Test::fail_current(
                format!("Expected NOT {:?} but got {:?}", lhs, rhs),
                Backtrace::new(),
            )
        }
    }
}
