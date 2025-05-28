use core::marker::PhantomData;
use std::borrow::Cow;

use cgp::prelude::*;
use hypershell_components::components::{
    CanExtractStringArg, StringArgExtractor, StringArgExtractorComponent,
};
use hypershell_components::dsl::UrlEncodeArg;
use url::form_urlencoded;

#[cgp_new_provider]
impl<Context, Arg> StringArgExtractor<Context, UrlEncodeArg<Arg>> for UrlEncodeStringArg
where
    Context: CanExtractStringArg<Arg>,
{
    fn extract_string_arg(
        context: &Context,
        _phantom: PhantomData<UrlEncodeArg<Arg>>,
    ) -> Cow<'_, str> {
        let raw_arg = context.extract_string_arg(PhantomData);
        form_urlencoded::byte_serialize(raw_arg.as_bytes()).collect()
    }
}
