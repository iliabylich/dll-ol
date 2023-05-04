use crate::{reporter::Reporter, test_suite::test_group::TestGroup};

pub struct TestSuite {
    groups: Vec<Box<TestGroup>>,
}

impl From<Vec<String>> for TestSuite {
    fn from(paths: Vec<String>) -> Self {
        let groups = paths
            .into_iter()
            .map(TestGroup::from)
            .map(Box::new)
            .collect::<Vec<_>>();
        Self { groups }
    }
}

impl TestSuite {
    pub fn run(&mut self) {
        self.started();
        for group in &mut self.groups {
            group.run();
        }
        self.finished();
    }

    fn started(&self) {
        Reporter::suite_started()
    }

    fn finished(&self) {
        Reporter::suite_finished()
    }
}
