use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::components::{ArgExtractor, ArgExtractorComponent, HasCommandArgType};
use crate::dsl::StaticArg;

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
