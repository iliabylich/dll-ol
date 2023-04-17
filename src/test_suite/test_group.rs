use std::cell::RefCell;
use std::rc::Rc;

use crate::context::Context;
use crate::reporter::Reporter;
use crate::{loader::Loader, parser::Parser};

use crate::test::{Test, TestName};

#[derive(Debug)]
pub(crate) struct TestGroup {
    pub(crate) dlib_path: String,

    pub(crate) tests: Vec<Test>,

    ctx: Option<Rc<RefCell<Context>>>,

    #[allow(dead_code)]
    dl: Loader,
}

impl From<String> for TestGroup {
    fn from(dlib_path: String) -> Self {
        let content = std::fs::read(&dlib_path).unwrap();
        let symbols = Parser::new(&content).parse_test_symbols();
        let loader = Loader::new(&dlib_path);

        let mut tests = vec![];
        for symbol in symbols {
            tests.push(Test::new(&loader, symbol));
        }

        Self {
            dlib_path,
            tests,
            dl: loader,
            ctx: None,
        }
    }
}

impl TestGroup {
    pub(crate) fn set_ctx(&mut self, ctx: Rc<RefCell<Context>>) {
        self.ctx = Some(ctx.clone());
        for test in &mut self.tests {
            test.set_ctx(ctx.clone());
        }
    }

    pub(crate) fn run(&self) {
        Reporter::test_group_started(&self.dlib_path, self.tests.len());
        for test in &self.tests {
            test.run();
        }
        Reporter::test_group_finished();
    }
}

#[test]
fn test_new_ok() {
    crate::assertions::trigger_inclusion();

    let runner = TestGroup::from(crate::fixtures::FOR_CURRENT_PLATFORM.to_string());
    assert_eq!(runner.tests.len(), 3);

    let mut test_names = runner
        .tests
        .iter()
        .map(|test| test.name.raw())
        .collect::<Vec<_>>();
    test_names.sort_unstable();
    assert_eq!(
        test_names,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}
