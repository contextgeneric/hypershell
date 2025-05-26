use std::process::Output;

use cgp::prelude::*;

#[derive(HasField)]
pub struct SimpleExecOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl From<Output> for SimpleExecOutput {
    fn from(output: Output) -> Self {
        Self {
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}
