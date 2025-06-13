// This example is the "Hello, world!" of Hypershell, demonstrating the
// basic execution of a shell command.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A `SimpleExec` handler is used to execute the `echo` command.
//
// 2. The arguments "hello" and "world!" are provided as a variadic list
//    of static arguments using `WithStaticArgs`.
//
// 3. The output of the `echo` command is piped to the `StreamToStdout`
//    handler, which prints it to the console.
//
// The program is executed using `HypershellCli`, a predefined context for
// running simple CLI-only Hypershell programs. No custom context is needed
// because the program only uses static arguments.
//
// The `main` function calls `handle` on `HypershellCli` to run the program.
// An empty `Vec<u8>` is passed as input, which is ignored by `echo`.

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
