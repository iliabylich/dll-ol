use crate::{
    assertions::trigger_inclusion as trigger_assertions_inclusion, reporter::Reporter,
    test_suite::test_group::TestGroup,
};

pub struct TestSuite {
    groups: Vec<TestGroup>,
}

impl TestSuite {
    pub fn new(paths: Vec<String>) -> Self {
        trigger_assertions_inclusion();

        let groups = paths.into_iter().map(TestGroup::new).collect::<Vec<_>>();
        Self { groups }
    }

    pub fn from_env() -> Self {
        let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
        Self::new(paths)
    }

    pub fn run(&self) {
        Reporter::report_suite_started();
        for group in &self.groups {
            group.run();
        }
        Reporter::report_suite_finished();
    }
}
