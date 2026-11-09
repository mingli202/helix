use helix_core::diagnostic::DiagnosticProvider;
use helix_lsp::lsp;
use helix_view::graphics::{Margin, Rect};
use helix_view::Theme;
use tui::buffer::Buffer;
use tui::text::{Span, Spans, Text};
use tui::widgets::{Paragraph, Widget, Wrap};

use crate::compositor::{Component, Context};

use super::diagnostics_styles::Styles;

pub struct DiagHover {
    diagnostics: Vec<(lsp::Diagnostic, DiagnosticProvider)>,
    styles: Styles,
}

impl DiagHover {
    pub const ID: &'static str = "diag-hover";

    pub fn new(diagnostics: Vec<(lsp::Diagnostic, DiagnosticProvider)>, theme: &Theme) -> Self {
        DiagHover {
            diagnostics,
            styles: Styles::new(theme),
        }
    }
}

impl Component for DiagHover {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _cx: &mut Context) {
        let margin = Margin::all(1);
        let area = area.inner(margin);

        let t = diag_to_text(&self.diagnostics, &self.styles);
        let p = Paragraph::new(&t);

        p.render(area, buf);
    }

    fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
        let t = diag_to_text(&self.diagnostics, &self.styles);
        let p = Paragraph::new(&t);

        Some(p.required_size(viewport.0))
    }
}

fn diag_to_text<'a>(
    diagnostics: &'a [(lsp::Diagnostic, DiagnosticProvider)],
    styles: &'a Styles,
) -> Text<'a> {
    Text::from(
        diagnostics
            .iter()
            .map(|(diag, _diag_provider)| {
                Spans::from(match diag.severity {
                    None => Span::raw(""),
                    Some(s) => Span::styled(&diag.message, styles.diag_severity_style(s)),
                })
            })
            .collect::<Vec<_>>(),
    )
}
