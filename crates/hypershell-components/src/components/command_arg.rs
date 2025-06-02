use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasCommandArgType {
    type CommandArg;
}

#[cgp_component {
    provider: CommandArgExtractor,
    derive_delegate: UseDelegate<Arg>,
}]
pub trait CanExtractCommandArg<Arg>: HasCommandArgType {
    fn extract_command_arg(&self, _phantom: PhantomData<Arg>) -> Self::CommandArg;
}
