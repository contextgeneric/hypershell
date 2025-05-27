use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasUrlType {
    type Url;
}

#[cgp_component(UrlArgExtractor)]
pub trait CanExtractUrlArg<Arg>: HasUrlType + HasErrorType {
    fn extract_url_arg(&self, _phantom: PhantomData<Arg>) -> Result<Self::Url, Self::Error>;
}

#[cgp_provider]
impl<Context, Arg, Components, Delegate> UrlArgExtractor<Context, Arg> for UseDelegate<Components>
where
    Context: HasUrlType + HasErrorType,
    Components: DelegateComponent<Arg, Delegate = Delegate>,
    Delegate: UrlArgExtractor<Context, Arg>,
{
    fn extract_url_arg(
        context: &Context,
        phantom: PhantomData<Arg>,
    ) -> Result<Context::Url, Context::Error> {
        Delegate::extract_url_arg(context, phantom)
    }
}
