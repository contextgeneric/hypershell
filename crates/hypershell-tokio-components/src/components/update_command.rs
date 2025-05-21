use core::marker::PhantomData;

use cgp::prelude::*;
use tokio::process::Command;

#[cgp_component(CommandUpdater)]
pub trait CanUpdateCommand<Args> {
    fn update_command(&self, _phantom: PhantomData<Args>, command: &mut Command);
}
