use backtrace::{Backtrace, BacktraceSymbol};

use crate::test::TestName;

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

pub(crate) struct FormattedBacktraceSymbol(BacktraceSymbol);

impl FormattedBacktraceSymbol {
    fn new(sym: BacktraceSymbol) -> Self {
        Self(sym)
    }

    pub(crate) fn symbol_name(&self) -> String {
        self.0
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    pub(crate) fn file_line_col(&self) -> Option<String> {
        let file = self.0.filename().map(|s| s.to_string_lossy().to_string())?;
        let line = self.0.lineno().map(|l| l.to_string())?;
        let col = self.0.colno().map(|c| c.to_string())?;
        Some(format!("at {}:{}:{}", file, line, col))
    }
}
