use hypershell::prelude::*;

pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithStaticArgs["hello", "world!"],
        >
    |   StreamToStdout
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    HypershellCli
        .handle(PhantomData::<Program>, Vec::new())
        .await?;

    Ok(())
}
