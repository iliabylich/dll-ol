mod test_group;

mod test_suite;
pub use test_suite::TestSuite;

mod test;
pub use test::Test;
pub(crate) use test::CURRENT as CURRENT_TEST;

pub trait Tests {
    fn tests(&self) -> Vec<Test>;
}
