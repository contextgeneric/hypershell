use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::contexts::CliApp;
use hypershell_components::dsl::{SimpleExec, StaticArg, WithStaticArgs};
use hypershell_macro::hypershell;

#[tokio::test]
async fn test_basic_exec() -> Result<(), Error> {
    pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithStaticArgs["hello", "world!"],
        >
    };

    let app = CliApp {};

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    assert_eq!(output, "hello world!\n".as_bytes());

    Ok(())
}
