// This example builds on "Hello, world!" to demonstrate how to use dynamic
// arguments in a Hypershell program.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A `SimpleExec` handler executes the `echo` command.
//
// 2. The `WithArgs` syntax is used to provide a list of arguments. This
//    is different from `WithStaticArgs` as it allows mixing static and
//    dynamic arguments.
//
// 3. The first argument is a `StaticArg` with the value "Hello,".
//
// 4. The second argument is a `FieldArg<"name">`, which dynamically pulls
//    its value from the `name` field of the context running the program.
//
// 5. The output is piped to `StreamToStdout`.
//
// To provide the dynamic `name` argument, a custom `MyApp` context is defined
// with a `name` field. The `#[cgp_inherit]` macro wires it up with the
// `HypershellPreset`, and `#[derive(HasField)]` makes its fields accessible
// to `FieldArg`.
//
// The `main` function creates an instance of `MyApp`, sets the `name`, and
// executes the program.

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

#[cgp_inherit(HypershellPreset)]
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
