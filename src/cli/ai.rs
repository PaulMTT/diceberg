use anyhow::{anyhow, Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use mistralrs::{
    ChatCompletionChunkResponse, ChatCompletionResponse, IsqType, McpClientConfig, McpServerConfig,
    McpServerSource, ModelDType, PagedAttentionMetaBuilder, RequestBuilder, ResponseOk,
    TextMessageRole, TextMessages, TextModelBuilder, Usage,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, prelude::*,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    DefaultTerminal,
    Frame,
};
use std::sync::atomic::Ordering;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, watch};
use tui_markdown::from_str as md_from_str;
use typed_builder::TypedBuilder;
trait Render {
    fn render(&mut self, frame: &mut Frame);
}
trait RenderArea {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
const BASE_SYSTEM_PROMPT: &str = "Answer using the shortest, most direct response possible. Take only the steps strictly necessary to find the answer. Do not overthink, speculate, or explore alternatives. Do not ask for permission. Perform required lookups immediately and provide the result without extra commentary.";
const THINK_OPEN: &str = "<think>";
const THINK_CLOSE: &str = "</think>";
pub async fn handle_ai() -> Result<()> {
    let (cmd_tx, evt_rx, cancel_tx, worker_task) = spawn_ai_worker().await?;
    let mut app = DiciAi::builder()
        .ai_bus(
            AiBus::builder()
                .cmd_tx(Some(cmd_tx))
                .evt_rx(Some(evt_rx))
                .cancel_tx(Some(cancel_tx))
                .build(),
        )
        .telemetry(Telemetry::default())
        .request(RequestState::default())
        .chat(ChatView::builder().state(ChatState::default()).build())
        .input(
            InputView::builder()
                .state(
                    InputState::builder()
                        .think_mode(ThinkMode::default())
                        .build(),
                )
                .build(),
        )
        .legend(LegendView::builder().state(LegendState::default()).build())
        .status(StatusView::builder().state(StatusState::default()).build())
        .worker(Some(worker_task))
        .build();
    app.new_chat();
    app.run()
}
#[derive(TypedBuilder)]
struct DiciAi {
    ai_bus: AiBus,
    telemetry: Telemetry,
    request: RequestState,
    chat: ChatView,
    input: InputView,
    legend: LegendView,
    status: StatusView,
    #[builder(default)]
    worker: Option<tokio::task::JoinHandle<()>>,
}
impl DiciAi {
    fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        let res = self.run_loop(&mut terminal);
        ratatui::restore();
        if let Some(j) = self.worker.take() {
            j.abort();
        }
        res
    }
    fn run_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            self.pump_ai();
            self.status
                .state
                .set_text(self.telemetry.status(self.request.busy));
            terminal.draw(|f| self.render(f))?;
            if event::poll(Duration::from_millis(33))? {
                if let Event::Key(key) = event::read()? {
                    if self.handle_key(key)? {
                        break Ok(());
                    }
                }
            }
        }
    }
    fn handle_key(
        &mut self,
        KeyEvent {
            code, modifiers, ..
        }: KeyEvent,
    ) -> Result<bool> {
        let ctrl = |ch| modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char(ch);
        if code == KeyCode::Esc {
            return Ok(true);
        }
        if ctrl('c') {
            self.handle_ctrl_c();
            return Ok(false);
        }
        if ctrl('n') {
            self.new_chat();
            return Ok(false);
        }
        if ctrl('t') {
            self.input.state.think_mode.toggle();
            self.status
                .state
                .set_text(match self.input.state.think_mode {
                    ThinkMode::Think => "Think mode: ON",
                    ThinkMode::NoThink => "Think mode: OFF",
                });
            return Ok(false);
        }
        match code {
            KeyCode::Enter => self.submit(),
            KeyCode::Backspace => {
                self.input.state.buffer.pop();
            }
            KeyCode::Char(ch) => self.input.state.buffer.push(ch),
            KeyCode::Tab => self.input.state.buffer.push('\t'),
            KeyCode::Up => self.chat.state.scroll.line_up(),
            KeyCode::Down => self.chat.state.scroll.line_down(),
            KeyCode::PageUp => self.chat.state.scroll.page_up(),
            KeyCode::PageDown => self.chat.state.scroll.page_down(),
            KeyCode::Home => self.chat.state.scroll.to_top(),
            KeyCode::End => self.chat.state.scroll.to_bottom(),
            _ => {}
        }
        Ok(false)
    }
    fn pump_ai(&mut self) {
        let mut events = Vec::new();
        if let Some(rx) = self.ai_bus.evt_rx.as_mut() {
            while let Ok(evt) = rx.try_recv() {
                events.push(evt);
            }
        }
        for evt in events {
            let mut next_to_start: Option<PendingReq> = None;
            match evt {
                AiEvt::Token(id, delta) if self.request.is_current(id) => {
                    self.telemetry.on_stream_text(&delta);
                    self.chat.state.partial.push_str(&delta);
                }
                AiEvt::Usage(id, u) if self.request.is_current(id) => {
                    self.telemetry.update_usage(&u);
                }
                AiEvt::Done(id, full) if self.request.is_current(id) => {
                    self.chat.state.history.push(Message::assistant(full));
                    self.chat.state.partial.clear();
                    self.request.busy = false;
                    self.chat.state.scroll.follow = true;
                    self.telemetry.finish_now();
                    next_to_start = self.request.pending.pop();
                }
                AiEvt::Error(id, e) if id == self.request.current_id => {
                    if e.to_string() != "canceled" {
                        self.request.busy = false;
                        self.chat.state.partial.clear();
                        self.telemetry.mark_stopped_now();
                        self.status.state.set_text(format!("Error: {e}"));
                        next_to_start = self.request.pending.pop();
                    }
                }
                _ => {}
            }
            if let Some(next) = next_to_start {
                self.start_request_with_prompt(next);
            }
        }
    }
    fn system_prompt(&self) -> String {
        BASE_SYSTEM_PROMPT.to_string()
    }
    fn new_chat(&mut self) {
        if self.request.busy {
            self.cancel();
        }
        self.request.pending.clear();
        self.chat.state.clear();
        self.chat.state.push_system(self.system_prompt());
        self.input.state.buffer.clear();
        self.telemetry.reset();
        self.status.state.set_text::<String>("Ready".into());
    }
    fn submit(&mut self) {
        let prompt = self.input.state.buffer.trim();
        if prompt.is_empty() {
            return;
        }
        let thinking = self.input.state.think_mode.as_bool();
        let prompt = prompt.to_string();
        self.input.state.buffer.clear();
        self.request.pending.push(PendingReq { prompt, thinking });
        if !self.request.busy {
            if let Some(next) = self.request.pending.pop() {
                self.start_request_with_prompt(next);
            }
        }
        self.chat.state.scroll.follow = true;
    }
    fn start_request_with_prompt(&mut self, req: PendingReq) {
        self.chat.state.push_user(req.prompt.clone());
        self.chat.state.partial.clear();
        self.telemetry.reset();
        self.telemetry.req_started_at = Some(Instant::now());
        self.telemetry.assume_thinking = req.thinking;
        let id = self.request.next_id;
        self.request.next_id = self.request.next_id.wrapping_add(1);
        self.request.current_id = id;
        let history_for_model = self.chat.state.as_model_messages();
        if let Some(tx) = &self.ai_bus.cmd_tx {
            let _ = tx.send(AiCmd::Ask {
                id,
                history: history_for_model,
                thinking: req.thinking,
            });
        }
        self.request.busy = true;
        self.chat.state.scroll.follow = true;
    }
    fn handle_ctrl_c(&mut self) {
        if self.request.busy {
            self.cancel();
            let _ = self.request.pending.pop();
        } else if !self.request.pending.is_empty() {
            let _ = self.request.pending.pop();
        } else if self.input.state.buffer.trim().is_empty() {
            if self.undo_last_exchange() {
                self.status.state.set_text("Undid last exchange");
            } else {
                self.status.state.set_text("Nothing to undo");
            }
        } else {
            self.input.state.buffer.clear();
        }
        self.chat.state.scroll.follow = true;
    }
    fn undo_last_exchange(&mut self) -> bool {
        let mut removed = false;
        if matches!(
            self.chat.state.history.last(),
            Some(m) if matches!(m.role, TextMessageRole::Assistant)
        ) {
            self.chat.state.history.pop();
            removed = true;
        }
        if matches!(
            self.chat.state.history.last(),
            Some(m) if matches!(m.role, TextMessageRole::User)
        ) {
            self.chat.state.history.pop();
            removed = true;
        }
        removed
    }
    fn cancel(&mut self) {
        self.request.cancel_counter = self.request.cancel_counter.wrapping_add(1);
        if let Some(ct) = &self.ai_bus.cancel_tx {
            let _ = ct.send(self.request.cancel_counter);
        }
        if matches!(
            self.chat.state.history.last(),
            Some(m) if m.role == TextMessageRole::User
        ) {
            self.chat.state.history.pop();
        }
        self.chat.state.partial.clear();
        self.request.busy = false;
        self.chat.state.scroll.follow = true;
        self.telemetry.mark_stopped_now();
    }
}
impl Render for DiciAi {
    fn render(&mut self, frame: &mut Frame) {
        self.legend.state.busy = self.request.busy;
        self.legend.state.think_mode = self.input.state.think_mode;
        self.legend.state.pending = self.request.pending.len();
        self.legend.state.input_empty = self.input.state.buffer.trim().is_empty();
        let h = &self.chat.state.history;
        self.legend.state.can_undo = h.len() >= 2
            && matches!(h[h.len() - 1].role, TextMessageRole::Assistant)
            && matches!(h[h.len() - 2].role, TextMessageRole::User);
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
#[derive(TypedBuilder, Default)]
struct ChatState {
    #[builder(default)]
    history: Messages,
    #[builder(default)]
    partial: String,
    #[builder(default)]
    scroll: ScrollModel,
    #[builder(default)]
    view_height: u16,
    #[builder(default)]
    content_height: u16,
}
impl ChatState {
    fn clear(&mut self) {
        self.history.clear();
        self.partial.clear();
        self.scroll = ScrollModel::default();
        self.view_height = 0;
        self.content_height = 0;
    }
    fn push_system(&mut self, s: String) {
        self.history.push(Message::system(s));
    }
    fn push_user(&mut self, s: String) {
        self.history.push(Message::user(s));
    }
    fn as_model_messages(&self) -> Vec<(TextMessageRole, String)> {
        self.history
            .iter()
            .map(|m| (m.role.clone(), m.content.clone()))
            .collect()
    }
}
#[derive(TypedBuilder)]
struct ChatView {
    state: ChatState,
}
impl RenderArea for ChatView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let partial_ref: Option<&str> = if self.state.partial.is_empty() {
            None
        } else {
            Some(&self.state.partial)
        };
        let lines = self.state.history.to_lines_with_partial(partial_ref);
        let block = Block::default().borders(Borders::ALL).title("Chat");
        let inner = block.inner(area);
        let measure_para = Paragraph::new(lines.clone()).wrap(Wrap { trim: false });
        self.state.view_height = inner.height;
        self.state.content_height =
            measure_para.line_count(inner.width).min(u16::MAX as usize) as u16;
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
#[derive(TypedBuilder, Default)]
struct InputState {
    #[builder(default)]
    buffer: String,
    #[builder(default = ThinkMode::NoThink)]
    think_mode: ThinkMode,
}
#[derive(TypedBuilder)]
struct InputView {
    state: InputState,
}
impl RenderArea for InputView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let title = match self.state.think_mode {
            ThinkMode::Think => "Your message (think)",
            ThinkMode::NoThink => "Your message (no-think)",
        };
        let input = Paragraph::new(self.state.buffer.as_str())
            .block(Block::default().borders(Borders::ALL).title(title));
        frame.render_widget(input, area);
    }
}
#[derive(TypedBuilder, Default)]
struct LegendState {
    #[builder(default)]
    busy: bool,
    #[builder(default = ThinkMode::NoThink)]
    think_mode: ThinkMode,
    #[builder(default)]
    pending: usize,
    #[builder(default)]
    input_empty: bool,
    #[builder(default)]
    can_undo: bool,
}
#[derive(TypedBuilder)]
struct LegendView {
    state: LegendState,
}
impl LegendView {
    fn line(&self) -> Line<'_> {
        let bold =
            |s: &str| Span::styled(s.to_string(), Style::default().add_modifier(Modifier::BOLD));
        let ctrlc_hint = if self.state.busy {
            "cancel + pop"
        } else if self.state.pending > 0 {
            "pop"
        } else if !self.state.input_empty {
            "clear input"
        } else if self.state.can_undo {
            "undo last exchange"
        } else {
            "clear input"
        };
        let toggle_hint = match self.state.think_mode {
            ThinkMode::NoThink => "thinking on",
            ThinkMode::Think => "thinking off",
        };
        let items = [
            ("Enter", "send/queue"),
            ("Ctrl+C", ctrlc_hint),
            ("Esc", "quit"),
            ("Ctrl+N", "new chat"),
            ("Ctrl+T", toggle_hint),
            ("↑/↓", "line"),
            ("PgUp/PgDn", "page"),
            ("Home/End", "top/bottom"),
        ];
        let mut spans: Vec<Span> = vec![bold("Keys: ")];
        for (i, (k, v)) in items.into_iter().enumerate() {
            spans.push(bold(k));
            spans.push(Span::raw(" "));
            spans.push(Span::raw(v));
            if i + 1 != items.len() {
                spans.push(Span::raw("  •  "));
            }
        }
        Line::from(spans)
    }
}
impl RenderArea for LegendView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.line()), area);
    }
}
#[derive(TypedBuilder, Default)]
struct StatusState {
    text: String,
}
impl StatusState {
    fn set_text<T: Into<String>>(&mut self, s: T) {
        self.text = s.into();
    }
}
#[derive(TypedBuilder)]
struct StatusView {
    state: StatusState,
}
impl RenderArea for StatusView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.state.text.as_str()), area);
    }
}
#[derive(Clone, TypedBuilder)]
struct Message {
    role: TextMessageRole,
    content: String,
}
impl Message {
    fn system(s: String) -> Self {
        Self {
            role: TextMessageRole::System,
            content: s,
        }
    }
    fn user(s: String) -> Self {
        Self {
            role: TextMessageRole::User,
            content: s,
        }
    }
    fn assistant(s: String) -> Self {
        Self {
            role: TextMessageRole::Assistant,
            content: s,
        }
    }
}
type Messages = Vec<Message>;
trait MessagesExt {
    fn to_lines_with_partial<'a>(&'a self, partial: Option<&'a str>) -> Vec<Line<'a>>;
}
impl MessagesExt for Vec<Message> {
    fn to_lines_with_partial<'a>(&'a self, partial: Option<&'a str>) -> Vec<Line<'a>> {
        let mut lines: Vec<Line<'a>> = Vec::new();
        for Message { role, content } in self.iter() {
            lines.extend(message_to_lines(role, content));
        }
        if let Some(p) = partial {
            lines.extend(message_to_lines(&TextMessageRole::Assistant, p));
        }
        lines
    }
}
impl RenderArea for Messages {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let lines = self.to_lines_with_partial(None);
        let chat = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(chat, area);
    }
}
#[derive(Copy, Clone, Default)]
struct ScrollModel {
    value: u16,
    follow: bool,
    view_height: u16,
    content_height: u16,
}
impl ScrollModel {
    fn reconcile(&mut self, content_height: u16, view_height: u16) {
        self.content_height = content_height;
        self.view_height = view_height;
        let max = self.max();
        self.value = if self.follow {
            max
        } else {
            self.value.min(max)
        };
    }
    fn max(&self) -> u16 {
        self.content_height.saturating_sub(self.view_height)
    }
    fn to_top(&mut self) {
        self.value = 0;
        self.follow = false;
    }
    fn to_bottom(&mut self) {
        self.value = self.max();
        self.follow = true;
    }
    fn line_up(&mut self) {
        self.value = self.value.saturating_sub(1);
        let max = self.max();
        self.follow = self.value == max && max == 0;
    }
    fn line_down(&mut self) {
        let max = self.max();
        if self.value < max {
            self.value += 1;
            self.follow = false;
        } else {
            self.follow = true;
        }
    }
    fn page_up(&mut self) {
        let step = self.view_height.max(1);
        self.value = self.value.saturating_sub(step);
        self.follow = false;
    }
    fn page_down(&mut self) {
        let step = self.view_height.max(1);
        let max = self.max();
        self.value = self.value.saturating_add(step).min(max);
        self.follow = self.value == max;
    }
}
#[derive(TypedBuilder, Default)]
struct AiBus {
    #[builder(default)]
    cmd_tx: Option<mpsc::UnboundedSender<AiCmd>>,
    #[builder(default)]
    evt_rx: Option<mpsc::UnboundedReceiver<AiEvt>>,
    #[builder(default)]
    cancel_tx: Option<watch::Sender<u64>>,
}
#[derive(Default)]
struct RequestState {
    busy: bool,
    current_id: u64,
    next_id: u64,
    cancel_counter: u64,
    pending: Vec<PendingReq>,
}
impl RequestState {
    #[inline]
    fn is_current(&self, id: u64) -> bool {
        self.busy && id == self.current_id
    }
}
#[derive(Clone)]
struct PendingReq {
    prompt: String,
    thinking: bool,
}
#[derive(Default)]
struct Telemetry {
    req_started_at: Option<Instant>,
    last_completed_at: Option<Instant>,
    inside_think: bool,
    saw_phase_tag: bool,
    assume_thinking: bool,
    have_any_text: bool,
    acc_completion_tokens: usize,
    acc_time_sec: f32,
    last_usage_snapshot: Option<(usize, f32)>,
    final_completion_tokens: usize,
    final_completion_time_sec: f32,
    have_any_usage: bool,
    finalized: bool,
}
impl Telemetry {
    fn reset(&mut self) {
        *self = Self::default();
    }
    fn phase_now_for_display(&self) -> bool {
        if self.saw_phase_tag {
            self.inside_think
        } else {
            self.assume_thinking
        }
    }
    fn on_stream_text(&mut self, delta: &str) {
        if delta.is_empty() {
            return;
        }
        self.have_any_text = true;
        let mut s = delta;
        let mut saw_tag_here = false;
        loop {
            let open = THINK_OPEN;
            let close = THINK_CLOSE;
            let o = s.find(open);
            let c = s.find(close);
            match (o, c) {
                (Some(i1), Some(i2)) => {
                    saw_tag_here = true;
                    if i1 < i2 {
                        self.inside_think = true;
                        s = &s[i1 + open.len()..];
                    } else {
                        self.inside_think = false;
                        s = &s[i2 + close.len()..];
                    }
                }
                (Some(i1), None) => {
                    saw_tag_here = true;
                    self.inside_think = true;
                    s = &s[i1 + open.len()..];
                }
                (None, Some(i2)) => {
                    saw_tag_here = true;
                    self.inside_think = false;
                    s = &s[i2 + close.len()..];
                }
                (None, None) => break,
            }
        }
        if saw_tag_here {
            self.saw_phase_tag = true;
        }
    }
    fn update_usage(&mut self, u: &Usage) {
        let ct = u.completion_tokens;
        let t = u.total_completion_time_sec.max(0.0);
        match self.last_usage_snapshot {
            Some((prev_ct, prev_t)) if ct >= prev_ct && t >= prev_t => {
                self.acc_completion_tokens = self
                    .acc_completion_tokens
                    .saturating_add(ct.saturating_sub(prev_ct));
                self.acc_time_sec += (t - prev_t).max(0.0);
            }
            _ => {
                self.acc_completion_tokens = self.acc_completion_tokens.saturating_add(ct);
                self.acc_time_sec += t;
            }
        }
        self.last_usage_snapshot = Some((ct, t));
        self.final_completion_tokens = self.acc_completion_tokens;
        self.final_completion_time_sec = self.acc_time_sec.max(0.0);
        self.have_any_usage = true;
    }
    fn mark_stopped_now(&mut self) {
        self.last_completed_at = Some(Instant::now());
        self.finalized = false;
        self.have_any_usage = false;
        self.acc_completion_tokens = 0;
        self.acc_time_sec = 0.0;
        self.last_usage_snapshot = None;
        self.final_completion_tokens = 0;
        self.final_completion_time_sec = 0.0;
    }
    fn e2e_elapsed_sec(&self) -> Option<f32> {
        match (self.req_started_at, self.last_completed_at) {
            (Some(s), Some(e)) => Some((e - s).as_secs_f32()),
            _ => None,
        }
    }
    fn finish_now(&mut self) {
        self.last_completed_at = Some(Instant::now());
        if let Some(e2e) = self.e2e_elapsed_sec() {
            self.final_completion_time_sec = e2e.max(self.final_completion_time_sec);
        }
        self.finalized = self.have_any_usage;
    }
    fn status(&self, busy: bool) -> String {
        if busy {
            return if self.phase_now_for_display() {
                "Thinking...".into()
            } else {
                "Answering...".into()
            };
        }
        if self.finalized {
            let tok = self.final_completion_tokens;
            let sec = self
                .e2e_elapsed_sec()
                .unwrap_or(self.final_completion_time_sec)
                .max(0.0);
            let r = if sec > 0.0 { tok as f32 / sec } else { 0.0 };
            return format!(
                "Ready • Total {} tok • {} • {:.1} tok/s",
                tok,
                fmt_secs_short(sec),
                r
            );
        }
        "Ready".into()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
enum ThinkMode {
    #[default]
    Think,
    NoThink,
}
impl ThinkMode {
    #[inline]
    fn as_bool(self) -> bool {
        matches!(self, ThinkMode::Think)
    }
    fn toggle(&mut self) {
        *self = match *self {
            ThinkMode::Think => ThinkMode::NoThink,
            ThinkMode::NoThink => ThinkMode::Think,
        };
    }
}
enum AiCmd {
    Ask {
        id: u64,
        history: Vec<(TextMessageRole, String)>,
        thinking: bool,
    },
}
enum AiEvt {
    Token(u64, String),
    Usage(u64, Usage),
    Done(u64, String),
    Error(u64, anyhow::Error),
}
async fn spawn_ai_worker() -> Result<(
    mpsc::UnboundedSender<AiCmd>,
    mpsc::UnboundedReceiver<AiEvt>,
    watch::Sender<u64>,
    tokio::task::JoinHandle<()>,
)> {
    let mcp = McpClientConfig {
        servers: vec![McpServerConfig {
            name: "The data and insights cloud integration (DICI) model context protocol (MCP) server.".into(),
            source: McpServerSource::Process {
                command: "dici".into(),
                args: vec!["serve".into(), "mcp".into()],
                work_dir: None,
                env: None,
            },
            tool_prefix: None,
            ..Default::default()
        }],
        auto_register_tools: true,
        tool_timeout_secs: Some(30),
        max_concurrent_calls: Some(4),
        ..Default::default()
    };
    let model = TextModelBuilder::new("Qwen/Qwen3-4B")
        .with_dtype(ModelDType::Auto)
        .with_isq(IsqType::AFQ4)
        .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?
        .with_mcp_client(mcp)
        .build()
        .await
        .context("failed to build model")?;
    let model = Arc::new(model);
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<AiCmd>();
    let (evt_tx, evt_rx) = mpsc::unbounded_channel::<AiEvt>();
    let (cancel_tx, cancel_rx) = watch::channel::<u64>(0);
    let worker_model = Arc::clone(&model);
    let task = tokio::spawn(async move {
        while let Some(AiCmd::Ask {
            id,
            history,
            thinking,
        }) = cmd_rx.recv().await
        {
            let mut msgs = TextMessages::new();
            for (role, text) in history {
                msgs = msgs.add_message(role, text);
            }
            let req = RequestBuilder::from(msgs)
                .enable_thinking(thinking)
                .set_sampler_temperature(0.25)
                .set_sampler_topp(0.95)
                .set_sampler_frequency_penalty(0.20)
                .set_sampler_presence_penalty(0.0);
            let mut stream = match worker_model.stream_chat_request(req).await {
                Ok(s) => s,
                Err(e) => {
                    let _ = evt_tx.send(AiEvt::Error(id, anyhow!(e)));
                    continue;
                }
            };
            let start_cancel = *cancel_rx.borrow();
            let mut my_cancel = cancel_rx.clone();
            let mut full = String::new();
            loop {
                tokio::select! {
                    _ = my_cancel.changed() => {
                        if *my_cancel.borrow() != start_cancel {
                            mistralrs::get_engine_terminate_flag().store(true, Ordering::SeqCst);
                            mistralrs::TERMINATE_ALL_NEXT_STEP.store(true, Ordering::SeqCst);
                            while let Some(_ignored) = stream.next().await {}
                            mistralrs::reset_engine_terminate_flag();
                            mistralrs::TERMINATE_ALL_NEXT_STEP.store(false, Ordering::SeqCst);
                            let _ = evt_tx.send(AiEvt::Error(id, anyhow!("canceled")));
                            break;
                        }
                    }
                    maybe = stream.next() => {
                        match maybe {
                            Some(resp) => match resp.as_result() {
                                Ok(ResponseOk::Chunk(chunk)) => {
                                    if let Some(delta) = chunk_delta_text(&chunk) {
                                        full.push_str(&delta);
                                        let _ = evt_tx.send(AiEvt::Token(id, delta));
                                    }
                                    if let Some(u) = &chunk.usage {
                                        let _ = evt_tx.send(AiEvt::Usage(id, u.clone()));
                                    }
                                }
                                Ok(ResponseOk::Done(done)) => {
                                    let _ = evt_tx.send(AiEvt::Usage(id, done.usage.clone()));
                                    let text = extract_final_text(&done).unwrap_or(full);
                                    let _ = evt_tx.send(AiEvt::Done(id, text));
                                    break;
                                }
                                Ok(_) => {
                                    if !full.is_empty() {
                                        let _ = evt_tx.send(AiEvt::Done(id, full));
                                    } else {
                                        let _ = evt_tx.send(AiEvt::Error(id, anyhow!("unexpected stream message")));
                                    }
                                    break;
                                }
                                Err(e) => {
                                    let _ = evt_tx.send(AiEvt::Error(id, anyhow::Error::new(e)));
                                    break;
                                }
                            },
                            None => {
                                if !full.is_empty() {
                                    let _ = evt_tx.send(AiEvt::Done(id, full));
                                }
                                break;
                            },
                        }
                    }
                }
            }
        }
    });
    Ok((cmd_tx, evt_rx, cancel_tx, task))
}
fn chunk_delta_text(chunk: &ChatCompletionChunkResponse) -> Option<String> {
    let mut out = String::new();
    for ch in &chunk.choices {
        if let Some(s) = ch.delta.content.as_deref() {
            out.push_str(s);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}
fn extract_final_text(done: &ChatCompletionResponse) -> Option<String> {
    done.choices
        .get(0)
        .and_then(|c| c.message.content.as_deref())
        .map(|s| s.to_string())
}
fn think_style() -> Style {
    Style::default()
        .fg(Color::LightBlue)
        .add_modifier(Modifier::DIM)
}
fn patch_lines_style(lines: &mut [Line<'_>], style: Style) {
    for line in lines.iter_mut() {
        for span in line.spans.iter_mut() {
            span.style = span.style.patch(style);
        }
    }
}
fn append_segment_lines<'a>(
    out: &mut Vec<Line<'a>>,
    mut seg_lines: Vec<Line<'a>>,
    join_with_prev: bool,
) {
    if out.is_empty() {
        out.append(&mut seg_lines);
        return;
    }
    if join_with_prev && !seg_lines.is_empty() {
        let dst = out.last_mut().unwrap();
        let mut first = seg_lines.remove(0);
        if dst.width() > 0 && !dst.spans.is_empty() && !first.spans.is_empty() {
            dst.spans.push(Span::raw(" "));
        }
        dst.spans.append(&mut first.spans);
    }
    out.append(&mut seg_lines);
}
fn markdown_text_with_think<'a>(src: &'a str) -> Text<'a> {
    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut inside = false;
    let mut rem = src;
    let mut prev_ended_with_nl = true;
    loop {
        let tag = if inside { THINK_CLOSE } else { THINK_OPEN };
        if let Some(i) = rem.find(tag) {
            let before = &rem[..i];
            if !before.is_empty() {
                let mut t: Text<'a> = md_from_str(before);
                if inside {
                    patch_lines_style(&mut t.lines, think_style());
                }
                let join_with_prev = !prev_ended_with_nl && !before.starts_with('\n');
                append_segment_lines(&mut lines, t.lines, join_with_prev);
                prev_ended_with_nl = before.ends_with('\n');
            }
            rem = &rem[i + tag.len()..];
            inside = !inside;
        } else {
            let before = rem;
            if !before.is_empty() {
                let mut t: Text<'a> = md_from_str(before);
                if inside {
                    patch_lines_style(&mut t.lines, think_style());
                }
                let join_with_prev = !prev_ended_with_nl && !before.starts_with('\n');
                append_segment_lines(&mut lines, t.lines, join_with_prev);
            }
            break;
        }
    }
    Text::from(lines)
}
fn prepend_who<'a>(mut t: Text<'a>, who: &str) -> Text<'a> {
    if let Some(idx) = t.lines.iter().position(|ln| ln.width() > 0) {
        let line = &mut t.lines[idx];
        let mut spans = Vec::with_capacity(line.spans.len() + 1);
        spans.push(Span::raw(format!("{who} ")));
        spans.extend(line.spans.drain(..));
        *line = Line::from(spans);
    } else {
        t = Text::from(vec![Line::from(vec![Span::raw(format!("{who} "))])]);
    }
    t
}
fn message_to_lines<'a>(role: &TextMessageRole, text: &'a str) -> Vec<Line<'a>> {
    let who = match role {
        TextMessageRole::System => "[system]",
        TextMessageRole::User => "[you]",
        TextMessageRole::Assistant => "[ai]",
        _ => "[?]",
    };
    let t = prepend_who(markdown_text_with_think(text), who);
    t.lines
}
fn fmt_dur_short(d: Duration) -> String {
    let s = d.as_secs();
    if s < 60 {
        format!("{:.1}s", d.as_secs_f64())
    } else if s < 3600 {
        format!("{}m{}s", s / 60, s % 60)
    } else {
        let h = s / 3600;
        let m = (s % 3600) / 60;
        format!("{}h{}m", h, m)
    }
}
fn fmt_secs_short(secs: f32) -> String {
    fmt_dur_short(Duration::from_secs_f32(secs.max(0.0)))
}
