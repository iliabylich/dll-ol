use backtrace::Backtrace;

use crate::{
    context::Context,
    failure::Failure,
    formatter::{Formatter, StdoutFormatter},
};

pub(crate) struct Reporter {
    formatter: Box<dyn Formatter>,
}

impl Reporter {
    pub(crate) fn stdout() -> Self {
        Self {
            formatter: Box::new(StdoutFormatter),
        }
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        use crate::formatter::InMemoryFormatter;

        Self {
            formatter: Box::new(InMemoryFormatter),
        }
    }

    pub(crate) fn suite_started(&mut self) {
        let test_suite = Context::current_test_suite();
        self.formatter.suite_started(test_suite);
    }

    pub(crate) fn suite_finished(&mut self) {
        let test_suite = Context::current_test_suite();
        self.formatter.suite_finished(test_suite);
    }

    pub(crate) fn test_group_started(&mut self) {
        let test_group = Context::current_test_group().unwrap();
        self.formatter.group_started(test_group);
    }

    pub(crate) fn test_group_finished(&mut self) {
        let test_group = Context::current_test_group().unwrap();
        self.formatter.group_finished(test_group);
    }

    pub(crate) fn test_started(&mut self) {
        let test = Context::current_test().unwrap();
        self.formatter.test_started(test);
    }

    pub(crate) fn test_passed(&mut self) {
        let test = Context::current_test().unwrap();
        self.formatter.test_passed(test);
    }

    pub(crate) fn test_failed(&mut self, message: String, backtrace: Backtrace) {
        let test = Context::current_test().unwrap();
        Context::failures().push(Failure {
            dlib_path: test.dlib_path.clone(),
            test_name: test.name.clone(),
            message: message.clone(),
            backtrace,
        });
        self.formatter.test_failed(test, message);
    }
}
