use crate::reporter::Reporter;
use std::cell::RefCell;

mod state;
use state::TestState;

mod name;
pub(crate) use name::TestName;

#[derive(Debug, Clone)]
pub(crate) struct Test {
    pub(crate) dlib_path: String,
    pub(crate) name: TestName,
    pub(crate) f: extern "C" fn() -> (),
    pub(crate) state: TestState,
}

extern "C" fn dummy_fn() {}

impl Default for Test {
    fn default() -> Self {
        Self {
            dlib_path: String::new(),
            name: TestName::default(),
            f: dummy_fn,
            state: TestState::default(),
        }
    }
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

    pub(crate) fn run(&self) {
        if !self.name.safe_to_run() {
            return;
        }

        self.set_current();
        Reporter::test_started(&self);
        (self.f)();
        Reporter::test_passed();
    }
}
