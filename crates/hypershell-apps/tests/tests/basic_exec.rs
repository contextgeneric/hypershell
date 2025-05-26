use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::contexts::HypershellApp;
use hypershell_components::dsl::{SimpleExec, StaticArg, WithArgs};

#[tokio::test]
async fn test_basic_exec() -> Result<(), Error> {
    pub type Program = SimpleExec<
        StaticArg<symbol!("echo")>,
        WithArgs<Product![StaticArg<symbol!("hello")>, StaticArg<symbol!("world!")>]>,
    >;

    let app = HypershellApp {};

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    assert_eq!(output.stdout, "hello world!\n".as_bytes());
    assert!(output.stderr.is_empty());

    Ok(())
}
