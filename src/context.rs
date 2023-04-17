use crate::{
    reporter::Reporter,
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

pub(crate) struct Context {
    test_suite: TestSuite,

    current_test_group: Option<&'static mut TestGroup>,
    current_test: Option<&'static mut Test>,

    reporter: Reporter,
}

static mut CONTEXT_REF: *mut Context = std::ptr::null_mut();

impl Context {
    fn current() -> &'static mut Context {
        // SAFETY: this object never leaves `run` function (and it's boxed), so it's safe to
        // transmute it to a static reference.
        unsafe { CONTEXT_REF.as_mut().unwrap() }
    }

    pub(crate) fn current_test_group() -> Option<&'static mut TestGroup> {
        Self::current().current_test_group.as_deref_mut()
    }

    pub(crate) fn set_current_test_group(test_group: &'static mut TestGroup) {
        Self::current().current_test_group = Some(test_group)
    }

    pub(crate) fn current_test() -> Option<&'static mut Test> {
        Self::current().current_test.as_deref_mut()
    }

    pub(crate) fn set_current_test(test: &'static mut Test) {
        Self::current().current_test = Some(test)
    }

    pub(crate) fn reporter() -> &'static mut Reporter {
        &mut Self::current().reporter
    }
}

pub fn run(paths: Vec<String>) {
    crate::assertions::trigger_inclusion();

    let ctx = Box::new(Context {
        test_suite: TestSuite::from(paths),
        current_test_group: None,
        current_test: None,
        reporter: Reporter::default(),
    });

    // Populate global context.
    unsafe { CONTEXT_REF = Box::into_raw(ctx) }

    Context::current().test_suite.run();

    // Free the context.
    unsafe { Box::from_raw(CONTEXT_REF) };
}

pub fn run_from_env() {
    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    run(paths)
}
