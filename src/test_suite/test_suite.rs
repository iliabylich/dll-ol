use std::{cell::RefCell, rc::Rc};

use crate::{context::Context, reporter::Reporter, test_suite::test_group::TestGroup};

pub struct TestSuite {
    ctx: Option<Rc<RefCell<Context>>>,
    groups: Vec<TestGroup>,
}

impl From<Vec<String>> for TestSuite {
    fn from(paths: Vec<String>) -> Self {
        let groups = paths.into_iter().map(TestGroup::from).collect::<Vec<_>>();
        Self { ctx: None, groups }
    }
}

impl TestSuite {
    pub(crate) fn set_ctx(&mut self, ctx: Rc<RefCell<Context>>) {
        self.ctx = Some(ctx.clone());
        for group in &mut self.groups {
            group.set_ctx(ctx.clone());
        }
    }

    pub fn run(&self) {
        Reporter::suite_started();
        for group in &self.groups {
            group.run();
        }
        Reporter::suite_finished();
    }
}
