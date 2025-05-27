use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasHttpMethodType {
    type HttpMethod;
}

#[cgp_component(MethodArgExtractor)]
pub trait CanExtractMethodArg<Arg>: HasHttpMethodType {
    fn extract_method_arg(&self, _phantom: PhantomData<Arg>) -> Self::HttpMethod;
}

#[cgp_provider]
impl<Context, Arg, Components> MethodArgExtractor<Context, Arg> for UseDelegate<Components>
where
    Context: HasHttpMethodType,
    Components: DelegateComponent<Arg>,
    Components::Delegate: MethodArgExtractor<Context, Arg>,
{
    fn extract_method_arg(context: &Context, phantom: PhantomData<Arg>) -> Context::HttpMethod {
        Components::Delegate::extract_method_arg(context, phantom)
    }
}
