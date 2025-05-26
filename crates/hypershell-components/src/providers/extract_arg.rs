use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::components::{
    CanExtractStringArg, CommandArgExtractor, CommandArgExtractorComponent, HasCommandArgType,
    StringArgExtractor, StringArgExtractorComponent,
};
use crate::dsl::{FieldArg, StaticArg};

#[cgp_new_provider]
impl<Context, Arg> CommandArgExtractor<Context, Arg> for ExtractStringCommandArg
where
    Context: HasCommandArgType + CanExtractStringArg<Arg>,
    Context::CommandArg: From<String>,
{
    fn extract_command_arg(context: &Context, phantom: PhantomData<Arg>) -> Context::CommandArg {
        context.extract_string_arg(phantom).into_owned().into()
    }
}

#[cgp_new_provider]
impl<Context, Arg> StringArgExtractor<Context, StaticArg<Arg>> for ExtractStaticArg
where
    Arg: Default + Display,
{
    fn extract_string_arg(
        _context: &Context,
        _phantom: PhantomData<StaticArg<Arg>>,
    ) -> Cow<'_, str> {
        Arg::default().to_string().into()
    }
}

#[cgp_new_provider]
impl<Context, Tag> StringArgExtractor<Context, FieldArg<Tag>> for ExtractFieldArg
where
    Context: HasField<Tag, Value: Display>,
{
    fn extract_string_arg(context: &Context, _phantom: PhantomData<FieldArg<Tag>>) -> Cow<'_, str> {
        context.get_field(PhantomData).to_string().into()
    }
}
