use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::contexts::CliApp;
use hypershell_components::dsl::{SimpleExec, StaticArg, StreamingExec, WithStaticArgs};

#[tokio::test]
async fn test_basic_exec() -> Result<(), Error> {
    pub type Program = StreamingExec<
        StaticArg<symbol!("echo")>,
        WithStaticArgs<Product![symbol!("hello"), symbol!("world!")]>,
    >;

    let app = CliApp {};

    // let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    // assert_eq!(output, "hello world!\n".as_bytes());

    Ok(())
}
