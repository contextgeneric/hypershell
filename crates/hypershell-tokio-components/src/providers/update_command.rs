use core::marker::PhantomData;
use std::ffi::OsStr;

use cgp::prelude::*;
use hypershell_components::components::CanExtractArg;
use hypershell_components::dsl::WithArgs;
use tokio::process::Command;

use crate::components::{CommandUpdater, CommandUpdaterComponent};

pub struct ExtractArgs;

#[cgp_provider]
impl<Context, Arg, Args> CommandUpdater<Context, WithArgs<Cons<Arg, Args>>> for ExtractArgs
where
    Context: CanExtractArg<Arg>,
    Context::CommandArg: AsRef<OsStr> + Send,
    Self: CommandUpdater<Context, WithArgs<Args>>,
{
    fn update_command(
        context: &Context,
        _phantom: PhantomData<WithArgs<Cons<Arg, Args>>>,
        command: &mut Command,
    ) {
        let arg = context.extract_arg(PhantomData);
        command.arg(arg);

        Self::update_command(context, PhantomData::<WithArgs<Args>>, command);
    }
}

#[cgp_provider]
impl<Context> CommandUpdater<Context, WithArgs<Nil>> for ExtractArgs {
    fn update_command(
        _context: &Context,
        _phantom: PhantomData<WithArgs<Nil>>,
        _command: &mut Command,
    ) {
    }
}
