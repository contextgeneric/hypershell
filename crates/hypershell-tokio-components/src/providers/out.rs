use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::StreamToStdout;
use tokio::io::{AsyncRead, copy};

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
        mut input: Input,
    ) -> Result<(), Context::Error> {
        let mut stdout = tokio::io::stdout();

        copy(&mut input, &mut stdout)
            .await
            .map_err(Context::raise_error)?;

        Ok(())
    }
}
