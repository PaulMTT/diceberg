use mistralrs::{RequestBuilder, TextMessageRole, TextMessages};
use std::collections::VecDeque;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::term::duplex::DuplexSource;
use crate::term::llm_chat_sink::{CancelCtl, ChatEvent};
use crate::term::ui::chat::message::Message;
use crate::term::ui::chat::state::ChatState;
use crate::term::ui::chat::view::ChatView;
use crate::term::ui::input::state::InputState;
use crate::term::ui::input::view::InputView;
use crate::term::ui::legend::state::LegendState;
use crate::term::ui::legend::view::LegendView;
use crate::term::ui::render::{Render, RenderArea};
use crate::term::ui::status::state::StatusState;
use crate::term::ui::status::view::StatusView;

const DEFAULT_SYSTEM_PROMPT: &str = r#"
You are a large language model that thinks and acts with unwavering decisiveness.
Always select a single best answer, action, or interpretation without hesitation.
Do not present multiple alternatives unless explicitly asked.
Avoid hedging language such as “possibly,” “might,” “it could be.”
Default to action: if information is missing, infer the most likely scenario and proceed.
Treat uncertainty as a normal part of decision-making — choose and move forward.
Present conclusions as final unless proven wrong by new facts.
Your tone is confident, clear, and committed.
Every output should read like the decision has already been made.
Your goal: Choose. State. Move on.
"#;

#[derive(Clone)]
struct PendingReq {
    prompt: String,
    think: bool,
}

pub struct MistralDuplexSourceUi<S>
where
    S: DuplexSource<FromSink = ChatEvent, Cancel = CancelCtl, ToSink = RequestBuilder>,
{
    source: S,
    chat: ChatView,
    input: InputView,
    legend: LegendView,
    status: StatusView,

    think_mode: bool,
    busy: bool,
    aborting: bool,
    pending: VecDeque<PendingReq>,
}

