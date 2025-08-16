use mistralrs::TextMessageRole;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::text::Text;
use tui_markdown::from_str as md_from_str;

#[derive(Clone)]
pub struct Message {
    pub role: TextMessageRole,
    pub content: String,
}
impl Message {
    pub fn system(s: String) -> Self {
        Self {
            role: TextMessageRole::System,
            content: s,
        }
    }
    pub fn user(s: String) -> Self {
        Self {
            role: TextMessageRole::User,
            content: s,
        }
    }
    pub fn assistant(s: String) -> Self {
        Self {
            role: TextMessageRole::Assistant,
            content: s,
        }
    }
}
pub type Messages = Vec<Message>;

const THINK_OPEN: &str = "<think>";
const THINK_CLOSE: &str = "</think>";

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

        if !dst.spans.is_empty() && !first.spans.is_empty() {
            dst.spans.push(Span::raw(" "));
        }

        dst.spans.append(&mut first.spans);
    }
    out.append(&mut seg_lines);
}

fn starts_markdown_block(s: &str) -> bool {
    let s = s.trim_start_matches(|c| c == '\r' || c == ' ' || c == '\t');
    if s.starts_with("```") || s.starts_with("~~~") {
        return true;
    }
    if s.starts_with('#') || s.starts_with("> ") || s.starts_with("- ") || s.starts_with("* ") {
        return true;
    }

    let mut it = s.chars().peekable();
    let mut saw_digit = false;
    while let Some(c) = it.peek().copied() {
        if c.is_ascii_digit() {
            saw_digit = true;
            it.next();
        } else {
            break;
        }
    }
    if saw_digit {
        if let (Some('.'), Some(' ')) = (it.next(), it.peek().copied()) {
            return true;
        }
    }
    false
}

fn strip_leading_newlines<'a>(mut s: &'a str) -> &'a str {
    loop {
        if let Some(rest) = s.strip_prefix("\r\n") {
            s = rest;
            continue;
        }
        if let Some(rest) = s.strip_prefix('\n') {
            s = rest;
            continue;
        }
        if let Some(rest) = s.strip_prefix('\r') {
            s = rest;
            continue;
        }
        break;
    }
    s
}

fn markdown_text_with_think<'a>(src: &'a str) -> Text<'a> {
    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut inside = false;
    let mut rem = src;
    let mut prev_ended_with_nl = true;
    let mut force_join_next = false;

    loop {
        let tag = if inside { THINK_CLOSE } else { THINK_OPEN };
        if let Some(i) = rem.find(tag) {
            let before = &rem[..i];
            if !before.is_empty() {
                let mut t: Text<'a> = md_from_str(before);
                if inside {
                    patch_lines_style(&mut t.lines, think_style());
                }
                let join_with_prev = ((!prev_ended_with_nl) || force_join_next)
                    && !before.starts_with('\n')
                    && !starts_markdown_block(before);
                append_segment_lines(&mut lines, t.lines, join_with_prev);
                prev_ended_with_nl = before.ends_with('\n');
                force_join_next = false;
            }

            rem = &rem[i + tag.len()..];

            if inside {
                let stripped = strip_leading_newlines(rem);
                if stripped.len() != rem.len() {
                    force_join_next = true;
                    prev_ended_with_nl = false;
                    rem = stripped;
                }
            }

            inside = !inside;
        } else {
            let before = rem;
            if !before.is_empty() {
                let mut t: Text<'a> = md_from_str(before);
                if inside {
                    patch_lines_style(&mut t.lines, think_style());
                }
                let join_with_prev = ((!prev_ended_with_nl) || force_join_next)
                    && !before.starts_with('\n')
                    && !starts_markdown_block(before);
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

pub fn message_to_lines<'a>(role: &TextMessageRole, text: &'a str) -> Vec<Line<'a>> {
    let who = match role {
        TextMessageRole::System => "[system]",
        TextMessageRole::User => "[you]",
        TextMessageRole::Assistant => "[ai]",
        _ => "[?]",
    };
    let t = prepend_who(markdown_text_with_think(text), who);
    t.lines
}
