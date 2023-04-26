use crate::{
    context::Context,
    formatter::Formatter,
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

pub(crate) struct InMemoryFormatter;

fn log(s: String) {
    Context::logged().push(s);
}

impl Formatter for InMemoryFormatter {
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
