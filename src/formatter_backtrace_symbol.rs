use backtrace::BacktraceSymbol;

pub(crate) struct FormattedBacktraceSymbol(BacktraceSymbol);

impl FormattedBacktraceSymbol {
    pub(crate) fn new(sym: BacktraceSymbol) -> Self {
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
