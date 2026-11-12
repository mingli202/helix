use helix_core::Diagnostic;
use helix_view::graphics::{Margin, Rect};
use helix_view::theme::Style;
use helix_view::Theme;
use tui::buffer::Buffer;
use tui::text::{Span, Spans, Text};
use tui::widgets::{Paragraph, Widget, Wrap};

use crate::compositor::{Component, Context};

use super::diagnostics_styles::Styles;

pub struct DiagHover {
    diagnostics: Vec<Diagnostic>,
    styles: Styles,
    fg: Style,
}

impl DiagHover {
    pub const ID: &'static str = "diag-hover";

    pub fn new(diagnostics: Vec<Diagnostic>, theme: &Theme) -> Self {
        DiagHover {
            diagnostics,
            styles: Styles::new(theme),
            fg: theme.get("ui.text"),
        }
    }
}

const MARGIN_SIZE: u16 = 1;

impl Component for DiagHover {
    fn render(&mut self, area: Rect, buf: &mut Buffer, cx: &mut Context) {
        let margin = Margin::all(MARGIN_SIZE);
        let area = area.inner(margin);

        let t = diag_to_text(&self.diagnostics, &self.styles, &self.fg);

        let p = Paragraph::new(&t)
            .wrap(Wrap { trim: false })
            .scroll((cx.scroll.unwrap_or_default() as u16, 0));

        p.render(area, buf);
    }

    fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
        let max_text_width = viewport.0.saturating_sub(MARGIN_SIZE * 2).clamp(10, 120);

        let t = diag_to_text(&self.diagnostics, &self.styles, &self.fg);
        let (content_width, content_height) = crate::ui::text::required_size(&t, max_text_width);

        let width = content_width + MARGIN_SIZE * 2;
        let height = MARGIN_SIZE * 2 + content_height;

        Some((width, height))
    }
}

/// List of diagnostics to a Text widget with all the styles
fn diag_to_text<'a>(diagnostics: &'a [Diagnostic], styles: &'a Styles, fg: &'a Style) -> Text<'a> {
    let mut lines = Vec::new();

    for (i, diag) in diagnostics.iter().enumerate() {
        let style = styles.severity_style(diag.severity());
        let prefix = format!("{}. ", i + 1);
        let indent = " ".repeat(prefix.len());

        let mut message_lines = diag.message.lines();
        let first = message_lines.next().unwrap_or_default();
        let first = match &diag.source {
            Some(source) => format!("{first} [{source}]"),
            None => first.to_string(),
        };
        lines.push(Spans::from(vec![
            Span::styled(prefix, *fg),
            Span::styled(first, style),
        ]));

        // continuation lines are indented to line up with the first line
        for line in message_lines {
            lines.push(Spans::from(vec![
                Span::raw(indent.clone()),
                Span::styled(line, style),
            ]));
        }
    }

    Text::from(lines)
}
