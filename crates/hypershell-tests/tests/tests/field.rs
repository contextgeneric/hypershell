use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{FieldArg, FieldArgs, JoinArgs, SimpleExec, StaticArg, WithArgs};
use hypershell_macro::hypershell;

#[tokio::test]
async fn test_join_fields() -> Result<(), Error> {
    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub base_dir: String,
    }

    pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"ls">,
            WithArgs[
                StaticArg<"-la">,
                JoinArgs [
                    FieldArg<"base_dir">,
                    StaticArg<"crates">,
                    StaticArg<"hypershell-apps">,
                ],
            ]
        >
    };

    let app = TestApp {
        base_dir: "../..".to_owned(),
    };

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("output: {}", String::from_utf8(output).unwrap());

    Ok(())
}

#[tokio::test]
async fn test_field_args() -> Result<(), Error> {
    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp<'a> {
        pub args: Vec<&'a str>,
    }

    pub type Program = SimpleExec<StaticArg<symbol!("echo")>, FieldArgs<symbol!("args")>>;

    let app = TestApp {
        args: vec!["hello", "world!"],
    };

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    assert_eq!(output, "hello world!\n".as_bytes());

    Ok(())
}
