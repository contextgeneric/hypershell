use core::marker::PhantomData;
use std::path::Path;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::CanExtractCommandArg;
use hypershell_components::dsl::{ReadFile, WriteFile};
use tokio::fs::File;
use tokio::io::AsyncRead;

#[cgp_impl(new HandleReadFile)]
impl<Context, PathArg> Handler<ReadFile<PathArg>, ()> for Context
where
    Context: CanExtractCommandArg<PathArg> + CanRaiseError<std::io::Error>,
    Context::CommandArg: AsRef<Path>,
{
    type Output = File;

    async fn handle(
        context: &Context,
        _tag: PhantomData<ReadFile<PathArg>>,
        _input: (),
    ) -> Result<File, Context::Error> {
        let file_path = context.extract_command_arg(PhantomData);

        let file = File::open(file_path.as_ref())
            .await
            .map_err(Context::raise_error)?;

        Ok(file)
    }
}

#[cgp_impl(new HandleWriteFile)]
impl<Context, PathArg, Input> Handler<WriteFile<PathArg>, Input> for Context
where
    Context: CanExtractCommandArg<PathArg> + CanRaiseError<std::io::Error>,
    Context::CommandArg: AsRef<Path>,
    Input: AsyncRead + Unpin,
{
    type Output = ();

    async fn handle(
        context: &Context,
        _tag: PhantomData<WriteFile<PathArg>>,
        mut input: Input,
    ) -> Result<(), Context::Error> {
        let file_path = context.extract_command_arg(PhantomData);

        let mut file = File::create(file_path.as_ref())
            .await
            .map_err(Context::raise_error)?;

        tokio::io::copy(&mut input, &mut file)
            .await
            .map_err(Context::raise_error)?;

        Ok(())
    }
}
