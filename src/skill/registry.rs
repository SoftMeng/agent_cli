use crate::skill::frontmatter;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SkillSummary {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct SkillRegistry {
    skills: Vec<SkillSummary>,
}

// ============ Builders ============

impl SkillRegistry {
    pub fn from_dir(root: &Path) -> anyhow::Result<Self> {
        let mut skills = Vec::new();

        if !root.is_dir() {
            tracing::warn!(
                "skill dir {} does not exist; registry will be empty",
                root.display()
            );
            return Ok(Self { skills });
        }

        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let dir = entry.path();
            if !dir.is_dir() {
                continue;
            }
            let skill_md = dir.join("SKILL.md");
            if !skill_md.is_file() {
                continue;
            }
            let raw = std::fs::read_to_string(&skill_md).map_err(|e| {
                anyhow::anyhow!("failed to read SKILL.md at {}: {e}", skill_md.display())
            })?;
            let fm = frontmatter::parse(&raw)
                .map_err(|e| anyhow::anyhow!("invalid skill at {}: {e}", dir.display()))?;
            skills.push(SkillSummary {
                name: fm.name,
                description: fm.description,
                path: dir,
            });
        }

        skills.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Self { skills })
    }
}

// ============ Accessors ============

impl SkillRegistry {
    pub fn list(&self) -> &[SkillSummary] {
        &self.skills
    }

    pub fn get(&self, name: &str) -> Option<&SkillSummary> {
        self.skills.iter().find(|s| s.name == name)
    }
}
