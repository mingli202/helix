use helix_core::diagnostic::Severity;
use helix_lsp::lsp::DiagnosticSeverity;
use helix_view::theme::Style;
use helix_view::Theme;

#[derive(Debug)]
pub struct Styles {
    hint: Style,
    info: Style,
    warning: Style,
    error: Style,
}

impl Styles {
    pub fn new(theme: &Theme) -> Styles {
        Styles {
            hint: theme.get("hint.diagnostic.inline"),
            info: theme.get("info.diagnostic.inline"),
            warning: theme.get("warning.diagnostic.inline"),
            error: theme.get("error.diagnostic.inline"),
        }
    }

    pub fn severity_style(&self, severity: Severity) -> Style {
        match severity {
            Severity::Hint => self.hint,
            Severity::Info => self.info,
            Severity::Warning => self.warning,
            Severity::Error => self.error,
        }
    }

    pub fn diag_severity_style(&self, severity: DiagnosticSeverity) -> Style {
        match severity {
            DiagnosticSeverity::HINT => self.hint,
            DiagnosticSeverity::INFORMATION => self.info,
            DiagnosticSeverity::WARNING => self.warning,
            DiagnosticSeverity::ERROR => self.error,
            _ => self.info,
        }
    }
}
