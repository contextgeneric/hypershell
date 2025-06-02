use core::marker::PhantomData;
use core::pin::Pin;
use std::path::Path;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractCommandArg;
use hypershell_components::dsl::ReadFile;
use tokio::fs::File;
use tokio::io::AsyncRead;

#[cgp_new_provider]
impl<Context, PathArg> Handler<Context, ReadFile<PathArg>, ()> for HandleReadFile
where
    Context: CanExtractCommandArg<PathArg> + CanRaiseAsyncError<std::io::Error>,
    PathArg: Send,
    Context::CommandArg: Send + AsRef<Path>,
{
    type Output = Pin<Box<dyn AsyncRead + Send>>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<ReadFile<PathArg>>,
        _input: (),
    ) -> Result<Pin<Box<dyn AsyncRead + Send>>, Context::Error> {
        let file_path = context.extract_command_arg(PhantomData);

        let file = File::open(file_path.as_ref())
            .await
            .map_err(Context::raise_error)?;

        Ok(Box::pin(file))
    }
}
