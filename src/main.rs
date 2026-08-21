use agent_cli::agent::{build_agent, build_provider};
use agent_cli::app::run_repl;
use agent_cli::config::AppConfig;
use agent_cli::memory::ConversationStore;
use agent_cli::preamble::{load_prompt_template, render};
use agent_cli::skill::SkillRegistry;
use agent_cli::tools::start_tool_server;

const DEFAULT_CONVERSATION_ID: &str = "default";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let app_cfg = AppConfig::from_yaml_file("config.yaml")?;
    let runtime = app_cfg.into_runtime_config();
    let registry = SkillRegistry::from_dir(&runtime.skill_dir)?;
    let template = load_prompt_template(&runtime.sys_prompt)?;
    let preamble = render(&template, &runtime, registry.list());
    let provider = build_provider(&runtime)?;
    let tools = start_tool_server(&runtime)?;
    let memory = ConversationStore::new();
    let agent = build_agent(
        provider,
        &runtime.llm_config.chat_model,
        &preamble,
        tools,
        memory,
        DEFAULT_CONVERSATION_ID,
        runtime.llm_config.extra_params.clone(),
    );
    run_repl(agent, DEFAULT_CONVERSATION_ID.to_string()).await
}

fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    let env = std::env::var("RUST_LOG").unwrap_or_default();
    if env.is_empty() {
        // Default: silence tracing to avoid polluting TUI.
        let _ = tracing_subscriber::fmt()
            .with_writer(std::io::sink)
            .with_env_filter(EnvFilter::new("off"))
            .try_init();
    } else {
        // RUST_LOG set: append to ./agent_cli.log
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("agent_cli.log");
        match file {
            Ok(f) => {
                let _ = tracing_subscriber::fmt()
                    .with_writer(move || f.try_clone().unwrap())
                    .with_env_filter(
                        EnvFilter::try_from_default_env()
                            .unwrap_or_else(|_| EnvFilter::new("info")),
                    )
                    .try_init();
            }
            Err(_) => {
                let _ = tracing_subscriber::fmt()
                    .with_writer(std::io::sink)
                    .with_env_filter(
                        EnvFilter::try_from_default_env()
                            .unwrap_or_else(|_| EnvFilter::new("info")),
                    )
                    .try_init();
            }
        }
    }
}
