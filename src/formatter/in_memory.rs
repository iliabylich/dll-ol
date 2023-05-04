use std::cell::RefCell;

use crate::{
    formatter::FormatterImpl,
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

pub(crate) struct InMemoryFormatter;

thread_local! {
    static LOGGED: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

impl InMemoryFormatter {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn logged() -> Vec<String> {
        LOGGED.with(|v| v.borrow().clone())
    }
}

fn log(s: String) {
    LOGGED.with_borrow_mut(|log| log.push(s));
}

impl FormatterImpl for InMemoryFormatter {
    fn suite_started(&self, _suite: &TestSuite) {
        log(format!("suite_started"))
    }

    fn suite_finished(&self, _suite: &TestSuite) {
        log(format!("suite_finished"))
    }

    fn group_started(&self, _group: &TestGroup) {
        log(format!("group_started"))
    }

    fn group_finished(&self, _group: &TestGroup) {
        log(format!("group_finished"))
    }

    fn test_started(&self, _test: &Test) {
        log(format!("test_started"))
    }

    fn test_passed(&self, _test: &Test) {
        log(format!("test_passed"))
    }

    fn test_failed(&self, _test: &Test, _message: String) {
        log(format!("test_failed"))
    }
}
