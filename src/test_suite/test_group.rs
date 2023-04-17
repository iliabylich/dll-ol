use crate::reporter::Reporter;
use crate::test_suite::test::TestState;
use crate::{loader::Loader, parser::Parser};

use crate::test_suite::{Test, Tests};

#[derive(Debug)]
pub struct TestGroup {
    pub(crate) dlib_path: String,

    pub(crate) tests: Vec<Test>,

    #[allow(dead_code)]
    dl: Loader,
}

impl TestGroup {
    pub fn new(dlib_path: &str) -> Self {
        let content = std::fs::read(dlib_path).unwrap();
        let symbols = Parser::new(&content)
            .parse_test_symbols()
            .unwrap_or_default();
        let dl = Loader::new(dlib_path);

        let mut tests = vec![];
        for symbol in symbols {
            let f = dl.get_symbol(&symbol);
            tests.push(Test {
                dlib_path: dlib_path.to_string(),
                name: symbol.clone(),
                f,
                state: TestState::Pending,
            });
        }

        Self {
            dlib_path: dlib_path.to_string(),
            tests,
            dl,
        }
    }

    pub(crate) fn run(&self) {
        Reporter::report_test_group_started(&self.dlib_path, self.tests.len());
        for test in &self.tests {
            test.run();
        }
        Reporter::report_test_group_finished();
    }
}

impl Tests for TestGroup {
    fn tests(&self) -> Vec<Test> {
        self.tests.clone()
    }
}

#[test]
fn test_new_ok() {
    crate::assertions::trigger_inclusion();

    let runner = TestGroup::new(crate::fixtures::FOR_CURRENT_PLATFORM);
    assert_eq!(runner.tests.len(), 3);

    let mut test_names = runner
        .tests
        .iter()
        .map(|test| test.name.to_string())
        .collect::<Vec<_>>();
    test_names.sort_unstable();
    assert_eq!(
        test_names,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}
