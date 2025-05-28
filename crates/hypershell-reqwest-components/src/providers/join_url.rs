use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::{
    CanExtractStringArg, CanExtractUrlArg, HasUrlType, UrlArgExtractor, UrlArgExtractorComponent,
};
use hypershell_components::dsl::JoinArgs;
use reqwest::Url;

#[cgp_new_provider]
impl<Context, Args> UrlArgExtractor<Context, JoinArgs<Args>>
    for JoinExtractUrl
where
    Context: HasUrlType<Url = Url> + CanExtractStringArg<JoinArgs<Args>> + HasErrorType,
{
    fn extract_url_arg(
        context: &Context,
        _phantom: PhantomData<JoinArgs<Args>>,
    ) -> Result<Url, Context::Error> {
        let url_str = context.extract_string_arg(PhantomData);

        todo!()
    }
}
