use backtrace::Backtrace;

use crate::{context::Context, loader::Loader, reporter::Reporter};

mod state;
use state::TestState;

mod name;
pub(crate) use name::TestName;

#[derive(Debug)]
pub(crate) struct Test {
    pub(crate) dlib_path: String,
    pub(crate) name: TestName,
    pub(crate) f: extern "C" fn() -> (),
    pub(crate) state: TestState,
}

impl Test {
    pub(crate) fn new(loader: &Loader, name: String) -> Self {
        let f = loader.get_symbol(&name);
        Self {
            dlib_path: loader.path.to_string(),
            name: TestName::new(&name),
            f,
            state: TestState::Pending,
        }
    }

    pub(crate) fn run(self: &mut Box<Self>) {
        if !self.name.safe_to_run() {
            return;
        }

        self.started();
        (self.f)();
        self.passed();
    }

    fn started(self: &mut Box<Self>) {
        let test = self.as_mut() as *mut Self;
        // SAFETY: the Test is boxed and it's never moved.
        Context::set_current_test(unsafe { test.as_mut().unwrap() });

        Reporter::test_started();
    }

    fn passed(&mut self) {
        if self.state.set_passed() {
            Reporter::test_passed();
        }
    }

    // Called by assertions in case of a failure
    pub(crate) fn failed(&mut self, message: String, backtrace: Backtrace) {
        if self.state.set_failed() {
            Reporter::test_failed(message, backtrace);
        }
    }
}
