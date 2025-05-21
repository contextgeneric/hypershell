use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::CanExtractArg;
use tokio::process::Command;

use crate::components::{CommandUpdater, CommandUpdaterComponent};

pub struct ExtractArgs;

#[cgp_provider]
impl<Context, Arg, Args> CommandUpdater<Context, Cons<Arg, Args>> for ExtractArgs
where
    Context: CanExtractArg<Arg>,
{
    fn update_command(
        context: &Context,
        _phantom: PhantomData<Cons<Arg, Args>>,
        command: &mut Command,
    ) {
        let arg = context.extract_arg(PhantomData);
        command.arg(arg);
    }
}

#[cgp_provider]
impl<Context> CommandUpdater<Context, Nil> for ExtractArgs {
    fn update_command(_context: &Context, _phantom: PhantomData<Nil>, _command: &mut Command) {}
}
