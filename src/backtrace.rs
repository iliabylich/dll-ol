#[derive(Debug, Clone)]
pub(crate) struct Backtrace(backtrace::Backtrace);
impl PartialEq for Backtrace {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for Backtrace {}

pub(crate) struct BacktraceSymbol(backtrace::BacktraceSymbol);

impl Backtrace {
    #[cfg(test)]
    pub(crate) fn new() -> Self {
        Self(backtrace::Backtrace::from(vec![]))
    }

    #[cfg(not(test))]
    pub(crate) fn new() -> Self {
        Self(backtrace::Backtrace::new())
    }

    pub(crate) fn symbols(&self) -> Vec<BacktraceSymbol> {
        self.0
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
            .map(|e| BacktraceSymbol(e))
            .collect::<Vec<_>>()
    }
}

impl BacktraceSymbol {
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
