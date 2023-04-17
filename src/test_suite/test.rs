use crate::reporter::Reporter;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum TestState {
    Pending,
    Passed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Test {
    pub dlib_path: String,
    pub name: String,
    pub f: extern "C" fn() -> (),
    pub state: TestState,
}

thread_local! {
    pub(crate) static CURRENT: RefCell<Option<Test>> = RefCell::new(None);
}

impl Test {
    pub(crate) fn set_current(&self) {
        CURRENT.with(|current| {
            *current.borrow_mut() = Some(self.clone());
        });
    }

    pub(crate) fn set_passed(&mut self) -> bool {
        if self.state == TestState::Pending {
            self.state = TestState::Passed;
            true
        } else {
            false
        }
    }

    pub(crate) fn set_failed(&mut self) -> bool {
        if self.state == TestState::Pending {
            self.state = TestState::Passed;
            true
        } else {
            false
        }
    }

    pub(crate) fn display_name(&self) -> &str {
        if self.name.starts_with("__ol_test_") {
            self.name.strip_prefix("__ol_test_").unwrap()
        } else {
            &self.name
        }
    }

    pub(crate) fn run(&self) {
        if self.name.contains("__ol_test_crash") {
            return;
        }

        self.set_current();
        Reporter::report_test_started(&self);
        (self.f)();
        Reporter::report_test_success();
    }
}
