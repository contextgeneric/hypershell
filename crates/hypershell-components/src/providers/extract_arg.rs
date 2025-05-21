use core::fmt::Display;
use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::prelude::*;

use crate::components::{ArgExtractor, ArgExtractorComponent, CanExtractArg, HasCommandArgType};
use crate::dsl::{Join, StaticArg};

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
impl<Context, Tag> ArgExtractor<Context, UseField<Tag>> for ExtractFieldArg
where
    Context: HasCommandArgType + HasField<Tag, Value: Display>,
    Context::CommandArg: From<String>,
{
    fn extract_arg(context: &Context, _phantom: PhantomData<UseField<Tag>>) -> Context::CommandArg {
        context.get_field(PhantomData).to_string().into()
    }
}

pub struct JoinExtractArgs;

#[cgp_provider]
impl<Context, Arg, Args> ArgExtractor<Context, Join<Cons<Arg, Args>>> for JoinExtractArgs
where
    Context: CanExtractArg<Arg> + HasCommandArgType<CommandArg = PathBuf>,
    Self: ArgExtractor<Context, Join<Args>>,
{
    fn extract_arg(context: &Context, _phantom: PhantomData<Join<Cons<Arg, Args>>>) -> PathBuf {
        let arg_a = context.extract_arg(PhantomData);
        let arg_b = Self::extract_arg(context, PhantomData::<Join<Args>>);
        arg_a.join(arg_b)
    }
}

#[cgp_provider]
impl<Context> ArgExtractor<Context, Join<Nil>> for JoinExtractArgs
where
    Context: HasCommandArgType<CommandArg = PathBuf>,
{
    fn extract_arg(_context: &Context, _phantom: PhantomData<Join<Nil>>) -> PathBuf {
        PathBuf::default()
    }
}
