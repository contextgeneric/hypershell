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
                        FieldArg<Symbol!("base_dir")>,
                        StaticArg<Symbol!("Cargo.toml")>,
                    ],
                >,
            >,
            StreamToBytes,
            SimpleExec<StaticArg<Symbol!("wc")>, WithArgs<Product![StaticArg<Symbol!("-l")>,]>>,
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
