#[derive(Default)]
pub struct LegendState {
    pub busy: bool,
    pub pending: usize,
    pub input_empty: bool,
    pub can_undo: bool,
    pub think_mode: bool,
}
