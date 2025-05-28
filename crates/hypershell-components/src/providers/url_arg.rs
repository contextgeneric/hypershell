use core::marker::PhantomData;
use core::str::FromStr;

use cgp::prelude::*;

use crate::components::{
    CanExtractStringArg, HasUrlType, UrlArgExtractor, UrlArgExtractorComponent,
};
use crate::dsl::FieldArg;

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
