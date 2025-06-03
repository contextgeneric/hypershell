use hypershell::prelude::*;
use hypershell::presets::HypershellPreset;

#[tokio::test]
async fn test_simple_pipe() -> Result<(), Error> {
    #[cgp_context(TestAppComponents: HypershellPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub base_dir: String,
    }

    pub type Program = Pipe<
        Product![
            ReadFile<
                JoinArgs<
                    Product![
                        FieldArg<symbol!("base_dir")>,
                        StaticArg<symbol!("Cargo.toml")>,
                    ],
                >,
            >,
            StreamToBytes,
            SimpleExec<StaticArg<symbol!("wc")>, WithArgs<Product![StaticArg<symbol!("-l")>,]>>,
            BytesToString,
        ],
    >;

    let app = TestApp {
        base_dir: "../..".to_owned(),
    };

    let output = app.handle(PhantomData::<Program>, ()).await?;

    println!("output: {}", output);

    Ok(())
}
