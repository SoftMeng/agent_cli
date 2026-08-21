use crate::skill::SkillRegistry;
use crate::tools::error::ToolError;
use rig_core::tool::Tool;
use rig_core::wasm_compat::WasmCompatSend;
use serde::Deserialize;
use std::path::PathBuf;

#[allow(clippy::manual_async_fn)]
// ============ ListSkillsTool ============
#[derive(Debug, Default, Deserialize)]
pub struct ListSkillsArgs {}

#[derive(Debug, Clone)]
pub struct ListSkillsTool {
    pub skill_root: PathBuf,
}

impl Tool for ListSkillsTool {
    const NAME: &'static str = "list_skills";

    type Args = ListSkillsArgs;
    type Output = String;
    type Error = ToolError;

    fn description(&self) -> String {
        "List every local skill (name + description) under the skill directory.".into()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({"type": "object", "properties": {}})
    }

    #[allow(clippy::manual_async_fn)]
    fn call(
        &self,
        _args: Self::Args,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + WasmCompatSend {
        async move {
            let registry = SkillRegistry::from_dir(&self.skill_root)
                .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;
            let view: Vec<serde_json::Value> = registry
                .list()
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "name": s.name,
                        "description": s.description
                    })
                })
                .collect();
            serde_json::to_string_pretty(&view).map_err(|e| ToolError::InvalidArgs(e.to_string()))
        }
    }
}

// ============ GetSkillTool ============

#[derive(Debug, Deserialize)]
pub struct GetSkillArgs {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GetSkillTool {
    pub skill_root: PathBuf,
}

impl Tool for GetSkillTool {
    const NAME: &'static str = "get_skill";

    type Args = GetSkillArgs;
    type Output = String;
    type Error = ToolError;

    fn description(&self) -> String {
        "Return the full SKILL.md content for a named skill.".into()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string", "description": "Skill name (frontmatter name field)" }
            },
            "required": ["name"]
        })
    }

    #[allow(clippy::manual_async_fn)]
    fn call(
        &self,
        args: Self::Args,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + WasmCompatSend {
        async move {
            let name = args.name.trim();
            if name.is_empty() {
                return Err(ToolError::InvalidArgs("name cannot be empty".into()));
            }
            let registry = SkillRegistry::from_dir(&self.skill_root)
                .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;
            let summary = registry
                .get(name)
                .ok_or_else(|| ToolError::SkillNotFound(name.to_string()))?;
            let skill_md = summary.path.join("SKILL.md");
            std::fs::read_to_string(&skill_md).map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ToolError::FileNotFound(skill_md.display().to_string())
                } else {
                    ToolError::Io(e)
                }
            })
        }
    }
}
