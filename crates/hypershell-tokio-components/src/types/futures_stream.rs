use core::pin::Pin;
use core::task::{Context, Poll};

use futures::Stream;

pub struct FuturesStream<S> {
    pub stream: S,
}

impl<S> From<S> for FuturesStream<S>
where
    S: Unpin + Stream,
{
    fn from(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> Stream for FuturesStream<S>
where
    S: Unpin + Stream,
{
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }
}
