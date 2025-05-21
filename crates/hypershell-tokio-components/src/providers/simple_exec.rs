use core::marker::PhantomData;
use std::process::Output;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractArg;
use hypershell_components::dsl::SimpleExec;
use tokio::process::Command;

use crate::components::CanUpdateCommand;

#[derive(Debug)]
pub struct ExecCommandFailure {
    pub command: Command,
    pub error: std::io::Error,
}

#[cgp_new_provider]
impl<Context, CommandPath, Args> Handler<Context, SimpleExec<CommandPath, Args>, Vec<u8>>
    for RunSimpleExec
where
    Context: HasAsyncErrorType
        + CanExtractArg<CommandPath>
        + CanUpdateCommand<Args>
        + CanRaiseAsyncError<ExecCommandFailure>,
    CommandPath: Send,
    Args: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<SimpleExec<CommandPath, Args>>,
        _input: Vec<u8>,
    ) -> Result<Output, Context::Error> {
        let command_path = context.extract_arg(PhantomData);

        let mut command = Command::new(command_path);

        context.update_command(PhantomData, &mut command);

        let output = command
            .output()
            .await
            .map_err(|error| Context::raise_error(ExecCommandFailure { command, error }))?;

        Ok(output)
    }
}
