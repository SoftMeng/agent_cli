use crate::terminal::cards::{CardType, ChatCard};
use crate::terminal::sanitize::sanitize_for_tui;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use std::io::Stdout;
use unicode_width::UnicodeWidthChar;

type Term = Terminal<CrosstermBackend<Stdout>>;

// ============ Card -> styled lines ============

fn card_style(t: &CardType) -> (&'static str, Color) {
    match t {
        CardType::System => ("⚙ ", Color::DarkGray),
        CardType::User => ("▶ ", Color::Green),
        CardType::Assistant => ("✓ ", Color::Cyan),
        CardType::Thinking => ("💭 ", Color::Yellow),
        CardType::Tool => ("🔧 ", Color::Magenta),
        CardType::Error => ("✗ ", Color::Red),
    }
}

pub fn render_card_lines(card: &ChatCard, width: usize) -> Vec<Line<'static>> {
    let usable = width.saturating_sub(2).max(8);
    let prefix = "  ";
    let (icon, fg) = card_style(&card.card_type);
    let style = Style::default().fg(fg);
    let header = style.add_modifier(Modifier::BOLD);
    let mut out: Vec<Line> = vec![Line::from(vec![
        Span::styled(icon, header),
        Span::styled(card.title.clone(), header),
    ])];

    let body = if card.card_type == CardType::Thinking && !card.expanded {
        String::from("💭 thinking collapsed")
    } else {
        sanitize_for_tui(&card.content)
    };
    for ln in body.lines() {
        for chunk in wrap_line(ln, usable) {
            out.push(Line::from(Span::styled(format!("{prefix}{chunk}"), style)));
        }
    }
    if card.expanded
        && let Some(resp) = &card.tool_response
    {
        for chunk in wrap_line("── result ──", usable) {
            out.push(Line::from(Span::styled(
                format!("{prefix}{chunk}"),
                style.add_modifier(Modifier::ITALIC),
            )));
        }
        for ln in sanitize_for_tui(resp).lines() {
            for chunk in wrap_line(ln, usable) {
                out.push(Line::from(Span::styled(format!("{prefix}{chunk}"), style)));
            }
        }
    }
    out.push(Line::from(""));
    out
}

/// Soft-wrap a line to `width` columns at character boundaries.
pub fn wrap_line(s: &str, width: usize) -> Vec<&str> {
    if width == 0 {
        return vec![s];
    }
    let mut out: Vec<&str> = Vec::new();
    let mut start = 0;
    let mut count = 0usize;
    for (i, c) in s.char_indices() {
        count += 1;
        if count >= width {
            out.push(&s[start..i + c.len_utf8()]);
            start = i + c.len_utf8();
            count = 0;
        }
    }
    if start < s.len() {
        out.push(&s[start..]);
    }
    if out.is_empty() {
        out.push(s);
    }
    out
}

fn clear_wide_placeholders(buf: &mut Buffer) {
    let w = buf.area.width as usize;
    let h = buf.area.height as usize;
    for y in 0..h {
        let mut to_clear: Vec<u16> = Vec::new();
        let mut x = 0usize;
        while x < w {
            let sym = buf[(x as u16, y as u16)].symbol();
            let cw = sym.chars().next().and_then(|c| c.width()).unwrap_or(0);
            if cw == 2 && x + 1 < w {
                to_clear.push((x + 1) as u16);
                x += 2;
            } else {
                x += 1;
            }
        }
        for cx in to_clear {
            buf[(cx, y as u16)].set_symbol("");
        }
    }
}

// ============ Commit into scrollback ============

pub fn commit_lines(terminal: &mut Term, lines: Vec<Line>) -> std::io::Result<()> {
    if lines.is_empty() {
        return Ok(());
    }
    let height = lines.len() as u16;
    terminal.insert_before(height, move |buf: &mut Buffer| {
        Paragraph::new(lines).render(buf.area, buf);
        clear_wide_placeholders(buf);
    })?;
    Ok(())
}

pub fn commit_card(terminal: &mut Term, card: &ChatCard) -> std::io::Result<()> {
    let width = terminal.size()?.width as usize;
    let lines = render_card_lines(card, width);
    commit_lines(terminal, lines)
}
