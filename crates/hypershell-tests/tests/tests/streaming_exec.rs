use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use futures::io::{Cursor, copy};
use hypershell_apps::contexts::CliApp;
use hypershell_components::dsl::{StaticArg, StreamingExec, WithStaticArgs};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = StreamingExec<
        StaticArg<symbol!("echo")>,
        WithStaticArgs<Product![symbol!("hello"), symbol!("world!")]>,
    >;

    let app = CliApp {};

    let input = Cursor::new(Vec::new());

    let output = app.handle(PhantomData::<Program>, input).await?;

    let mut stdout = tokio::io::stdout().compat_write();

    copy(output, &mut stdout).await?;

    Ok(())
}
