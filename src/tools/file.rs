use crate::tools::error::ToolError;
use rig_core::tool::Tool;
use rig_core::wasm_compat::WasmCompatSend;
use serde::Deserialize;

// Trait bounds require an explicit `impl Future + WasmCompatSend` return type.
#[allow(clippy::manual_async_fn)]
#[derive(Debug, Deserialize)]
pub struct ReadFileArgs {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct ReadFileTool;

impl Tool for ReadFileTool {
    const NAME: &'static str = "read_file";

    type Args = ReadFileArgs;
    type Output = String;
    type Error = ToolError;

    fn description(&self) -> String {
        "Read a local file by relative or absolute path and return its contents.".into()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path to the file to read" }
            },
            "required": ["path"]
        })
    }

    #[allow(clippy::manual_async_fn)]
    fn call(
        &self,
        args: Self::Args,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + WasmCompatSend {
        async move {
            let path = args.path.trim();
            if path.is_empty() {
                return Err(ToolError::InvalidArgs("path cannot be empty".into()));
            }
            match std::fs::read_to_string(path) {
                Ok(content) => Ok(content),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    Err(ToolError::FileNotFound(path.to_string()))
                }
                Err(e) => Err(ToolError::Io(e)),
            }
        }
    }
}
