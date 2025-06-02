use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasCommandArgType {
    type CommandArg;
}

#[cgp_component {
    provider: CommandArgExtractor,
    use_delegate: Arg,
}]
pub trait CanExtractCommandArg<Arg>: HasCommandArgType {
    fn extract_command_arg(&self, _phantom: PhantomData<Arg>) -> Self::CommandArg;
}
