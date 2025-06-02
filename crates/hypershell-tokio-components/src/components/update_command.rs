use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use tokio::process::Command;

#[cgp_component {
    provider: CommandUpdater,
    derive_delegate: UseDelegate<Args>,
}]
pub trait CanUpdateCommand<Args> {
    fn update_command(&self, _phantom: PhantomData<Args>, command: &mut Command);
}
