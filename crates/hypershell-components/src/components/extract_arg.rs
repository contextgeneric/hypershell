use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component(ArgExtractor)]
pub trait CanExtractArg<Arg> {
    fn extract_arg(&self, _phantom: PhantomData<Arg>) -> String;
}

#[cgp_provider]
impl<Context, Arg, Components, Delegate> ArgExtractor<Context, Arg> for UseDelegate<Components>
where
    Components: DelegateComponent<Arg, Delegate = Delegate>,
    Delegate: ArgExtractor<Context, Arg>,
{
    fn extract_arg(context: &Context, phantom: PhantomData<Arg>) -> String {
        Delegate::extract_arg(context, phantom)
    }
}