impl<S> MistralDuplexSourceUi<S>
where
    S: DuplexSource<FromSink = ChatEvent, Cancel = CancelCtl, ToSink = RequestBuilder>,
{
    pub fn new(source: S) -> Self {
        let mut chat = ChatView::builder().state(ChatState::default()).build();
        chat.state.system_prompt = DEFAULT_SYSTEM_PROMPT.to_string();

        Self {
            source,
            chat,
            input: InputView::builder().state(InputState::default()).build(),
            legend: LegendView::builder().state(LegendState::default()).build(),
            status: StatusView::builder()
                .state(StatusState::from("Ready"))
                .build(),
            think_mode: false,
            busy: false,
            aborting: false,
            pending: VecDeque::new(),
        }
    }

    #[inline]
    fn occupied(&self) -> bool {
        self.busy || self.aborting
    }

    #[inline]
    fn set_status<T: Into<String>>(&mut self, msg: T) {
        self.status.state.set_text(msg.into());
    }

    #[inline]
    fn update_pending_badge(&mut self) {
        self.legend.state.pending = self.pending.len();
    }

    #[inline]
    fn clear_queue(&mut self) {
        self.pending.clear();
        self.update_pending_badge();
    }

    fn start_or_queue(&mut self, req: PendingReq) {
        if self.occupied() {
            self.pending.push_back(req);
            self.update_pending_badge();
            let n = self.pending.len();
            self.set_status(if n == 1 {
                "Queued (1 item)".to_string()
            } else {
                format!("Queued ({} items)", n)
            });
        } else {
            self.start_request(req);
        }
    }

    fn start_request(&mut self, req: PendingReq) {
        self.chat.state.history.push(Message::user(req.prompt));
        self.chat.state.partial.clear();
        self.busy = true;
        self.aborting = false;
        self.chat.state.scroll.follow = true;

        let mut msgs = TextMessages::new();
        for (role, text) in self.chat.state.as_model_messages() {
            msgs = msgs.add_message(role, text);
        }

        let rb = RequestBuilder::from(msgs)
            .enable_thinking(req.think)
            .set_sampler_temperature(if req.think { 0.6 } else { 0.7 })
            .set_sampler_topp(if req.think { 0.95 } else { 0.8 })
            .set_sampler_topk(20)
            .set_sampler_minp(0.0)
            .set_sampler_frequency_penalty(0.20)
            .set_sampler_presence_penalty(0.5);

        if let Err(e) = self.source.try_send_to_sink(rb) {
            self.busy = false;
            if matches!(self.chat.state.history.last(), Some(m) if m.role == TextMessageRole::User)
            {
                self.chat.state.history.pop();
            }
            self.set_status(format!("Send failed: {e}"));
            self.set_status("Ready");
        } else {
            self.set_status(if self.think_mode {
                "Sending... (think)"
            } else {
                "Sending..."
            });
        }
    }

    fn finalize_turn_ok(&mut self, final_text_opt: Option<String>) {
        if !self.busy {
            return;
        }

        let final_text =
            final_text_opt.unwrap_or_else(|| std::mem::take(&mut self.chat.state.partial));
        if !final_text.is_empty() {
            self.chat.state.history.push(Message::assistant(final_text));
        } else {
            self.chat.state.partial.clear();
        }
        self.busy = false;
        self.after_turn_completed();
    }

    fn on_error(&mut self, err: Box<dyn std::error::Error + Send + Sync>) {
        let msg = err.to_string();
        self.chat.state.partial.clear();

        if matches!(self.chat.state.history.last(), Some(m) if m.role == TextMessageRole::User) {
            self.chat
                .state
                .history
                .push(Message::assistant(format!("Error: {msg}")));
        }

        self.busy = false;
        self.aborting = false;
        self.set_status(format!("Error: {msg}"));
        self.after_turn_completed();
    }

    fn on_cancel_ack(&mut self) {
        self.chat.state.partial.clear();
        self.busy = false;
        self.aborting = false;
        self.set_status("Canceled");
        self.after_turn_completed();
    }

    fn after_turn_completed(&mut self) {
        if self.pending.is_empty() {
            self.set_status("Completed • Ready");
            return;
        }
        let n = self.pending.len();
        self.set_status(if n == 1 {
            "Completed • Next up (1 item)".to_string()
        } else {
            format!("Completed • Next up ({} items)", n)
        });

        if let Some(next) = self.pending.pop_front() {
            self.update_pending_badge();
            self.set_status("Dequeuing...");
            self.start_request(next);
        } else {
            self.set_status("Ready");
        }
    }

    fn abort_current_turn_and_clear_queue(&mut self) {
        if !self.occupied() {
            return;
        }

        let _ = self.source.cancel_tx().send(CancelCtl::AbortCurrent);
        self.aborting = true;

        if matches!(self.chat.state.history.last(), Some(m) if m.role == TextMessageRole::User) {
            self.chat.state.history.pop();
        }

        self.chat.state.partial.clear();
        self.clear_queue();

        self.set_status("Canceling current turn…");
    }

    fn new_chat_reset(&mut self) {
        if self.occupied() {
            let _ = self.source.cancel_tx().send(CancelCtl::AbortCurrent);
            self.aborting = true;
        }

        self.clear_queue();
        self.chat.state.clear();
        self.chat.state.system_prompt = DEFAULT_SYSTEM_PROMPT.to_string();
        self.input.state.buffer.clear();
        self.chat.state.partial.clear();

        self.busy = false;

        self.set_status("Ready (new chat)");
    }

    fn drain_sink(&mut self) -> bool {
        use tokio::sync::mpsc::error as mpsc_err;

        loop {
            match self.source.try_recv_from_sink() {
                Ok(ev) => {
                    if self.aborting {
                        match ev {
                            ChatEvent::Cancelled => {
                                self.on_cancel_ack();
                            }
                            ChatEvent::Complete => {}
                            ChatEvent::Response(_r) => {}
                        }
                        continue;
                    }

                    match ev {
                        ChatEvent::Response(resp) => match resp {
                            mistralrs::Response::Chunk(chunk) => {
                                if let Some(delta) = chunk_delta_text(&chunk) {
                                    let first = self.chat.state.partial.is_empty();
                                    self.busy = true;
                                    self.chat.state.partial.push_str(&delta);
                                    if first {
                                        self.set_status(if self.think_mode {
                                            "Streaming... (think)"
                                        } else {
                                            "Streaming..."
                                        });
                                    }
                                }
                            }
                            mistralrs::Response::Done(done) => {
                                let final_text = done
                                    .choices
                                    .get(0)
                                    .and_then(|c| c.message.content.as_deref())
                                    .map(|s| s.to_string());
                                self.finalize_turn_ok(final_text);
                            }
                            mistralrs::Response::InternalError(err) => {
                                self.on_error(err);
                            }
                            _ => {}
                        },
                        ChatEvent::Complete => {
                            self.finalize_turn_ok(None);
                        }
                        ChatEvent::Cancelled => {
                            self.on_cancel_ack();
                        }
                    }
                }
                Err(mpsc_err::TryRecvError::Empty) => break,
                Err(mpsc_err::TryRecvError::Disconnected) => {
                    self.set_status("Worker disconnected");
                    self.busy = false;
                    return true;
                }
            }
        }

        false
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
        use std::time::Duration;

        let mut terminal = ratatui::init();
        let res = (|| -> anyhow::Result<()> {
            loop {
                if self.drain_sink() {
                    break Ok(());
                }

                terminal.draw(|f| self.render(f))?;

                if event::poll(Duration::from_millis(33))? {
                    if let Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) = event::read()?
                    {
                        let ctrl = |ch| {
                            modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char(ch)
                        };

                        if code == KeyCode::Esc {
                            self.set_status("Exiting...");
                            break Ok(());
                        }

                        if ctrl('n') {
                            self.new_chat_reset();
                            continue;
                        }

                        if ctrl('t') {
                            self.think_mode = !self.think_mode;
                            self.set_status(if self.think_mode {
                                "Think mode: ON"
                            } else {
                                "Think mode: OFF"
                            });
                            continue;
                        }

                        if ctrl('c') {
                            if !self.input.state.buffer.is_empty() {
                                self.input.state.buffer.clear();
                                self.set_status("Input cleared");
                                continue;
                            }

                            if self.occupied() {
                                self.abort_current_turn_and_clear_queue();
                                continue;
                            }

                            let mut removed = false;
                            if matches!(self.chat.state.history.last(), Some(m) if m.role == TextMessageRole::Assistant)
                            {
                                self.chat.state.history.pop();
                                removed = true;
                            }
                            if matches!(self.chat.state.history.last(), Some(m) if m.role == TextMessageRole::User)
                            {
                                self.chat.state.history.pop();
                                removed = true;
                            }
                            if removed {
                                self.set_status("Removed last exchange");
                            } else {
                                self.set_status("Nothing to cancel");
                            }
                            continue;
                        }

                        match code {
                            KeyCode::Enter => {
                                let raw = std::mem::take(&mut self.input.state.buffer);
                                let prompt = raw.trim().to_string();
                                if prompt.is_empty() {
                                    self.input.state.buffer = raw;
                                    self.set_status("Nothing to send");
                                } else {
                                    let req = PendingReq {
                                        prompt,
                                        think: self.think_mode,
                                    };
                                    self.start_or_queue(req);
                                }
                            }
                            KeyCode::Backspace => {
                                self.input.state.buffer.pop();
                                let len = self.input.state.buffer.len();
                                self.set_status(format!("Editing input • {} chars", len));
                            }
                            KeyCode::Char(ch) => {
                                self.input.state.buffer.push(ch);
                                let len = self.input.state.buffer.len();
                                self.set_status(format!("Editing input • {} chars", len));
                            }
                            KeyCode::Tab => {
                                self.input.state.buffer.push('\t');
                                let len = self.input.state.buffer.len();
                                self.set_status(format!("Editing input • {} chars", len));
                            }
                            KeyCode::Up => {
                                self.chat.state.scroll.line_up();
                            }
                            KeyCode::Down => {
                                self.chat.state.scroll.line_down();
                            }
                            KeyCode::PageUp => {
                                self.chat.state.scroll.page_up();
                            }
                            KeyCode::PageDown => {
                                self.chat.state.scroll.page_down();
                            }
                            KeyCode::Home => {
                                self.chat.state.scroll.to_top();
                            }
                            KeyCode::End => {
                                self.chat.state.scroll.to_bottom();
                            }
                            _ => {}
                        }
                    }
                }
            }
        })();

        ratatui::restore();
        res
    }
}

