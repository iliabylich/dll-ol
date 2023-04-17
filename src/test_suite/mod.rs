mod test_group;

mod test_suite;
pub use test_suite::TestSuite;

mod test;
pub use test::Test;

pub trait Tests {
    fn tests(&self) -> Vec<Test>;
}
