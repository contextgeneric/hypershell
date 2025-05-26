use alloc::borrow::Cow;
use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component(StringArgExtractor)]
pub trait CanExtractStringArg<Arg> {
    fn extract_string_arg(&self, _phantom: PhantomData<Arg>) -> Cow<'_, str>;
}

#[cgp_provider]
impl<Context, Arg, Components, Delegate> StringArgExtractor<Context, Arg>
    for UseDelegate<Components>
where
    Components: DelegateComponent<Arg, Delegate = Delegate>,
    Delegate: StringArgExtractor<Context, Arg>,
{
    fn extract_string_arg(context: &Context, phantom: PhantomData<Arg>) -> Cow<'_, str> {
        Delegate::extract_string_arg(context, phantom)
    }
}
