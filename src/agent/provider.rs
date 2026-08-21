use crate::config::{LLMConfig, LLMProvider, RuntimeConfig};
use anyhow::Context;

// ============ Types ============

pub enum ProviderKind {
    OpenAI(rig_core::providers::openai::CompletionsClient),
    Anthropic(rig_core::providers::anthropic::Client),
}

// ============ Builders ============

pub fn build_provider(runtime: &RuntimeConfig) -> anyhow::Result<ProviderKind> {
    match runtime.llm_provider {
        LLMProvider::Openai => build_openai(&runtime.llm_config).context("openai provider init"),
        LLMProvider::Anthropic => {
            build_anthropic(&runtime.llm_config).context("anthropic provider init")
        }
    }
}

fn build_openai(cfg: &LLMConfig) -> anyhow::Result<ProviderKind> {
    use rig_core::providers::openai;
    let api_key = resolve_api_key(cfg.api_key.as_deref(), "OPENAI_API_KEY")
        .context("missing openai api key")?;
    let client: openai::CompletionsClient = openai::Client::builder()
        .api_key(api_key)
        .base_url(&cfg.base_url)
        .build()
        .map_err(|e| anyhow::anyhow!("openai client build failed: {e}"))?
        .completions_api();
    Ok(ProviderKind::OpenAI(client))
}

fn build_anthropic(cfg: &LLMConfig) -> anyhow::Result<ProviderKind> {
    use rig_core::providers::anthropic;
    let api_key = resolve_api_key(cfg.api_key.as_deref(), "ANTHROPIC_API_KEY")
        .context("missing anthropic api key")?;
    let mut builder = anthropic::Client::builder()
        .api_key(api_key)
        .anthropic_version(anthropic::completion::ANTHROPIC_VERSION_LATEST)
        .anthropic_beta("prompt-caching-2024-07-31");
    if !cfg.base_url.is_empty() {
        builder = builder.base_url(&cfg.base_url);
    }
    let client = builder
        .build()
        .map_err(|e| anyhow::anyhow!("anthropic client build failed: {e}"))?;
    Ok(ProviderKind::Anthropic(client))
}

fn resolve_api_key(configured: Option<&str>, env_var: &'static str) -> anyhow::Result<String> {
    if let Some(key) = configured
        && !key.trim().is_empty()
    {
        return Ok(key.to_string());
    }
    std::env::var(env_var).map_err(|_| {
        anyhow::anyhow!("missing api key: set llm_config.api_key or {env_var} env var")
    })
}
