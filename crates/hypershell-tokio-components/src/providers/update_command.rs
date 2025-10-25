use core::marker::PhantomData;
use std::ffi::OsStr;

use cgp::prelude::*;
use hypershell_components::components::CanExtractCommandArg;
use hypershell_components::dsl::{FieldArgs, WithArgs};
use tokio::process::Command;

use crate::components::{CommandUpdater, CommandUpdaterComponent};

pub struct ExtractArgs;

#[cgp_impl(ExtractArgs)]
impl<Context, Arg, Args> CommandUpdater<WithArgs<Cons<Arg, Args>>> for Context
where
    Context: CanExtractCommandArg<Arg>,
    Context::CommandArg: AsRef<OsStr> + Send,
    Self: CommandUpdater<Context, WithArgs<Args>>,
{
    fn update_command(
        context: &Context,
        _phantom: PhantomData<WithArgs<Cons<Arg, Args>>>,
        command: &mut Command,
    ) {
        let arg = context.extract_command_arg(PhantomData);
        command.arg(arg);

        Self::update_command(context, PhantomData::<WithArgs<Args>>, command);
    }
}

#[cgp_impl(ExtractArgs)]
impl<Context> CommandUpdater<WithArgs<Nil>> for Context {
    fn update_command(
        _context: &Context,
        _phantom: PhantomData<WithArgs<Nil>>,
        _command: &mut Command,
    ) {
    }
}

#[cgp_impl(new ExtractFieldArgs)]
impl<Context, Tag> CommandUpdater<FieldArgs<Tag>> for Context
where
    Context: HasField<Tag>,
    for<'a> &'a Context::Value: IntoIterator<Item: AsRef<OsStr>>,
{
    fn update_command(
        context: &Context,
        _phantom: PhantomData<FieldArgs<Tag>>,
        command: &mut Command,
    ) {
        let args = context.get_field(PhantomData);
        command.args(args);
    }
}
