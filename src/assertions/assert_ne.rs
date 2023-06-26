use backtrace::Backtrace;

use crate::context::Context;

pub(crate) struct AssertNe<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: PartialEq + std::fmt::Debug> AssertNe<T> {
    pub(crate) fn run(lhs: T, rhs: T) {
        if lhs == rhs {
            Context::current_test().unwrap().failed(
                format!("Expected {:?} but got {:?}", lhs, rhs),
                Backtrace::new(),
            )
        }
    }
}
