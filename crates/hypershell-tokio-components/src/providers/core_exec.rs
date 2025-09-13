use core::fmt::Debug;
use core::marker::PhantomData;
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::process::Stdio;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractCommandArg;
use itertools::Itertools;
use tokio::process::{Child, Command};

use crate::components::CanUpdateCommand;
use crate::dsl::CoreExec;

#[cgp_new_provider]
impl<Context, CommandPath, Args> Handler<Context, CoreExec<CommandPath, Args>, ()>
    for HandleCoreExec
where
    Context: HasErrorType
        + CanExtractCommandArg<CommandPath>
        + CanUpdateCommand<Args>
        + CanRaiseError<std::io::Error>
        + for<'a> CanWrapError<CommandNotFound<'a>>
        + for<'a> CanWrapError<SpawnCommandFailure<'a>>,
    Context::CommandArg: AsRef<OsStr>,
{
    type Output = Child;

    async fn handle(
        context: &Context,
        _tag: PhantomData<CoreExec<CommandPath, Args>>,
        _input: (),
    ) -> Result<Child, Context::Error> {
        let command_path = context.extract_command_arg(PhantomData);

        let mut command = Command::new(&command_path);

        context.update_command(PhantomData, &mut command);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let spawn_res = command.spawn();

        let child = spawn_res.map_err(|e| {
            let is_not_found = e.kind() == ErrorKind::NotFound;

            let mut e = Context::raise_error(e);

            if is_not_found {
                e = Context::wrap_error(e, CommandNotFound { command: &command });
            }

            Context::wrap_error(e, SpawnCommandFailure { command: &command })
        })?;

        Ok(child)
    }
}

pub struct SpawnCommandFailure<'a> {
    pub command: &'a Command,
}

pub struct CommandNotFound<'a> {
    pub command: &'a Command,
}

impl<'a> Debug for CommandNotFound<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "command not found: {}",
            self.command.as_std().get_program().to_string_lossy(),
        )
    }
}

impl<'a> Debug for SpawnCommandFailure<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "error executing command: {} {}",
            self.command.as_std().get_program().to_string_lossy(),
            self.command
                .as_std()
                .get_args()
                .map(|arg| arg.to_string_lossy())
                .join(" "),
        )
    }
}
