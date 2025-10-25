use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::StreamingExec;
use tokio::io::{AsyncRead, Empty, copy, empty};
use tokio::process::{Child, ChildStdout};
use tokio::spawn;
use tokio_util::either::Either;

use crate::dsl::CoreExec;

#[cgp_impl(new HandleStreamingExec)]
impl<Context, CommandPath, Args, Input> Handler<StreamingExec<CommandPath, Args>, Input> for Context
where
    Context:
        CanHandle<CoreExec<CommandPath, Args>, (), Output = Child> + CanRaiseError<std::io::Error>,
    Input: Send + Unpin + AsyncRead + 'static,
{
    type Output = Either<ChildStdout, Empty>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<StreamingExec<CommandPath, Args>>,
        mut input: Input,
    ) -> Result<Either<ChildStdout, Empty>, Context::Error> {
        let mut child = context.handle(PhantomData, ()).await?;

        if let Some(mut stdin) = child.stdin.take() {
            spawn(async move {
                let _ = copy(&mut input, &mut stdin).await;
            });
        }

        let output = match child.stdout.take() {
            Some(stdout) => Either::Left(stdout),
            None => Either::Right(empty()),
        };

        Ok(output)
    }
}
