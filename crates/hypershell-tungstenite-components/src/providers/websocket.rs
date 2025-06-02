use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::AsyncRead;
use hypershell_components::components::CanExtractStringArg;
use hypershell_components::dsl::Websocket;
use tokio::io::{join, simplex};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{client_async, tungstenite};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};

#[cgp_new_provider]
impl<Context, UrlArg, Headers, Input> Handler<Context, Websocket<UrlArg, Headers>, Input>
    for HandleWebsocket
where
    Context: CanExtractStringArg<UrlArg> + CanRaiseAsyncError<tungstenite::Error>,
    UrlArg: Send,
    Headers: Send,
    Input: Send + AsyncRead + Unpin,
{
    type Output = Pin<Box<dyn AsyncRead + Send>>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Websocket<UrlArg, Headers>>,
        input: Input,
    ) -> Result<Pin<Box<dyn AsyncRead + Send>>, Context::Error> {
        let url = context
            .extract_string_arg(PhantomData)
            .into_client_request()
            .map_err(Context::raise_error)?;

        let (read, write) = simplex(102400);

        let _websocket = client_async(url, join(input.compat(), write))
            .await
            .map_err(Context::raise_error)?;

        Ok(Box::pin(read.compat()))
    }
}
