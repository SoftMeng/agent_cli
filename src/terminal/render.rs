use crate::terminal::scrollback::wrap_line;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use std::io::Stdout;

type Term = Terminal<CrosstermBackend<Stdout>>;

/// Render the bottom inline viewport: streaming tail on top, input box below.
pub fn draw_viewport(
    terminal: &mut Term,
    stream_tail: &str,
    input: &str,
    cursor: usize,
    blink_on: bool,
) -> std::io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(5)])
            .split(f.area());

        // ============ Stream tail ============
        let tail_style = Style::default().fg(Color::Cyan);
        let tail_lines: Vec<Line> = stream_tail
            .lines()
            .map(|l| Line::from(Span::styled(l.to_string(), tail_style)))
            .collect();
        let tail = Paragraph::new(tail_lines).wrap(Wrap { trim: false });
        f.render_widget(tail, chunks[0]);

        // ============ Input box ============
        let width = chunks[1].width as usize;
        let usable = width.saturating_sub(4).max(8);
        let cursor_prefix = "▸ ";
        let cursor_bg = if blink_on {
            Color::Cyan
        } else {
            Color::DarkGray
        };
        let cursor_fg = if blink_on { Color::Black } else { Color::Reset };
        let cursor_style = Style::default()
            .fg(cursor_fg)
            .bg(cursor_bg)
            .add_modifier(Modifier::BOLD | Modifier::REVERSED);

        let before = input
            .char_indices()
            .nth(cursor)
            .map(|(i, _)| &input[..i])
            .unwrap_or(input);
        let after = input
            .char_indices()
            .nth(cursor)
            .map(|(i, _)| &input[i..])
            .unwrap_or("");
        let cursor_char = after.chars().next().unwrap_or(' ');
        let cursor_rest = if cursor_char == ' ' {
            ""
        } else {
            &after[cursor_char.len_utf8()..]
        };

        let mut input_lines: Vec<Line> = Vec::new();
        input_lines.push(Line::from(vec![
            Span::styled(
                cursor_prefix,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(before),
            Span::styled(cursor_char.to_string(), cursor_style),
            Span::raw(cursor_rest),
        ]));

        let wrap_width = usable.saturating_sub(cursor_prefix.chars().count()).max(4);
        let full_after = format!("{before}{after}");
        if full_after.chars().count() > wrap_width {
            let overflow_start = full_after
                .char_indices()
                .nth(wrap_width)
                .map(|(i, _)| i)
                .unwrap_or(full_after.len());
            let overflow = &full_after[overflow_start..];
            if !overflow.is_empty() {
                for chunk in wrap_line(overflow, usable) {
                    input_lines.push(Line::from(Span::raw(format!("  {chunk}"))));
                }
            }
        }

        let input_box = Paragraph::new(input_lines)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green))
                    .title("输入"),
            );
        f.render_widget(input_box, chunks[1]);
    })?;
    Ok(())
}
