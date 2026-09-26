use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::StreamingExec;
use tokio::io::AsyncRead;
use tokio::process::Child;

use crate::dsl::CoreExec;
use crate::types::ChildOutputStream;

#[cgp_impl(new HandleStreamingExec)]
impl<Context, CommandPath, Args, Input> Handler<StreamingExec<CommandPath, Args>, Input> for Context
where
    Context:
        CanHandle<CoreExec<CommandPath, Args>, (), Output = Child> + CanRaiseError<std::io::Error>,
    Input: Send + Unpin + AsyncRead + 'static,
{
    type Output = ChildOutputStream;

    async fn handle(
        context: &Context,
        _tag: PhantomData<StreamingExec<CommandPath, Args>>,
        input: Input,
    ) -> Result<ChildOutputStream, Context::Error> {
        let child = context.handle(PhantomData, ()).await?;

        // The stream ends with an error when reading the input failed or the child exits with a
        // non-success status, carrying the child's stderr.
        Ok(ChildOutputStream::new(child, input))
    }
}
