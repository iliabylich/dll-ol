use backtrace::Backtrace;

use crate::{formatter_backtrace_symbol::FormattedBacktraceSymbol, test::TestName};

pub(crate) struct Failure {
    pub(crate) dlib_path: String,
    pub(crate) test_name: TestName,
    pub(crate) message: String,
    pub(crate) backtrace: Backtrace,
}

impl Failure {
    pub(crate) fn user_backtrace(&self) -> Vec<FormattedBacktraceSymbol> {
        self.backtrace
            .frames()
            .iter()
            .flat_map(|f| f.symbols())
            .filter(|sym| {
                if let Some(filename) = sym.filename() {
                    let ext = filename
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();
                    if ext == "rs" {
                        // Rust, ignore
                        false
                    } else {
                        // Non-Rust file with debug symbols
                        true
                    }
                } else {
                    // Non-Rust file without debug symbols
                    true
                }
            })
            .cloned()
            .map(FormattedBacktraceSymbol::new)
            .collect::<Vec<_>>()
    }
}
