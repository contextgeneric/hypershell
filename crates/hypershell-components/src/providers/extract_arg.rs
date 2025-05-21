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

#[cgp_new_provider]
impl<Context, ArgA, ArgB> ArgExtractor<Context, Join<ArgA, ArgB>> for JoinExtractArgs
where
    Context: CanExtractArg<ArgA> + CanExtractArg<ArgB> + HasCommandArgType<CommandArg = PathBuf>,
{
    fn extract_arg(context: &Context, _phantom: PhantomData<Join<ArgA, ArgB>>) -> PathBuf {
        let arg_a = context.extract_arg(PhantomData::<ArgA>);
        let arg_b = context.extract_arg(PhantomData::<ArgB>);
        arg_a.join(arg_b)
    }
}
