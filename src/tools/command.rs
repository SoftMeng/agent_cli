use crate::tools::error::ToolError;
use rig_core::tool::Tool;
use rig_core::wasm_compat::WasmCompatSend;
use serde::Deserialize;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[allow(clippy::manual_async_fn)]
#[derive(Debug, Deserialize)]
pub struct RunCommandArgs {
    pub command: String,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RunCommandTool;

const DEFAULT_TIMEOUT_SECS: u64 = 5;
const MAX_TIMEOUT_SECS: u64 = 300;

impl Tool for RunCommandTool {
    const NAME: &'static str = "run_command";

    type Args = RunCommandArgs;
    type Output = String;
    type Error = ToolError;

    fn description(&self) -> String {
        "Execute a shell command via `sh -c`, capture stdout+stderr, return as text.".into()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "Shell command to execute" },
                "timeout_secs": { "type": "integer", "minimum": 1, "maximum": 300, "default": 5 }
            },
            "required": ["command"]
        })
    }

    #[allow(clippy::manual_async_fn)]
    fn call(
        &self,
        args: Self::Args,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + WasmCompatSend {
        async move {
            let command = args.command.trim();
            if command.is_empty() {
                return Err(ToolError::InvalidArgs("command cannot be empty".into()));
            }
            let timeout = args
                .timeout_secs
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .clamp(1, MAX_TIMEOUT_SECS);

            let mut child = Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(ToolError::Io)?;

            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            let stdout_fut = async {
                let mut buf = Vec::new();
                if let Some(mut s) = stdout {
                    let _ = s.read_to_end(&mut buf).await;
                }
                buf
            };
            let stderr_fut = async {
                let mut buf = Vec::new();
                if let Some(mut s) = stderr {
                    let _ = s.read_to_end(&mut buf).await;
                }
                buf
            };

            let (stdout_bytes, stderr_bytes, wait_res) =
                tokio::join!(stdout_fut, stderr_fut, async {
                    match tokio::time::timeout(Duration::from_secs(timeout), child.wait()).await {
                        Ok(res) => res.map_err(ToolError::Io),
                        Err(_) => Err(ToolError::Timeout),
                    }
                });

            let stdout_text = String::from_utf8_lossy(&stdout_bytes);
            let stderr_text = String::from_utf8_lossy(&stderr_bytes);
            let code = match wait_res {
                Ok(status) => status.code().unwrap_or(-1),
                Err(_) => -1,
            };

            let mut out = String::new();
            if !stdout_text.is_empty() {
                out.push_str(&stdout_text);
            }
            if !stderr_text.is_empty() {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str("[stderr]\n");
                out.push_str(&stderr_text);
            }
            out.push_str(&format!("\n[exit_code: {code}]"));
            Ok(out)
        }
    }
}
