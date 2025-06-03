use core::pin::Pin;
use core::task::{Context, Poll};

use futures::AsyncRead;

pub struct FuturesAsyncReadStream<S> {
    pub stream: S,
}

impl<S> From<S> for FuturesAsyncReadStream<S>
where
    S: Send + Unpin + AsyncRead + 'static,
{
    fn from(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> AsyncRead for FuturesAsyncReadStream<S>
where
    S: Send + Unpin + AsyncRead + 'static,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
