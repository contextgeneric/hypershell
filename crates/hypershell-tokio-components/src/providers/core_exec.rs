use core::fmt::Debug;
use core::marker::PhantomData;
use std::ffi::OsStr;
use std::process::Stdio;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractArg;
use tokio::process::{Child, Command};

use crate::components::CanUpdateCommand;
use crate::dsl::CoreExec;

pub struct SpawnCommandFailure<'a, Context> {
    pub context: &'a Context,
    pub command: &'a Command,
    pub error: std::io::Error,
}

#[cgp_new_provider]
impl<Context, CommandPath, Args> Handler<Context, CoreExec<CommandPath, Args>, ()> for RunCoreExec
where
    Context: HasAsyncErrorType
        + CanExtractArg<CommandPath>
        + CanUpdateCommand<Args>
        + for<'a> CanRaiseAsyncError<SpawnCommandFailure<'a, Context>>,
    Context::CommandArg: AsRef<OsStr> + Send,
    CommandPath: Send,
    Args: Send,
{
    type Output = Child;

    async fn handle(
        context: &Context,
        _tag: PhantomData<CoreExec<CommandPath, Args>>,
        _input: (),
    ) -> Result<Child, Context::Error> {
        let command_path = context.extract_arg(PhantomData);

        let mut command = Command::new(&command_path);

        context.update_command(PhantomData, &mut command);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let child = command.spawn().map_err(|error| {
            Context::raise_error(SpawnCommandFailure {
                context,
                command: &command,
                error,
            })
        })?;

        Ok(child)
    }
}

impl<'a, Context> Debug for SpawnCommandFailure<'a, Context> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error executing command: {}", self.error)
    }
}
