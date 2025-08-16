use crate::term::ui::chat::message::Messages;
use crate::term::ui::chat::scroll::ScrollModel;
use mistralrs::TextMessageRole;

#[derive(Default)]
pub struct ChatState {
    pub history: Messages,
    pub partial: String,
    pub scroll: ScrollModel,
    pub view_height: u16,
    pub content_height: u16,
    pub system_prompt: String,
}

impl ChatState {
    pub fn clear(&mut self) {
        self.history.clear();
        self.partial.clear();
        self.scroll = ScrollModel::default();
        self.view_height = 0;
        self.content_height = 0;
    }

    pub fn as_model_messages(&self) -> Vec<(TextMessageRole, String)> {
        let mut msgs = Vec::new();
        msgs.push((TextMessageRole::System, self.system_prompt.clone()));
        msgs.extend(
            self.history
                .iter()
                .map(|m| (m.role.clone(), m.content.clone())),
        );
        msgs
    }
}
