use backtrace::Backtrace;

use crate::context::Context;

pub(crate) struct AssertEq<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: PartialEq + std::fmt::Debug> AssertEq<T> {
    pub(crate) fn run(lhs: T, rhs: T) {
        if lhs != rhs {
            Context::current_test().unwrap().failed(
                format!("Expected {:?} but got {:?}", lhs, rhs),
                Backtrace::new(),
            )
        }
    }
}
