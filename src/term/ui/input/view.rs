use crate::term::ui::input::state::InputState;
use crate::term::ui::render::RenderArea;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};

#[derive(typed_builder::TypedBuilder)]
pub struct InputView {
    pub state: InputState,
}
impl RenderArea for InputView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.state.buffer.as_str())
            .block(Block::default().borders(Borders::ALL).title("Your message"));
        frame.render_widget(input, area);
    }
}
