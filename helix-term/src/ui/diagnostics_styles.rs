use helix_core::diagnostic::Severity;
use helix_view::theme::Style;
use helix_view::Theme;

#[derive(Debug)]
pub struct DiagnosticStyles {
    hint: Style,
    info: Style,
    warning: Style,
    error: Style,
}

impl DiagnosticStyles {
    pub fn new(theme: &Theme) -> DiagnosticStyles {
        DiagnosticStyles {
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
}
