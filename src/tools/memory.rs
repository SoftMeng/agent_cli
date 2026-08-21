use crate::memory::add_entry;
use crate::tools::error::ToolError;
use rig_core::tool::Tool;
use rig_core::wasm_compat::WasmCompatSend;
use serde::Deserialize;
use std::path::PathBuf;

#[allow(clippy::manual_async_fn)]
#[derive(Debug, Deserialize)]
pub struct AddMemoryArgs {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct AddMemoryTool {
    pub memory_dir: PathBuf,
}

impl Tool for AddMemoryTool {
    const NAME: &'static str = "add_memory";

    type Args = AddMemoryArgs;
    type Output = String;
    type Error = ToolError;

    fn description(&self) -> String {
        "Append a timestamped entry to today's memory markdown file.".into()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "content": { "type": "string", "description": "Memory content to record" }
            },
            "required": ["content"]
        })
    }

    #[allow(clippy::manual_async_fn)]
    fn call(
        &self,
        args: Self::Args,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + WasmCompatSend {
        async move {
            let content = args.content.trim();
            if content.is_empty() {
                return Err(ToolError::EmptyContent);
            }
            let path = add_entry(&self.memory_dir, content).map_err(ToolError::Io)?;
            Ok(format!("memory saved: {}", path.display()))
        }
    }
}
