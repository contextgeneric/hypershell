use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use futures::io::{AsyncReadExt, Cursor, copy};
use hypershell_apps::contexts::CliApp;
use hypershell_components::dsl::{Pipe, StaticArg, StreamingExec, WithStaticArgs};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = Pipe<
        Product![
            StreamingExec<
                StaticArg<symbol!("nix-shell")>,
                WithStaticArgs<
                    Product![
                        symbol!("-p"),
                        symbol!("websocat"),
                        symbol!("--run"),
                        symbol!("websocat -nU wss://jetstream1.us-west.bsky.network/subscribe"),
                    ],
                >,
            >,
        ],
    >;

    let app = CliApp {};

    let input = Cursor::new(Vec::new());

    let output = app.handle(PhantomData::<Program>, input).await?;

    let output = output.take(409600);

    let mut stdout = tokio::io::stdout().compat_write();

    copy(output, &mut stdout).await?;

    Ok(())
}
