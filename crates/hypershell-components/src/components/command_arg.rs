use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

/// A context's abstract command-argument type.
#[cgp_type]
#[prefix(@hypershell.core in DefaultNamespace)]
pub trait HasCommandArgType {
    type CommandArg;
}

/// Extracts a command argument from the context.
#[cgp_component(CommandArgExtractor)]
#[prefix(@hypershell.core in DefaultNamespace)]
#[derive_delegate(UseDelegate<Arg>)]
pub trait CanExtractCommandArg<Arg>: HasCommandArgType {
    fn extract_command_arg(&self, _phantom: PhantomData<Arg>) -> Self::CommandArg;
}
