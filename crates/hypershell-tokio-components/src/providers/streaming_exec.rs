use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use futures::AsyncRead;
use futures::io::{copy, empty};
use hypershell_components::dsl::StreamingExec;
use tokio::process::Child;
use tokio::spawn;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

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
        let mut child = context.handle(PhantomData, ()).await?;

        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin.compat_write();
            spawn(async move {
                let _ = copy(&mut input, &mut stdin).await;
            });
        }

        let output: Pin<Box<dyn AsyncRead + Send>> = match child.stdout.take() {
            Some(stdout) => Box::pin(stdout.compat()),
            None => Box::pin(empty()),
        };

        Ok(output)
    }
}
