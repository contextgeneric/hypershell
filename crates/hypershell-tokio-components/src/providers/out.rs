use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::StreamToStdout;
use tokio::io::{AsyncRead, copy};

#[cgp_impl(new HandleStreamToStdout)]
impl<Context, Input> Handler<StreamToStdout, Input> for Context
where
    Context: CanRaiseError<std::io::Error>,
    Input: AsyncRead + Unpin,
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
