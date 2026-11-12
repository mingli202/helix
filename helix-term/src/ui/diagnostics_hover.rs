use helix_core::Diagnostic;
use helix_view::graphics::{Margin, Rect};
use helix_view::Theme;
use tui::buffer::Buffer;
use tui::text::{Span, Spans, Text};
use tui::widgets::{Paragraph, Widget, Wrap};

use crate::compositor::{Component, Context};

use super::diagnostics_styles::Styles;

pub struct DiagHover {
    diagnostics: Vec<Diagnostic>,
    styles: Styles,
}

impl DiagHover {
    pub const ID: &'static str = "diag-hover";

    pub fn new(diagnostics: Vec<Diagnostic>, theme: &Theme) -> Self {
        DiagHover {
            diagnostics,
            styles: Styles::new(theme),
        }
    }
}

const MARGIN_SIZE: u16 = 1;

impl Component for DiagHover {
    fn render(&mut self, area: Rect, buf: &mut Buffer, cx: &mut Context) {
        let margin = Margin::all(MARGIN_SIZE);
        let area = area.inner(margin);

        let t = diag_to_text(&self.diagnostics, &self.styles);

        let p = Paragraph::new(&t)
            .wrap(Wrap { trim: false })
            .scroll((cx.scroll.unwrap_or_default() as u16, 0));

        p.render(area, buf);
    }

    fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
        let t = diag_to_text(&self.diagnostics, &self.styles);
        let (content_width, content_height) = crate::ui::text::required_size(&t, viewport.0);

        let width = content_width + MARGIN_SIZE * 2;
        let height = MARGIN_SIZE * 2 + content_height;

        Some((width, height))
    }
}

/// List of diagnostics to a Text widget with all the styles
fn diag_to_text<'a>(diagnostics: &'a [Diagnostic], styles: &'a Styles) -> Text<'a> {
    Text::from(
        diagnostics
            .iter()
            .enumerate()
            .map(|(i, diag)| {
                Spans::from(Span::styled(
                    format!(
                        "{}. {} [{:?}]",
                        i + 1,
                        &diag.message,
                        diag.source.as_ref().unwrap_or(&"none".to_string())
                    ),
                    styles.severity_style(diag.severity()),
                ))
            })
            .collect::<Vec<_>>(),
    )
}
