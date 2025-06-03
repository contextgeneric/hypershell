use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::StreamingExec;
use tokio::io::{AsyncRead, Empty, copy, empty};
use tokio::process::{Child, ChildStdout};
use tokio::spawn;
use tokio_util::either::Either;

use crate::dsl::CoreExec;
use crate::types::tokio_async_read::TokioAsyncReadStream;

#[cgp_new_provider]
impl<Context, CommandPath, Args, Input> Handler<Context, StreamingExec<CommandPath, Args>, Input>
    for HandleStreamingExec
where
    Context: CanHandle<CoreExec<CommandPath, Args>, (), Output = Child>
        + CanRaiseAsyncError<std::io::Error>,
    CommandPath: Send,
    Args: Send,
    Input: Send + Unpin + AsyncRead + 'static,
{
    type Output = TokioAsyncReadStream<Either<ChildStdout, Empty>>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<StreamingExec<CommandPath, Args>>,
        mut input: Input,
    ) -> Result<TokioAsyncReadStream<Either<ChildStdout, Empty>>, Context::Error> {
        let mut child = context.handle(PhantomData, ()).await?;

        if let Some(mut stdin) = child.stdin.take() {
            spawn(async move {
                let _ = copy(&mut input, &mut stdin).await;
            });
        }

        let output = match child.stdout.take() {
            Some(stdout) => TokioAsyncReadStream::new(Either::Left(stdout)),
            None => TokioAsyncReadStream::new(Either::Right(empty())),
        };

        Ok(output)
    }
}
