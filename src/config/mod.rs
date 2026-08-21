use serde::Deserialize;
use std::path::{Path, PathBuf};

// ============ Types ============

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LLMProvider {
    Openai,
    Anthropic,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LLMConfig {
    pub api_key: Option<String>,
    pub base_url: String,
    pub chat_model: String,
    #[serde(default)]
    pub extra_params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub skill_dir: PathBuf,
    #[serde(default = "default_memory_dir")]
    pub memory_dir: PathBuf,
    pub llm_provider: LLMProvider,
    pub llm_config: LLMConfig,
    #[serde(default)]
    pub sys_prompt: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub skill_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub llm_provider: LLMProvider,
    pub llm_config: LLMConfig,
    pub sys_prompt: PathBuf,
}

// ============ Loaders ============

impl AppConfig {
    pub fn from_yaml_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("failed to read config file {}: {e}", path.display()))?;
        let cfg: Self = serde_yaml::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("failed to parse yaml config {}: {e}", path.display()))?;
        Ok(cfg)
    }

    pub fn into_runtime_config(self) -> RuntimeConfig {
        let sys_prompt = self
            .sys_prompt
            .unwrap_or_else(|| PathBuf::from("./AGENT_PROMPT.txt"));
        RuntimeConfig {
            skill_dir: self.skill_dir,
            memory_dir: self.memory_dir,
            llm_provider: self.llm_provider,
            llm_config: self.llm_config,
            sys_prompt,
        }
    }
}

fn default_memory_dir() -> PathBuf {
    PathBuf::from("./memory")
}
