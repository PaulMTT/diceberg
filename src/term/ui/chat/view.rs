use crate::term::ui::chat::message::{assistant_text_from_responses, message_to_lines};
use crate::term::ui::chat::state::ChatState;
use crate::term::ui::traits::{Clearable, RenderArea};
use mistralrs::TextMessageRole;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
#[derive(typed_builder::TypedBuilder)]
pub struct ChatView {
    pub state: ChatState,
}
impl RenderArea for ChatView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("Chat");
        let inner = block.inner(area);
        let mut lines: Vec<Line> = Vec::new();
        for turn in &self.state.turns {
            lines.extend(message_to_lines(&TextMessageRole::User, &turn.user));
            let a_text = assistant_text_from_responses(&turn.responses);
            if !a_text.is_empty() {
                lines.extend(message_to_lines(&TextMessageRole::Assistant, &a_text));
            }
        }
        let measuring = Paragraph::new(lines.clone()).wrap(Wrap { trim: false });
        self.state.view_height = inner.height;
        self.state.content_height = measuring.line_count(inner.width) as u16;
        self.state
            .scroll
            .reconcile(self.state.content_height, self.state.view_height);
        frame.render_widget(block, area);
        let chat = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.state.scroll.value, 0));
        frame.render_widget(chat, inner);
        let scroll_range = self
            .state
            .content_height
            .saturating_sub(self.state.view_height) as usize;
        let mut sb_state = ScrollbarState::default()
            .content_length(scroll_range.saturating_add(1))
            .viewport_content_length(self.state.view_height as usize)
            .position(self.state.scroll.value as usize);
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);
        let bar_area = Rect {
            x: area.x + area.width - 1,
            y: area.y + 1,
            width: 1,
            height: inner.height,
        };
        frame.render_stateful_widget(scrollbar, bar_area, &mut sb_state);
    }
}
impl Clearable for ChatView {
    fn clear(&mut self) {
        self.state.clear();
    }
}
