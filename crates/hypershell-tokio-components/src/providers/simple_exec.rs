use core::marker::PhantomData;
use std::process::{Output, Stdio};

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractArg;
use hypershell_components::dsl::SimpleExec;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::components::CanUpdateCommand;

#[derive(Debug)]
pub struct ExecCommandFailure<'a> {
    pub command: &'a Command,
    pub error: std::io::Error,
}

#[cgp_new_provider]
impl<Context, CommandPath, Args> Handler<Context, SimpleExec<CommandPath, Args>, Vec<u8>>
    for RunSimpleExec
where
    Context: HasAsyncErrorType
        + CanExtractArg<CommandPath>
        + CanUpdateCommand<Args>
        + for<'a> CanRaiseAsyncError<ExecCommandFailure<'a>>,
    CommandPath: Send,
    Args: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<SimpleExec<CommandPath, Args>>,
        input: Vec<u8>,
    ) -> Result<Output, Context::Error> {
        let command_path = context.extract_arg(PhantomData);

        let mut command = Command::new(&command_path);

        context.update_command(PhantomData, &mut command);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().map_err(|error| {
            Context::raise_error(ExecCommandFailure {
                command: &command,
                error,
            })
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&input).await.map_err(|error| {
                Context::raise_error(ExecCommandFailure {
                    command: &command,
                    error,
                })
            })?;
        }

        let output = child.wait_with_output().await.map_err(|error| {
            Context::raise_error(ExecCommandFailure {
                command: &command,
                error,
            })
        })?;

        Ok(output)
    }
}
