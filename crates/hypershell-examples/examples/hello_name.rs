use hypershell::prelude::*;

pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithArgs[
                StaticArg<"Hello,">,
                FieldArg<"name">,
            ],
        >
    |   StreamToStdout
};

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        name: "Alice".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
