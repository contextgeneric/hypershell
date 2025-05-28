use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::StreamingExec;
use tokio::io::{AsyncRead, copy, empty};
use tokio::process::Child;
use tokio::spawn;

use crate::dsl::CoreExec;

#[cgp_new_provider]
impl<Context, CommandPath, Args, Input> Handler<Context, StreamingExec<CommandPath, Args>, Input>
    for HandleStreamingExec
where
    Context: CanHandle<CoreExec<CommandPath, Args>, (), Output = Child>
        + CanRaiseAsyncError<std::io::Error>,
    CommandPath: Send,
    Args: Send,
    Input: Send + AsyncRead + Unpin + 'static,
{
    type Output = Pin<Box<dyn AsyncRead + Send>>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<StreamingExec<CommandPath, Args>>,
        mut input: Input,
    ) -> Result<Pin<Box<dyn AsyncRead + Send>>, Context::Error> {
        let child = context.handle(PhantomData, ()).await?;

        if let Some(mut stdin) = child.stdin {
            spawn(async move {
                let _ = copy(&mut input, &mut stdin).await;
            });
        }

        let output: Pin<Box<dyn AsyncRead + Send>> = match child.stdout {
            Some(stdout) => Box::pin(stdout),
            None => Box::pin(empty()),
        };

        Ok(output)
    }
}
