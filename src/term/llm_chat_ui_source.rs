use crate::term::duplex::DuplexSource;
use crate::term::llm_chat_sink::{CancelCtl, ChatEvent};
use crate::term::ui::chat::state::ChatState;
use crate::term::ui::chat::view::ChatView;
use crate::term::ui::input::state::InputState;
use crate::term::ui::input::view::InputView;
use crate::term::ui::legend::state::LegendState;
use crate::term::ui::legend::view::LegendView;
use crate::term::ui::status::state::StatusState;
use crate::term::ui::status::view::StatusView;
use crate::term::ui::traits::{Clearable, Render, RenderArea};
use mistralrs::{RequestBuilder, TextMessages};
use ratatui::Frame;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use std::collections::VecDeque;
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
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
    fn set_status<T: Into<String>>(&mut self, msg: T) {
        self.status.state.set_text(msg.into());
    }
    fn start_or_queue(&mut self, req: PendingReq) {
        if self.busy || self.aborting {
            self.queue_request(req);
        } else {
            self.start_request(req);
        }
    }
    fn queue_request(&mut self, req: PendingReq) {
        self.pending.push_back(req);
        self.set_status(format!("Queued ({} items)", self.pending.len()));
    }
    fn start_request(&mut self, req: PendingReq) {
        self.chat.state.push_turn(req.prompt.clone());
        let msgs_vec = self.chat.state.model_messages_for_send();
        let mut msgs = TextMessages::new();
        for (role, text) in msgs_vec {
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
        self.send_request(rb);
    }
    fn send_request(&mut self, req: RequestBuilder) {
        self.busy = true;
        self.aborting = false;
        self.chat.state.scroll.follow = true;
        if let Err(e) = self.source.try_send_to_sink(req) {
            self.busy = false;
            self.chat.state.pop_last_turn();
            self.set_status(format!("Send failed: {e}"));
        } else {
            self.set_status(if self.think_mode {
                "Sending... (think)"
            } else {
                "Sending..."
            });
        }
    }
    fn finalize_turn_ok(&mut self) {
        self.chat.state.mark_last_complete();
        if let Some(usage) = self.chat.state.last_usage() {
            self.set_status(format!(
                "Completed • total={} tok ({} prompt + {} completion) • {:.2} tok/s",
                usage.total_tokens,
                usage.prompt_tokens,
                usage.completion_tokens,
                usage.avg_tok_per_sec,
            ));
        }
        self.after_turn_completed();
    }
    fn on_error(&mut self, err: Box<dyn std::error::Error + Send + Sync>) {
        let msg = err.to_string();
        self.chat.state.finish_last_with_error(msg.clone());
        self.set_status(format!("Error: {msg}"));
        self.after_turn_completed();
    }
    fn on_cancel_ack(&mut self) {
        self.set_status("Canceled");
        self.after_turn_completed();
    }
    fn after_turn_completed(&mut self) {
        self.busy = false;
        self.aborting = false;
        if let Some(next) = self.pending.pop_front() {
            self.set_status("Dequeuing...");
            self.start_request(next);
        }
    }
    fn cancel_current(&mut self) {
        self.abort_and_clear_queue();
        self.chat.state.pop_last_turn();
        self.set_status("Canceling current turn…");
    }
    fn new_chat(&mut self) {
        self.abort_and_clear_queue();
        self.chat.clear();
        self.input.clear();
        self.set_status("Ready (new chat)");
    }
    fn abort_and_clear_queue(&mut self) {
        let _ = self.source.cancel_tx().send(CancelCtl::AbortCurrent);
        self.aborting = true;
        self.pending.clear();
    }
    fn drain_sink(&mut self) -> bool {
        loop {
            match self.source.try_recv_from_sink() {
                Ok(ev) => {
                    if self.aborting {
                        match ev {
                            ChatEvent::Cancelled => self.on_cancel_ack(),
                            ChatEvent::Complete => { /* ignore while aborting */ }
                            ChatEvent::Response(_) => { /* swallow */ }
                        }
                        continue;
                    }
                    match ev {
                        ChatEvent::Response(resp) => match resp {
                            mistralrs::Response::InternalError(err) => {
                                self.on_error(err);
                            }
                            other => {
                                self.set_status(if self.think_mode {
                                    "Streaming... (think)"
                                } else {
                                    "Streaming..."
                                });
                                self.chat.state.append_response(other);
                            }
                        },
                        ChatEvent::Complete => {
                            self.finalize_turn_ok();
                        }
                        ChatEvent::Cancelled => {
                            self.on_cancel_ack();
                        }
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.set_status("Worker disconnected");
                    return true;
                }
            }
        }
        false
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
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
                            self.new_chat();
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
                            if self.busy {
                                self.cancel_current();
                                continue;
                            }
                            if self.chat.state.pop_last_completed_turn() {
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
                            }
                            KeyCode::Char(ch) => {
                                self.input.state.buffer.push(ch);
                            }
                            KeyCode::Tab => {
                                self.input.state.buffer.push('\t');
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
        self.input.state.think_mode = self.think_mode;
        self.legend.state.pending = self.pending.len();
        self.legend.state.input_empty = self.input.state.buffer.trim().is_empty();
        self.legend.state.can_undo = self.chat.state.has_completed_turn();
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
        self.input.render(frame, chunks[1]);
        self.legend.render(frame, chunks[2]);
        self.status.render(frame, chunks[3]);
    }
}
