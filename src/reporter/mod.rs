use backtrace::Backtrace;

use crate::{context::Context, failure::Failure, formatter::Formatter};

pub(crate) struct Reporter;

impl Reporter {
    pub(crate) fn suite_started() {
        let test_suite = Context::current_test_suite();
        Formatter::suite_started(test_suite)
    }

    pub(crate) fn suite_finished() {
        let test_suite = Context::current_test_suite();
        Formatter::suite_finished(test_suite);
    }

    pub(crate) fn test_group_started() {
        let test_group = Context::current_test_group().unwrap();
        Formatter::group_started(test_group);
    }

    pub(crate) fn test_group_finished() {
        let test_group = Context::current_test_group().unwrap();
        Formatter::group_finished(test_group);
    }

    pub(crate) fn test_started() {
        let test = Context::current_test().unwrap();
        Formatter::test_started(test);
    }

    pub(crate) fn test_passed() {
        let test = Context::current_test().unwrap();
        Formatter::test_passed(test);
    }

    pub(crate) fn test_failed(message: String, backtrace: Backtrace) {
        let test = Context::current_test().unwrap();
        Context::failures().push(Failure {
            dlib_path: test.dlib_path.clone(),
            test_name: test.name.clone(),
            message: message.clone(),
            backtrace,
        });
        Formatter::test_failed(test, message);
    }
}
