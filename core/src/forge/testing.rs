use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRequest {
    pub working_directory: String,
    pub command: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Default)]
pub struct TestRunner;

impl TestRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, request: &TestRequest) -> std::io::Result<TestResult> {
        let output = Command::new(&request.command)
            .args(&request.arguments)
            .current_dir(&request.working_directory)
            .output()?;

        Ok(TestResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
