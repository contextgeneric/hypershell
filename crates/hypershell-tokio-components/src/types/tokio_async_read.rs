use core::pin::Pin;
use core::task::{Context, Poll};

use tokio::io::{AsyncRead as TokioAsyncRead, ReadBuf};

pub struct TokioAsyncReadStream<S> {
    pub stream: S,
}

impl<S> TokioAsyncReadStream<S>
where
    S: Send + Unpin + TokioAsyncRead + 'static,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> TokioAsyncRead for TokioAsyncReadStream<S>
where
    S: Send + Unpin + TokioAsyncRead + 'static,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
