use std::cell::RefCell;

use crate::{
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

mod stdout;
use stdout::StdoutFormatter;

#[cfg(test)]
mod in_memory;
#[cfg(test)]
pub(crate) use in_memory::InMemoryFormatter;

trait FormatterImpl {
    fn suite_started(&self, suite: &TestSuite);
    fn suite_finished(&self, suite: &TestSuite);

    fn group_started(&self, group: &TestGroup);
    fn group_finished(&self, group: &TestGroup);

    fn test_started(&self, test: &Test);
    fn test_passed(&self, test: &Test);
    fn test_failed(&self, test: &Test, message: String);
}

thread_local! {
    static FORMATTER: RefCell<Box<dyn FormatterImpl>> = RefCell::new(Box::new(StdoutFormatter::new()));
}

pub(crate) struct Formatter;
impl Formatter {
    fn with_formatter<F: FnOnce(&dyn FormatterImpl)>(f: F) {
        FORMATTER.with(|formatter| f(&**formatter.borrow()))
    }

    pub(crate) fn suite_started(suite: &TestSuite) {
        Self::with_formatter(|formatter| formatter.suite_started(suite))
    }
    pub(crate) fn suite_finished(suite: &TestSuite) {
        Self::with_formatter(|formatter| formatter.suite_finished(suite))
    }

    pub(crate) fn group_started(group: &TestGroup) {
        Self::with_formatter(|formatter| formatter.group_started(group))
    }
    pub(crate) fn group_finished(group: &TestGroup) {
        Self::with_formatter(|formatter| formatter.group_finished(group))
    }

    pub(crate) fn test_started(test: &Test) {
        Self::with_formatter(|formatter| formatter.test_started(test))
    }
    pub(crate) fn test_passed(test: &Test) {
        Self::with_formatter(|formatter| formatter.test_passed(test))
    }
    pub(crate) fn test_failed(test: &Test, message: String) {
        Self::with_formatter(|formatter| formatter.test_failed(test, message))
    }

    #[cfg(test)]
    pub(crate) fn set_in_memory() {
        FORMATTER.set(Box::new(InMemoryFormatter::new()));
    }
}
