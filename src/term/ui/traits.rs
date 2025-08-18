use ratatui::Frame;
use ratatui::layout::Rect;
pub trait Render {
    fn render(&mut self, frame: &mut Frame);
}
pub trait RenderArea {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
pub trait Clearable {
    fn clear(&mut self);
}
