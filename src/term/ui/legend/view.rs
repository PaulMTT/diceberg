use crate::term::ui::legend::state::LegendState;
use crate::term::ui::render::RenderArea;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, Span, Style};
use ratatui::widgets::Paragraph;

#[derive(typed_builder::TypedBuilder)]
pub struct LegendView {
    pub state: LegendState,
}

impl LegendView {
    fn line(&self) -> Line<'_> {
        let bold =
            |s: &str| Span::styled(s.to_string(), Style::default().add_modifier(Modifier::BOLD));

        let enter_action = if self.state.busy { "queue" } else { "send" };

        let ctrl_c_action = if !self.state.input_empty {
            "clear input"
        } else if self.state.busy {
            "cancel current"
        } else if self.state.can_undo {
            "undo last"
        } else {
            "nothing"
        };

        let toggle_hint = if self.state.think_mode {
            "thinking off"
        } else {
            "thinking on"
        };

        let items = [
            ("Enter", enter_action),
            ("Ctrl+C", ctrl_c_action),
            ("Esc", "quit"),
            ("Ctrl+N", "new chat"),
            ("Ctrl+T", toggle_hint),
            ("↑/↓", "line"),
            ("PgUp/PgDn", "page"),
            ("Home/End", "top/bottom"),
        ];

        let mut spans: Vec<Span> = vec![bold("Keys: ")];
        for (i, (k, v)) in items.into_iter().enumerate() {
            spans.push(bold(k));
            spans.push(Span::raw(" "));
            spans.push(Span::raw(v));
            if i + 1 != items.len() {
                spans.push(Span::raw("  •  "));
            }
        }

        if self.state.pending > 0 {
            spans.push(Span::raw("  •  "));
            spans.push(bold("Pending"));
            spans.push(Span::raw(": "));
            spans.push(Span::raw(self.state.pending.to_string()));
        }

        Line::from(spans)
    }
}

impl RenderArea for LegendView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.line()), area);
    }
}
