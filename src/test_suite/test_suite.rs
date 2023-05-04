use crate::{formatter::Formatter, test_suite::test_group::TestGroup};

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
        Formatter::suite_started(self)
    }

    fn finished(&self) {
        Formatter::suite_finished(self)
    }
}
