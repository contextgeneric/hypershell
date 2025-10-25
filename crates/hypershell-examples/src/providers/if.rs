use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell::prelude::CanHandle;

use crate::dsl::If;

#[cgp_impl(new HandleIf)]
impl<Context, CodeCond, CodeThen, CodeElse, InputCond, InputBranch, Output>
    Handler<If<CodeCond, CodeThen, CodeElse>, (InputCond, InputBranch)> for Context
where
    Context: CanHandle<CodeCond, InputCond, Output = bool>
        + CanHandle<CodeThen, InputBranch, Output = Output>
        + CanHandle<CodeElse, InputBranch, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<If<CodeCond, CodeThen, CodeElse>>,
        (input_cond, input_branch): (InputCond, InputBranch),
    ) -> Result<Output, Context::Error> {
        let cond = context.handle(PhantomData::<CodeCond>, input_cond).await?;

        if cond {
            context.handle(PhantomData::<CodeThen>, input_branch).await
        } else {
            context.handle(PhantomData::<CodeElse>, input_branch).await
        }
    }
}
