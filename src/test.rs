use crate::{
    backtrace::Backtrace,
    dylib::loader::Loader,
    logger::{LogEvent, Logger},
};
use std::cell::RefCell;

pub(crate) struct Test {
    pub(crate) name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TestResult {
    Passed,
    Failed {
        name: String,
        message: String,
        backtrace: Backtrace,
    },
    Skip,
}

pub(crate) enum AssertionFailed {
    Yes(String, Backtrace),
    No,
}

thread_local! {
    static CURRENT_TEST_RESULT: RefCell<AssertionFailed> = RefCell::new(AssertionFailed::No);
}

impl Test {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }

    pub(crate) fn run(self, loader: &Loader, logger: &mut Logger) -> TestResult {
        logger.push(LogEvent::TestStarted {
            test_name: self.pretty_name(),
        });

        let test_name = self.pretty_name();

        if self.name.contains("__ol_test_crash") {
            logger.push(LogEvent::TestSkipped);

            return TestResult::Skip;
        }

        CURRENT_TEST_RESULT.with(|result| {
            *result.borrow_mut() = AssertionFailed::No;
        });

        let f = loader.get_symbol(&self.name);
        f();

        CURRENT_TEST_RESULT.with(|result| match *result.borrow() {
            AssertionFailed::Yes(ref message, ref backtrace) => {
                logger.push(LogEvent::TestFailed);

                return TestResult::Failed {
                    name: test_name,
                    message: message.clone(),
                    backtrace: backtrace.clone(),
                };
            }
            AssertionFailed::No => {
                logger.push(LogEvent::TestPassed);

                return TestResult::Passed;
            }
        })
    }

    pub(crate) fn fail_current(message: String, backtrace: Backtrace) {
        CURRENT_TEST_RESULT.with(|result| {
            *result.borrow_mut() = AssertionFailed::Yes(message, backtrace);
        });
    }

    const PREFIX: &str = "__ol_test_";

    pub(crate) fn pretty_name(&self) -> String {
        if self.name.starts_with(Self::PREFIX) {
            self.name.strip_prefix(Self::PREFIX).unwrap()
        } else {
            &self.name
        }
        .to_string()
    }
}
