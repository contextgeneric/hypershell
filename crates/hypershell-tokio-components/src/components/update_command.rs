use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use tokio::process::Command;

/// Applies an argument-list syntax to a Tokio `Command`.
#[cgp_component(CommandUpdater)]
#[prefix(@hypershell.tokio in DefaultNamespace)]
#[derive_delegate(UseDelegate<Args>)]
pub trait CanUpdateCommand<Args> {
    fn update_command(&self, _phantom: PhantomData<Args>, command: &mut Command);
}
