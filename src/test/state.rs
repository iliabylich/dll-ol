#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TestState {
    Pending,
    Passed,
    Failed,
}

impl TestState {
    pub(crate) fn set_passed(&mut self) -> bool {
        if *self == TestState::Pending {
            *self = TestState::Passed;
            true
        } else {
            false
        }
    }

    pub(crate) fn set_failed(&mut self) -> bool {
        if *self == TestState::Pending {
            *self = TestState::Failed;
            true
        } else {
            false
        }
    }
}

impl Default for TestState {
    fn default() -> Self {
        Self::Pending
    }
}
