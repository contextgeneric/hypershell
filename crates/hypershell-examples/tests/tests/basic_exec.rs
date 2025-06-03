use hypershell::prelude::*;

#[tokio::test]
async fn test_basic_exec() -> Result<(), Error> {
    pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithStaticArgs["hello", "world!"],
        >
    };

    let app = HypershellCli {};

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    assert_eq!(output, "hello world!\n".as_bytes());

    Ok(())
}
