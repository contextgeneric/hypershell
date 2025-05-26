use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasCommandArgType {
    type CommandArg;
}

#[cgp_component(CommandArgExtractor)]
pub trait CanExtractCommandArg<Arg>: HasCommandArgType {
    fn extract_arg(&self, _phantom: PhantomData<Arg>) -> Self::CommandArg;
}

#[cgp_provider]
impl<Context, Arg, Components, Delegate> CommandArgExtractor<Context, Arg>
    for UseDelegate<Components>
where
    Context: HasCommandArgType,
    Components: DelegateComponent<Arg, Delegate = Delegate>,
    Delegate: CommandArgExtractor<Context, Arg>,
{
    fn extract_arg(context: &Context, phantom: PhantomData<Arg>) -> Context::CommandArg {
        Delegate::extract_arg(context, phantom)
    }
}
