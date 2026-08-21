use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
}

// ============ Parser ============

pub fn parse(raw: &str) -> anyhow::Result<SkillFrontmatter> {
    let trimmed_start = raw.trim_start();
    if !trimmed_start.starts_with("---") {
        anyhow::bail!("invalid SKILL.md: missing frontmatter start delimiter '---'");
    }

    let after_first = trimmed_start
        .strip_prefix("---")
        .ok_or_else(|| anyhow::anyhow!("invalid SKILL.md: cannot strip leading '---'"))?;

    let mut lines = after_first.lines();
    let mut yaml_lines: Vec<&str> = Vec::new();
    let mut closed = false;
    for line in lines.by_ref() {
        if line.trim() == "---" {
            closed = true;
            break;
        }
        yaml_lines.push(line);
    }

    if !closed {
        anyhow::bail!("invalid SKILL.md: missing frontmatter end delimiter '---'");
    }
    if yaml_lines.is_empty() {
        anyhow::bail!("invalid SKILL.md: frontmatter is empty");
    }

    let fm: SkillFrontmatter = serde_yaml::from_str(&yaml_lines.join("\n"))
        .map_err(|e| anyhow::anyhow!("invalid SKILL.md frontmatter yaml: {e}"))?;

    if fm.name.trim().is_empty() {
        anyhow::bail!("invalid SKILL.md: frontmatter field 'name' cannot be empty");
    }
    if fm.description.trim().is_empty() {
        anyhow::bail!("invalid SKILL.md: frontmatter field 'description' cannot be empty");
    }

    Ok(fm)
}
