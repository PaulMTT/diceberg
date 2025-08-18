use crate::term::ui::traits::Clearable;
#[derive(Default)]
pub struct InputState {
    pub buffer: String,
    pub think_mode: bool,
}
impl Clearable for InputState {
    fn clear(&mut self) {
        self.buffer.clear();
    }
}
