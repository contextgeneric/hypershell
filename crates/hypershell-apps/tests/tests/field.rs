use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{FieldArg, FieldArgs, JoinArgs, SimpleExec, StaticArg, WithArgs};

#[tokio::test]
async fn test_join_fields() -> Result<(), Error> {
    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub base_dir: String,
    }

    pub type Program = SimpleExec<
        StaticArg<symbol!("ls")>,
        WithArgs<
            Product![
                StaticArg<symbol!("-la")>,
                JoinArgs<
                    Product![
                        FieldArg<symbol!("base_dir")>,
                        StaticArg<symbol!("crates")>,
                        StaticArg<symbol!("hypershell-apps")>,
                    ],
                >,
            ],
        >,
    >;

    let app = TestApp {
        base_dir: "../..".to_owned(),
    };

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("output: {}", String::from_utf8(output.stdout).unwrap());

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

    assert_eq!(output.stdout, "hello world!\n".as_bytes());
    assert!(output.stderr.is_empty());

    Ok(())
}
