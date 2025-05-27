use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::fmt::Display;
use core::marker::PhantomData;
use core::str::FromStr;

use cgp::prelude::*;

use crate::components::{
    CanExtractStringArg, CommandArgExtractor, CommandArgExtractorComponent, HasCommandArgType,
    HasHttpMethodType, HasUrlType, MethodArgExtractor, MethodArgExtractorComponent,
    StringArgExtractor, StringArgExtractorComponent, UrlArgExtractor, UrlArgExtractorComponent,
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
impl<Context, Arg> UrlArgExtractor<Context, Arg> for ExtractStringUrlArg
where
    Context: HasUrlType + CanExtractStringArg<Arg> + CanRaiseError<<Context::Url as FromStr>::Err>,
    Context::Url: FromStr,
{
    fn extract_url_arg(
        context: &Context,
        phantom: PhantomData<Arg>,
    ) -> Result<Context::Url, Context::Error> {
        let url_str = context.extract_string_arg(phantom);
        url_str.parse().map_err(Context::raise_error)
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

#[cgp_new_provider]
impl<Context, Tag> UrlArgExtractor<Context, FieldArg<Tag>> for ExtractUrlFieldArg
where
    Context: HasUrlType + HasErrorType + HasField<Tag, Value = Context::Url>,
    Context::Url: Clone,
{
    fn extract_url_arg(
        context: &Context,
        _phantom: PhantomData<FieldArg<Tag>>,
    ) -> Result<Context::Url, Context::Error> {
        Ok(context.get_field(PhantomData).clone())
    }
}

#[cgp_new_provider]
impl<Context, Tag> MethodArgExtractor<Context, FieldArg<Tag>> for ExtractMethodFieldArg
where
    Context: HasHttpMethodType + HasField<Tag, Value = Context::HttpMethod>,
    Context::HttpMethod: Clone,
{
    fn extract_method_arg(
        context: &Context,
        _phantom: PhantomData<FieldArg<Tag>>,
    ) -> Context::HttpMethod {
        context.get_field(PhantomData).clone()
    }
}
