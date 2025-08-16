use crate::term::ui::render::RenderArea;
use crate::term::ui::status::state::StatusState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

#[derive(typed_builder::TypedBuilder)]
pub struct StatusView {
    pub state: StatusState,
}
impl RenderArea for StatusView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.state.text.as_str()), area);
    }
}