impl<S> MistralDuplexSourceUi<S>
where
    S: DuplexSource<FromSink = ChatEvent, Cancel = CancelCtl, ToSink = RequestBuilder>
        + Send
        + 'static,
{
    pub fn spawn(mut self) -> tokio::task::JoinHandle<anyhow::Result<()>> {
        tokio::task::spawn_blocking(move || self.run())
    }
}

impl<S> Render for MistralDuplexSourceUi<S>
where
    S: DuplexSource<FromSink = ChatEvent, Cancel = CancelCtl, ToSink = RequestBuilder>,
{
    fn render(&mut self, frame: &mut Frame) {
        self.legend.state.busy = self.busy || self.aborting;
        self.legend.state.think_mode = self.think_mode;
        self.legend.state.input_empty = self.input.state.buffer.trim().is_empty();
        self.legend.state.pending = self.pending.len();
        self.legend.state.can_undo = {
            let h = &self.chat.state.history;
            if h.len() >= 2 {
                matches!(h[h.len() - 2].role, TextMessageRole::User)
                    && matches!(h[h.len() - 1].role, TextMessageRole::Assistant)
            } else {
                false
            }
        };

        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(area);

        self.chat.render(frame, chunks[0]);

        {
            let title = if self.think_mode {
                "Your message (think)"
            } else {
                "Your message"
            };
            let block = Block::default().borders(Borders::ALL).title(title);
            let p = Paragraph::new(self.input.state.buffer.as_str()).block(block);
            frame.render_widget(p, chunks[1]);
        }

        self.legend.render(frame, chunks[2]);
        self.status.render(frame, chunks[3]);
    }
}

fn chunk_delta_text(chunk: &mistralrs::ChatCompletionChunkResponse) -> Option<String> {
    let mut out = String::new();
    for ch in &chunk.choices {
        if let Some(s) = ch.delta.content.as_deref() {
            out.push_str(s);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}
