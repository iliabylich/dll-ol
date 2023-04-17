use crate::reporter::Reporter;
use crate::{loader::Loader, parser::Parser};

use crate::test::{Test, TestName};

#[derive(Debug)]
pub(crate) struct TestGroup {
    pub(crate) dlib_path: String,

    pub(crate) tests: Vec<Test>,

    #[allow(dead_code)]
    dl: Loader,
}

impl TestGroup {
    pub(crate) fn new(dlib_path: String) -> Self {
        let content = std::fs::read(&dlib_path).unwrap();
        let symbols = Parser::new(&content).parse_test_symbols();
        let dl = Loader::new(&dlib_path);

        let mut tests = vec![];
        for symbol in symbols {
            let f = dl.get_symbol(&symbol);
            tests.push(Test {
                dlib_path: dlib_path.clone(),
                name: TestName::new(&symbol),
                f,
                ..Default::default()
            });
        }

        Self {
            dlib_path,
            tests,
            dl,
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

    let runner = TestGroup::new(crate::fixtures::FOR_CURRENT_PLATFORM.to_string());
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
