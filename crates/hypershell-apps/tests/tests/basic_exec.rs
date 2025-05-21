use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::contexts::HypershellApp;
use hypershell_components::dsl::{SimpleExec, StaticArg};

#[tokio::test]
async fn test_basic_exec() -> Result<(), Error> {
    let app = HypershellApp {};

    let output = app
        .handle(
            PhantomData::<
                SimpleExec<
                    StaticArg<symbol!("echo")>,
                    Product![StaticArg<symbol!("hello")>, StaticArg<symbol!("world!")>],
                >,
            >,
            Vec::new(),
        )
        .await?;

    assert!(output.status.success());
    assert_eq!(output.stdout, "hello world!\n".as_bytes());
    assert!(output.stderr.is_empty());

    Ok(())
}
