use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{FieldArg, JoinArgs, SimpleExec, StaticArg, WithArgs};

#[cgp_context(TestAppComponents: HypershellAppPreset)]
#[derive(HasField)]
pub struct TestApp {
    pub base_dir: String,
}

#[tokio::test]
async fn test_join_fields() -> Result<(), Error> {
    let app = TestApp {
        base_dir: "../..".to_owned(),
    };

    let output = app
        .handle(
            PhantomData::<
                SimpleExec<
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
                >,
            >,
            Vec::new(),
        )
        .await?;

    println!("output: {}", String::from_utf8(output.stdout).unwrap());

    Ok(())
}
