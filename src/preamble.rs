use crate::config::RuntimeConfig;
use crate::skill::SkillSummary;
use std::path::Path;

// ============ Loader ============

pub fn load_prompt_template(path: &Path) -> anyhow::Result<String> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("failed to read prompt template {}: {e}", path.display()))?;
    Ok(raw)
}

// ============ Builder ============

pub fn render(template: &str, runtime: &RuntimeConfig, skills: &[SkillSummary]) -> String {
    let skills_block = if skills.is_empty() {
        String::new()
    } else {
        skills
            .iter()
            .map(|s| format!("- {}：{}", s.name, s.description))
            .collect::<Vec<_>>()
            .join("\n")
    };

    template
        .replace("{memoryDir}", &runtime.memory_dir.display().to_string())
        .replace("{skillDir}", &runtime.skill_dir.display().to_string())
        .replace("{skills}", &skills_block)
}
