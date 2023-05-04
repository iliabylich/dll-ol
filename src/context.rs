use crate::{
    failure::Failure,
    reporter::Reporter,
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

pub(crate) struct Context {
    test_suite: TestSuite,

    current_test_group: Option<&'static mut TestGroup>,
    current_test: Option<&'static mut Test>,

    reporter: Reporter,

    failures: Vec<Failure>,

    #[cfg(test)]
    log: Vec<String>,
}

static mut CONTEXT_REF: *mut Context = std::ptr::null_mut();

impl Context {
    fn new() -> Self {
        Context {
            test_suite: TestSuite::from(vec![]),
            current_test_group: None,
            current_test: None,
            reporter: Reporter::stdout(),
            failures: vec![],
            #[cfg(test)]
            log: vec![],
        }
    }

    fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.test_suite = TestSuite::from(paths);
        self
    }

    #[cfg(test)]
    fn with_reporter(mut self, reporter: Reporter) -> Self {
        self.reporter = reporter;
        self
    }

    fn current() -> &'static mut Context {
        // SAFETY: this object never leaves `run` function (and it's boxed), so it's safe to
        // transmute it to a static reference.
        unsafe { CONTEXT_REF.as_mut().unwrap() }
    }

    pub(crate) fn current_test_suite() -> &'static mut TestSuite {
        &mut Self::current().test_suite
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

    pub(crate) fn failures() -> &'static mut Vec<Failure> {
        &mut Self::current().failures
    }

    #[cfg(test)]
    pub(crate) fn logged() -> &'static mut Vec<String> {
        &mut Self::current().log
    }
}

pub fn run(paths: Vec<String>) {
    crate::assertions::trigger_inclusion();

    let ctx = Box::new(Context::new().with_paths(paths));

    // Populate global context.
    unsafe { CONTEXT_REF = Box::into_raw(ctx) }

    Context::current().test_suite.run();

    // Free the context.
    unsafe { Box::from_raw(CONTEXT_REF) };
}

pub fn run_from_env() {
    let paths = std::env::args().skip(1).collect::<Vec<_>>();
    run(paths)
}

#[test]
fn test_everything() {
    crate::assertions::trigger_inclusion();

    let path = crate::fixtures::FOR_CURRENT_PLATFORM.to_string();
    let ctx = Box::new(
        Context::new()
            .with_paths(vec![path])
            .with_reporter(Reporter::in_memory()),
    );
    unsafe { CONTEXT_REF = Box::into_raw(ctx) }
    Context::current().test_suite.run();
    let logged = Context::logged();
    assert_eq!(
        logged,
        &[
            "suite_started",
            "group_started",
            "test_started",
            "test_failed",
            "test_started",
            "test_passed",
            "group_finished",
            "suite_finished",
        ]
    );
    unsafe { Box::from_raw(CONTEXT_REF) };
}
