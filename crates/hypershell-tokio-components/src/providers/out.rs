use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::AsyncRead;
use futures::io::copy;
use hypershell_components::dsl::StreamToStdout;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[cgp_new_provider]
impl<Context, Input> Handler<Context, StreamToStdout, Input> for HandleStreamToStdout
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsyncRead + Unpin,
{
    type Output = ();

    async fn handle(
        _context: &Context,
        _tag: PhantomData<StreamToStdout>,
        input: Input,
    ) -> Result<(), Context::Error> {
        let mut stdout = tokio::io::stdout().compat_write();

        copy(input, &mut stdout)
            .await
            .map_err(Context::raise_error)?;

        Ok(())
    }
}
