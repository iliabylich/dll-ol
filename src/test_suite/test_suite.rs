use crate::{reporter::Reporter, test_suite::test_group::TestGroup};

pub struct TestSuite {
    groups: Vec<TestGroup>,
}

impl TestSuite {
    pub fn new(paths: &[String]) -> Self {
        let mut groups = vec![];
        for path in paths {
            let file = TestGroup::new(path);
            groups.push(file);
        }
        Self { groups }
    }

    pub fn run(&self) {
        Reporter::report_suite_started();
        for group in &self.groups {
            group.run();
        }
        Reporter::report_suite_finished();
    }
}
