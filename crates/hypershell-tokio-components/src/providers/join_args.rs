use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::prelude::*;
use hypershell_components::components::{
    CanExtractCommandArg, CommandArgExtractor, CommandArgExtractorComponent, HasCommandArgType,
};
use hypershell_components::dsl::JoinArgs;

pub struct JoinExtractArgs;

#[cgp_impl(JoinExtractArgs)]
impl<Context, Arg, Args> CommandArgExtractor<JoinArgs<Cons<Arg, Args>>> for Context
where
    Context: CanExtractCommandArg<Arg> + HasCommandArgType<CommandArg = PathBuf>,
    JoinExtractArgs: CommandArgExtractor<Context, JoinArgs<Args>>,
{
    fn extract_command_arg(
        context: &Context,
        _phantom: PhantomData<JoinArgs<Cons<Arg, Args>>>,
    ) -> PathBuf {
        let arg_a = context.extract_command_arg(PhantomData);
        let arg_b = JoinExtractArgs::extract_command_arg(context, PhantomData::<JoinArgs<Args>>);

        if arg_b.as_os_str().is_empty() {
            arg_a
        } else {
            arg_a.join(arg_b)
        }
    }
}

#[cgp_impl(JoinExtractArgs)]
impl<Context> CommandArgExtractor<JoinArgs<Nil>> for Context
where
    Context: HasCommandArgType<CommandArg = PathBuf>,
{
    fn extract_command_arg(_context: &Context, _phantom: PhantomData<JoinArgs<Nil>>) -> PathBuf {
        PathBuf::new()
    }
}
