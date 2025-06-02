use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use tokio::process::Command;

#[cgp_component {
    provider: CommandUpdater,
    use_delegate: Args,
}]
pub trait CanUpdateCommand<Args> {
    fn update_command(&self, _phantom: PhantomData<Args>, command: &mut Command);
}
