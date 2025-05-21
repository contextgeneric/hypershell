use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use tokio::process::Command;

#[cgp_component(CommandUpdater)]
pub trait CanUpdateCommand<Args> {
    fn update_command(&self, _phantom: PhantomData<Args>, command: &mut Command);
}

#[cgp_provider]
impl<Context, Args, Components, Delegate> CommandUpdater<Context, Args> for UseDelegate<Components>
where
    Components: DelegateComponent<Args, Delegate = Delegate>,
    Delegate: CommandUpdater<Context, Args>,
{
    fn update_command(context: &Context, phantom: PhantomData<Args>, command: &mut Command) {
        Delegate::update_command(context, phantom, command);
    }
}
