use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::Stream;
use hypershell_components::dsl::StreamToLines;
use tokio::io::AsyncRead;
use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

#[cgp_impl(new HandleStreamToLines)]
impl<Context, Input> Handler<StreamToLines, Input> for Context
where
    Context: HasErrorType,
    Input: AsyncRead + Unpin + 'static,
{
    type Output = Box<dyn Stream<Item = Result<String, LinesCodecError>>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<StreamToLines>,
        input: Input,
    ) -> Result<Box<dyn Stream<Item = Result<String, LinesCodecError>>>, Context::Error> {
        let stream = FramedRead::new(input, LinesCodec::new());

        Ok(Box::new(stream))
    }
}
