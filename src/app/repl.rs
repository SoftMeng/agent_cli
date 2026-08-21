use crate::agent::AgentKind;
use crate::app::input::InputBuffer;
use crate::app::stream::consume_stream;
use crate::terminal::scrollback::commit_card;
use crate::terminal::{CardType, ChatCard, TerminalGuard, draw_viewport, sanitize_for_tui};
use anyhow::Context;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use rig_core::prelude::StreamingPrompt;
use std::time::{Duration, Instant};

const MAX_TURNS: usize = 8;
const BLINK_INTERVAL: Duration = Duration::from_millis(530);
const POLL_INTERVAL: Duration = Duration::from_millis(120);

// ============ REPL ============

pub async fn run_repl(agent: AgentKind, conversation_id: String) -> anyhow::Result<()> {
    let mut guard = TerminalGuard::new().context("failed to initialize terminal")?;
    let mut conversation_id = conversation_id;
    let mut input = InputBuffer::new();
    let mut blink_on = true;
    let mut last_blink = Instant::now();

    commit_card(
        &mut guard.terminal,
        &ChatCard::new(
            CardType::System,
            "agent_cli",
            "Ready. /id <name> switch · /exit or Esc quit",
        ),
    )?;
    draw_viewport(
        &mut guard.terminal,
        "",
        &input.text,
        input.cursor_chars(),
        blink_on,
    )?;

    loop {
        let ev_ready = crossterm::event::poll(POLL_INTERVAL).context("failed to poll event")?;
        if !ev_ready {
            tick_blink(&mut last_blink, &mut blink_on);
            draw_viewport(
                &mut guard.terminal,
                "",
                &input.text,
                input.cursor_chars(),
                blink_on,
            )?;
            continue;
        }
        let ev = crossterm::event::read().context("failed to read event")?;
        match ev {
            Event::Paste(text) => {
                for c in text.chars() {
                    if c == '\r' {
                        continue;
                    }
                    input.insert_char(c);
                }
                draw_viewport(
                    &mut guard.terminal,
                    "",
                    &input.text,
                    input.cursor_chars(),
                    blink_on,
                )?;
            }
            Event::Key(key) => {
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                if ctrl && key.code == KeyCode::Char('c') {
                    break;
                }

                tick_blink(&mut last_blink, &mut blink_on);

                let mut consumed = true;
                match key.code {
                    KeyCode::Char(c) => {
                        if ctrl && (c == 'a' || c == 'e') {
                            if c == 'a' {
                                input.move_home();
                            } else {
                                input.move_end();
                            }
                        } else {
                            input.insert_char(c);
                        }
                    }
                    KeyCode::Backspace => input.backspace(),
                    KeyCode::Delete => input.delete_forward(),
                    KeyCode::Left => input.move_left(),
                    KeyCode::Right => input.move_right(),
                    KeyCode::Home => input.move_home(),
                    KeyCode::End => input.move_end(),
                    KeyCode::Up => input.recall_history(-1),
                    KeyCode::Down => input.recall_history(1),
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        let prompt = input.commit();
                        let prompt = prompt.trim().to_string();
                        if prompt.is_empty() {
                            consumed = false;
                        } else if matches!(prompt.as_str(), "/exit" | "/quit" | ":q") {
                            break;
                        } else if let Some(rest) = prompt.strip_prefix("/id ") {
                            let new_id = rest.trim().to_string();
                            if new_id.is_empty() {
                                commit_card(
                                    &mut guard.terminal,
                                    &ChatCard::new(CardType::System, "Usage", "/id <name>"),
                                )?;
                            } else {
                                conversation_id = new_id.clone();
                                commit_card(
                                    &mut guard.terminal,
                                    &ChatCard::new(
                                        CardType::System,
                                        "conversation",
                                        format!("switched to '{new_id}'"),
                                    ),
                                )?;
                            }
                        } else {
                            commit_card(
                                &mut guard.terminal,
                                &ChatCard::new(CardType::User, "You", sanitize_for_tui(&prompt)),
                            )?;
                            draw_viewport(
                                &mut guard.terminal,
                                "",
                                &input.text,
                                input.cursor_chars(),
                                blink_on,
                            )?;
                            let prompt_for_agent = prompt.clone();
                            match &agent {
                                AgentKind::OpenAI(a) => {
                                    let req = a
                                        .stream_prompt(prompt_for_agent.as_str())
                                        .max_turns(MAX_TURNS)
                                        .conversation(&conversation_id);
                                    let stream = req.await;
                                    consume_stream(&mut guard, stream).await?;
                                }
                                AgentKind::Anthropic(a) => {
                                    let req = a
                                        .stream_prompt(prompt_for_agent.as_str())
                                        .max_turns(MAX_TURNS)
                                        .conversation(&conversation_id);
                                    let stream = req.await;
                                    consume_stream(&mut guard, stream).await?;
                                }
                            }
                        }
                    }
                    _ => consumed = false,
                }
                let _ = consumed;
                draw_viewport(
                    &mut guard.terminal,
                    "",
                    &input.text,
                    input.cursor_chars(),
                    blink_on,
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn tick_blink(last_blink: &mut Instant, blink_on: &mut bool) {
    let now = Instant::now();
    if now.duration_since(*last_blink) >= BLINK_INTERVAL {
        *blink_on = !*blink_on;
        *last_blink = now;
    }
}
