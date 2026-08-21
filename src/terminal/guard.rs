use anyhow::Context;
use crossterm::{execute, terminal::disable_raw_mode, terminal::enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal, TerminalOptions, Viewport};
use std::io::{Stdout, stdout};

// Inline viewport: bottom N lines reserved (stream tail + input box).
// Everything above is the host terminal's native scrollback.
pub const VIEWPORT_HEIGHT: u16 = 9;

pub struct TerminalGuard {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode().context("enable raw mode failed")?;
        let mut stdout = stdout();
        // Bracketed paste: terminal emits one Event::Paste for multi-line pastes.
        execute!(stdout, crossterm::event::EnableBracketedPaste)
            .context("enable bracketed paste failed")?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(VIEWPORT_HEIGHT),
            },
        )
        .context("create inline terminal failed")?;
        Ok(Self { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(
            self.terminal.backend_mut(),
            crossterm::event::DisableBracketedPaste
        );
        let _ = disable_raw_mode();
    }
}
