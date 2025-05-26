use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::prelude::*;
use hypershell_components::components::{
    ArgExtractor, ArgExtractorComponent, CanExtractArg, HasCommandArgType,
};
use hypershell_components::dsl::JoinArgs;

pub struct JoinExtractArgs;

#[cgp_provider]
impl<Context, Arg, Args> ArgExtractor<Context, JoinArgs<Cons<Arg, Args>>> for JoinExtractArgs
where
    Context: CanExtractArg<Arg> + HasCommandArgType<CommandArg = PathBuf>,
    Self: ArgExtractor<Context, JoinArgs<Args>>,
{
    fn extract_arg(context: &Context, _phantom: PhantomData<JoinArgs<Cons<Arg, Args>>>) -> PathBuf {
        let arg_a = context.extract_arg(PhantomData);
        let arg_b = Self::extract_arg(context, PhantomData::<JoinArgs<Args>>);

        if arg_b.as_os_str().is_empty() {
            arg_a
        } else {
            arg_a.join(arg_b)
        }
    }
}

#[cgp_provider]
impl<Context> ArgExtractor<Context, JoinArgs<Nil>> for JoinExtractArgs
where
    Context: HasCommandArgType<CommandArg = PathBuf>,
{
    fn extract_arg(_context: &Context, _phantom: PhantomData<JoinArgs<Nil>>) -> PathBuf {
        PathBuf::new()
    }
}
