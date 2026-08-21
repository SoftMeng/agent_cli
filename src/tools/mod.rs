pub mod command;
pub mod error;
pub mod file;
pub mod memory;
pub mod skill;

pub use error::ToolError;

use crate::config::RuntimeConfig;
use crate::tools::command::RunCommandTool;
use crate::tools::file::ReadFileTool;
use crate::tools::memory::AddMemoryTool;
use crate::tools::skill::{GetSkillTool, ListSkillsTool};
use rig_core::tool::server::{ToolServer, ToolServerHandle};

// ============ Server Bootstrap ============

pub fn start_tool_server(runtime: &RuntimeConfig) -> anyhow::Result<ToolServerHandle> {
    let memory_dir = runtime.memory_dir.clone();
    let skill_root = runtime.skill_dir.clone();

    let server = ToolServer::new()
        .tool(ReadFileTool)
        .tool(RunCommandTool)
        .tool(AddMemoryTool { memory_dir })
        .tool(ListSkillsTool {
            skill_root: skill_root.clone(),
        })
        .tool(GetSkillTool { skill_root });

    Ok(server.run())
}
