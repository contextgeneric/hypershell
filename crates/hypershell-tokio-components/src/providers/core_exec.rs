use core::fmt::Debug;
use core::marker::PhantomData;
use std::ffi::OsStr;
use std::process::{Output, Stdio};

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractArg;
use tokio::process::{Child, Command};

use crate::components::CanUpdateCommand;

pub struct ExecCommandFailure<'a, Context> {
    pub context: &'a Context,
    pub command: &'a Command,
    pub error: std::io::Error,
}

pub struct ExecOutputError<'a, Context> {
    pub context: &'a Context,
    pub output: Output,
}

#[cgp_new_provider]
impl<Context, Code, CommandPath, Args> Handler<Context, Code, ()> for CoreExec<CommandPath, Args>
where
    Context: HasAsyncErrorType
        + CanExtractArg<CommandPath>
        + CanUpdateCommand<Args>
        + for<'a> CanRaiseAsyncError<ExecOutputError<'a, Context>>
        + for<'a> CanRaiseAsyncError<ExecCommandFailure<'a, Context>>,
    Context::CommandArg: AsRef<OsStr> + Send,
    CommandPath: Send,
    Args: Send,
    Code: Send,
{
    type Output = Child;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Code>,
        _input: (),
    ) -> Result<Child, Context::Error> {
        let command_path = context.extract_arg(PhantomData);

        let mut command = Command::new(&command_path);

        context.update_command(PhantomData, &mut command);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let child = command.spawn().map_err(|error| {
            Context::raise_error(ExecCommandFailure {
                context,
                command: &command,
                error,
            })
        })?;

        Ok(child)
    }
}

impl<'a, Context> Debug for ExecCommandFailure<'a, Context> {
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
