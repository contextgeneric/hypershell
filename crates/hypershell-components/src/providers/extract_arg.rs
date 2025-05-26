use core::fmt::Display;
use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::prelude::*;

use crate::components::{ArgExtractor, ArgExtractorComponent, CanExtractArg, HasCommandArgType};
use crate::dsl::{FieldArg, JoinArgs, StaticArg};

#[cgp_new_provider]
impl<Context, Arg> ArgExtractor<Context, StaticArg<Arg>> for ExtractStaticArg
where
    Context: HasCommandArgType,
    Context::CommandArg: From<String>,
    Arg: Default + Display,
{
    fn extract_arg(
        _context: &Context,
        _phantom: PhantomData<StaticArg<Arg>>,
    ) -> Context::CommandArg {
        Arg::default().to_string().into()
    }
}

#[cgp_new_provider]
impl<Context, Tag> ArgExtractor<Context, FieldArg<Tag>> for ExtractFieldArg
where
    Context: HasCommandArgType + HasField<Tag, Value: Display>,
    Context::CommandArg: From<String>,
{
    fn extract_arg(context: &Context, _phantom: PhantomData<FieldArg<Tag>>) -> Context::CommandArg {
        context.get_field(PhantomData).to_string().into()
    }
}

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
        "".into()
    }
}
