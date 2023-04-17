use std::{cell::RefCell, rc::Rc};

use crate::{
    reporter::Reporter,
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

pub(crate) struct Context {
    test_suite: TestSuite,

    current_test_group: Option<Rc<RefCell<TestGroup>>>,
    current_test: Option<Rc<RefCell<Test>>>,

    reporter: Reporter,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}

pub fn run(paths: Vec<String>) {
    crate::assertions::trigger_inclusion();

    let ctx = Rc::new(RefCell::new(Context {
        test_suite: TestSuite::from(paths),
        current_test_group: None,
        current_test: None,
        reporter: Reporter::default(),
    }));

    {
        let mut ctx_mut = ctx.borrow_mut();
        ctx_mut.test_suite.set_ctx(ctx.clone());
        ctx_mut.reporter.set_ctx(ctx.clone());
    }

    {
        ctx.borrow().test_suite.run();
    }
}

pub fn run_from_env() {
    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    run(paths)
}
