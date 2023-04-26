use crate::{
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

mod stdout;
pub(crate) use stdout::StdoutFormatter;

#[cfg(test)]
mod in_memory;
#[cfg(test)]
pub(crate) use in_memory::InMemoryFormatter;

pub(crate) trait Formatter {
    fn suite_started(&self, suite: &TestSuite);
    fn suite_finished(&self, suite: &TestSuite);

    fn group_started(&self, group: &TestGroup);
    fn group_finished(&self, group: &TestGroup);

    fn test_started(&self, test: &Test);
    fn test_passed(&self, test: &Test);
    fn test_failed(&self, test: &Test, message: String);
}
