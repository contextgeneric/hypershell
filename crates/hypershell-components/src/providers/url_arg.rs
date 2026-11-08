use core::marker::PhantomData;
use core::str::FromStr;

use cgp::prelude::*;

use crate::components::{
    CanExtractStringArg, HasUrlType, UrlArgExtractor, UrlArgExtractorComponent,
};
use crate::dsl::FieldArg;

/// Parses the argument's string form into the context's URL type. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new ExtractStringUrlArg)]
impl<Context, Arg> UrlArgExtractor<Arg> for Context
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

/// Reads a pre-parsed URL from the context field named `Tag`. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new ExtractUrlFieldArg)]
impl<Context, Tag> UrlArgExtractor<FieldArg<Tag>> for Context
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
