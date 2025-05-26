use core::fmt::Debug;
use core::marker::PhantomData;
use std::process::Output;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::SimpleExec;
use tokio::io::AsyncWriteExt;
use tokio::process::Child;

use crate::dsl::CoreExec;

pub struct ExecOutputError {
    pub output: Output,
}

#[cgp_new_provider]
impl<Context, CommandPath, Args> Handler<Context, SimpleExec<CommandPath, Args>, Vec<u8>>
    for RunSimpleExec
where
    Context: CanHandle<CoreExec<CommandPath, Args>, (), Output = Child>
        + for<'a> CanRaiseAsyncError<ExecOutputError>
        + CanWrapAsyncError<StdinPipeError>
        + CanWrapAsyncError<WaitWithOutputError>
        + CanRaiseAsyncError<std::io::Error>,
    CommandPath: Send,
    Args: Send,
{
    type Output = Vec<u8>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<SimpleExec<CommandPath, Args>>,
        input: Vec<u8>,
    ) -> Result<Vec<u8>, Context::Error> {
        let mut child = context.handle(PhantomData, ()).await?;

        if !input.is_empty() {
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(&input)
                    .await
                    .map_err(Context::raise_error)
                    .map_err(|e| Context::wrap_error(e, StdinPipeError))?;
            }
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(Context::raise_error)
            .map_err(|e| Context::wrap_error(e, WaitWithOutputError))?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(Context::raise_error(ExecOutputError { output }))
        }
    }
}

pub struct StdinPipeError;

impl Debug for StdinPipeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error piping input to stdin of child process")
    }
}

pub struct WaitWithOutputError;

impl Debug for WaitWithOutputError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error waiting for output from child process")
    }
}

impl Debug for ExecOutputError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "child process exited with non-success code {:?}, stderr: {}",
            self.output.status.code(),
            String::from_utf8_lossy(&self.output.stderr),
        )
    }
}
