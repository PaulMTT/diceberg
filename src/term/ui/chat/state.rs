use crate::term::ui::chat::message::assistant_text_from_responses;
use crate::term::ui::chat::scroll::ScrollModel;
use crate::term::ui::traits::Clearable;
use mistralrs::{Response, TextMessageRole};
#[derive(Default)]
pub struct Turn {
    pub user: String,
    pub responses: Vec<Response>,
    pub complete: bool,
    pub error: Option<String>,
}
#[derive(Default)]
pub struct ChatState {
    pub system_prompt: String,
    pub turns: Vec<Turn>,
    pub scroll: ScrollModel,
    pub view_height: u16,
    pub content_height: u16,
}
impl Clearable for ChatState {
    fn clear(&mut self) {
        self.turns.clear();
        self.scroll = ScrollModel::default();
        self.view_height = 0;
        self.content_height = 0;
    }
}
impl ChatState {
    pub fn push_turn(&mut self, user: String) {
        self.turns.push(Turn {
            user,
            responses: Vec::new(),
            complete: false,
            error: None,
        });
    }
    pub fn pop_last_turn(&mut self) {
        let _ = self.turns.pop();
    }
    pub fn pop_last_completed_turn(&mut self) -> bool {
        if matches!(self.turns.last(), Some(t) if t.complete) {
            self.turns.pop();
            true
        } else {
            false
        }
    }
    pub fn has_completed_turn(&self) -> bool {
        self.turns.iter().any(|t| t.complete)
    }
    pub fn append_response(&mut self, resp: Response) {
        if let Some(t) = self.turns.last_mut() {
            t.responses.push(resp);
        }
    }
    pub fn mark_last_complete(&mut self) {
        if let Some(t) = self.turns.last_mut() {
            t.complete = true;
        }
    }
    pub fn finish_last_with_error(&mut self, msg: String) {
        if let Some(t) = self.turns.last_mut() {
            t.error = Some(msg);
            t.complete = true;
        }
    }
    pub fn assistant_text_for_turn(&self, t: &Turn) -> String {
        if let Some(e) = &t.error {
            return format!("Error: {e}");
        }
        assistant_text_from_responses(&t.responses)
    }
    pub fn current_assistant_text(&self) -> String {
        if let Some(t) = self.turns.last() {
            return self.assistant_text_for_turn(t);
        }
        String::new()
    }
    pub fn model_messages_for_send(&self) -> Vec<(TextMessageRole, String)> {
        let mut out = Vec::new();
        out.push((TextMessageRole::System, self.system_prompt.clone()));
        for t in &self.turns {
            if t.complete {
                out.push((TextMessageRole::User, t.user.clone()));
                let a = self.assistant_text_for_turn(t);
                if !a.is_empty() {
                    out.push((TextMessageRole::Assistant, a));
                }
            } else {
                out.push((TextMessageRole::User, t.user.clone()));
                break;
            }
        }
        out
    }
    pub fn last_usage(&self) -> Option<&mistralrs::Usage> {
        self.turns.last().and_then(|t| {
            t.responses.iter().rev().find_map(|r| {
                if let mistralrs::Response::Chunk(c) = r {
                    c.usage.as_ref()
                } else {
                    None
                }
            })
        })
    }
}
