use crate::context::Context;
use crate::{loader::Loader, parser::Parser};

use crate::test::Test;

#[derive(Debug)]
pub(crate) struct TestGroup {
    dlib_path: String,

    tests: Vec<Box<Test>>,

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
            tests.push(Box::new(Test::new(&loader, symbol)));
        }

        Self {
            dlib_path,
            tests,
            dl: loader,
        }
    }
}

impl TestGroup {
    pub(crate) fn run(self: &mut Box<Self>) {
        self.started();
        for test in &mut self.tests {
            test.run();
        }
        self.finished();
    }

    pub(crate) fn tests_count(&self) -> usize {
        self.tests.len()
    }

    pub(crate) fn name(&self) -> &str {
        &self.dlib_path
    }

    fn started(self: &mut Box<Self>) {
        let test_group = self.as_mut() as *mut Self;
        // SAFETY: the Test is boxed and it's never moved.
        Context::set_current_test_group(unsafe { test_group.as_mut().unwrap() });

        Context::reporter().test_group_started()
    }

    fn finished(&self) {
        Context::reporter().test_group_finished()
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
