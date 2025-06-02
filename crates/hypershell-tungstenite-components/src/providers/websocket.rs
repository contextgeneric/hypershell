use core::marker::PhantomData;
use core::pin::Pin;
use std::io::ErrorKind;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::{AsyncRead, StreamExt, TryStreamExt};
use hypershell_components::components::CanExtractStringArg;
use hypershell_components::dsl::WebSocket;
use tokio::spawn;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite};
use tokio_util::bytes::Bytes;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::io::ReaderStream;

#[cgp_new_provider]
impl<Context, UrlArg, Headers, Input> Handler<Context, WebSocket<UrlArg, Headers>, Input>
    for HandleWebsocket
where
    Context: CanExtractStringArg<UrlArg> + CanRaiseAsyncError<tungstenite::Error>,
    UrlArg: Send,
    Headers: Send,
    Input: Send + AsyncRead + Unpin + 'static,
{
    type Output = Pin<Box<dyn AsyncRead + Send>>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<WebSocket<UrlArg, Headers>>,
        input: Input,
    ) -> Result<Pin<Box<dyn AsyncRead + Send>>, Context::Error> {
        let input = ReaderStream::new(input.compat());

        let url = context
            .extract_string_arg(PhantomData)
            .into_client_request()
            .map_err(Context::raise_error)?;

        let (websocket, _) = connect_async(url).await.unwrap();
        let (writer, reader) = websocket.split();

        spawn(async move {
            let _ = input
                .map_ok(|data| Message::Binary(data))
                .map_err(|e| tungstenite::Error::from(e))
                .forward(writer)
                .await;
        });

        let output = reader
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            .filter_map(async |res| match res {
                Ok(Message::Text(data)) => {
                    let text = format!("{}\n", data.as_str());
                    Some(Ok(Bytes::from(text)))
                }
                Ok(message) => Some(Ok(message.into_data())),
                Err(e) => Some(Err(std::io::Error::new(ErrorKind::Other, e))),
            })
            .into_async_read();

        Ok(Box::pin(output))
    }
}
