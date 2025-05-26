use core::fmt::Debug;
use core::marker::PhantomData;
use std::process::Output;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::SimpleExec;
use tokio::io::AsyncWriteExt;
use tokio::process::Child;

use crate::providers::CoreExec;

pub struct SimpleExecFailure<'a, Context> {
    pub context: &'a Context,
    pub error: std::io::Error,
}

pub struct ExecOutputError<'a, Context> {
    pub context: &'a Context,
    pub output: Output,
}

#[cgp_new_provider]
impl<Context, CommandPath, Args> Handler<Context, SimpleExec<CommandPath, Args>, Vec<u8>>
    for RunSimpleExec
where
    Context: for<'a> CanRaiseAsyncError<ExecOutputError<'a, Context>>
        + for<'a> CanRaiseAsyncError<SimpleExecFailure<'a, Context>>,
    CommandPath: Send,
    Args: Send,
    CoreExec<CommandPath, Args>: Handler<Context, (), (), Output = Child>,
{
    type Output = Vec<u8>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<SimpleExec<CommandPath, Args>>,
        input: Vec<u8>,
    ) -> Result<Vec<u8>, Context::Error> {
        let mut child = <CoreExec<CommandPath, Args>>::handle(context, PhantomData, ()).await?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&input)
                .await
                .map_err(|error| Context::raise_error(SimpleExecFailure { context, error }))?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|error| Context::raise_error(SimpleExecFailure { context, error }))?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(Context::raise_error(ExecOutputError { context, output }))
        }
    }
}

impl<'a, Context> Debug for SimpleExecFailure<'a, Context> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error executing command: {}", self.error)
    }
}

impl<'a, Context> Debug for ExecOutputError<'a, Context> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "child process exited with non-success code: {:?}",
            self.output.status.code()
        )
    }
}
