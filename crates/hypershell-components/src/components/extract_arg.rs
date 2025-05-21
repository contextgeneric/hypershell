use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasCommandArgType {
    type CommandArg;
}

#[cgp_component(ArgExtractor)]
pub trait CanExtractArg<Arg>: HasCommandArgType {
    fn extract_arg(&self, _phantom: PhantomData<Arg>) -> Self::CommandArg;
}

#[cgp_provider]
impl<Context, Arg, Components, Delegate> ArgExtractor<Context, Arg> for UseDelegate<Components>
where
    Context: HasCommandArgType,
    Components: DelegateComponent<Arg, Delegate = Delegate>,
    Delegate: ArgExtractor<Context, Arg>,
{
    fn extract_arg(context: &Context, phantom: PhantomData<Arg>) -> Context::CommandArg {
        Delegate::extract_arg(context, phantom)
    }
}
