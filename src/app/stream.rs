use crate::terminal::scrollback::commit_card;
use crate::terminal::{CardType, ChatCard, TerminalGuard, draw_viewport};
use futures::StreamExt;
use rig_core::OneOrMany;
use rig_core::agent::MultiTurnStreamItem;
use rig_core::agent::StreamingError;
use rig_core::message::ToolResultContent;
use rig_core::streaming::{StreamedAssistantContent, StreamedUserContent};
use std::collections::HashMap;

const STREAM_TAIL_LINES: usize = 4;

// ============ Stream accumulator ============

struct StreamAccum {
    assistant: String,
    thinking: String,
    pending_tools: HashMap<String, String>,
}

impl StreamAccum {
    fn new() -> Self {
        Self {
            assistant: String::new(),
            thinking: String::new(),
            pending_tools: HashMap::new(),
        }
    }

    // Flush accumulated assistant/thinking text into scrollback as cards.
    fn flush(&mut self, guard: &mut TerminalGuard) -> std::io::Result<()> {
        if !self.assistant.trim().is_empty() {
            let card = ChatCard::new(
                CardType::Assistant,
                "Assistant",
                std::mem::take(&mut self.assistant),
            );
            commit_card(&mut guard.terminal, &card)?;
        } else {
            self.assistant.clear();
        }
        if !self.thinking.trim().is_empty() {
            let mut card = ChatCard::new(
                CardType::Thinking,
                "Thinking",
                std::mem::take(&mut self.thinking),
            );
            card.expanded = false;
            commit_card(&mut guard.terminal, &card)?;
        } else {
            self.thinking.clear();
        }
        Ok(())
    }
}

// ============ Stream consumption ============

type DynStream<'a, R> = std::pin::Pin<
    Box<dyn futures::Stream<Item = Result<MultiTurnStreamItem<R>, StreamingError>> + Send + 'a>,
>;

pub async fn consume_stream<R>(
    guard: &mut TerminalGuard,
    mut stream: DynStream<'_, R>,
) -> anyhow::Result<()> {
    let mut accum = StreamAccum::new();
    while let Some(item) = stream.next().await {
        match item {
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(text))) => {
                accum.assistant.push_str(&text.text);
                let tail = tail_lines(&accum.assistant, STREAM_TAIL_LINES);
                draw_viewport(&mut guard.terminal, &tail, "", 0, true)?;
            }
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Reasoning(
                _,
            ))) => {}
            Ok(MultiTurnStreamItem::StreamAssistantItem(
                StreamedAssistantContent::ReasoningDelta { reasoning, .. },
            )) => {
                accum.thinking.push_str(&reasoning);
            }
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::ToolCall {
                tool_call,
                internal_call_id,
            })) => {
                accum.flush(guard)?;
                let args = serde_json::to_string_pretty(&tool_call.function.arguments)
                    .unwrap_or_else(|_| tool_call.function.arguments.to_string());
                accum
                    .pending_tools
                    .insert(internal_call_id.clone(), tool_call.function.name.clone());
                let card = ChatCard::new_tool(tool_call.function.name, internal_call_id, args);
                commit_card(&mut guard.terminal, &card)?;
            }
            Ok(MultiTurnStreamItem::StreamUserItem(StreamedUserContent::ToolResult {
                tool_result,
                internal_call_id,
            })) => {
                accum.flush(guard)?;
                let rendered = render_tool_result(&tool_result.content);
                let title = accum
                    .pending_tools
                    .remove(&internal_call_id)
                    .unwrap_or_else(|| "(orphan tool)".to_string());
                let mut card = ChatCard::new(CardType::Tool, title, String::new());
                card.tool_response = Some(rendered);
                card.expanded = true;
                commit_card(&mut guard.terminal, &card)?;
            }
            Ok(MultiTurnStreamItem::FinalResponse(_)) => {
                accum.flush(guard)?;
                commit_card(
                    &mut guard.terminal,
                    &ChatCard::new(CardType::System, "done", String::new()),
                )?;
                break;
            }
            Ok(MultiTurnStreamItem::CompletionCall(_)) => {}
            Err(err) => {
                accum.flush(guard)?;
                commit_card(
                    &mut guard.terminal,
                    &ChatCard::new(CardType::Error, "Stream Error", format!("{err}")),
                )?;
                break;
            }
            _ => {}
        }
    }
    draw_viewport(&mut guard.terminal, "", "", 0, true)?;
    Ok(())
}

// ============ Helpers ============

fn tail_lines(s: &str, n: usize) -> String {
    let lines: Vec<&str> = s.lines().collect();
    if lines.is_empty() {
        return String::new();
    }
    let take = lines.len().min(n);
    let start = lines.len() - take;
    lines[start..].join("\n")
}

fn render_tool_result(content: &OneOrMany<ToolResultContent>) -> String {
    let rendered: Vec<String> = content
        .iter()
        .map(|item| match item {
            ToolResultContent::Text(text) => text.text.clone(),
            ToolResultContent::Image(_) => "[image]".to_string(),
        })
        .collect();
    if rendered.is_empty() {
        "(empty)".to_string()
    } else {
        rendered.join("\n")
    }
}
