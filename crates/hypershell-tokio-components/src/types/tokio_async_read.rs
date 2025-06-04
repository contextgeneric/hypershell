use core::pin::Pin;
use core::task::{Context, Poll};

use tokio::io::{AsyncRead, ReadBuf};

pub struct TokioAsyncReadStream<S> {
    pub stream: S,
}

impl<S> From<S> for TokioAsyncReadStream<S>
where
    S: Unpin + AsyncRead,
{
    fn from(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> AsyncRead for TokioAsyncReadStream<S>
where
    S: Unpin + AsyncRead,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
