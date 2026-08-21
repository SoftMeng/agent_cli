use crate::agent::provider::ProviderKind;
use crate::memory::ConversationStore;
use rig_core::client::CompletionClient;
use rig_core::tool::server::ToolServerHandle;

// ============ Types ============

pub enum AgentKind {
    OpenAI(rig_core::agent::Agent<rig_core::providers::openai::completion::CompletionModel>),
    Anthropic(rig_core::agent::Agent<rig_core::providers::anthropic::completion::CompletionModel>),
}

// ============ Builders ============

pub fn build_agent(
    provider: ProviderKind,
    model: &str,
    preamble: &str,
    tools: ToolServerHandle,
    memory: ConversationStore,
    conversation_id: &str,
    extra_params: Option<serde_json::Value>,
) -> AgentKind {
    match provider {
        ProviderKind::OpenAI(client) => {
            let mut b = client
                .agent(model)
                .preamble(preamble)
                .name("agent_cli")
                .tool_server_handle(tools)
                .memory(memory)
                .conversation(conversation_id);
            if let Some(v) = extra_params {
                b = b.additional_params(v);
            }
            AgentKind::OpenAI(b.build())
        }
        ProviderKind::Anthropic(client) => {
            let mut b = client
                .agent(model)
                .preamble(preamble)
                .name("agent_cli")
                .tool_server_handle(tools)
                .memory(memory)
                .conversation(conversation_id);
            if let Some(v) = extra_params {
                b = b.additional_params(v);
            }
            AgentKind::Anthropic(b.build())
        }
    }
}
