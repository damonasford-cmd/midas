use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationRequest {
    pub working_directory: String,
    pub command: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Default)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, request: &CompilationRequest) -> Result<CompilationResult> {
        if request.command.trim().is_empty() {
            return Err(anyhow!("compiler command cannot be empty"));
        }

        let output = Command::new(&request.command)
            .args(&request.arguments)
            .current_dir(&request.working_directory)
            .output()?;

        Ok(CompilationResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
